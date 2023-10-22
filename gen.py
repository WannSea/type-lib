import toml

RUST_OUT = "./rust/src/types.rs"
RUST_ENUM_NAME = "Metric"
CARGO_TOML = "./rust/Cargo.toml"

PY_OUT = "./python/metric.py"
PYPROJECT_TOML = "./python/pyproject.toml"

def snake_to_camel_case(input):
    temp = input.split('_')
    return ''.join(ele.title() for ele in temp[0:])

def gen_rs(metric_types):

    metrics = [snake_to_camel_case(x) for x in metric_types]
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

def gen_py(metric_types):
    out = "class Metric:\n"
    for i in range(len(metric_types)):
        out += f"    {metric_types[i]} = {i}\n"

    out += f"\n\nMETRIC_TYPE_VALUES =  {{value: name for name, value in vars(MetricTypes).items() if name.isupper()}}"

    with open(PY_OUT, "w") as f:
        f.write(out)

    with open(PYPROJECT_TOML, "r") as pyproject:
        data = toml.load(pyproject)
        project_version = data["project"]["version"].split(".")
        version = int(project_version[2]) + 1
        data["project"]["version"] = f"{project_version[0]}.{project_version[1]}.{version}"
        with open(PYPROJECT_TOML, "w") as ft:
            ft.write(toml.dumps(data))

if __name__ == "__main__":
    with open("./metrics.txt", "r") as metrics:
        metric_types = list(filter(None, map(lambda x: x.replace("\n", ""), metrics.readlines())))
        gen_rs(metric_types)
        gen_py(metric_types)