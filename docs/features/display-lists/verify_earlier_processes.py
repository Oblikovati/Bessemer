"""Verify frozen 12000-range roots and buckets in two isolated reconstruction processes."""
from pathlib import Path
import hashlib
import json
import platform
import sys
from generate_earlier_range import generate as full_root
from generate_earlier_buckets import generate as bucket_root
from generate_earlier_geometry import generate as geometry_root
from verify_decal_processes import frozen_cases
from verify_configuration_processes import frames,two_processes


def verify_groups(executable: str, groups: list[tuple[str,object]], report_name: str, modules: list[str]) -> None:
    cases = []
    for name,generator in groups:
        cases += frozen_cases(Path(__file__).with_name(name+".json"),Path("tests/fixtures/display-lists")/(name+".bin"),generator)
    verify_records(executable,cases,report_name,modules)


def verify_records(executable: str, cases: list[tuple[dict[str,object],bytes]], report_name: str, modules: list[str]) -> None:
    source,expected = frames(cases)
    trace_hash = two_processes(executable,source,expected)
    provenance = {name:hashlib.sha256(Path(f"src/display_lists/{name}.rs").read_bytes()).hexdigest() for name in modules}
    report = {"fixtures":len(cases),"platform":platform.platform(),"two_processes_identical":True,"all_forced_bytes_match_independent_fixtures":True,"input_sha256":hashlib.sha256(source).hexdigest(),"result_sha256":hashlib.sha256(expected).hexdigest(),"trace_sha256":trace_hash,"source_sha256":provenance,"scope":"byte reconstruction and process determinism; field oracles checked separately by integration tests"}
    Path(__file__).with_name(report_name).write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(report))


def main(executable: str) -> None:
    groups = [("earlier-range-part",full_root),("earlier-range-assembly",full_root),("earlier-buckets-3d",bucket_root),("earlier-buckets-2d",bucket_root),("earlier-geometry",geometry_root)]
    modules = ["modern_root","modern_reference","modern_component","modern_part_tess","modern_bucket","modern_bucket_poly","legacy_bucket_poly"]
    verify_groups(executable,groups,"earlier-verification.json",modules)


if __name__=="__main__":
    main(sys.argv[1])
