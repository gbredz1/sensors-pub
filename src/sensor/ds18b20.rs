use std::error::Error;
use std::fs;

use log::debug;
use serde::{Deserialize, Serialize};

use crate::{Measure, Sensor, SensorMeasureType};

pub struct Ds18b20 {
    measure_types: Vec<SensorMeasureType>,
    path: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct Ds18b20Config {
    pub identifier: String,
}

impl Ds18b20 {
    const ONE_WIRE_FAMILY: u16 = 0x28;

    pub fn new(config: &Ds18b20Config) -> Result<Self, Box<dyn Error>> {
        let measure_types = vec![SensorMeasureType::Temperature];
        let path = format!(
            "/sys/bus/w1/devices/{:x}-{}",
            Ds18b20::ONE_WIRE_FAMILY,
            config.identifier
        );

        debug!("Check if device exist at: {path}");
        let _ = fs::metadata(&path)?;

        Ok(Self {
            measure_types,
            path,
        })
    }

    fn measure_from(string: &str) -> Result<Measure, Box<dyn Error>> {
        let string = string.strip_suffix('\n').unwrap_or(string);
        let temperature = string.parse::<isize>()? as f64 * 0.001;

        Ok(Measure {
            temperature: Some(temperature as f32),
            ..Default::default()
        })
    }
}

impl Sensor for Ds18b20 {
    fn measure(&mut self) -> Result<crate::Measure, Box<dyn Error>> {
        let string = fs::read_to_string(format!("{}/temperature", self.path))?;

        debug!("string: {string:?}");

        let measure = Ds18b20::measure_from(&string)?;

        Ok(measure)
    }

    fn measure_types(&self) -> &Vec<crate::SensorMeasureType> {
        &self.measure_types
    }
}

#[cfg(test)]
mod tests {
    use crate::Measure;

    use super::Ds18b20;

    #[test]
    fn ds18b20_basic_measure() {
        let measure = Ds18b20::measure_from("20250\n").unwrap();

        assert_eq!(
            measure,
            Measure {
                temperature: Some(20.25),
                ..Default::default()
            }
        )
    }

    #[test]
    fn ds18b20_negative_measure() {
        let measure = Ds18b20::measure_from("-5000\n").unwrap();

        assert_eq!(
            measure,
            Measure {
                temperature: Some(-5.0),
                ..Default::default()
            }
        )
    }
}
