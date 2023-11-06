from py_markdown_table.markdown_table import markdown_table
import yaml

def fill_in_opt_fields(arr):
    for dic in arr:
        dic.setdefault("description", "")
        dic.setdefault("unit", "")

with open("../metrics.yaml", "r") as file:
    parsed = yaml.safe_load(file)["metrics"]
    fill_in_opt_fields(parsed)
    markdown = markdown_table(parsed).get_markdown()
    
    with open("../Metrics.md", "w") as of:
        of.write(markdown)