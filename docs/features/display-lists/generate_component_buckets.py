"""Complete bucket roots at each 10000-range scalar boundary."""
import json
from generate_component_transition import VERSIONS
from generate_earlier_buckets import generate
from generate_earlier_range import write_group


def main() -> None:
    totals = []
    for two_dimensional in [False,True]:
        cases = [generate(version,"part",16 if two_dimensional else 0) for version in VERSIONS]+[generate(10000,"part",17 if two_dimensional else 2)]
        scope = "2d" if two_dimensional else "3d"
        totals.append(write_group(f"component-buckets-{scope}",cases))
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))


if __name__=="__main__":
    main()
