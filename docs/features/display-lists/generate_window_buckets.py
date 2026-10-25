"""Unshaded roots preserve prefix omission on either side of the window-dependent span."""
from pathlib import Path
import hashlib
import json
import struct
from generate_window_boundaries import VERSIONS
from generate_earlier_buckets import encoded,displaylists,swcontainer
from generate_earlier_range import write_group
from window_boundary_patches import environment_reader
from lower_range_patches import tail_capture
from mfcobjects import ObjectArchive
import dl_doc
import bucket_corpus


def root_bytes(document: object, path: Path, raw: bytes, version: int, scenario: int) -> bytes:
    patches: list[tuple[int,int,bytes]] = []
    original_environment = ObjectArchive.readers["uiUserModelEnv_c"]
    original_annotation = dl_doc.read_AnnotationViewTransform
    ObjectArchive.readers["uiUserModelEnv_c"] = environment_reader(original_environment,version,patches)
    dl_doc.read_AnnotationViewTransform = tail_capture(original_annotation,version<=0x1fa8,8,patches)
    try:
        return encoded(document,path,raw,version,scenario,patches)
    finally:
        ObjectArchive.readers["uiUserModelEnv_c"] = original_environment
        dl_doc.read_AnnotationViewTransform = original_annotation


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    assert kind=="part" and not 0x1fad<version<0x1fcf
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(path))
    raw = root_bytes(document,path,document.read("Contents/DisplayLists"),version,scenario)
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version = version
    root = displaylists.display_lists(archive,kind)
    assert archive.p==len(raw)
    trace = bucket_corpus.trace(root)
    framed = struct.pack("<IBBI",version,0,scenario,len(raw))+raw+struct.pack("<I",len(trace))+trace+struct.pack("<I",0)
    entries = 45 if scenario%16==2 else 83 if scenario>=16 else 88
    row = {"version":version,"kind":kind,"scenario":scenario,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"trace_fields":len(trace.splitlines()),"aggregate_entries":entries,"objects":sum(archive.classes.values())}
    return framed,row


if __name__=="__main__":
    totals = []
    for two_dimensional in [False,True]:
        cases = [generate(version,"part",16 if two_dimensional else 0) for version in VERSIONS]+[generate(8000,"part",17 if two_dimensional else 2)]
        totals.append(write_group("window-buckets-"+("2d" if two_dimensional else "3d"),cases))
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))
