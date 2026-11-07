"""Complete part/assembly roots with independently encoded shared UI CObjects."""
import json
import struct
import hashlib
from pathlib import Path
from functools import partial
from generate_earlier_range import converted as convert_root,generate as generate_root,write_group
from generate_early_appearance_roots import item_bytes,face_identity_trace
from generate_early_view_roots import entry_budget
from early_appearance_patches import rv_source
from ui_object_patches import convert,native_root
from ui_object_wire import entries

VERSIONS = [5000,0x1be4,0x1be5,19000]


def converted(document: object, path: object, raw: bytes, version: int, kind: str, scenario: int, class_name: str, variant: int) -> bytes:
    if version==19000:return native_root(document,path,rv_source(document,path,raw),class_name,variant)
    converter = partial(convert,class_name=class_name,variant=variant)
    return convert_root(document,path,rv_source(document,path,raw),version,kind,0,converter,b"",item_bytes(version))


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    class_name = "uiLayerMgr_c" if scenario>=16 else "moLineStyle_c"
    variant = scenario%16
    converter = partial(converted,class_name=class_name,variant=variant)
    counter = lambda v,k,s:(143 if k=="part" else 148)+entries(class_name,variant) if v==19000 else entry_budget(v,k,0)+2+entries(class_name,variant)
    framed,row = generate_root(version,kind,0,converter,counter,face_identity_trace)
    row["scenario"],row["class_name"] = scenario,class_name
    return framed[:5]+bytes([scenario])+framed[6:],row


def write_budgets() -> None:
    rows = []
    for name in ["line-styles","layer-maps"]:
        rows += json.loads(Path(__file__).with_name(name+".json").read_text(encoding="utf-8"))["fixtures"]
    raw = struct.pack("<I",len(rows))+b"".join(struct.pack("<II",row["aggregate_entries"],row["objects"]) for row in rows)
    Path("tests/fixtures/display-lists/ui-object-budgets.bin").write_bytes(raw)
    report = {"fixtures":len(rows),"sha256":hashlib.sha256(raw).hexdigest(),"scope":"independent collection-entry arithmetic and reference object-load counts in root fixture order"}
    Path(__file__).with_name("ui-object-budgets.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")


if __name__=="__main__":
    totals = []
    for label,offset in [("line-styles",0),("layer-maps",16)]:
        cases = [generate(v,k,offset+s) for v in VERSIONS for k in ["part","assembly"] for s in [0,1,2]]
        if offset==0:cases += [generate(19000,"part",3)]
        else:cases += [generate(v,k,offset+s) for v in VERSIONS if v<0x1be5 for k in ["part","assembly"] for s in [3,4,5]]
        totals.append(write_group(label,cases))
    write_budgets()
    print(json.dumps({"fixtures":sum(n for n,_ in totals),"fields":sum(n for _,n in totals)}))
