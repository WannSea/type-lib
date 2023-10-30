from enum import Enum
from typing import Union

class ParsedMetric:
    name: str
    
    is_str: bool
    is_f32: bool

    def get_camel_case_name(self):
        temp = self.name.split('_')
        return ''.join(ele.title() for ele in temp[0:])

    def __init__(self, name: str, type: Union[str, None]) -> None:
        self.name = name
        self.is_str = type == "String"
        self.is_f32 = type == "f32" or type == None

    def __repr__(self) -> str:
        return f"{self.name}: {self.type}"