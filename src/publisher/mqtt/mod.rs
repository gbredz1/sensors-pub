use crate::APP_NAME;

pub(crate) mod ha_discovery;
pub(crate) mod mqtt_publisher;

pub const MQTT_STATE_TOPIC_BASE: &str = APP_NAME;
