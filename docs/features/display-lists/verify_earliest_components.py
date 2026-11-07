"""Regenerate complete component roots and check isolated deterministic reconstruction."""
import sys
import json
import struct
import hashlib
from pathlib import Path
from generate_earliest_components import generate as root
from generate_earliest_component_properties import generate as properties
from generate_earliest_component_buckets import generate as buckets
from generate_earliest_component_decals import generate as decals
from verify_earlier_processes import verify_groups

GROUPS = [("earliest-component-"+kind,root) for kind in ["part","assembly"]]
GROUPS += [("earliest-component-"+kind,properties) for kind in ["properties","holders"]]
GROUPS += [("earliest-component-buckets-"+kind,buckets) for kind in ["3d","2d"]]
GROUPS += [("earliest-component-decals",decals)]


def budgets() -> None:
    rows = [row for name,_ in GROUPS for row in json.loads(Path(__file__).with_name(name+".json").read_text())["fixtures"]]
    raw = struct.pack("<I",len(rows))+b"".join(struct.pack("<II",row["aggregate_entries"],row["objects"]) for row in rows)
    Path("tests/fixtures/display-lists/earliest-component-budgets.bin").write_bytes(raw)
    report = {"fixtures":len(rows),"sha256":hashlib.sha256(raw).hexdigest(),"scope":"independent entry arithmetic and reference object counts in fixture order"}
    Path(__file__).with_name("earliest-component-budgets.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")


if __name__=="__main__":
    if len(sys.argv)==1:budgets()
    else:verify_groups(sys.argv[1],GROUPS,"earliest-component-verification.json",["modern_root","modern_component","modern_component_properties","legacy_bucket_poly","modern_decals"])
