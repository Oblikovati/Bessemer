"""Replay frozen decal roots through the shared two-process byte reconstruction gate."""
from pathlib import Path
from collections.abc import Callable
import hashlib
import json
import platform
import struct
import sys
from generate_decals import generate
from reference_corpus import frame_at
from verify_configuration_processes import frames,two_processes


def frozen_cases(manifest_path: Path, fixture_path: Path, generator: Callable[[int,str,int],tuple[bytes,dict[str,object]]]) -> list[tuple[dict[str,object],bytes]]:
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    cases,framed = [],[]
    for row in manifest["fixtures"]:
        raw,actual = generator(row["version"],row["kind"],row["scenario"])
        assert row==actual
        payload,_ = frame_at(raw,6)
        # #39: omitted declarations default to empty; explicit directory context must survive.
        cases.append((dict(row,external_ids=row.get("external_ids",[])),payload))
        framed.append(raw)
    rebuilt = struct.pack("<I",len(cases))+b"".join(framed)
    assert hashlib.sha256(rebuilt).hexdigest()==manifest["sha256"]
    assert rebuilt==fixture_path.read_bytes()
    return cases


def main(executable: str) -> None:
    cases = frozen_cases(Path(__file__).with_name("decals.json"),Path("tests/fixtures/display-lists/decals.bin"),generate)
    source,expected = frames(cases)
    trace_hash = two_processes(executable,source,expected)
    modules = ["modern_decals","modern_material","modern_part_tess","modern_body_tess","modern_assembly"]
    provenance = {name:hashlib.sha256(Path(f"src/display_lists/{name}.rs").read_bytes()).hexdigest() for name in modules}
    report = {"fixtures":len(cases),"platform":platform.platform(),"two_processes_identical":True,"all_forced_bytes_match_independent_fixtures":True,"input_sha256":hashlib.sha256(source).hexdigest(),"result_sha256":hashlib.sha256(expected).hexdigest(),"trace_sha256":trace_hash,"source_sha256":provenance,"scope":"byte reconstruction and process determinism; independent decal scalar oracles are checked by integration tests"}
    Path(__file__).with_name("decal-verification.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(report))


if __name__=="__main__":
    main(sys.argv[1])
