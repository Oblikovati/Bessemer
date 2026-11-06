"""Freeze complete kind-one bodies with uninterpreted binary64 patterns."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "flow-metadata"))
from wire_packet import ROOT, SOURCE, blob, counted, packed, native_records, optional
import flowmediumobjects as schema

MASK = (1 << 64) - 1
PATTERNS = [0, 1, 0x8000000000000000, 0x7ff0000000000000, 0xfff0000000000000,
            0x7ff8000000000042, 0x7ff0000000000001, 0x7fefffffffffffff,
            *[1 << bit for bit in range(64)], *[MASK ^ (1 << bit) for bit in range(64)]]


def patterns(count: int, seed: int) -> list[int]:
    return [PATTERNS[(seed + index) % len(PATTERNS)] for index in range(count)]


def curve(count: int, flag: int, seed: int) -> bytes:
    raw = b"\0" + struct.pack("<Qi", PATTERNS[seed % len(PATTERNS)], count)
    raw += packed(patterns(count, seed), "Q") + packed(patterns(count, seed + 1), "Q")
    raw += bytes((flag,))
    if flag:
        raw += packed(patterns(count, seed + 2), "Q") + packed(patterns(count, seed + 3), "Q")
    return raw


def text(raw: bytes) -> bytes:
    assert len(raw) % 2 == 0 and (not raw or raw[-2:] == b"\0\0")
    return struct.pack("<i", len(raw) // 2) + raw


def body(base: int, state: int, count: int, flag: int, name: bytes, seed: int) -> bytes:
    raw = bytes((0, base)) + bytes((seed + index) % 256 for index in range(16)) + text(name)
    raw += b"".join(curve(count, flag, seed + ordinal) for ordinal in range(7))
    raw += struct.pack("<i", count) + b"".join(packed(patterns(count, seed + index), "Q") for index in range(2))
    second = count // 2
    raw += struct.pack("<i", second) + b"".join(packed(patterns(second, seed + index), "Q") for index in range(4))
    return raw + packed(patterns(4, seed), "Q") + bytes((state,))


def native_cases() -> list[tuple[str, bytes]]:
    result = []
    for name, raw in native_records(424):
        profile = schema.read_record_424_kind1_profile(dict(identifier=424, size=len(raw), payload=raw), string_mode=0, fresh_backend=True)
        for entry in profile["first_table"]:
            start = entry["start"] + 20
            result.append((f"native-{name}-body-tail", raw[start:]))
            result.append((f"native-{name}-body-exact", raw[start:entry["end"]]))
    return result


def cases() -> list[tuple[str, bytes]]:
    result = native_cases()
    for value in range(256):
        result.append((f"base-state-{value}", body(value, value ^ 255, value % 4, value, b"a\0\0\0", value)))
        for count in (0, 1):
            result.append((f"flag-{value}-count-{count}", body(19, 7, count, value, b"", value) + b"opaque\0\xff"))
    for index, pattern in enumerate(PATTERNS):
        result.append((f"bits-{pattern:016x}-{index}", body(255, 128, 3, 255, b"\0\xd8\0\0", index)))
    for index, raw in enumerate((b"", b"\0\0", b"a\0\0\0b\0\0\0", b"\xff\xfe\xff\xff\x00\xd8\x00\xdc\0\0", b"x\0" * 65535 + b"\0\0")):
        result.append((f"text-{index}", body(0, 0, 0, 0, raw, index)))
    result.append(("large-arrays", body(1, 2, 1024, 128, b"\0\0", 3)))
    return result


def text_packet(value: dict) -> bytes:
    native = value["text"].encode("utf-16le", "surrogatepass")
    return packed([value[key] for key in ("start", "end", "stored_units", "text_start")], "I") + blob(value["raw_bytes"]) + blob(native)


def curve_packet(value: dict) -> bytes:
    raw = struct.pack("<IIBQI", value["start"], value["end"], value["version"], value["scalar_pattern"], value["count"])
    raw += counted(value["first_patterns"], "Q") + counted(value["second_patterns"], "Q") + bytes((value["extra_flag_byte"],))
    raw += optional(value["count"] if value["extra_flag_byte"] else None, "I")
    return raw + counted(value["extra_first_patterns"], "Q") + counted(value["extra_second_patterns"], "Q")


def case_packet(name: str, raw: bytes) -> bytes:
    value = schema.read_kind1_body(raw, string_mode=0)
    out = blob(name.encode()) + blob(raw) + bytes((value["version"], value["base_byte"], value["base_byte_dispatch"]))
    out += blob(value["base_key_raw"]) + text_packet(value["name"])
    out += struct.pack("<I", len(value["curves"])) + b"".join(map(curve_packet, value["curves"]))
    for key, arrays in (("first_count", "first_arrays"), ("second_count", "second_arrays")):
        out += struct.pack("<II", value[key], len(value[arrays])) + b"".join(counted(row, "Q") for row in value[arrays])
    out += counted(value["scalar_patterns"], "Q") + bytes((value["state_byte"], value["native_state_after_load"]))
    out += struct.pack("<I", value["body_end"]) + blob(value["opaque_tail"])
    return out + bytes((value["body_framing_complete"], value["native_runtime_state_reproduced"], value["engineering_columns_assigned"]))


def freeze(output_root: Path = ROOT) -> None:
    rows = cases()
    packet = b"FMO1" + struct.pack("<I", len(rows)) + b"".join(case_packet(*row) for row in rows)
    destination = output_root / "tests/fixtures/flow-medium-objects/kind1.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(rows), bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowmediumobjects.py").read_bytes()).hexdigest(),
                 scope="Complete kind-one serialized body, raw numeric and UTF-16 patterns, native loaded-state projection and exact following tail. No engineering meanings, numeric evaluation or native runtime acceptance.")
    report = output_root / "docs/features/flow-medium-objects/baseline-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
