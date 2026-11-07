"""Complete roots around the first lightweight-rendering and appearance-name fields."""
from pathlib import Path
from functools import partial
from legacy_visual_wire import visual_root
from generate_legacy_panes import physical_reference
from generate_unsigned_versions import compact_manifest
from generate_blank_scalars import flatten
from complete_root_case import frame_case
from generate_earlier_range import write_group
from generate_earliest_components import write_documents

VERSIONS = [2891,2899,2900,2901,2902]


def visual_scalars(root: dict[str,object]) -> bytes:
    rows = {}
    for name,value in root.get("visual_properties",{}).items():
        if name not in ("_class","material"):flatten("root.visual_properties."+name,value,rows)
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    raw,identities = visual_root(version,kind,scenario)
    root = physical_reference(raw,version,kind,identities)
    return frame_case(version,kind,scenario,raw,identities,root,visual_scalars(root))


if __name__=="__main__":
    cases = [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in range(3)]
    cases += [generate(v,"assembly",s) for v in VERSIONS for s in (3,4)]
    cases += [generate(2891,k,16+light) for k in ["part","assembly"] for light in range(5)]
    print(write_group("legacy-visual-roots",cases,("displaylists.py",)))
    compact_manifest(Path(__file__).with_name("legacy-visual-roots.json"))
    write_documents(partial(generate,scenario=0),2891,"legacy-visual")
