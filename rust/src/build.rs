use std::{path::Path, env};
use codegen::{Scope, Impl, Enum};

static METRIC_ID_ENUM: &str = "MetricId";
static METRIC_TYPES: &'static [&str] = &["String", "u8", "i16", "f32"];

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

fn gen_metric_type_enum() -> Enum {
    let mut new_enum = Enum::new("MetricType");
    let metric_type_enum = new_enum
    .allow("non_camel_case_types")
    .vis("pub")
    .derive("Clone")
    .derive("Debug")
    .derive("Copy");
    for metric_type in METRIC_TYPES {
        metric_type_enum.new_variant(metric_type.to_string());
    }
    return new_enum;
}

fn gen_metric_ids_enum(config: &Config) -> Enum {
    let mut new_enum = Enum::new(METRIC_ID_ENUM);
    let metric_ids_enum = new_enum
    .allow("non_camel_case_types")
    .vis("pub")
    .derive("EnumString")
    .derive("Display")

    .derive("Clone")
    .derive("Debug")
    .derive("Copy");
    for (idx, metric) in config.metrics.iter().enumerate() {
        metric_ids_enum.new_variant(format!("{} = {}", metric.name, idx));
    }
    return new_enum;
}

fn gen_metric_id_impl(config: &Config) -> Impl {
    let mut new_impl = Impl::new(METRIC_ID_ENUM);
    let metrics_impl_fn = new_impl
    .new_fn("get_type")
    .arg_ref_self()
    .ret("MetricType")
    .vis("pub")
    .line("match self {");
    for metric in &config.metrics {
        metrics_impl_fn.line(format!("{}::{} => MetricType::{},", METRIC_ID_ENUM, metric.name, metric.metric_type));
    }
    metrics_impl_fn.line("}");
    return new_impl;
}

// ToDo: move to build script
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");

    let f = std::fs::File::open("../metrics.yaml")?;
    let d: Config = serde_yaml::from_reader(f)?;

    let mut scope = Scope::new();
    scope.import("strum_macros", "EnumString");
    scope.import("strum_macros", "Display");

    scope.push_enum(gen_metric_type_enum());
    scope.push_enum(gen_metric_ids_enum(&d));
    scope.push_impl(gen_metric_id_impl(&d));
   
    std::fs::write(path, scope.to_string())?;

    Ok(())
}