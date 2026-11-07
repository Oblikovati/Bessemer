"""Deterministic fixture framing and audited native metadata record selection."""
import hashlib
import json
import struct
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(SOURCE))
import flowmetadataprefix
import flowresults


def packed(values: tuple | list, shape: str) -> bytes:
    return struct.pack("<" + shape * len(values), *values)


def blob(raw: bytes) -> bytes:
    return struct.pack("<I", len(raw)) + raw


def counted(values: tuple | list, shape: str) -> bytes:
    return struct.pack("<I", len(values)) + packed(values, shape)


def optional(value: int | None, shape: str) -> bytes:
    return b"\0" if value is None else b"\1" + struct.pack("<" + shape, value)


def native_records(identifier: int) -> list[tuple[str, bytes]]:
    rows = json.loads((SOURCE / "research/flow-metadata-prefix-corpus.json").read_text())
    result = []
    for row in rows:
        expected = next((record for record in row["records"] if record["identifier"] == identifier), None)
        if expected is None:
            continue
        raw = Path(row["path"]).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == row["source_sha256"]
        record = flowresults.latest(flowresults.read(raw), identifier)
        assert hashlib.sha256(record["payload"]).hexdigest() == expected["payload_sha256"]
        result.append((Path(row["path"]).suffix[1:], record["payload"]))
    return result
