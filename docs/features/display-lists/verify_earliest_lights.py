"""Regenerate old-light roots and compare isolated exact archive reconstruction."""
import sys
import json
import struct
import hashlib
from pathlib import Path
from generate_earliest_lights import generate as root
from generate_earliest_light_buckets import generate as buckets
from generate_earliest_light_decals import generate as decals
from generate_earliest_light_properties import generate as properties
from verify_earlier_processes import verify_groups

GROUPS = [("earliest-light-"+label,root) for label in ["roots","kinds"]]
GROUPS += [("earliest-light-buckets-"+label,buckets) for label in ["3d","2d"]]
GROUPS += [("earliest-light-decals",decals),("earliest-light-properties",properties)]


def budgets() -> None:
    rows = [row for name,_ in GROUPS for row in json.loads(Path(__file__).with_name(name+".json").read_text())["fixtures"]]
    raw = struct.pack("<I",len(rows))+b"".join(struct.pack("<II",row["aggregate_entries"],row["objects"]) for row in rows)
    Path("tests/fixtures/display-lists/earliest-light-budgets.bin").write_bytes(raw)
    report = {"fixtures":len(rows),"sha256":hashlib.sha256(raw).hexdigest(),"scope":"independent collection arithmetic and reference object counts"}
    Path(__file__).with_name("earliest-light-budgets.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")


if __name__=="__main__":
    if len(sys.argv)==1:budgets()
    else:verify_groups(sys.argv[1],GROUPS,"earliest-light-verification.json",["modern_root","modern_lights","modern_material","modern_pw_scene","modern_assembly","modern_bucket","modern_decals"])
