"""Seven reference kinds at their earlier version boundaries, including nullable object roles."""
from pathlib import Path
import hashlib
import json
import struct
from generate_earlier_range import source,converted,texture_source,displaylists
from generate_reference_vectors import reference,coordinate_system,plane,cpoint_extra
from generate_named_vectors import cstring
from generate_component_vectors import integers
from generate_transform_vectors import doubles
import dl_tess
import reference_corpus

VERSIONS = [0x2f45,0x2f5b,0x2f5c,0x2f66,0x2f67,0x2fb8,0x2fb9,0x31ff,0x3200]


def earlier_cpoint(version: int) -> bytes:
    if version<=0x2fb8:
        return b""
    raw = cstring("cpoint 鋼",True)+integers(-176)+doubles(9,1)+doubles(3,4)+doubles(2,7)
    extra = cpoint_extra(0x7ff0000000000042)
    if version<0x3200:
        extra = extra[:72]+bytes(6)+extra[72:88]+bytes(2)+extra[88:92]+bytes(4)+extra[92:]
    return raw+extra


def geometry(kind: int, version: int) -> bytes:
    base = reference(kind,0)[:len(integers(kind)+cstring(f"kind {kind} 鋼",True)+integers(-10,14,-18))]
    bodies = [b"",doubles(3,7),doubles(9)+doubles(3,3)+(doubles(2,6) if version>0x2f5b else b""),plane() if version>0x2f66 else plane()[:-4],coordinate_system(),earlier_cpoint(version),doubles(3,2)+doubles(3,5) if version>0x2fb8 else b""]
    return base+bodies[kind]


def collector_span(document: object, path: Path, raw: bytes, version: int) -> tuple[int,int]:
    spans: list[tuple[int,int]] = []
    original = dl_tess.read_moRefGeomFeatureDataCollector
    def capture(archive: object) -> object:
        start = archive.p
        result = original(archive)
        spans.append((start,archive.p))
        return result
    try:
        dl_tess.read_moRefGeomFeatureDataCollector = capture
        reference_root(document,path,raw,version)
    finally:
        dl_tess.read_moRefGeomFeatureDataCollector = original
    return spans[0]


def reference_root(document: object, path: Path, raw: bytes, version: int) -> dict[str,object]:
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version,archive.doc_kind,archive.face_tess_comp_ids = version,"assembly",set()
    root = displaylists.display_lists(archive,"assembly")
    assert archive.p==len(raw)
    return root


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    assert kind=="assembly" and scenario==0
    path,document,raw = source(kind)
    raw = converted(document,path,texture_source(document,path,raw),version,kind,0)
    start,end = collector_span(document,path,raw,version)
    raw = raw[:start]+struct.pack("<II",0,7)+b"".join(geometry(kind,version) for kind in range(7))+raw[end:]
    root = reference_root(document,path,raw,version)
    cpoint = root["tessellation"][0]["viewer_comp_data"]["ref_geom_data_collector"][5]["unknown_c0"]
    for key in ["dims","dim70","params"]:
        cpoint.pop(key,None)
    trace = reference_corpus.trace(root)
    framed = struct.pack("<IBBI",version,1,0,len(raw))+raw+struct.pack("<I",len(trace))+trace+struct.pack("<I",0)
    return framed,{"version":version,"kind":kind,"scenario":scenario,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"trace_fields":len(trace.splitlines())}


if __name__=="__main__":
    cases = [generate(version,"assembly") for version in VERSIONS]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path("tests/fixtures/display-lists/earlier-geometry.bin").write_bytes(framed)
    evidence = {"scope":"all seven kinds with null dimensions and configurable parameters; non-null bodies remain unsupported","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest(),"reader_sha256":hashlib.sha256(Path(dl_tess.__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name("earlier-geometry.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({"fixtures":len(cases),"fields":sum(row["trace_fields"] for _,row in cases)}))
