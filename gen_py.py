import toml
from parsed_metric import ParsedMetric


PY_OUT = "./python/metric.py"
PYPROJECT_TOML = "./python/pyproject.toml"

def gen_py(metric_types: list[ParsedMetric]):
    out = f"""import struct
class Metric:
    {"\n    ".join([f"{metric_types[i].name} = {i}" for i in range (len(metric_types))])}

METRIC_TYPE_VALUES =  {{value: name for name, value in vars(Metric).items() if name.isupper()}}

def get_metric_val(id, val):
    {"\n".join([f"if id == Metric.{x.name}:\n        return val.decode(\"utf-8\")" for x in filter(lambda x: x.is_str, metric_types)])}
    else:
        return struct.unpack('<f', val)[0]
"""

    with open(PY_OUT, "r") as f:
        if f.read() == out:
            print("No changes in python lib. Leaving as is.")
            return

    with open(PY_OUT, "w") as f:
        f.write(out)
        
        with open(PYPROJECT_TOML, "r") as pyproject:
            data = toml.load(pyproject)
            project_version = data["project"]["version"].split(".")
            version = int(project_version[2]) + 1
            new_version = f"{project_version[0]}.{project_version[1]}.{version}"
            data["project"]["version"] = new_version
            with open(PYPROJECT_TOML, "w") as ft:
                ft.write(toml.dumps(data))

            print(f"Updated python lib to {new_version}")

            