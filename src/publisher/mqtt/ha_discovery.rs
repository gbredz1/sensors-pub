use serde::Serialize;

use crate::config::ConfigDevice;
use crate::publisher::mqtt::MQTT_STATE_TOPIC_BASE;
use crate::sensor::SensorMeasureType;

fn secure_mqtt_topic_name(string: &str) -> String {
    string
        .to_lowercase()
        .replace(&['(', ')', ',', '"', '\''][..], "")
        .replace(&['#', '+', '*', '\\', '/'][..], "_")
        .replace(' ', "-")
}

#[derive(Debug, Serialize, Default, Clone, PartialEq, Eq)]
pub struct HADevice {
    name: String,
    manufacturer: String,
    model: String,
    identifiers: Vec<String>,
}

impl HADevice {
    pub fn new(device: &ConfigDevice) -> Self {
        Self {
            name: device.name.clone(),
            manufacturer: device.manufacturer.clone(),
            model: device.model.clone(),
            identifiers: vec![device.name.clone()],
        }
    }

    pub fn get_mqtt_state_topic(&self, sensor_name: &str) -> String {
        format!(
            "{}/{}/{}/state",
            MQTT_STATE_TOPIC_BASE,
            secure_mqtt_topic_name(&self.name),
            secure_mqtt_topic_name(sensor_name),
        )
    }
}

#[derive(Debug, Serialize, Default, PartialEq, Eq)]
pub struct HASensor {
    name: String,
    device_class: &'static str,
    state_class: &'static str,
    unit_of_measurement: &'static str,
    state_topic: String,
    unique_id: String,
    value_template: String,
    device: HADevice,
}

impl HASensor {
    pub fn new(
        measure: &SensorMeasureType,
        device: HADevice,
        sensor_id: &str,
    ) -> Self {
        let name = measure.name();
        let device_class = measure.device_class();
        let state_class = measure.state_class();
        let unit_of_measurement = measure.unit_of_measurement();
        let state_topic = device.get_mqtt_state_topic(sensor_id);
        let unique_id =
            format!("{}_{}", sensor_id, measure.name().to_lowercase());
        let value_template =
            format!("{{{{ value_json.{} }}}}", measure.name().to_lowercase());

        Self {
            name,
            device_class,
            state_class,
            unit_of_measurement,
            state_topic,
            unique_id,
            value_template,
            device,
        }
    }
}

trait HASensorInfo {
    fn name(&self) -> String;
    fn device_class(&self) -> &'static str;
    fn state_class(&self) -> &'static str;
    fn unit_of_measurement(&self) -> &'static str;
}

impl HASensorInfo for SensorMeasureType {
    fn name(&self) -> String {
        self.to_string()
    }

    fn device_class(&self) -> &'static str {
        match self {
            SensorMeasureType::Temperature => "temperature",
            SensorMeasureType::Humidity => "humidity",
        }
    }

    fn state_class(&self) -> &'static str {
        match self {
            SensorMeasureType::Temperature => "measurement",
            SensorMeasureType::Humidity => "measurement",
        }
    }

    fn unit_of_measurement(&self) -> &'static str {
        match self {
            SensorMeasureType::Temperature => "°C",
            SensorMeasureType::Humidity => "%",
        }
    }
}

#[derive(Debug, Serialize, Default, PartialEq, Eq)]
pub struct HADiscovery {
    pub payload: HASensor,
    pub topic: String,
}

impl HADiscovery {
    pub fn new(
        measure: &SensorMeasureType,
        sensor_id: &str,
        device: HADevice,
    ) -> Self {
        let sensor = HASensor::new(measure, device, sensor_id);
        let topic = format!(
            "homeassistant/sensor/{}_{}_{}_{}/config",
            crate::APP_NAME,
            secure_mqtt_topic_name(&sensor.device.name),
            secure_mqtt_topic_name(sensor_id),
            secure_mqtt_topic_name(&measure.name()),
        );

        Self {
            payload: sensor,
            topic,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::ConfigDevice;
    use crate::publisher::mqtt::ha_discovery::{
        secure_mqtt_topic_name, HADiscovery,
    };
    use crate::publisher::mqtt::MQTT_STATE_TOPIC_BASE;
    use crate::sensor::SensorMeasureType;

    use super::{HADevice, HASensor};
    use rumqttc::valid_topic;

    #[test]
    fn topic_from_devicename() {
        for s in [
            "wrong+".to_string(),
            "wro#ng".into(),
            "w/r/o/n/g+".into(),
            "wrong/#/path".into(),
        ] {
            assert!(valid_topic(&secure_mqtt_topic_name(&s)));
        }
    }

    #[test]
    fn simple_device() {
        let device = HADevice::new(&ConfigDevice {
            name: "Device Name".into(),
            manufacturer: "My Manufacturer".into(),
            model: "Model XYZ".into(),
        });
        assert_eq!(
            device,
            HADevice {
                name: "Device Name".into(),
                manufacturer: "My Manufacturer".into(),
                model: "Model XYZ".into(),
                identifiers: vec!["Device Name".into()]
            }
        )
    }

    #[test]
    fn sensor_with_name_device_invalid() {
        let device = HADevice {
            name: "W*ird+ma/hine#N@me".into(),
            manufacturer: "My Manufacturer".into(),
            model: "Model XYZ".into(),
            identifiers: vec!["Device Name".into()],
        };

        let sensor_id = String::from("sensor-001");
        let measure = SensorMeasureType::Humidity;
        let sensor = HASensor::new(&measure, device.clone(), &sensor_id);

        assert_eq!(
            sensor,
            HASensor {
                name: "Humidity".into(),
                device_class: "humidity",
                state_class: "measurement",
                unit_of_measurement: "%",
                state_topic: format!(
                    "{}/w_ird_ma_hine_n@me/sensor-001/state",
                    MQTT_STATE_TOPIC_BASE
                ),
                unique_id: "sensor-001_humidity".into(),
                value_template: r"{{ value_json.humidity }}".into(),
                device: device,
            },
        );
    }

    #[test]
    fn discovery_with_name_device_invalid() {
        let device = HADevice {
            name: "W*ird+ma/hine#N@me".into(),
            manufacturer: "My Manufacturer".into(),
            model: "Model XYZ".into(),
            identifiers: vec!["Device Name".into()],
        };

        let sensor_id = String::from("sensor-001");
        let measure = SensorMeasureType::Humidity;
        let discovery = HADiscovery::new(&measure, &sensor_id, device.clone());

        assert_eq!(
        discovery.topic,
        "homeassistant/sensor/sensors-pub_w_ird_ma_hine_n@me_sensor-001_humidity/config"
            .to_string()
    );
    }
}
