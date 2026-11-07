"""Independent complete and deliberately incomplete external-cache document fixtures."""
from pathlib import Path
import hashlib
import json
from document_context_wire import document
from generate_alias_fixture import swcontainer
from old_shaded_wire import facet_wire
from generate_vectors import words,string
from external_link_reference import expected

ROOT = Path("tests/fixtures/display-lists")


def entry(name: str, index: int = 42, key: int = 0x123456789abcdef0) -> bytes:
    raw = words(-1)+string("Assem1")+words(0x87654321)
    raw += words(0x01234567,0x89abcdef,0xfedcba98,0x76543210,key>>32,key)
    return raw+string(name)+words(index,2,-1,1,2,-1,3,4)


def streams(scenario: str) -> list[tuple[str,bytes]]:
    display = (ROOT/"external-shaded-assembly.bin").read_bytes()
    entries = [entry("" if scenario=="unlocated" else "000-000-007")]
    block = Path("tests/fixtures/tessellation/block.bin").read_bytes()
    if scenario=="short":block = words(42,0x12345678,0x9abcdef0,1)+facet_wire(19000,-17,False,True)
    if scenario=="duplicates":entries.append(entries[0])
    if scenario=="ambiguous":
        entries.append(entry("000-000-007",43,0x123456789abcdef1))
        block += words(43,0x12345678,0x9abcdef1,2)+facet_wire(19000,-17,False,True)*2
    if scenario=="empty":
        entries = []
        display = swcontainer.Document("tests/fixtures/blank-assembly.SLDASM").read("Contents/DisplayLists")
    result = [("_DL_VERSION_19000/DLUpdateStamp",b""),("Contents/DisplayLists",display),("FaceTessellations/Directory",words(2,7,len(entries))+b"".join(entries))]
    if scenario not in ("missing","unlocated","empty"):result.append(("FaceTessellations/000-000-007",block))
    return result


if __name__=="__main__":
    rows = []
    for scenario in ["complete","missing","unlocated","short","duplicates","ambiguous","empty"]:
        path = ROOT/("external-links-"+scenario+".SLDASM")
        raw = document(streams(scenario))
        path.write_bytes(raw)
        reference = expected(str(path))
        rows.append({"fixture":path.name,"sha256":hashlib.sha256(raw).hexdigest(),"expected_status":reference["status"],"coverage":reference["coverage"],"requested_faces":reference["requested_faces"]})
    Path(__file__).with_name("external-link-documents.json").write_text(json.dumps({"scope":"exact archive links; missing caches remain incomplete","fixtures":rows},indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({"fixtures":len(rows),"statuses":[row["expected_status"] for row in rows]}))
