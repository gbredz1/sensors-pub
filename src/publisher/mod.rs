mod mqtt;
mod stdout_publisher;

use std::error::Error;

use async_trait::async_trait;
pub use mqtt::mqtt_publisher::{MqttPublisher, MqttPublisherConfig};
pub use stdout_publisher::{StdoutPublisher, StdoutPublisherConfig};

use crate::config::ConfigPublisher;
use crate::sensor::Measure;
use crate::{Config, SensorMeasureType};

#[async_trait]
pub trait Publisher {
    async fn publish<'a>(
        &self,
        measure: &Measure,
        sensor_id: &'a str,
    ) -> Result<(), Box<dyn Error>>;

    async fn declare_sensor_measure_type<'a>(
        &self,
        _measure_type: &SensorMeasureType,
        _sensor_id: &'a str,
    ) -> Result<(), Box<dyn Error>>;
}

impl dyn Publisher {
    pub fn new(
        config: &Config,
        publisher: &ConfigPublisher,
    ) -> Result<Box<dyn Publisher>, Box<dyn Error>> {
        match publisher {
            ConfigPublisher::Mqtt(c) => {
                Ok(Box::new(MqttPublisher::create(&config.device, c)))
            }
            ConfigPublisher::Stdout(_) => Ok(Box::new(StdoutPublisher::new())),
        }
    }
}
