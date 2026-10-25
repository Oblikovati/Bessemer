"""Verify frozen 13000-range roots and buckets in two isolated reconstruction processes."""
from pathlib import Path
import hashlib
import json
import platform
import sys
from generate_transition_range import generate as full_root
from generate_transition_buckets import generate as bucket_root
from verify_decal_processes import frozen_cases
from verify_configuration_processes import frames,two_processes


def main(executable: str) -> None:
    cases = []
    for name,generator in [("transition-range-part",full_root),("transition-range-assembly",full_root),("transition-buckets-3d",bucket_root),("transition-buckets-2d",bucket_root)]:
        cases += frozen_cases(Path(__file__).with_name(name+".json"),Path("tests/fixtures/display-lists")/(name+".bin"),generator)
    source,expected = frames(cases)
    trace_hash = two_processes(executable,source,expected)
    modules = ["modern_root","modern_body_tess","modern_bucket_poly","legacy_bucket_poly","markup_ink"]
    provenance = {name:hashlib.sha256(Path(f"src/display_lists/{name}.rs").read_bytes()).hexdigest() for name in modules}
    report = {"fixtures":len(cases),"platform":platform.platform(),"two_processes_identical":True,"all_forced_bytes_match_independent_fixtures":True,"input_sha256":hashlib.sha256(source).hexdigest(),"result_sha256":hashlib.sha256(expected).hexdigest(),"trace_sha256":trace_hash,"source_sha256":provenance,"scope":"byte reconstruction and process determinism; field oracles checked separately by integration tests"}
    Path(__file__).with_name("transition-verification.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(report))


if __name__=="__main__":
    main(sys.argv[1])
