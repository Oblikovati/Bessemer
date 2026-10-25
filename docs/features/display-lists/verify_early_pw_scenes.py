"""Verify new PW roots and unchanged historical activity oracles in fresh processes."""
from pathlib import Path
import hashlib
import json
import sys
from generate_early_pw_scenes import generate
from generate_legacy_zero import generate as zero_root
from generate_legacy_component import generate as component_root
from generate_named_branch_roots import generate as named_root
from verify_decal_processes import frozen_cases
from verify_earlier_processes import verify_groups


def frozen_activity_baselines() -> list[dict[str,object]]:
    groups = [("legacy-zero-roots",zero_root),("legacy-component-roots",component_root),("named-branch-roots",named_root)]
    report = []
    for name,generator in groups:
        manifest = Path(__file__).with_name(name+".json")
        fixture = Path("tests/fixtures/display-lists")/(name+".bin")
        records = frozen_cases(manifest,fixture,generator)
        report.append({"group":name,"roots":len(records),"sha256":hashlib.sha256(fixture.read_bytes()).hexdigest()})
    return report


def verify(executable: str) -> None:
    controls = frozen_activity_baselines()
    report_name = "early-pw-scene-verification.json"
    verify_groups(executable,[("early-pw-scene-roots",generate)],report_name,
        ["modern_pw_scene","modern_bucket","archive_objects","modern_root"])
    path = Path(__file__).with_name(report_name)
    report = json.loads(path.read_text(encoding="utf-8"))
    report["unchanged_frozen_activity_oracles"] = controls
    helper = Path(__file__).with_name("legacy_component_reference.py")
    report["reference_helper_sha256"] = hashlib.sha256(helper.read_bytes()).hexdigest()
    path.write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")


if __name__=="__main__":
    verify(sys.argv[1])
