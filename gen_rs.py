import toml
from parsed_metric import ParsedMetric


RUST_OUT = "./rust/src/types.rs"
RUST_ENUM_NAME = "Metric"
CARGO_TOML = "./rust/Cargo.toml"

def get_string_metric_matches_rs(metric_types: list[ParsedMetric]):
    return map(lambda x: f"{RUST_ENUM_NAME}::{x.get_camel_case_name()} => String::from_utf8(value).unwrap()", filter(lambda x: x.type == ParsedMetric.TYPE_STRING, metric_types))

def gen_rs(metric_types: list[ParsedMetric]):

    metrics = [x.get_camel_case_name() for x in metric_types]
    metric_values = [f"{metrics[i]} = {i}" for i in range(len(metric_types))]

    match_str_metrics = map(lambda x: f"\"{x}\" => Ok({RUST_ENUM_NAME}::{x})", metrics)
    match_int_metrics = [f"{i} if {i} == {RUST_ENUM_NAME}::{metrics[i]} as i32 => Ok({RUST_ENUM_NAME}::{metrics[i]})" for i in range(len(metrics))]

    out = f"""use std::fmt;
use std::convert::TryFrom;
    
#[derive(Debug)]
pub enum {RUST_ENUM_NAME} {{
    {",\n    ".join(metric_values)}
}}

impl std::str::FromStr for {RUST_ENUM_NAME} {{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        match s {{
            {",\n            ".join(match_str_metrics)},
            _ => Err(format!("'{{}}' is not a valid value for {RUST_ENUM_NAME}", s)),
        }}
    }}
}}

impl fmt::Display for {RUST_ENUM_NAME} {{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
        write!(f, "{{:?}}", self)
    }}
}}


impl TryFrom<u32> for {RUST_ENUM_NAME} {{
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {{
        match v {{
            {",\n            ".join(match_int_metrics)},
            _ => Err(()),
        }}
    }}
}}

pub fn transform_metric_val(id: {RUST_ENUM_NAME}, value: Vec<u8>) -> String {{
    match id {{
        {",\n     ".join(get_string_metric_matches_rs(metric_types))},
        _ => f32::from_be_bytes(value[0..4].try_into().unwrap()).to_string()
    }}
}}
"""


    with open(RUST_OUT, "w") as f:
        f.write(out)

    with open(CARGO_TOML, "r") as cargo:
        data = toml.load(cargo)
        pkg_version = data["package"]["version"].split(".")
        version = int(pkg_version[2]) + 1
        data["package"]["version"] = f"{pkg_version[0]}.{pkg_version[1]}.{version}"
        with open(CARGO_TOML, "w") as ft:
            ft.write(toml.dumps(data))