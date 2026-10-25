"""Reference-readable 8000-range roots around the uncounted-window interval."""
import json
from generate_earlier_range import converted as convert_root,generate as generate_root,write_group
from generate_grouped_components import entry_budget as previous_entries
from early_annotation_wire import minimal_items
from window_boundary_patches import convert
from minimal_root_wire import unshaded_prefix

VERSIONS = [8000,8001,0x1fa8,0x1fa9,0x1fad,0x1fcf,0x1fd4,0x1fd5,0x1ff0,0x1ff1,0x2001,0x2002,0x2003,0x2004,0x2007,0x2008,0x200d,0x200e,0x2012,0x2013,0x2017,0x2018,8999,9000]
MINIMAL_VERSIONS = [8000,0x1fa8,0x1fa9,0x1fcf,0x1fd4,0x1fd5]


def converted(document: object, path: object, raw: bytes, version: int, kind: str, variant: int) -> bytes:
    output = convert_root(document,path,raw,version,kind,variant,convert,b"",minimal_items(version) if variant==8 else None)
    return unshaded_prefix(document,path,output,version,kind) if variant==8 else output


def entry_budget(version: int, kind: str, variant: int) -> int:
    original = previous_entries(version,kind,0)
    return 152 if variant==8 else original-2*(version<=0x1fd4)


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    assert not 0x1fad<version<0x1fcf
    return generate_root(version,kind,scenario,converted,entry_budget)


def main() -> None:
    totals = []
    for kind in ["part","assembly"]:
        cases = [generate(version,kind) for version in VERSIONS]+[generate(8000,kind,1)]
        totals.append(write_group(f"window-boundary-{kind}",cases))
    cases = [generate(version,kind,8) for version in MINIMAL_VERSIONS for kind in ["part"]]
    totals.append(write_group("early-minimal-annotations",cases))
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))


if __name__=="__main__":
    main()
