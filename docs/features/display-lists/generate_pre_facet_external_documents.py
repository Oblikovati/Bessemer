"""Complete older assemblies with independently versioned, resolvable face-cache streams."""
from pathlib import Path
import json
import hashlib
from pre_facet_external import external_root
from document_context_wire import document
from generate_external_link_documents import entry
from generate_component_vectors import integers
from external_link_reference import expected

ROOT = Path("tests/fixtures/display-lists")


def streams(version: int, edited: bool) -> list[tuple[str,bytes]]:
    raw,identities = external_root(version,edited)
    assert identities==("Assem1",)
    directory = integers(2,7,1)+entry("000-000-007")
    block = Path("tests/fixtures/tessellation/block.bin").read_bytes()
    return [("Contents/DisplayLists",raw),(f"_DL_VERSION_{version}/DLUpdateStamp",b""),
        ("_MO_VERSION_19000/History",b""),("FaceTessellations/Directory",directory),
        ("FaceTessellations/000-000-007",block),("Unrelated/Payload",b"retained\0payload")]


if __name__=="__main__":
    rows = []
    for version in [3065,3195]:
        for edited in [False,True]:
            raw = document(streams(version,edited))
            path = ROOT/f"pre-facet-external-{version}-{int(edited)}.SLDASM"
            path.write_bytes(raw)
            reference = expected(str(path))
            assert reference["status"]==0 and reference["requested_faces"]==2
            rows.append({"fixture":path.name,"kind":"assembly","display_version":version,"file_version":19000,"directory_version":19000,"component_ids":["Assem1"],"edited":edited,"sha256":hashlib.sha256(raw).hexdigest(),"coverage":reference["coverage"]})
    Path(__file__).with_name("pre-facet-external-documents.json").write_text(json.dumps({"fixtures":rows},indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({"documents":len(rows),"requested_faces":len(rows)*2}))
