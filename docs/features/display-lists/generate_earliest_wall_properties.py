"""Shared property/holder graphs with old wall and appearance layouts."""
import json
from generate_earliest_component_properties import generate as property_root
from generate_earliest_face_properties import entry_count
from generate_earlier_range import write_group
from earliest_wall_patches import convert


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    return property_root(version,kind,scenario,convert,entry_count)


if __name__=="__main__":
    cases = [generate(v,k,s) for v in [4400,0x1165,0x117d,0x117f,4500] for k in ["part","assembly"] for s in [0,16]]
    count,fields = write_group("earliest-wall-properties",cases,("displaylists.py",))
    print(json.dumps({"fixtures":count,"fields":fields}))
