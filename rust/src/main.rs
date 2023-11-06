use codegen::Scope;
pub mod config;

static METRIC_ID_ENUM: &str = "MetricId";
use crate::config::Config;


fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("../metrics.yaml")?;
    let d: Config = serde_yaml::from_reader(f)?;

    let mut scope = Scope::new();
    scope.import("super", "MetricType");

    let metric_ids_enum = scope.new_enum(METRIC_ID_ENUM)
    .allow("non_camel_case_types")
    .vis("pub")
    .derive("EnumString")
    .derive("Clone")
    .derive("Debug")
    .derive("Copy");
    for (idx, metric) in d.metrics.iter().enumerate() {
        metric_ids_enum.new_variant(format!("{} = {}", metric.name, idx));
    }

    let metrics_impl_fn = scope.new_impl(METRIC_ID_ENUM)
    .new_fn("get_type")
    .arg_ref_self()
    .ret("MetricType")
    .vis("pub")
    .line("match self {");
    for metric in &d.metrics {
        metrics_impl_fn.line(format!("{}::{} => MetricType::{},", METRIC_ID_ENUM, metric.name, metric.metric_type));
    }
    metrics_impl_fn.line("}");
    

    std::fs::write("./src/config/generated.rs", scope.to_string())?;

    Ok(())
}