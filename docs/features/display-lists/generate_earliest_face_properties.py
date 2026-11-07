"""Complete shared property graphs on both sides of the 24-to-25 font transition."""
import json
from generate_earliest_component_properties import generate as property_root
from generate_early_view_roots import entry_budget as early_entries
from generate_earlier_range import write_group
from earliest_face_patches import convert

VERSIONS = [4600,0x12a2,0x12a3,4800]


def entry_count(version: int, kind: str) -> int:
    return early_entries(version,kind,0)+2-4*int(version<0x1203)


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    return property_root(version,kind,scenario,convert,entry_count)


if __name__=="__main__":
    totals = []
    for label,scenario in [("properties",0),("holders",16)]:
        cases = [generate(v,k,scenario+s) for v in VERSIONS for k in ["part","assembly"] for s in [0,2]]
        totals.append(write_group("earliest-face-"+label,cases))
    print(json.dumps({"fixtures":sum(n for n,_ in totals),"fields":sum(n for _,n in totals)}))
