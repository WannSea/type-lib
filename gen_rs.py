import toml
from parsed_metric import ParsedMetric


RUST_OUT = "./rust/src/types.rs"
RUST_ENUM_NAME = "Metric"
CARGO_TOML = "./rust/Cargo.toml"

def get_metric_enum(metric):
    return f"{RUST_ENUM_NAME}::{metric}"

def gen_rs(metric_types: list[ParsedMetric]):

    metrics = [x.get_camel_case_name() for x in metric_types]
    metric_values = [f"{metrics[i]} = {i}" for i in range(len(metric_types))]

    match_str_metrics = map(lambda x: f"\"{x}\" => Ok({get_metric_enum(x)})", metrics)
    match_int_metrics = [f"{i} => Ok({get_metric_enum(metrics[i])})" for i in range(len(metrics))]

    out = f"""use std::fmt;
use std::convert::TryFrom;
    
#[derive(Debug, Clone)]
pub enum {RUST_ENUM_NAME} {{
    {",\n    ".join(metric_values)}
}}

// string to enum
impl std::str::FromStr for {RUST_ENUM_NAME} {{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        match s {{
            {",\n            ".join(match_str_metrics)},
            _ => Err(format!("'{{}}' is not a valid value for {RUST_ENUM_NAME}", s)),
        }}
    }}
}}

// enum to string representation
impl fmt::Display for {RUST_ENUM_NAME} {{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
        write!(f, "{{:?}}", self)
    }}
}}

// u8 to enum
impl TryFrom<u8> for {RUST_ENUM_NAME} {{
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {{
        match v {{
            {",\n            ".join(match_int_metrics)},
            _ => Err(()),
        }}
    }}
}}

impl {RUST_ENUM_NAME} {{
    pub fn val_f32(value: f32) -> Vec<u8> {{
        value.to_be_bytes().to_vec()
    }}

    pub fn val_str(value: &str) -> Vec<u8> {{
        value.as_bytes().to_vec()
    }}

    pub fn to_f32(value: Vec<u8>) -> f32 {{
        f32::from_ne_bytes(value[0..4].try_into().unwrap())
    }}

    pub fn to_string(value: Vec<u8>) -> String {{
        String::from_utf8(value).unwrap()
    }}

    // transform enum value to json
    pub fn get_val_as_json(&self, value: Vec<u8>) -> String {{
        match self {{
            {",\n".join(map(lambda x: f"""{get_metric_enum(x.get_camel_case_name())} => format!("\\"{{}}\\"", Self::to_string(value))""", filter(lambda x: x.is_str, metric_types)))},
            _ => Self::to_f32(value).to_string()
        }}
    }}
}}
"""

    with open(RUST_OUT, "r") as f:
        if f.read() == out:
            print("No changes in rust lib. Leaving as is.")
            return
        
    with open(RUST_OUT, "w") as f:
        f.write(out)

        with open(CARGO_TOML, "r") as cargo:
            data = toml.load(cargo)
            pkg_version = data["package"]["version"].split(".")
            version = int(pkg_version[2]) + 1
            new_version = f"{pkg_version[0]}.{pkg_version[1]}.{version}"
            data["package"]["version"] = new_version
            with open(CARGO_TOML, "w") as ft:
                ft.write(toml.dumps(data))
            print(f"Updated rust lib to {new_version}")
