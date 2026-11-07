"""Freeze record-402 fields independently before implementing the Rust API."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import sys
import zlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(SOURCE))
import flowlinkrecords
import flowresults

PATTERNS = (0, 0x8000000000000000, 0x7ff8000000001234, 0x7ff0000000000001,
            0x7ff0000000000000, 0xfff0000000000000, 0xffffffffffffffff)


def numbers(values: tuple | list, shape: str) -> bytes:
    return struct.pack("<" + shape * len(values), *values)


def counted(values: tuple | list, shape: str) -> bytes:
    return struct.pack("<I", len(values)) + numbers(values, shape)


def blob(value: bytes) -> bytes:
    return struct.pack("<I", len(value)) + value


def optional(value: int | None) -> bytes:
    return b"\0" if value is None else b"\1" + struct.pack("<i", value)


def entry_wire(ordinal: int, profile: str, code: int) -> bytes:
    references = (-1, -1) if profile == "sentinels" else (-2147483648, 2147483647) if profile == "unbounded" else (-1, ordinal)
    variable = () if profile == "zero" else tuple((i if i % 2 else -i) for i in range(255 if profile == "max-variable" else 3))
    return numbers(references, "i") + numbers(PATTERNS, "Q") + bytes((code, len(variable))) + numbers(variable, "i")


def tail_wire(version: int, profile: str) -> bytes:
    count = 0 if profile == "zero" else 3
    members = [-2147483648, -1, 0, 2147483647, 17, -19, count, 23, -29, 31, -37]
    if profile == "negative-length":
        members[6] = -2147483648
    raw = numbers(members, "i")
    raw += numbers((-7,) if version == 2 else (-7, 0, 2147483647) if version == 3 else (), "i")
    control = 0 if profile == "control-zero" else -1 if profile == "control-negative" else 2
    if version >= 5:
        raw += struct.pack("<i", control)
        if control > 0 and members[6] > 0:
            raw += numbers((-2147483648, 0, 2147483647), "i")
    if version >= 6:
        raw += counted(PATTERNS[:count], "Q")
        if version >= 8:
            raw += numbers((-1, -2147483648, 2147483647)[:count], "i")
    if version >= 7:
        values = (0xffffffffffffffff, 0x8000000000000000, 1) if version == 9 else (-2147483648, -1, 2147483647)
        raw += counted(values[:count], "Q" if version == 9 else "i")
    return raw


def owned_cases() -> list[dict]:
    cases = []
    profiles = ("zero", "full", "sentinels", "unbounded", "max-variable", "control-zero", "control-negative", "negative-length")
    for version in range(10):
        for profile in profiles:
            count = 0 if profile == "zero" else 2
            decoded = struct.pack("<2i", version, count)
            decoded += b"".join(entry_wire(i, profile, 31 + i) for i in range(count)) + tail_wire(version, profile)
            contexts = (None,) if profile == "unbounded" else (None, 0 if profile in ("zero", "sentinels") else 2, 2147483647)
            cases += transport_cases(f"v{version}-{profile}", decoded, contexts)
    for code in range(256):
        decoded = struct.pack("<2i", 0, 1) + entry_wire(0, "full", code) + tail_wire(0, "full")
        cases += transport_cases(f"v0-code-{code}", decoded, (None,))
    return cases


def transport_cases(name: str, decoded: bytes, contexts: tuple) -> list[dict]:
    return [dict(name=name, decoded=decoded, pointer_count=context, compressed=compressed,
                 stored=zlib.compress(decoded, 9) if compressed else decoded)
            for context in contexts for compressed in (False, True)]


def native_cases() -> list[dict]:
    row = json.loads((SOURCE / "research/flow-result-carrier-corpus.json").read_text())[1]
    raw = Path(row["path"]).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == row["source_sha256"]
    record = flowresults.latest(flowresults.read(raw), 402)
    expected = next(item for item in row["records"] if item["identifier"] == 402)
    assert hashlib.sha256(record["payload"]).hexdigest() == expected["payload_sha256"]
    return [dict(name="native-cpt-402", stored=record["payload"], decoded=zlib.decompress(record["payload"]),
                 compressed=True, pointer_count=context) for context in (None, 5952)]


def entry_packet(entry: dict) -> bytes:
    raw = numbers([entry[key] for key in ("ordinal", "start", "end")], "I")
    raw += numbers(entry["reference_indices"], "i") + numbers(entry["wide_bits"], "Q")
    raw += bytes((entry["code_byte"], entry["native_code_byte"]))
    return raw + counted(entry["variable_i32"], "i")


def numeric_packet(tail: dict | None) -> bytes:
    if tail is None:
        return b"\0"
    raw = b"\1" + numbers((tail["start"], tail["end"]), "I") + counted(tail["wide_bits"], "Q")
    raw += bytes((tail["stored_indices"] is not None,))
    if tail["stored_indices"] is not None:
        raw += numbers(tail["stored_indices"], "i")
    return raw + optional(tail["runtime_default_index"])


def final_packet(tail: dict | None) -> bytes:
    if tail is None:
        return b"\0"
    raw = b"\1" + numbers((tail["start"], tail["end"], tail["element_bytes"]), "I")
    return raw + counted(tail["raw_values"], "Q" if tail["element_bytes"] == 8 else "i")


def case_packet(case: dict) -> bytes:
    parsed = flowlinkrecords.read_record(dict(identifier=402, payload=case["stored"], size=len(case["stored"])),
        compression="zlib" if case["compressed"] else "none", global_pointer_count=case["pointer_count"])
    raw = blob(case["name"].encode()) + bytes((case["compressed"],)) + optional(case["pointer_count"])
    raw += blob(case["stored"]) + blob(case["decoded"]) + struct.pack("<iI", parsed["version"], parsed["count"])
    raw += b"".join(map(entry_packet, parsed["entries"])) + struct.pack("<II", parsed["variable_integer_count"], parsed["tail_start"])
    raw += numbers([parsed["member_values"][hex(offset)] for offset in flowlinkrecords.TAIL_MEMBERS], "i")
    raw += counted(parsed["discarded_words"], "i") + optional(parsed["member_2f0_control"])
    raw += numbers((parsed["conditional_start"], parsed["conditional_end"]), "I") + counted(parsed["conditional_i32"], "i")
    raw += numeric_packet(parsed["numeric_tail"]) + final_packet(parsed["final_tail"])
    return raw + bytes((parsed["global_pointer_indices_checked"], parsed["native_runtime_state_reproduced"], parsed["engineering_roles_resolved"]))


def freeze(output_root: Path = ROOT) -> None:
    cases = native_cases() + owned_cases()
    packet = b"FLR1" + struct.pack("<I", len(cases)) + b"".join(map(case_packet, cases))
    destination = output_root / "tests/fixtures/flow-link-records/links.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(cases), native_views=2, bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowlinkrecords.py").read_bytes()).hexdigest(),
                 scope="Every legacy version, all code-byte values, sentinel/context distinctions, raw fields and versioned tails; no native runtime or engineering roles")
    report = output_root / "docs/features/flow-link-records/fixture-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
