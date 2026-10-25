"""Regenerate complete old face/property roots and verify isolated exact reconstruction."""
import sys
import json
import struct
import hashlib
from pathlib import Path
from generate_earliest_faces import generate as root
from generate_earliest_face_properties import generate as properties
from verify_earlier_processes import verify_groups

GROUPS = [("earliest-face-"+kind,root) for kind in ["part","assembly"]]
GROUPS += [("earliest-face-"+kind,properties) for kind in ["properties","holders"]]


def budgets() -> None:
    rows = [row for name,_ in GROUPS for row in json.loads(Path(__file__).with_name(name+".json").read_text())["fixtures"]]
    raw = struct.pack("<I",len(rows))+b"".join(struct.pack("<II",row["aggregate_entries"],row["objects"]) for row in rows)
    Path("tests/fixtures/display-lists/earliest-face-budgets.bin").write_bytes(raw)
    report = {"fixtures":len(rows),"sha256":hashlib.sha256(raw).hexdigest(),"scope":"independent collection arithmetic and reference object counts"}
    Path(__file__).with_name("earliest-face-budgets.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")


if __name__=="__main__":
    if len(sys.argv)==1:budgets()
    else:verify_groups(sys.argv[1],GROUPS,"earliest-face-verification.json",["modern_root","modern_body_tess","modern_scene","modern_component_properties","modern_item_geometry"])
