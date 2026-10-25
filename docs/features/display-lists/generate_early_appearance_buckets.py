"""Independent complete buckets across the cross-hatch scalar gate."""
from generate_early_appearance_roots import VERSIONS
from generate_historical_buckets import generate as historical_bucket
from generate_earlier_range import write_group
import json


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    variant = scenario%32
    framed,row = historical_bucket(version,kind,variant)
    row["scenario"] = 128+variant
    return framed[:5]+bytes([128+variant])+framed[6:],row


if __name__=="__main__":
    totals = []
    for dimension in ["3d","2d"]:
        cases = [generate(v,"part",16 if dimension=="2d" else 0) for v in VERSIONS]
        cases += [generate(5000,"part",17 if dimension=="2d" else 2)]
        totals.append(write_group("early-appearance-buckets-"+dimension,cases))
    print(json.dumps({"fixtures":sum(n for n,_ in totals),"fields":sum(n for _,n in totals)}))
