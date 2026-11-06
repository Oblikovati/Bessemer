"""Source-bound record-144 fixtures with explicit matched-count context and raw numeric words."""
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
import flowmappedobjects
import flowresults

BITS = [0x8000000000000000, 0x7ff8000000001234, 0x7ff0000000000000,
        0xfff0000000000000, 1, 0x8000000000000001, 0x7ff0000000000001, 0, 0x3ff0000000000000]


def blob(raw: bytes) -> bytes:
    return struct.pack("<I", len(raw)) + raw


def words(values: list[int] | tuple[int, ...]) -> bytes:
    return struct.pack("<" + "Q" * len(values), *values)


def context_packet(context: list[int | None] | None) -> bytes:
    if context is None:
        return b"\0"
    return b"\1" + struct.pack("<I", len(context)) + b"".join(struct.pack("<Bi", value is not None, value or 0) for value in context)


def legacy_wire(counts: tuple[int, int, int]) -> bytes:
    raw = words(BITS + BITS[:7])
    for count in counts:
        raw += struct.pack("<i", count) + words(BITS[:count])
    return raw + words([BITS[index % len(BITS)] for index in range(3 * counts[0] * counts[1] * counts[2])])


def body_wire(version: int, factor: int, matched: int | None, dimensions: tuple[int, int, int]) -> bytes:
    pairs = [(101, 7), (101, -1), (-2147483648, 2147483647), (202, -1)]
    raw = struct.pack("<IB", version, 250) + words(BITS) + struct.pack("<i", len(pairs))
    raw += b"".join(struct.pack("<ii", *pair) for pair in pairs) + struct.pack("<i", factor)
    count = factor * (matched or 0)
    raw += words([BITS[index % len(BITS)] for index in range(count)])
    return raw + (legacy_wire(dimensions) if version == 5 else b"")


def list_wire(version: int, entries: list[bytes]) -> bytes:
    return struct.pack("<Ii", version, len(entries)) + b"".join(entries)


def owned_cases() -> list[tuple[str, bytes, list[int | None] | None]]:
    cases = []
    for outer in range(7):
        cases += [("empty", list_wire(outer, []), None), ("empty-context", list_wire(outer, []), [])]
        cases += [("nulls", list_wire(outer, [b"\0"] * 3), context) for context in (None, [None] * 3)]
        for body in range(7):
            for label, factor, matched, dimensions in [("matched", 2, 1, (2, 1, 1)), ("zero-factor", 0, None, (0, 2, 1)), ("zero-matched", 3, 0, (1, 0, 2))]:
                entry = bytes([1 + body * 37]) + body_wire(body, factor, matched, dimensions)
                cases.append((label, list_wire(outer, [entry]), None if matched is None else [matched]))
        first = b"\xff" + body_wire(5, 2, 2, (2, 2, 1))
        second = b"\x7f" + body_wire(6, 1, 1, (0, 0, 0))
        cases.append(("mixed", list_wire(outer, [b"\0", first, b"\0", second]), [None, 2, None, 1]))
        maximum = b"\x01" + body_wire(6, 0, 2147483647, (0, 0, 0))
        cases.append(("maximum-unused-context", list_wire(outer, [maximum]), [2147483647]))
    return cases


def source_cases() -> tuple[list[tuple[str, bytes, list[int | None] | None]], list[dict]]:
    corpus = json.loads((SOURCE / "research/flow-mapped-object-corpus.json").read_text())
    cases, audit = [], []
    for source in corpus["installed_sources"]:
        raw = Path(source["path"]).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == source["source_sha256"]
        record = flowresults.latest(flowresults.read(raw), 144)
        assert (record is None) == (source["projection"] is None)
        audit.append(dict(source_sha256=source["source_sha256"], present=record is not None))
        if record is not None:
            assert hashlib.sha256(record["payload"]).hexdigest() == source["payload_sha256"]
            assert flowmappedobjects.read_payload(record["payload"])["count"] == 0
            cases.append(("native-empty-fld", record["payload"], None))
    for row in corpus["synthetic_fixtures"]:
        assert row["kind"] == "synthetic_native_branch_fixture"
        payload = bytes.fromhex(row["payload_hex"])
        assert hashlib.sha256(payload).hexdigest() == row["payload_sha256"]
        cases.append(("source-synthetic", payload, row["matched_counts"]))
    return cases, audit


def words_packet(block: dict) -> bytes:
    return struct.pack("<I", len(block["raw_bits"])) + words(block["raw_bits"])


def object_packet(obj: dict) -> bytes:
    packet = struct.pack("<IIB", obj["start"], obj["end"], obj["presence_byte"])
    if not obj["present"]:
        return packet
    packet += struct.pack("<IB", obj["body_version"], obj["state_byte"]) + words_packet(obj["numeric"])
    packet += struct.pack("<I", len(obj["pairs"])) + b"".join(struct.pack("<ii", *pair) for pair in obj["pairs"])
    packet += struct.pack("<IBiI", obj["stored_factor"], obj["matched_object_count"] is not None,
                          obj["matched_object_count"] or 0, obj["mapped_value_count"]) + words_packet(obj["mapped_values"])
    legacy = obj["legacy_version_5"]
    packet += bytes([legacy is not None])
    if legacy is not None:
        packet += words_packet(legacy["prefix"]) + b"".join(map(words_packet, legacy["arrays"])) + words_packet(legacy["tripled_values"])
    return packet + bytes([obj["native_object_context_validated"]])


def case_packet(case: tuple[str, bytes, list[int | None] | None]) -> bytes:
    name, payload, context = case
    parsed = flowmappedobjects.read_payload(payload, context)
    prefix = blob(name.encode()) + blob(payload) + context_packet(context)
    return prefix + struct.pack("<II", parsed["version"], parsed["count"]) + b"".join(map(object_packet, parsed["objects"]))


def freeze(output_root: Path = ROOT) -> None:
    sources, audit = source_cases()
    cases = sources + owned_cases()
    packet = b"FMO1" + struct.pack("<I", len(cases)) + b"".join(map(case_packet, cases))
    fixture = output_root / "tests/fixtures/flow-mapped-objects/mapped.bin.gz"
    fixture.parent.mkdir(parents=True, exist_ok=True)
    fixture.write_bytes(gzip.compress(packet, mtime=0))
    report = dict(cases=len(cases), native_empty_payloads=1, native_payload_bytes=8, populated_native_payloads=0,
                  source_synthetic_payloads=2, installed_sources=audit, bytes=len(packet),
                  sha256=hashlib.sha256(packet).hexdigest(),
                  reference_sha256=hashlib.sha256((SOURCE / "flowmappedobjects.py").read_bytes()).hexdigest(),
                  scope="Explicit-context synthetic populated bodies; no automatic runtime-map matching or additional native-byte credit")
    destination = output_root / "docs/features/flow-mapped-objects/fixture-verification.json"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
