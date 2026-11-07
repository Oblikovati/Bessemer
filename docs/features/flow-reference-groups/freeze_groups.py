"""Freeze complete record-416 wire fields with explicit mapping-branch context."""
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
import flowreferencegroups
import flowresults

WORDS = (0, 0x80000000, 0x7fc01234, 0x7f800001, 0x7f800000, 0xff800000, 1, 0xffffffff)
WIDE = (0, 0x8000000000000000, 0x7ff8000000001234, 0x7ff0000000000001,
        0x7ff0000000000000, 0xfff0000000000000, 1, 0xffffffffffffffff)
PROFILES = ("empty", "minimum", "rich", "flags-zero", "flags-noncanonical", "first-only",
            "second-only", "negative-secondary", "positive-secondary", "zero-secondary", "nondefault-header", "many")


def packed(values: tuple | list, shape: str) -> bytes:
    return struct.pack("<" + shape * len(values), *values)


def blob(raw: bytes) -> bytes:
    return struct.pack("<I", len(raw)) + raw


def optional(value: int | None, shape: str) -> bytes:
    return b"\0" if value is None else b"\1" + struct.pack("<" + shape, value)


def counted(values: tuple | list, shape: str) -> bytes:
    return struct.pack("<I", len(values)) + packed(values, shape)


def patterns(count: int, wide: bool = False) -> bytes:
    choices = WIDE if wide else WORDS
    return packed([choices[index % len(choices)] for index in range(count)], "Q" if wide else "I")


def descriptors(version: int, name: str) -> tuple[int, ...]:
    maximum = 255 if version == 1 else 65535
    if name in ("minimum", "empty"):
        return (0,) * 7
    first = maximum if name == "maximum" else 3
    return (0, maximum, 1, 127, 128, 255, first)


def secondary(version: int, name: str, raw_count: int | None) -> int:
    if raw_count is not None:
        return raw_count if raw_count < 128 else raw_count - 256
    if name in ("minimum", "empty", "zero-secondary"):
        return 0
    if name == "negative-secondary":
        return -128 if version == 1 else -(1 << 31) if version >= 10 else 0
    if name == "maximum":
        return 127 if version == 1 else 65535
    return 7 if name == "positive-secondary" else 2


def selectors(version: int, name: str) -> bytes:
    values = (0, 0, 0) if name in ("minimum", "flags-zero") else (128, 255, 2)
    if name == "flags-noncanonical":
        values = (255, 128, 255)
    prefix = b"\xff" if 11 <= version <= 13 else b""
    return prefix + bytes(values[:1 + (version >= 5) + (version >= 17)])


def entry_wire(version: int, name: str, ordinal: int, raw_count: int | None = None) -> bytes:
    references = () if name == "minimum" or ordinal == 1 else (-1, 0, -(1 << 31), (1 << 31) - 1)
    raw = struct.pack("<i", len(references))
    if version < 12:
        raw += struct.pack("<i", -2147483648 + ordinal)
    raw += packed(references, "i") + selectors(version, name)
    desc = descriptors(version, name)
    raw += packed(desc, "B" if version == 1 else "H") + patterns(desc[6])
    count = secondary(version, name, raw_count)
    raw += struct.pack("<b" if version == 1 else "<H" if version < 10 else "<i", count)
    raw += patterns(max(count, 0))
    if version >= 16:
        raw += struct.pack("<H", 0xf0aa)
    return raw


def group_wire(version: int, name: str, group: int, mapping: bool, raw_count: int | None) -> bytes:
    count = 0 if name == "empty" or name == ("second-only" if group == 0 else "first-only") else 3 if name == "many" else 1
    raw = struct.pack("<i", count) + b"".join(entry_wire(version, name, ordinal, raw_count) for ordinal in range(count))
    if mapping and group == 0 and count:
        raw += struct.pack("<i", -17) + packed([(-1 if index % 2 == 0 else 2147483647) for index in range(count * 2)], "i")
    if group == 1 or version >= 15:
        raw += patterns(count * 3, True)
    return raw


def profile(version: int, name: str, mapping: bool, raw_count: int | None = None) -> bytes:
    raw = struct.pack("<i", version)
    if version < 6:
        raw += struct.pack("<i", -2147483648)
    if version >= 3:
        raw += struct.pack("<i", -999 if name == "nondefault-header" else 2)
    return raw + b"".join(group_wire(version, name, group, mapping, raw_count) for group in range(2))


