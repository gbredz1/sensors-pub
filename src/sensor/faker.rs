use rand::Rng;
use serde::*;
use std::error::Error;

use crate::sensor::{Measure, SensorMeasureType};
use crate::Sensor;

pub struct Faker {
    measure: Measure,
    measure_types: Vec<SensorMeasureType>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct FakerConfig {
    pub measures: Vec<SensorMeasureType>,
}

impl Default for FakerConfig {
    fn default() -> Self {
        Self {
            measures: vec![
                SensorMeasureType::Temperature,
                SensorMeasureType::Humidity,
            ],
        }
    }
}

impl Faker {
    pub fn new(config: &FakerConfig) -> Self {
        let mut measure = Measure {
            ..Default::default()
        };

        let mut rng = rand::thread_rng();

        let mut measure_types = Vec::new();

        config.measures.iter().for_each(|t| match t {
            SensorMeasureType::Temperature => {
                measure.temperature = Some(rng.gen_range(19.0..25.0));
                measure_types.push(SensorMeasureType::Temperature);
            }
            SensorMeasureType::Humidity => {
                measure.humidity = Some(rng.gen_range(40.0..60.0));
                measure_types.push(SensorMeasureType::Humidity);
            }
        });

        Faker {
            measure,
            measure_types,
        }
    }
}

impl Sensor for Faker {
    fn measure(&mut self) -> Result<Measure, Box<dyn Error>> {
        let mut rng = rand::thread_rng();

        if let Some(ref mut h) = self.measure.humidity {
            *h += rng.gen_range(-2.0..2.0);
            *h = (*h).clamp(0.0, 100.0);
            *h = (*h * 100.0).round() / 100.0; // round at 2 decimal
        }
        if let Some(ref mut t) = self.measure.temperature {
            *t += rng.gen_range(-2.0..2.0);
            *t = (*t).clamp(-20.0, 40.0);
            *t = (*t * 100.0).round() / 100.0; // round at 2 decimal
        }

        Ok(self.measure)
    }

    fn measure_types(&self) -> &Vec<SensorMeasureType> {
        &self.measure_types
    }
}
