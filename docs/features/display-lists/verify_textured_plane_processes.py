"""Replay frozen null-bitmap plane records with the shared independent process verifier."""
from pathlib import Path
import hashlib
import json
import platform
import sys
from generate_textured_planes import generate
from verify_decal_processes import frozen_cases
from verify_configuration_processes import frames,two_processes


def main(executable: str) -> None:
    cases = frozen_cases(Path(__file__).with_name("textured-planes.json"),Path("tests/fixtures/display-lists/textured-planes.bin"),generate)
    source,expected = frames(cases)
    trace_hash = two_processes(executable,source,expected)
    modules = ["textured_planes","modern_root"]
    provenance = {name:hashlib.sha256(Path(f"src/display_lists/{name}.rs").read_bytes()).hexdigest() for name in modules}
    report = {"fixtures":len(cases),"platform":platform.platform(),"two_processes_identical":True,"all_forced_bytes_match_independent_fixtures":True,"input_sha256":hashlib.sha256(source).hexdigest(),"result_sha256":hashlib.sha256(expected).hexdigest(),"trace_sha256":trace_hash,"source_sha256":provenance,"scope":"null-bitmap records only; byte reconstruction and process determinism; transform field oracles are checked separately by integration tests"}
    Path(__file__).with_name("textured-plane-verification.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(report))


if __name__=="__main__":
    main(sys.argv[1])
