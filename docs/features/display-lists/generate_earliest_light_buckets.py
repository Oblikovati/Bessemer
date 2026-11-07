"""Independent populated weld-bead lists before and after their storage boundary."""
import json
from generate_earliest_component_buckets import generate as bucket_root
from generate_earlier_range import write_group
import bucket_vector_wire as wire


def weld_reader(original: object) -> object:
    def read(name: str, dimensions: int, empty: bool) -> bytes:
        if name in {"weldbead_v","weldbead_h"} and not empty:
            return wire.poly_list([wire.poly(1,dimensions,9,False,False)])
        return original(name,dimensions,empty)
    return read


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    variant = scenario%32
    original,labels = wire.main_list,wire.LIST_ORDER
    wire.main_list = weld_reader(original)
    if version<=0x11a5:wire.LIST_ORDER = [label for label in labels if label not in {"weldbead_v","weldbead_h"}]
    try:framed,row = bucket_root(version,kind,variant)
    finally:wire.main_list,wire.LIST_ORDER = original,labels
    if version>0x11a5 and variant%16!=2:row["aggregate_entries"]+=2*(1+(2 if variant>=16 else 3))
    return framed,row


if __name__=="__main__":
    totals = []
    for label,scenario in [("3d",0),("2d",16)]:
        cases = [generate(v,"part",scenario) for v in [4500,0x11a5,0x11a6,4600]]
        cases += [generate(4500,"part",scenario+1),generate(4500,"part",2)]
        totals.append(write_group("earliest-light-buckets-"+label,cases,("displaylists.py",)))
    print(json.dumps({"fixtures":sum(n for n,_ in totals),"fields":sum(n for _,n in totals)}))
