"""Complete roots around simple-text, viewer-double and camera-name boundaries."""
from pathlib import Path
from legacy_camera_wire import camera_root
from generate_legacy_panes import physical_reference
from generate_unsigned_versions import compact_manifest
from complete_root_case import frame_case
from generate_earlier_range import write_group

VERSIONS = [2902,2932,2933,2970,2971,2998,2999]


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    raw,identities = camera_root(version,kind,scenario)
    root = physical_reference(raw,version,kind,identities)
    return frame_case(version,kind,scenario,raw,identities,root)


if __name__=="__main__":
    cases = [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in range(3)]
    cases += [generate(v,"assembly",s) for v in VERSIONS for s in (3,4)]
    cases += [generate(2902,k,16+light) for k in ["part","assembly"] for light in range(5)]
    print(write_group("legacy-camera-roots",cases,("displaylists.py",)))
    compact_manifest(Path(__file__).with_name("legacy-camera-roots.json"))
