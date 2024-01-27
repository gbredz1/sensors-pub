use crc::Crc;
use log::debug;
use rppal::i2c::I2c;
use serde::*;
use std::error::Error;
use std::thread;
use std::time::Duration;

use crate::sensor::{Measure, SensorMeasureType};
use crate::Sensor;

pub struct AM2320 {
    i2c: I2c,
    buffer: [u8; 8],
    measure_types: Vec<SensorMeasureType>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct AM2320Config {}

impl AM2320 {
    const I2C_ADDR: u16 = 0x5c;
    const CRC: Crc<u16> = Crc::<u16>::new(&crc::CRC_16_MODBUS);

    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut i2c = I2c::new()?;
        i2c.set_slave_address(Self::I2C_ADDR)?;
        let buffer = [0u8; 8];

        let measure_types =
            vec![SensorMeasureType::Humidity, SensorMeasureType::Temperature];

        Ok(AM2320 {
            i2c,
            buffer,
            measure_types,
        })
    }

    fn measure_from(bytes: [u8; 8]) -> Result<Measure, String> {
        let crc = Self::CRC.checksum(&bytes);
        if crc != 0 {
            return Err(format!("CRC error: 0x{:04X}", crc));
        }

        Ok(Measure {
            temperature: Some(
                match (
                    bytes[4] & 0x80 > 0,
                    i16::from_be_bytes([bytes[4] & 0x7F, bytes[5]]) as f64,
                ) {
                    (true, v) => (v * -0.1) as f32,
                    (false, v) => (v * 0.1) as f32,
                },
            ),
            humidity: Some(
                (u16::from_be_bytes([bytes[2], bytes[3]]) as f64 * 0.1) as f32,
            ),
        })
    }
}

impl Sensor for AM2320 {
    fn measure(&mut self) -> Result<Measure, Box<dyn Error>> {
        // AM2320 won't ACK if in sleeping mode
        if self.i2c.write(&[0x00]).is_err() {
            thread::sleep(Duration::from_millis(3)); // wakeup take 0.8...3ms.
        }

        // 0x03 : read
        // 0x00 : from $00
        // 0x04 : to   $04
        self.i2c.write(&[0x03, 0x00, 0x04])?;

        // wait for measure done
        thread::sleep(Duration::from_nanos(1600));

        // read results
        self.i2c.read(&mut self.buffer)?;
        debug!("read: {:02X?}", self.buffer);

        Ok(AM2320::measure_from(self.buffer)?)
    }

    fn measure_types(&self) -> &Vec<SensorMeasureType> {
        &self.measure_types
    }
}

#[cfg(test)]
mod tests {
    use crate::sensor::am2320::AM2320;

    use super::Measure;

    #[test]
    fn am2320_basic_measure() {
        let measure = AM2320::measure_from([
            0x03, 0x04, 0x02, 0x6A, 0x00, 0xD3, 0x91, 0xD1,
        ])
        .unwrap();

        assert_eq!(
            measure,
            Measure {
                humidity: Some(61.8),
                temperature: Some(21.1),
            }
        )
    }

    #[test]
    fn am2320_negative_temperature() {
        let measure = AM2320::measure_from([
            0x03, 0x04, 0x00, 0x00, 0x80, 0x64, 0x91, 0xC3,
        ])
        .unwrap();

        assert_eq!(measure.temperature, Some(-10.0))
    }

    #[test]
    fn am2320_good_crc() {
        assert!(AM2320::measure_from([
            0x03, 0x04, 0x12, 0x34, 0x56, 0x78, 0x8A, 0xDC,
        ])
        .is_ok())
    }

    #[test]
    fn am2320_bad_crc() {
        assert!(AM2320::measure_from([
            0x03, 0x04, 0x12, 0x34, 0x56, 0x78, 0x00, 0x00,
        ])
        .is_err())
    }
}
