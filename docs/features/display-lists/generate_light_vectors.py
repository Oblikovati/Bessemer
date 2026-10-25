"""Independently encode modern old-light bodies inside the owned blank part."""
from pathlib import Path
import hashlib
import json
import struct
from generate_alias_fixture import collect,displaylists,swcontainer
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_transform_vectors import doubles
import light_corpus


def properties(empty: bool) -> bytes:
    if empty:
        return struct.pack("<I",0xffffffff)
    raw = struct.pack("<IIIiI",4,0,0x80000000,1,2**32-123)
    return raw+struct.pack("<I",2)+cstring("property 😀鋼",True)+struct.pack("<I",0xffffffff)


def base(empty: bool) -> bytes:
    raw = doubles(3)+struct.pack("<I",0xfedcba98)+integers(-1,2,-3,4)
    raw += cstring("light ID\0鋼",True)+cstring("description 😀",True)+integers(-5,6,-7)
    return raw+doubles(1)+integers(-8)+doubles(1)+properties(empty)+integers(-9)


def payload(spot: bool, empty: bool, edited: bool) -> bytes:
    raw = base(empty)+doubles(3)
    if not spot:
        return raw+doubles(1)+integers(-10)+doubles(2)
    exponent = struct.pack("<d",23.0) if edited else doubles(1,1)
    raw += doubles(3)+doubles(1)+exponent+doubles(6,2)
    return raw+integers(-10)+doubles(2,1)


def write_trace(document: swcontainer.Document, path: Path, raw: bytes, destination: Path) -> int:
    archive = displaylists.archive_for(document,str(path),raw)
    root = displaylists.display_lists(archive,archive.doc_kind)
    assert archive.p == len(raw)
    trace = light_corpus.trace(root)
    destination.with_suffix(".txt").write_bytes(trace)
    return len(trace.splitlines())


def generate(spot: bool, empty: bool = False, edited: bool = False) -> dict[str,object]:
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(path))
    source = document.read("Contents/DisplayLists")
    slots = [row for row in collect(document,path,source) if row[3] == "moDirectionLight_c"]
    assert len(slots) == 1
    start,end,identity,_ = slots[0]
    name = "moSpotLight_c" if spot else "moPointLight_c"
    prefix = struct.pack("<HHH",0xffff,1,len(name))+name.encode("ascii")
    derived = source[:start]+prefix+payload(spot,empty,edited)+source[end:]
    fixture = "lights-edited" if edited else "lights-empty-properties" if empty else "lights-spot" if spot else "lights-point"
    destination = Path(f"tests/fixtures/display-lists/{fixture}.bin")
    destination.write_bytes(derived)
    fields = write_trace(document,path,derived,destination)
    return {"fixture":fixture,"bytes":len(derived),"sha256":hashlib.sha256(derived).hexdigest(),"object_id":identity,"object_start":start,"object_end":start+len(prefix)+len(payload(spot,empty,edited)),"semantic_fields":fields,"aggregate_entries":53 if empty else 57}


def main() -> None:
    evidence = {"context":"version-19000 DisplayLists serializeOldLights; not model-feature light serialization","fixtures":[generate(True),generate(False),generate(True,True),generate(True,False,True)]}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_light.py","displaylists.py","mfcobjects.py"]}
    Path(__file__).with_name("light-vectors.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(evidence))


if __name__ == "__main__":
    main()
