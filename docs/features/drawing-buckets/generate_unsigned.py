"""Complete drawing streams under high unsigned model markers, including signed aliases."""
from pathlib import Path
import json
import hashlib
import struct
from generate_vectors import generate
from verify_corpus import reference,frames
from unsigned_version_cases import VERSIONS
from document_context_wire import document


def write() -> None:
    cases = [(version,index,generate(index)) for version in VERSIONS for index in range(5)]
    destination = Path("tests/fixtures/drawing-buckets")
    (destination/"unsigned.bin").write_bytes(frames([raw for _,_,raw in cases]))
    (destination/"unsigned-fields.bin").write_bytes(frames([reference(raw,v) for v,_,raw in cases]))
    (destination/"unsigned-versions.bin").write_bytes(struct.pack("<"+"I"*len(cases),*(v for v,_,_ in cases)))
    rows = [{"version":v,"scenario":index,"sha256":hashlib.sha256(raw).hexdigest(),"fields":len(reference(raw,v).splitlines())} for v,index,raw in cases]
    Path(__file__).with_name("unsigned.json").write_text(json.dumps({"fixtures":rows},indent=2)+"\n",encoding="utf-8",newline="\n")
    streams = [("Contents/VBLists",generate(2)),("_MO_VERSION_-1/History",b""),("_MO_VERSION_4294967295/Other",b""),("_DL_VERSION_7000/DLUpdateStamp",b""),("Unrelated/Payload",b"retained\0payload")]
    (destination/"unsigned-document.bin").write_bytes(document(streams))
    print(json.dumps({"fixtures":len(cases),"fields":sum(row["fields"] for row in rows)}))


if __name__=="__main__":
    write()
