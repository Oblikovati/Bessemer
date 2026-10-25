"""Complete cache-backed old documents across the wide line-word boundary."""
from pathlib import Path
import json
import hashlib
from legacy_pane_wire import root_bytes
from generate_pre_facet_external_documents import streams as earlier_streams
from document_context_wire import document
from external_link_reference import expected


if __name__=="__main__":
    rows = []
    for version in [2999,3064]:
        for edited in [False,True]:
            display,identities = root_bytes(version,"assembly",4 if edited else 3)
            streams = [(name,display if name=="Contents/DisplayLists" else raw) for name,raw in earlier_streams(version,edited)]
            path = Path(f"tests/fixtures/display-lists/legacy-pane-external-{version}-{int(edited)}.SLDASM")
            raw = document(streams)
            path.write_bytes(raw)
            reference = expected(str(path))
            assert reference["status"]==0 and reference["requested_faces"]==2
            rows.append({"fixture":path.name,"kind":"assembly","display_version":version,"file_version":19000,"directory_version":19000,"component_ids":list(identities),"edited":edited,"sha256":hashlib.sha256(raw).hexdigest(),"coverage":reference["coverage"]})
    Path(__file__).with_name("legacy-pane-external-documents.json").write_text(json.dumps({"fixtures":rows},indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({"documents":len(rows),"requested_faces":len(rows)*2}))
