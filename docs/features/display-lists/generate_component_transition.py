"""Complete part and assembly roots at each 10000-range component/facet boundary."""
import json
from generate_earlier_range import converted as convert_root,generate as generate_root,write_group
from generate_precollector_range import entry_budget as previous_entries
from component_transition_patches import convert
from transient_viewer_wire import populate,transient_trace

VERSIONS = sorted({10000,10001,0x2789,0x278a,0x2791,0x2792,0x2794,0x2795,0x279c,0x279d,0x279e,0x279f,0x27a0,0x27ad,0x27ae,0x27b1,0x27b2,0x27b5,0x27b6,0x27b7,0x27c1,0x27c2,0x27e0,0x27e1,0x27e2,0x27e3,10999,11000})


def converted(document: object, path: object, raw: bytes, version: int, kind: str, variant: int) -> bytes:
    output = convert_root(document,path,raw,version,kind,variant,convert,b"" if kind=="part" else None)
    return populate(document,path,output,version,variant==7) if variant in (6,7) else output


def entry_budget(version: int, kind: str, variant: int) -> int:
    return previous_entries(version,kind,0)-4*(version<=0x2791)-4*(version<=0x27ad)+5*(variant in (6,7))


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    return generate_root(version,kind,scenario,converted,entry_budget,transient_trace if kind=="assembly" else None)


def main() -> None:
    totals = []
    for kind in ["part","assembly"]:
        cases = [generate(version,kind) for version in VERSIONS]+[generate(10000,kind,1)]
        totals.append(write_group(f"component-transition-{kind}",cases))
    cases = [generate(version,"assembly",scenario) for version in [0x27c2,0x27e2] for scenario in [6,7]]
    totals.append(write_group("transient-viewers",cases))
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))


if __name__=="__main__":
    main()
