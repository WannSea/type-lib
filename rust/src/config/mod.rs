pub mod generated;

#[derive(Debug, serde::Deserialize, PartialEq, strum_macros::Display)]
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