"""Complete owned part/assembly roots on both sides of the two 4850-range gates."""
import json
import struct
from generate_earlier_range import converted as convert_root,generate as generate_root,write_group
from generate_early_appearance_roots import item_bytes,face_identity_trace
from generate_early_view_roots import entry_budget
from early_appearance_patches import rv_source
from earliest_scene_patches import convert
from swarchive import Archive

VERSIONS = [4850,0x12fc,0x12fd,0x1310,0x1311,4999,5000]


def converted(document: object, path: object, raw: bytes, version: int, kind: str, variant: int) -> bytes:
    return convert_root(document,path,rv_source(document,path,raw),version,kind,variant,convert,b"",item_bytes(version))


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    variant = scenario%16
    original = Archive.f32
    Archive.f32 = raw_nan_reader(original)
    try:framed,row = generate_root(version,kind,variant,converted,lambda v,k,s:entry_budget(v,k,s)+2,face_identity_trace)
    finally:Archive.f32 = original
    row["scenario"] = 240+variant
    return framed[:5]+bytes([240+variant])+framed[6:],row


def raw_nan_reader(original: object) -> object:
    def read(archive: object) -> float:
        start = archive.p
        value = original(archive)
        bits = int.from_bytes(archive.b[start:archive.p],"little")
        if bits&0x7f800000!=0x7f800000 or bits&0x7fffff==0:return value
        # Python's hardware float promotion quiets signaling NaNs; trace the stored payload instead.
        widened = nan_payload64(bits)
        return struct.unpack("<d",struct.pack("<Q",widened))[0]
    return read


def nan_payload64(bits: int) -> int:
    return ((bits>>31)<<63)|(0x7ff<<52)|((bits&0x7fffff)<<29)


if __name__=="__main__":
    totals = []
    for kind in ["part","assembly"]:
        cases = [generate(v,kind) for v in VERSIONS]+[generate(4850,kind,1)]
        totals.append(write_group("earliest-scene-"+kind,cases))
    print(json.dumps({"fixtures":sum(n for n,_ in totals),"fields":sum(n for _,n in totals)}))
