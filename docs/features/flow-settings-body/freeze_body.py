"""Freeze explicitly synthetic record-511 versions and native absence evidence."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(SOURCE))
import flowsettingsbody
import flowresults

CONTEXTS = [None, 0, 1, -1, -2147483648, 2147483647]


def corpus() -> dict:
    return json.loads((SOURCE / "research/flow-settings-body-corpus.json").read_text())


def blob(raw: bytes) -> bytes:
    return struct.pack("<I", len(raw)) + raw


def field_packet(name: str, field: dict, payload: bytes) -> bytes:
    offset, size = field["offset"], field["size"]
    shape = {"u8": 0, "u32": 1}.get(field["kind"], 2 if size == 4 else 3)
    return struct.pack("<BIB", int(name.removeprefix("member_"), 16), offset, shape) + blob(payload[offset:offset + size])


def context_packet(payload: bytes, context: int | None) -> bytes:
    legacy = flowsettingsbody.read_payload(payload, context)["member_0f"]
    status = {"requires_runtime_integer_slot_88": 0, "derived_runtime_context": 1, "stored_u8": 2}[legacy["status"]]
    return struct.pack("<BiBB", context is not None, context or 0, status, legacy["value"] or 0)


def body_packet(row: dict) -> bytes:
    assert row["kind"] == "synthetic_native_branch_fixture"
    payload = bytes.fromhex(row["payload_hex"])
    assert hashlib.sha256(payload).hexdigest() == row["payload_sha256"]
    parsed = flowsettingsbody.read_payload(payload)
    assert parsed["end"] == row["end"]
    fields = [(name, field) for name, field in parsed["fields"].items() if name.startswith("member_")]
    packet = struct.pack("<I", row["version"]) + blob(payload) + struct.pack("<I", len(fields))
    packet += b"".join(field_packet(name, field, payload) for name, field in fields)
    return packet + struct.pack("<I", len(CONTEXTS)) + b"".join(context_packet(payload, context) for context in CONTEXTS)


def audit_absences(rows: list[dict]) -> list[dict]:
    retained = []
    for row in rows:
        raw = Path(row["path"]).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == row["source_sha256"]
        assert not row["record_511_present"]
        assert not flowsettingsbody.read_record(flowresults.read(raw))["present"]
        retained.append(dict(source_sha256=row["source_sha256"], record_511_present=False))
    return retained


def freeze(output_root: Path = ROOT) -> None:
    source = corpus()
    rows = source["synthetic_fixtures"]
    packet = b"FSB1" + struct.pack("<I", len(rows)) + b"".join(map(body_packet, rows))
    fixture = output_root / "tests/fixtures/flow-settings-body/body.bin.gz"
    fixture.parent.mkdir(parents=True, exist_ok=True)
    fixture.write_bytes(gzip.compress(packet, mtime=0))
    report = dict(synthetic_versions=len(rows), context_projections=len(rows) * len(CONTEXTS),
                  bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                  native_absences=audit_absences(source["installed_sources"]),
                  reference_sha256=hashlib.sha256((SOURCE / "flowsettingsbody.py").read_bytes()).hexdigest(),
                  scope="Synthetic populated fixtures only; installed sources establish absence, not populated-body acceptance")
    destination = output_root / "docs/features/flow-settings-body/fixture-verification.json"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
