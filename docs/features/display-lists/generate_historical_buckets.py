"""Independent complete buckets across the historical halo-repair gate."""
from generate_historical_roots import VERSIONS
from generate_early_view_buckets import generate as earlier_bucket
from generate_earlier_range import write_group
import json


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    variant = scenario%32
    framed,row = earlier_bucket(version,kind,variant)
    row["scenario"] = 96+variant
    row["aggregate_entries"] -= 4*int(version<=0x184c and variant!=2)
    return framed[:5]+bytes([96+variant])+framed[6:],row


if __name__=="__main__":
    totals = []
    for dimension in ["3d","2d"]:
        cases = [generate(v,"part",16 if dimension=="2d" else 0) for v in VERSIONS]
        cases += [generate(6000,"part",17 if dimension=="2d" else 2)]
        totals.append(write_group("historical-buckets-"+dimension,cases))
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))
