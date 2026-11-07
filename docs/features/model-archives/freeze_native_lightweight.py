"""Freeze semantic fields for the owned complete lightweight section."""
from pathlib import Path
import hashlib
import json
import sys
from freeze_blank_part import reference_snapshot
from freeze_lightweight_sections import scalar_rows

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(REFERENCE))
import modelprefix
from swcontainer import Document


def walk(path: str, value: object, rows: list[str], seen: set[int]) -> None:
    if isinstance(value, dict):
        if id(value) in seen: return
        seen.add(id(value))
        for key, child in value.items():
            if key.startswith("_") or key == "lightweight_end": continue
            child_path = path if path == "root" and key == "concept" else f"{path}.{key}"
            walk(child_path, child, rows, seen)
    elif isinstance(value, (list, tuple)):
        for index, child in enumerate(value): walk(f"{path}[{index}]", child, rows, seen)
    else: scalar_rows(path, value, rows)


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    fixture = WORKSPACE / "tests/fixtures/blank-part.SLDPRT"
    document = Document(fixture, table={})
    root, archive = modelprefix.read(document, through_lightweight=True)
    if archive.p != root["lightweight_end"]: raise ValueError("lightweight end mismatch")
    rows: list[str] = []
    walk("root", root, rows, set())
    payload = "".join(rows).encode("utf-8")
    if before != reference_snapshot(REFERENCE): raise ValueError("reference sources changed")
    (WORKSPACE / "tests/fixtures/model-archives/native-lightweight.txt").write_bytes(payload)
    report = {"fields": len(rows), "section_end": archive.p, "section_size": root["lightweight_size"],
              "wire_sha256": hashlib.sha256(archive.b[:archive.p]).hexdigest(),
              "oracle_sha256": hashlib.sha256(payload).hexdigest(), "source_hashes": before,
              "generator_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name("native-lightweight-reference.json").write_text(json.dumps(report, sort_keys=True, separators=(",", ":")) + "\n")
    print({key: value for key, value in report.items() if key != "source_hashes"})


if __name__ == "__main__":
    generate()
