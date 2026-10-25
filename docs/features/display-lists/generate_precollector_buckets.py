"""Reuse reviewed pre-pointer bucket wire records throughout the remaining 11000 interval."""
import json
from generate_precollector_range import VERSIONS
from generate_earlier_buckets import generate
from generate_earlier_range import write_group


def main() -> None:
    totals = []
    for two_dimensional in [False,True]:
        scenario = 16 if two_dimensional else 0
        cases = [generate(version,"part",scenario) for version in VERSIONS]+[generate(11000,"part",17 if two_dimensional else 2)]
        scope = "2d" if two_dimensional else "3d"
        totals.append(write_group(f"precollector-buckets-{scope}",cases))
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))


if __name__=="__main__":
    main()
