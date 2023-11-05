import yaml

with open("./metrics.yaml") as file:
    t = yaml.safe_load(file)

    out = ""
    print (t["metrics"])
    for m in t["metrics"]:
        m["type"] = "f32"
        out += f"- {{ name: {m["name"]}, type: {m["type"]} }}\n"

    with open("./transform.txt", "w") as of:
        of.write(out)