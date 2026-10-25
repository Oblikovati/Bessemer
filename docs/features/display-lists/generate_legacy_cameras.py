"""Complete roots around simple-text, viewer-double and camera-name boundaries."""
import hashlib
import struct
from pathlib import Path
from legacy_camera_wire import camera_root
from generate_legacy_panes import physical_reference
from generate_unsigned_versions import traces,compact_manifest
from generate_pre_facet_roots import scalars
from generate_earlier_range import write_group

VERSIONS = [2902,2932,2933,2970,2971,2998,2999]


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    raw,identities = camera_root(version,kind,scenario)
    root = physical_reference(raw,version,kind,identities)
    expected,scalar = traces(root,kind),scalars(root,kind)
    framed = struct.pack("<IBBI",version,kind=="assembly",scenario,len(raw))+raw
    framed += struct.pack("<I",len(expected))+expected+struct.pack("<I",len(scalar))+scalar
    row = {"version":version,"kind":kind,"scenario":scenario,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"oracle_fields":len(expected.splitlines())+len(scalar.splitlines())}
    if identities:row["external_ids"] = list(identities)
    return framed,row


if __name__=="__main__":
    cases = [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in range(3)]
    cases += [generate(v,"assembly",s) for v in VERSIONS for s in (3,4)]
    cases += [generate(2902,k,16+light) for k in ["part","assembly"] for light in range(5)]
    print(write_group("legacy-camera-roots",cases,("displaylists.py",)))
    compact_manifest(Path(__file__).with_name("legacy-camera-roots.json"))
