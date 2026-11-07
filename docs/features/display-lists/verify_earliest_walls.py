"""Regenerate old wall roots and compare deterministic exact reconstruction in two processes."""
import sys
import json
import struct
import hashlib
from pathlib import Path
from generate_earliest_walls import generate as root
from generate_earliest_wall_properties import generate as properties
from verify_earlier_processes import verify_groups

GROUPS = [("earliest-wall-"+kind,root) for kind in ["part","assembly"]]+[("earliest-wall-properties",properties)]


def budgets() -> None:
    rows = [row for name,_ in GROUPS for row in json.loads(Path(__file__).with_name(name+".json").read_text())["fixtures"]]
    raw = struct.pack("<I",len(rows))+b"".join(struct.pack("<II",row["aggregate_entries"],row["objects"]) for row in rows)
    Path("tests/fixtures/display-lists/earliest-wall-budgets.bin").write_bytes(raw)
    report = {"fixtures":len(rows),"sha256":hashlib.sha256(raw).hexdigest(),"scope":"independent entry arithmetic and reference object counts"}
    Path(__file__).with_name("earliest-wall-budgets.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")


if __name__=="__main__":
    if len(sys.argv)==1:budgets()
    else:verify_groups(sys.argv[1],GROUPS,"earliest-wall-verification.json",["modern_root","modern_body_tess","modern_material","modern_walls"])
