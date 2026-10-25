"""Complete buckets coupled to the earliest populated view-prefix layouts."""
from pathlib import Path
import hashlib
import json
import struct
from generate_early_view_roots import VERSIONS
from generate_window_buckets import root_bytes,displaylists,swcontainer
from generate_named_vectors import collection as named_collection
from generate_annotation_vectors import collection as annotation_collection
from early_view_patches import named_entries,annotation_entries
from generate_earlier_range import write_group
import bucket_corpus


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    assert kind=="part"
    variant = scenario%64
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(path))
    payloads = {"read_NamedViewTransform":named_collection(version,3),"read_AnnotationViewTransform":annotation_collection(version,2),"read_3DViewTransform":b"" if version<=0x1c18 else bytes(4)}
    raw = root_bytes(document,path,document.read("Contents/DisplayLists"),version,variant,payloads)
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version = version
    root = displaylists.display_lists(archive,kind)
    assert archive.p==len(raw)
    trace = bucket_corpus.trace(root)
    tag = 64+variant
    framed = struct.pack("<IBBI",version,0,tag,len(raw))+raw+struct.pack("<I",len(trace))+trace+struct.pack("<I",0)
    entries = (45 if variant==2 else 83 if variant>=16 else 88)+named_entries(version)+annotation_entries(version)-1
    row = {"version":version,"kind":kind,"scenario":tag,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"trace_fields":len(trace.splitlines()),"aggregate_entries":entries,"objects":sum(archive.classes.values())}
    return framed,row


if __name__=="__main__":
    totals = []
    for two_dimensional in [False,True]:
        cases = [generate(version,"part",16 if two_dimensional else 0) for version in VERSIONS]+[generate(7000,"part",17 if two_dimensional else 2)]
        totals.append(write_group("early-view-buckets-"+("2d" if two_dimensional else "3d"),cases))
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))
