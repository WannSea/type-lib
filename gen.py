from gen_py import gen_py
from parsed_metric import ParsedMetric
from gen_rs import gen_rs

def line_to_metric(line: str):
    typed_metric = line.split(":")
    return ParsedMetric(typed_metric[0], typed_metric[1] if len(typed_metric) > 1 else None)

if __name__ == "__main__":
    with open("./metrics.txt", "r") as metrics:
        metric_types = list(map(line_to_metric, filter(lambda x: x and not x.startswith('#'), map(lambda x: x.replace("\n", ""), metrics.readlines()))))

        print(metric_types)
        gen_rs(metric_types)
        gen_py(metric_types)