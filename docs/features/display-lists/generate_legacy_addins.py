"""Complete roots around the body-property pointer and add-in appearance gates."""
from pathlib import Path
from functools import partial
from legacy_addin_wire import addin_root
from generate_legacy_panes import physical_reference
from generate_unsigned_versions import compact_manifest
from generate_legacy_visuals import visual_scalars
from complete_root_case import frame_case
from generate_earlier_range import write_group
from generate_earliest_components import write_documents

VERSIONS = [2796,2859,2860,2890,2891]


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    raw,identities = addin_root(version,kind,scenario)
    root = physical_reference(raw,version,kind,identities)
    return frame_case(version,kind,scenario,raw,identities,root,visual_scalars(root))


if __name__=="__main__":
    cases = [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in range(3)]
    cases += [generate(v,"assembly",s) for v in VERSIONS for s in (3,4)]
    cases += [generate(2796,k,16+light) for k in ["part","assembly"] for light in range(5)]
    print(write_group("legacy-addin-roots",cases,("displaylists.py",)))
    compact_manifest(Path(__file__).with_name("legacy-addin-roots.json"))
    write_documents(partial(generate,scenario=0),2796,"legacy-addin")
