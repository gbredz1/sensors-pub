use async_trait::async_trait;
use log::{debug, error, trace};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde::*;
use std::error::Error;
use std::time::Duration;

use super::ha_discovery::{HADevice, HADiscovery};
use crate::config::ConfigDevice;
use crate::sensor::Measure;
use crate::{Publisher, SensorMeasureType, APP_NAME};

pub struct MqttPublisher {
    client: AsyncClient,
    ha_device: HADevice,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct MqttPublisherConfig {
    pub client_id: String,
    pub host: String,
    pub port: u16,
}
impl Default for MqttPublisherConfig {
    fn default() -> Self {
        Self {
            client_id: gethostname::gethostname()
                .to_str()
                .unwrap_or(APP_NAME)
                .into(),
            host: "localhost".into(),
            port: 1883,
        }
    }
}

impl MqttPublisher {
    pub fn create(device: &ConfigDevice, mqtt: &MqttPublisherConfig) -> Self {
        let mut mqttoptions =
            MqttOptions::new(&mqtt.client_id, &mqtt.host, mqtt.port);
        mqttoptions.set_keep_alive(Duration::from_secs(15));

        let ha_device = HADevice::new(device);

        let (client, mut event_loop) = AsyncClient::new(mqttoptions, 10);
        tokio::spawn(async move {
            trace!("event loop started");
            loop {
                let event = event_loop.poll().await;
                match event {
                    Ok(event) => {
                        trace!("{event:?}");
                    }
                    Err(err) => {
                        error!("{err}");
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        });

        Self { client, ha_device }
    }
}

#[async_trait]
impl Publisher for MqttPublisher {
    async fn publish<'a>(
        &self,
        measure: &Measure,
        sensor_id: &'a str,
    ) -> Result<(), Box<dyn Error>> {
        let topic = self.ha_device.get_mqtt_state_topic(sensor_id);
        let payload = serde_json::to_string(&measure)?;

        debug!("mqtt publish: {} => {}", topic, payload);

        self.client
            .publish(&topic, QoS::AtLeastOnce, false, payload)
            .await?;

        Ok(())
    }

    async fn declare_sensor_measure_type<'a>(
        &self,
        measure_type: &SensorMeasureType,
        sensor_id: &'a str,
    ) -> Result<(), Box<dyn Error>> {
        let discovery =
            HADiscovery::new(measure_type, sensor_id, self.ha_device.clone());

        let payload = serde_json::to_string(&discovery.payload)?;
        self.client
            .clone()
            .publish(discovery.topic, QoS::AtLeastOnce, true, payload)
            .await?;

        Ok(())
    }
}
