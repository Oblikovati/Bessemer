"""Independent shared property graphs at the 25-to-26 embedded font boundary."""
import json
from functools import partial
from generate_earlier_range import write_group
from generate_earliest_scene_roots import raw_nan_reader
from earliest_component_patches import convert
from swarchive import Archive
import generate_component_properties as properties

VERSIONS = [4800,0x12ce,0x12cf,4850]


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    original,original_float = properties.convert,Archive.f32
    properties.convert = partial(original,base_converter=convert)
    Archive.f32 = raw_nan_reader(original_float)
    try:return properties.generate(version,kind,scenario)
    finally:properties.convert,Archive.f32 = original,original_float


if __name__=="__main__":
    totals = []
    for label,scenario in [("properties",0),("holders",16)]:
        cases = [generate(v,k,scenario+s) for v in VERSIONS for k in ["part","assembly"] for s in [0,2]]
        totals.append(write_group("earliest-component-"+label,cases))
    print(json.dumps({"fixtures":sum(n for n,_ in totals),"fields":sum(n for _,n in totals)}))
