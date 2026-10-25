"""Independent shared tessellation fields across the complete modern drawing interval."""
from pathlib import Path
import importlib.util
import struct
import json
import hashlib
from verify_corpus import frames,reference
from generate_shaded_vectors import ObjectBuilder
from generate_named_vectors import cstring
from generate_transform_vectors import doubles

spec = importlib.util.spec_from_file_location("owned_facets",Path(__file__).resolve().parents[1]/"face-tessellations/generate_vectors.py")
facets = importlib.util.module_from_spec(spec)
spec.loader.exec_module(facets)
VERSIONS = [0x27cc,0x27e0,0x27e1,0x2b96,0x2b97,0x2b9d,0x2b9e,0x2ba3,0x2ba4,0x2bcf,0x2bd0,0x2bd1,0x2be4,0x2be5,0x2f5d,0x2f5e,0x336b,0x336c,0x3742,0x3743,0x3751,0x3752,0x3757,0x3758,0x3b0d,0x3b0e,0x3b28,0x3b29,0x3ee7,0x3ee8,0x3f0e,0x3f0f,0x3f10,19000]


def ints(*values: int) -> bytes:
    return struct.pack("<"+"i"*len(values),*values)


def face(version: int) -> bytes:
    raw = facets.facet(version,-17,version>=0x2be5,True)
    raw += ints(0,-34,-68,0,-38)
    if version>0x2bcf:raw += ints(0)+cstring("face 鋼",True)
    if version>0x336b:raw += ints(-77)
    if version>=0x3743:raw += ints(-78)
    return raw


def body(builder: ObjectBuilder, version: int) -> bytes:
    raw = struct.pack("<H",1)+builder.object("uoTempFaceTessData_c",lambda _:face(version))
    return raw+ints(-20,-24,-44,0,0)+cstring("body 鋼",True)+ints(-104)


def part(builder: ObjectBuilder, version: int) -> bytes:
    raw = struct.pack("<H",1)+builder.object("uoTempBodyTessData_c",lambda state:body(state,version))
    raw += ints(0,-1,0,0,-168,2)
    if version>0x2b96:raw += ints(-2)+cstring("configuration 鋼",True)
    if version>0x2b9d:raw += ints(0)
    if version>0x2ba3:raw += doubles(10)
    if version>0x2f5d:raw += bytes(8)
    if version>0x3751:raw += bytes([255])
    return raw


def generate(version: int) -> bytes:
    builder = ObjectBuilder(1)
    raw = ints(1)+bytes(2)+ints(-1)+struct.pack("<II",0xffffffff,0x80000000)+ints(0,0,2)
    raw += ints(1)+cstring("part 鋼",True)+struct.pack("<H",65535)+ints(0)+bytes(2)+part(builder,version)
    raw += ints(0)+cstring("assembly")+struct.pack("<H",32768)+ints(0)+bytes(4)
    return raw+ints(0,0,0)


if __name__=="__main__":
    cases = [generate(version) for version in VERSIONS]
    destination = Path("tests/fixtures/drawing-buckets")
    (destination/"range.bin").write_bytes(frames(cases))
    (destination/"range-fields.bin").write_bytes(frames([reference(raw,v) for v,raw in zip(VERSIONS,cases)]))
    (destination/"range-versions.bin").write_bytes(struct.pack("<"+"I"*len(VERSIONS),*VERSIONS))
    rows = [{"version":v,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"fields":len(reference(raw,v).splitlines())} for v,raw in zip(VERSIONS,cases)]
    Path(__file__).with_name("range.json").write_text(json.dumps({"fixtures":rows},indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({"fixtures":len(rows),"fields":sum(row["fields"] for row in rows)}))
