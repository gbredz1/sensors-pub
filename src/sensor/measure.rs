use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Default, Clone, Copy)]
pub struct Measure {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub humidity: Option<f32>,
}
