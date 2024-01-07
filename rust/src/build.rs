use std::{path::Path, env};
use codegen::{Scope, Enum};

static METRIC_ID_ENUM: &str = "MetricId";

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct MetricDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub metric_type: String
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Config {
    pub metrics: Vec<MetricDefinition>
}

fn gen_metric_ids_enum(config: &Config) -> Enum {
    let mut new_enum = Enum::new(METRIC_ID_ENUM);
    let metric_ids_enum = new_enum
    .allow("non_camel_case_types")
    .vis("pub")
    .derive("EnumString")
    .derive("FromRepr")
    .derive("Display")

    .derive("Clone")
    .derive("Debug")
    .derive("Copy");
    for (idx, metric) in config.metrics.iter().enumerate() {
        metric_ids_enum.new_variant(format!("{} = {}", metric.name, idx));
    }
    return new_enum;
}

// ToDo: move to build script
fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(&["src/wannsea.proto"], &["src/"])?;

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");

    let f = std::fs::File::open("../metrics.yaml")?;
    let d: Config = serde_yaml::from_reader(f)?;

    let mut scope = Scope::new();
    scope.import("strum_macros", "EnumString");
    scope.import("strum_macros", "Display");
    scope.import("strum_macros", "FromRepr");

    scope.push_enum(gen_metric_ids_enum(&d));
   
    std::fs::write(path, scope.to_string())?;

    Ok(())
}