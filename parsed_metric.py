from enum import Enum
from typing import Union

class ParsedMetric:
    TYPE_F32 = "f32"
    TYPE_STRING = "String"

    name: str
    type: str

    def get_camel_case_name(self):
        temp = self.name.split('_')
        return ''.join(ele.title() for ele in temp[0:])

    def __init__(self, name: str, type: Union[str, None]) -> None:
        self.name = name
        self.type = type if type else self.TYPE_F32

    def __repr__(self) -> str:
        return f"{self.name}: {self.type}"