def transports(name: str, decoded: bytes, mapping: bool) -> list[dict]:
    return [dict(name=name, decoded=decoded, mapping=mapping, compressed=compressed,
                 stored=zlib.compress(decoded, 9) if compressed else decoded) for compressed in (False, True)]


def owned_cases() -> list[dict]:
    cases = []
    for version in range(18):
        for mapping in (False, True):
            for name in PROFILES:
                cases += transports(f"v{version}-{name}-mapping-{int(mapping)}", profile(version, name, mapping), mapping)
    for raw_count in range(256):
        cases += transports(f"v1-raw-secondary-{raw_count}", profile(1, "minimum", False, raw_count), False)
    for version in (0, 1, 9, 10, 17):
        for mapping in (False, True):
            cases += transports(f"v{version}-maximum-mapping-{int(mapping)}", profile(version, "maximum", mapping), mapping)
    return cases


def native_case() -> dict:
    row = json.loads((SOURCE / "research/flow-reference-groups-corpus.json").read_text())[1]
    raw = Path(row["path"]).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == row["source_sha256"]
    record = flowresults.latest(flowresults.read(raw), 416)
    assert hashlib.sha256(record["payload"]).hexdigest() == row["payload_sha256"]
    decoded = zlib.decompress(record["payload"])
    assert hashlib.sha256(decoded).hexdigest() == row["decoded_sha256"]
    return dict(name="native-cpt-416", stored=record["payload"], decoded=decoded, compressed=True, mapping=False)


def entry_packet(entry: dict) -> bytes:
    raw = packed([entry[key] for key in ("start", "end", "reference_count")], "I")
    raw += optional(entry["member_20_i32"], "i") + packed(entry["reference_words"], "i")
    raw += optional(entry["discarded_prefix_byte"], "B") + bytes((entry["state_byte"],))
    raw += optional(entry["flag_2_byte"], "B") + optional(entry["flag_4_byte"], "B")
    raw += packed(entry["descriptors"], "H") + counted(entry["first_words"], "I")
    raw += struct.pack("<i", entry["second_count_i32"]) + optional(entry["second_count_raw_small"], "H")
    return raw + counted(entry["second_words"], "I") + optional(entry["discarded_suffix_word"], "H")


def group_packet(group: dict) -> bytes:
    raw = packed([group[key] for key in ("ordinal", "start", "end", "count")], "I")
    raw += b"".join(map(entry_packet, group["entries"]))
    return raw + optional(group["stored_refinement_level"], "i") + counted(group["mapping_words"], "i") + counted(group["wide_bits"], "Q")


def case_packet(case: dict) -> bytes:
    parsed = flowreferencegroups.read_record(dict(identifier=416, size=len(case["stored"]), payload=case["stored"]),
        compression="zlib" if case["compressed"] else "none", first_group_mapping_enabled=case["mapping"])
    raw = blob(case["name"].encode()) + bytes((case["compressed"], case["mapping"]))
    raw += blob(case["stored"]) + blob(case["decoded"]) + struct.pack("<i", parsed["version"])
    raw += optional(parsed["discarded_header_word"], "i") + optional(parsed["stored_header_word"], "i")
    raw += struct.pack("<i", parsed["native_header_default"]) + b"".join(map(group_packet, parsed["groups"]))
    return raw + bytes(parsed[key] for key in ("native_runtime_state_reproduced", "reference_fixups_reproduced",
                                              "refinement_postprocessing_reproduced", "engineering_roles_resolved"))


def freeze(output_root: Path = ROOT) -> None:
    cases = [native_case()] + owned_cases()
    packet = b"FRG1" + struct.pack("<I", len(cases)) + b"".join(map(case_packet, cases))
    destination = output_root / "tests/fixtures/flow-reference-groups/groups.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(cases), bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowreferencegroups.py").read_bytes()).hexdigest(),
                 native_version=14, native_group_counts=[64, 0], native_reference_words=128,
                 native_first_words=424, native_second_words=176,
                 scope="Complete legacy versions zero through seventeen wire fields and intervals with explicit mapping assumption; no runtime, fixups, refinement or engineering roles")
    report = output_root / "docs/features/flow-reference-groups/fixture-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
