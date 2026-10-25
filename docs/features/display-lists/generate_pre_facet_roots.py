"""Full older archives with resolved bodies and explicit absent or empty geometry."""
from pathlib import Path
import json
import struct
import hashlib
from functools import partial
from pre_facet_wire import converted
from generate_unsigned_versions import reference,traces,compact_manifest
from generate_earlier_range import write_group
from middle_range_source import component_trace
from generate_blank_scalars import flatten
from generate_earliest_components import write_documents

VERSIONS = [3065,0xc57,0xc58,0xc59,0xc6e,0xc6f,0xc7b,0xc7c]


def scalars(root: dict[str,object], kind: str) -> bytes:
    rows = {}
    flatten("root.has_shaded_data",root["has_shaded_data"],rows)
    if root["has_shaded_data"] and kind=="part":
        # The reference maps the stored u32 FFFFFFFF to its logical -1 time sentinel.
        flatten("root.tessellation.ddata_time_stamp",root["tessellation"]["ddata_time_stamp"] & 0xffffffff,rows)
    if root["has_shaded_data"] and kind=="assembly" and "unknown_320" in root["tessellation"][0]:
        flatten("root.tessellation.components[0].unknown_320",root["tessellation"][0]["unknown_320"],rows)
    extra = component_trace(root,kind) if root["has_shaded_data"] else b""
    return extra+"".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    raw = converted(version,kind,scenario)
    root = reference(raw,version,kind,())
    expected,scalar = traces(root,kind),scalars(root,kind)
    framed = struct.pack("<IBBI",version,kind=="assembly",scenario,len(raw))+raw
    framed += struct.pack("<I",len(expected))+expected+struct.pack("<I",len(scalar))+scalar
    row = {"version":version,"kind":kind,"scenario":scenario,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"oracle_fields":len(expected.splitlines())+len(scalar.splitlines())}
    return framed,row


if __name__=="__main__":
    cases = [generate(version,kind,scenario) for version in VERSIONS for kind in ["part","assembly"] for scenario in range(3)]
    cases += [generate(version,kind,16+light) for version in [3065,0xc6e,0xc6f] for kind in ["part","assembly"] for light in range(5)]
    print(write_group("pre-facet-roots",cases,("displaylists.py",)))
    compact_manifest(Path(__file__).with_name("pre-facet-roots.json"))
    write_documents(partial(generate,scenario=0),3065,"pre-facet")
