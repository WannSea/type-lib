import toml

RUST_OUT = "./rust/src/types.rs"
CARGO_TOML = "./rust/Cargo.toml"

PY_OUT = "./python/types.py"
PYPROJECT_TOML = "./python/pyproject.toml"

def snake_to_camel_case(input):
    temp = input.split('_')
    return ''.join(ele.title() for ele in temp[0:])

def gen_rs(metric_types):
    out = "pub enum Metric {\n"

    for i in range(len(metric_types)):
        out += f"    {snake_to_camel_case(metric_types[i])} = {i},\n"

    out += "}"

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