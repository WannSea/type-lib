from py_markdown_table.markdown_table import markdown_table
from proto_schema_parser.parser import Parser
import yaml

def fill_in_opt_fields(arr):
    for dic in arr:
        dic.setdefault("description", "")
        dic.setdefault("unit", "")

def get_option_val(el, name):
    for option in el.options:
        if option.name == name:
            return option.value
    return ""

with open("./protos/wannsea.proto", "r") as file:
    metricList = []

    ast = Parser().parse(file.read())
    for el in ast.file_elements:
        # Find Enum
        if hasattr(el, "name") and el.name == "MessageId":
            for enumVal in el.elements:
                metricList.append({ "id": str(enumVal.number), "name": enumVal.name, "description": get_option_val(enumVal, "(description)"), "unit": get_option_val(enumVal, "(unit)") })


    markdown = markdown_table(metricList).set_params(row_sep = 'markdown', quote=False, emoji_spacing="mono").get_markdown()
    with open("./Metrics.md", "w") as of:
        of.write(markdown)
    print("Successfully generated markdown")