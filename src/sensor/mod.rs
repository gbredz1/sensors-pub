use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;

use crate::config::ConfigSensor;

pub use self::am2320::{AM2320Config, AM2320};
pub use self::ds18b20::{Ds18b20, Ds18b20Config};
pub use self::faker::{Faker, FakerConfig};
pub use self::measure::Measure;

mod am2320;
pub mod ds18b20;
mod faker;
mod measure;

pub trait Sensor {
    fn measure(&mut self) -> Result<Measure, Box<dyn Error>>;
    fn measure_types(&self) -> &Vec<SensorMeasureType>;
}

impl dyn Sensor {
    pub fn new(config: &ConfigSensor) -> Box<dyn Sensor> {
        match config {
            ConfigSensor::AM2320(_cfg) => {
                Box::new(AM2320::new().expect("I2C is enabled ?"))
            }
            ConfigSensor::Faker(cfg) => Box::new(Faker::new(cfg)),
            ConfigSensor::Ds18b20(cfg) => {
                Box::new(Ds18b20::new(cfg).expect("1-Wire is enabled ?"))
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SensorMeasureType {
    Temperature,
    Humidity,
}

impl Display for SensorMeasureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
