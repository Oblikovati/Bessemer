"""Build owned containers with explicit older displays and independently versioned caches."""
from pathlib import Path
from collections.abc import Callable
import hashlib
import json
from generate_pre_facet_external_documents import streams as earlier_streams
from document_context_wire import document
from external_link_reference import expected

RootBuilder = Callable[[int,str,int],tuple[bytes,tuple[str,...]]]


def document_case(builder: RootBuilder, version: int, edited: bool, prefix: str) -> dict[str,object]:
    display,identities = builder(version,"assembly",4 if edited else 3)
    streams = [(name,display if name=="Contents/DisplayLists" else raw) for name,raw in earlier_streams(version,edited)]
    path = Path(f"tests/fixtures/display-lists/{prefix}-{version}-{int(edited)}.SLDASM")
    raw = document(streams)
    path.write_bytes(raw)
    reference = expected(str(path))
    assert reference["status"]==0 and reference["requested_faces"]==2
    return {"fixture":path.name,"kind":"assembly","display_version":version,"file_version":19000,"directory_version":19000,"component_ids":list(identities),"edited":edited,"sha256":hashlib.sha256(raw).hexdigest(),"coverage":reference["coverage"]}


def write_documents(builder: RootBuilder, versions: list[int], prefix: str) -> None:
    rows = [document_case(builder,version,edited,prefix) for version in versions for edited in [False,True]]
    Path(__file__).with_name(prefix+"-documents.json").write_text(json.dumps({"fixtures":rows},indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({"documents":len(rows),"requested_faces":len(rows)*2}))
