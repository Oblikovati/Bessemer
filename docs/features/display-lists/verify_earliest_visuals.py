"""Compare frozen implicit-visual roots in two isolated reconstruction processes."""
import sys
import json
import struct
from pathlib import Path
from generate_earliest_visuals import generate
from generate_earliest_visual_properties import generate as properties
from generate_earliest_widths import generate as widths
from generate_earliest_light_states import generate as lights
from verify_earlier_processes import verify_groups

GROUPS = [("earliest-visual-"+kind,generate) for kind in ["part","assembly"]]

GROUPS.append(("earliest-visual-properties",properties))
GROUPS.append(("earliest-width-items",widths))
GROUPS.append(("earliest-light-state-kinds",lights))

if __name__ == "__main__":
    rows = [row for name,_ in GROUPS for row in json.loads(Path(__file__).with_name(name+".json").read_text())["fixtures"]]
    raw = struct.pack("<I",len(rows))+b"".join(struct.pack("<II",row["aggregate_entries"],row["objects"]) for row in rows)
    Path("tests/fixtures/display-lists/earliest-visual-budgets.bin").write_bytes(raw)
    verify_groups(sys.argv[1],GROUPS,"earliest-visual-verification.json",["modern_root","modern_lights","modern_bucket_poly","modern_item_style","modern_ui","modern_component_properties","modern_decals","modern_assembly","modern_body_tess","modern_part_tess","modern_material","modern_component","modern_item_annotation"])
