"""Freeze independent complete physical Flow partitions and original installed hashes."""
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
import flowresults


def blob(value: bytes) -> bytes:
    return struct.pack("<Q", len(value)) + value


def owned(version: int, records: list[tuple[int, bytes]]) -> bytes:
    return struct.pack("<H", version) + b"".join(
        struct.pack("<IQ", identifier, len(payload)) + payload for identifier, payload in records
    )


def native_cases() -> list[tuple[str, bytes]]:
    corpus = json.loads((SOURCE / "research/flow-result-carrier-corpus.json").read_text())
    cases = []
    for entry in corpus:
        raw = Path(entry["path"]).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == entry["source_sha256"]
        decoded = flowresults.read(raw)
        assert len(decoded["records"]) == len(entry["records"])
        for actual, expected in zip(decoded["records"], entry["records"]):
            assert (actual["identifier"], actual["start"], actual["end"]) == (
                expected["identifier"], expected["start"], expected["end"]
            )
            assert hashlib.sha256(actual["payload"]).hexdigest() == expected["payload_sha256"]
        cases.append((Path(entry["path"]).name, raw))
    return cases


def packet_case(name: str, raw: bytes) -> bytes:
    decoded = flowresults.read(raw)
    packet = blob(name.encode()) + blob(raw) + struct.pack("<I", len(decoded["records"]))
    for record in decoded["records"]:
        packet += struct.pack("<IQQ", record["identifier"], record["start"], record["size"])
    return packet


def freeze() -> None:
    cases = native_cases() + [
        ("empty", owned(7, [])),
        ("duplicates", owned(8, [(123, b"first"), (0xffffffff, b""), (123, b"last")])),
        ("reserved", owned(7, [(0xffffff9b, b"index"), (0xffffff9d, b""), (0, b"\0\xff")])),
    ]
    packet = b"FLC1" + struct.pack("<I", len(cases)) + b"".join(packet_case(*case) for case in cases)
    target = ROOT / "tests/fixtures/flow-carriers/carriers.bin.gz"
    target.write_bytes(gzip.compress(packet, mtime=0))
    report = dict(cases=len(cases), native_records=66, packet_bytes=len(packet),
                  packet_sha256=hashlib.sha256(packet).hexdigest(),
                  source_sha256=hashlib.sha256((SOURCE / "flowresults.py").read_bytes()).hexdigest(),
                  inputs=[dict(name=name, bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest()) for name, raw in cases])
    (ROOT / "docs/features/flow-carriers/fixture-verification.json").write_text(
        json.dumps(report, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(dict(cases=len(cases), packet_bytes=len(packet)))


if __name__ == "__main__":
    freeze()
