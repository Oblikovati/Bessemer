"""Complete 11000-range roots, including inline configuration tessellation (#36)."""
import json
from generate_earlier_range import converted as convert_root,generate as generate_root,entry_budget as base_entries,write_group
from precollector_configuration_wire import configurations,configuration_entries
from precollector_patches import convert

VERSIONS = sorted({11000,11001,0x2b5e,0x2b5f,0x2b61,0x2b62,0x2b66,0x2b67,0x2b6f,0x2b70,0x2b77,0x2b78,0x2b79,0x2b7a,0x2b7f,0x2b80,0x2b88,0x2b89,0x2b96,0x2b97,0x2b9b,0x2b9c,0x2b9d,0x2b9e,0x2ba3,0x2ba4,0x2bc3,0x2bc4,0x2bcf,0x2bd0,0x2bd1,11999,12000})
CONFIG_VERSIONS = [0x2b5f,0x2b61,0x2b62,0x2b66,0x2b67,0x2b6f,0x2b70,0x2b77,0x2b78,0x2b79,0x2b7a,0x2b7f,0x2b80]


def converted(document: object, path: object, raw: bytes, version: int, kind: str, variant: int) -> bytes:
    configuration_bytes = b"" if kind=="part" and version<=0x2b5e else None
    if variant in (4,5):
        configuration_bytes = configurations(version,kind,variant==5)
    return convert_root(document,path,raw,version,kind,variant,convert,configuration_bytes)


def entry_budget(version: int, kind: str, variant: int) -> int:
    return base_entries(version,kind,0)-2*(version<=0x2bd0)+(configuration_entries(version,kind) if variant in (4,5) else 0)


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    return generate_root(version,kind,scenario,converted,entry_budget)


def main() -> None:
    totals = []
    for kind in ["part","assembly"]:
        cases = [generate(version,kind) for version in VERSIONS]+[generate(11000,kind,1)]
        totals.append(write_group(f"precollector-range-{kind}",cases))
    cases = [generate(version,kind,4) for kind in ["part","assembly"] for version in CONFIG_VERSIONS]+[generate(0x2b5f,"part",5)]
    totals.append(write_group("precollector-configurations",cases))
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))


if __name__=="__main__":
    main()
