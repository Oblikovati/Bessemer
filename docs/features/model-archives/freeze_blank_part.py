"""Freeze complete reference reads of the owned model stream for feature #44."""
from dataclasses import asdict
from pathlib import Path
from types import ModuleType
import hashlib
import importlib
import json
import sys
from primitive_inventory import PrimitiveRead, observe_primitives, verify_primitive_coverage
from map_inventory import map_slot_inventory

WORKSPACE = Path(__file__).resolve().parents[3]


def reference_snapshot(root: Path) -> dict[str, str]:
    """Bind reference sources; e.g. reference_snapshot(Path('../inventor-interface/solidworks'))."""
    return {str(path.relative_to(root)).replace("\\", "/"): hashlib.sha256(path.read_bytes()).hexdigest()
            for path in sorted(root.rglob("*.py")) if "__pycache__" not in path.parts}


def require_complete_model(position: int, size: int) -> None:
    """Reject a partial root; e.g. require_complete_model(24028, 24028)."""
    if position != size:
        raise ValueError(f"model end {position}; expected stream end {size}")


def source_module(name: str, root: Path) -> ModuleType:
    """Prevent a foreign imported reader; e.g. source_module('modelprefix', source_root)."""
    module = importlib.import_module(name)
    origin = Path(module.__file__ or "").resolve()
    if origin.parent != root.resolve():
        raise ValueError(f"module {name} at {origin}; expected source directory {root.resolve()}")
    return module


def oracle_payload(fixture: Path, raw: bytes, reads: list[PrimitiveRead],
                   sources: dict[str, str], version: int) -> dict[str, object]:
    """Bind an exact primitive oracle; e.g. oracle_payload(fixture, raw, reads, hashes, 19000)."""
    verify_primitive_coverage(raw, reads)
    return {"feature_issue": 44, "fixture": str(fixture.relative_to(WORKSPACE)).replace("\\", "/"),
            "fixture_sha256": hashlib.sha256(fixture.read_bytes()).hexdigest(),
            "stream": "Contents/Config-0", "stream_sha256": hashlib.sha256(raw).hexdigest(),
            "stream_bytes": len(raw), "model_version": version,
            "context": "ordinary_external_xml", "root_class": "moPart_c",
            "scope": "reference primitive partition; semantic fields and Bessemer behavior require separate checks",
            "source_hashes": sources, "reads": [asdict(read) for read in reads]}


def freeze_blank_part(source_root: Path, fixture: Path) -> dict[str, object]:
    """Read the complete independent baseline; e.g. freeze_blank_part(source_root, fixture)."""
    before = reference_snapshot(source_root)
    sys.path.insert(0, str(source_root))
    container = source_module("swcontainer", source_root)
    archive = source_module("swarchive", source_root)
    model = source_module("modelprefix", source_root)
    fixture_bytes = fixture.read_bytes()
    document = container.Document(fixture_bytes, table={})
    raw = document.read("Contents/Config-0")
    with observe_primitives(archive.Archive, raw) as reads:
        _, cursor = model.read(document, through_part_resolved=True)
    require_complete_model(cursor.p, len(raw))
    if fixture.read_bytes() != fixture_bytes:
        raise ValueError(f"fixture {fixture} changed during observation; expected unchanged input bytes")
    if before != reference_snapshot(source_root):
        raise ValueError("reference sources changed during observation; expected unchanged source hashes")
    payload = oracle_payload(fixture, raw, reads, before, cursor.version)
    payload.update(map_slots=map_slot_inventory(cursor.map), loaded_class_counts=cursor.classes)
    return payload


if __name__ == "__main__":
    reference_root = WORKSPACE.parent / "inventor-interface/solidworks"
    frozen = freeze_blank_part(reference_root, WORKSPACE / "tests/fixtures/blank-part.SLDPRT")
    destination = Path(__file__).with_name("blank-part-primitive-oracle.json")
    destination.write_text(json.dumps(frozen, sort_keys=True, separators=(",", ":")) + "\n", encoding="utf-8")
    print(f"Frozen {len(frozen['reads'])} reads covering {frozen['stream_bytes']} bytes")
