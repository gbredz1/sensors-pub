use async_trait::async_trait;
use serde::*;
use std::error::Error;

use crate::sensor::Measure;
use crate::{Publisher, SensorMeasureType};

pub struct StdoutPublisher {}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct StdoutPublisherConfig {}

impl Default for StdoutPublisher {
    fn default() -> Self {
        Self::new()
    }
}

impl StdoutPublisher {
    pub fn new() -> Self {
        StdoutPublisher {}
    }
}

#[async_trait]
impl Publisher for StdoutPublisher {
    async fn publish<'a>(
        &self,
        measure: &Measure,
        sensor_id: &'a str,
    ) -> Result<(), Box<dyn Error>> {
        let payload = serde_yaml::to_string(&measure)?;
        println!("-- {sensor_id} -- \n{payload}");

        Ok(())
    }

    async fn declare_sensor_measure_type<'a>(
        &self,
        measure_type: &SensorMeasureType,
        sensor_id: &'a str,
    ) -> Result<(), Box<dyn Error>> {
        println!("@DECLARE {sensor_id} [{measure_type}]");

        Ok(())
    }
}
