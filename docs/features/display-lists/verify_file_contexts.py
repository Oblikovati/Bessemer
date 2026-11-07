"""Replay explicitly framed file contexts without replacing them with process defaults."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from generate_file_contexts import generate
from reference_corpus import frame_at
from verify_earlier_processes import verify_records


def records(name: str) -> list[tuple[dict[str,object],bytes]]:
    manifest = json.loads(Path(__file__).with_name(name+".json").read_text(encoding="utf-8"))
    cases,framed = [],[]
    for row in manifest["fixtures"]:
        raw,actual = generate(row["version"],row["kind"],row["scenario"],row["file_version"])
        assert actual==row
        payload,_ = frame_at(raw,18)
        cases.append((dict(row,external_ids=[]),payload))
        framed.append(raw)
    rebuilt = struct.pack("<I",len(cases))+b"".join(framed)
    assert hashlib.sha256(rebuilt).hexdigest()==manifest["sha256"]
    assert rebuilt==Path(f"tests/fixtures/display-lists/{name}.bin").read_bytes()
    return cases


def main(executable: str) -> None:
    cases = [record for name in ["file-context-9000","file-context-16000","file-context-19000","file-context-shaded"] for record in records(name)]
    modules = ["archive_decode","archive_types","modern_root","modern_bucket"]
    verify_records(executable,cases,"file-context-verification.json",modules)


if __name__=="__main__":
    main(sys.argv[1])
