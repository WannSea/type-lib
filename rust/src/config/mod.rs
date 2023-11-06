use core::fmt;

pub mod generated;

#[derive(Debug, serde::Deserialize, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MetricType {
    String,
    f32
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct MetricDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub metric_type: MetricType
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Config {
    pub metrics: Vec<MetricDefinition>
}

impl fmt::Display for MetricType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}