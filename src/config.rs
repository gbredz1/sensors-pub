use figment::providers::{Format, Serialized, Yaml};
use figment::Figment;
use serde::*;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use validator::Validate;

use crate::publisher::*;
use crate::sensor::*;
use crate::APP_NAME;

#[derive(Deserialize, Serialize, Debug, Validate, Default)]
#[serde(default)]
pub struct Config {
    pub debug: bool,
    pub interval: ConfigInterval,
    pub device: ConfigDevice,
    pub sensors: HashMap<String, ConfigSensor>,
    pub publishers: HashMap<String, ConfigPublisher>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let config: Config =
            Figment::from(Serialized::defaults(Config::default()))
                .merge(Yaml::file("config.yml"))
                .extract()?;

        match config.validate() {
            Ok(_) => Ok(config),
            Err(e) => Err(Box::new(e)),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConfigDevice {
    pub name: String,
    pub manufacturer: String,
    pub model: String,
}

impl Default for ConfigDevice {
    fn default() -> Self {
        Self {
            name: gethostname::gethostname()
                .to_str()
                .unwrap_or(APP_NAME)
                .into(),
            manufacturer: APP_NAME.into(),
            model: APP_NAME.into(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct ConfigInterval(#[serde(with = "humantime_serde")] Duration);
impl Default for ConfigInterval {
    fn default() -> Self {
        ConfigInterval(Duration::from_secs(480))
    }
}
impl From<ConfigInterval> for Duration {
    fn from(c: ConfigInterval) -> Duration {
        let ConfigInterval(a) = c;
        a
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ConfigSensor {
    #[serde(alias = "faker")]
    Faker(FakerConfig),
    #[serde(alias = "am2320")]
    AM2320(AM2320Config),
    #[serde(alias = "ds18b20")]
    Ds18b20(Ds18b20Config),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ConfigPublisher {
    Mqtt(MqttPublisherConfig),
    Stdout(StdoutPublisherConfig),
}
