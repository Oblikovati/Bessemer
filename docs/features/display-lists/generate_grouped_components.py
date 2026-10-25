"""Complete 9000-range roots at grouped-folder, scene, appearance and footer gates."""
import json
from generate_earlier_range import converted as convert_root,generate as generate_root,write_group
from generate_component_transition import entry_budget as previous_entries
from precollector_configuration_wire import configurations,configuration_entries
from grouped_component_patches import convert

VERSIONS = sorted({9000,9001,0x238c,0x238d,0x239c,0x239d,0x23b3,0x23b4,0x23cd,0x23ce,0x23d7,0x23d8,0x23e4,0x23e5,0x23fe,0x23ff,0x2403,0x2404,0x240e,0x240f,0x2419,0x241a,9999,10000})


def converted(document: object, path: object, raw: bytes, version: int, kind: str, variant: int) -> bytes:
    configuration_bytes = b"" if kind=="part" or version<0x2404 else None
    if variant==4:configuration_bytes = configurations(version,kind)
    return convert_root(document,path,raw,version,kind,variant,convert,configuration_bytes)


def entry_budget(version: int, kind: str, variant: int) -> int:
    return previous_entries(version,kind,0)+(configuration_entries(version,kind) if variant==4 else 0)


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    return generate_root(version,kind,scenario,converted,entry_budget)


def main() -> None:
    totals = []
    for kind in ["part","assembly"]:
        cases = [generate(version,kind) for version in VERSIONS]+[generate(9000,kind,1)]
        totals.append(write_group(f"grouped-component-{kind}",cases))
    cases = [generate(version,"assembly",4) for version in [0x2404,0x240e,0x240f,0x2419,0x241a,9999,10000]]
    totals.append(write_group("early-assembly-configurations",cases))
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))


if __name__=="__main__":
    main()
