"""Derive a shared-font graph from the owned native blank part, retaining exact other fields."""
from pathlib import Path
import hashlib
import json
import struct
import sys
sys.path.insert(0,str(Path("../inventor-interface/solidworks").resolve()))
import swcontainer
import displaylists


def collect(document: swcontainer.Document, path: Path, raw: bytes) -> list[tuple[int,int,int,str]]:
    archive = displaylists.archive_for(document,str(path),raw)
    original = archive.object
    records: list[tuple[int,int,int,str]] = []
    def capture(expect: object = None) -> object:
        start = archive.p
        result = original(expect)
        identity = next((index for index,value in enumerate(archive.map) if value is result),0)
        name = result.get("_class","") if isinstance(result,dict) else ""
        records.append((start,archive.p,identity,name))
        return result
    archive.object = capture
    displaylists.display_lists(archive,archive.doc_kind)
    assert archive.p == len(raw)
    return sorted(records)


def remapped_tag(raw: bytes, start: int, removed: int) -> bytes | None:
    tag, = struct.unpack_from("<H",raw,start)
    if tag in (0,0xffff):
        return None
    big = tag == 0x7fff
    value = struct.unpack_from("<I",raw,start+2)[0] if big else tag
    mask = 0x80000000 if big else 0x8000
    index = value & (mask-1)
    if index <= removed:
        return None
    adjusted = (value & mask) | (index-1)
    return struct.pack("<HI",tag,adjusted) if big else struct.pack("<H",adjusted)


def derive(raw: bytes, records: list[tuple[int,int,int,str]]) -> tuple[bytes,int,int]:
    fonts = [record for record in records if record[3] == "uiLFConfig_c"]
    first,second = fonts[:2]
    patches = [(second[0],second[1],struct.pack("<H",first[2]))]
    for start,_,_,_ in records:
        if second[0] <= start < second[1]:
            continue
        replacement = remapped_tag(raw,start,second[2])
        if replacement is not None:
            patches.append((start,start+len(replacement),replacement))
    output = bytearray()
    offset = 0
    for start,end,replacement in sorted(patches):
        output.extend(raw[offset:start]+replacement)
        offset = end
    output.extend(raw[offset:])
    return bytes(output),first[2],second[2]


if __name__ == "__main__":
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(path))
    raw = document.read("Contents/DisplayLists")
    derived,shared,removed = derive(raw,collect(document,path,raw))
    archive = displaylists.archive_for(document,str(path),derived)
    parsed = displaylists.display_lists(archive,archive.doc_kind)
    assert archive.p == len(derived)
    fonts = parsed["line_font_mgr"]["line_fonts"]
    assert fonts[0][1] is fonts[1][1]
    Path("tests/fixtures/display-lists/shared-font-part.bin").write_bytes(derived)
    evidence = {"reason":"independently derived alias/map-remapping regression; no application acceptance claim","source_sha256":hashlib.sha256(raw).hexdigest(),"derived_sha256":hashlib.sha256(derived).hexdigest(),"bytes":len(derived),"shared_object_index":shared,"removed_object_index":removed,"runtime_classes":archive.classes,"map_slots":len(archive.map)}
    Path("docs/features/display-lists/shared-font-fixture.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(evidence))
