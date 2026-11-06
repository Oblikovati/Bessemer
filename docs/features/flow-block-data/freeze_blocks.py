"""Freeze complete legacy record-406 fields without assigning blocks to tree nodes."""
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
import flowblockdata
import flowcontextcounts
import flowresults


def blob(raw: bytes) -> bytes:
    return struct.pack("<I", len(raw)) + raw


def words(values: tuple[int, ...]) -> bytes:
    return struct.pack("<I", len(values)) + struct.pack("<" + "I" * len(values), *values)


def optional(value: int | None, shape: str) -> bytes:
    return b"\0" if value is None else b"\1" + struct.pack("<" + shape, value)


def verify_entry(actual: dict, expected: dict) -> None:
    for key, value in expected.items():
        if key == "integer_words_sha256":
            integers = actual["first_words"] + actual["second_words"]
            raw = struct.pack("<" + "I" * len(integers), *integers)
            assert hashlib.sha256(raw).hexdigest() == value
            continue
        assert actual[key] == (tuple(value) if key == "descriptors" else value)


def verify_native(parsed: dict, expected: dict) -> None:
    for key in ("version", "entry_count", "first_integer_count", "second_integer_count", "end"):
        assert parsed[key] == expected[key]
    assert hashlib.sha256(parsed["bitmap_bytes"]).hexdigest() == expected["bitmap_sha256"]
    assert len(parsed["blocks"]) == len(expected["blocks"])
    for block, frozen in zip(parsed["blocks"], expected["blocks"]):
        for key in ("ordinal", "start", "end", "block_byte", "word_10_u16", "count_12_i16"):
            assert block[key] == frozen[key]
        assert len(block["entries"]) == len(frozen["entries"])
        for entry, expected_entry in zip(block["entries"], frozen["entries"]):
            verify_entry(entry, expected_entry)


def native_cases() -> list[dict]:
    cases = []
    for row in json.loads((SOURCE / "research/flow-block-data-corpus.json").read_text()):
        wire = Path(row["path"]).read_bytes()
        assert hashlib.sha256(wire).hexdigest() == row["source_sha256"]
        record = flowresults.latest(flowresults.read(wire), 406)
        assert (record is not None) == row["record_406_present"]
        if record is None:
            continue
        stored = record["payload"]
        assert hashlib.sha256(stored).hexdigest() == row["payload_sha256"]
        parsed = flowblockdata.read_record(record, leaf_count=row["leaf_count"], compression="zlib")
        verify_native(parsed, row)
        cases.append(dict(name="native-cpt", compressed=True, leaf_count=row["leaf_count"],
                          stored=stored, decoded=zlib.decompress(stored)))
    return cases


def entry_prefix(version: int, variant: int) -> bytes:
    raw = struct.pack("<i", (-7, -2147483648, 2147483647)[variant])
    if version < 12:
        raw += struct.pack("<i", -11 + variant)
    if 1 < version < 12:
        raw += struct.pack("<i", 0x11223344 + variant)
    raw += struct.pack("<i", -13 - variant)
    if 11 <= version <= 13:
        raw += bytes([238 ^ variant])
    raw += bytes([(250, 0, 255)[variant]])
    if version >= 5:
        raw += bytes([(240, 7, 255)[variant]])
    if version >= 17:
        raw += bytes([(15, 128, 0)[variant]])
    return raw


def entry_wire(version: int, variant: int, negative: bool) -> bytes:
    first = ((0xdeadbeef,), (), (0, 0x80000000, 0xffffffff))[variant]
    second = () if negative else ((0x89abcdef,), (0, 0xffffffff), ())[variant]
    descriptors = (255 if version == 1 else 65535, 2, 3, 4, 5, 6, len(first))
    raw = entry_prefix(version, variant)
    raw += struct.pack("<" + ("B" if version == 1 else "H") * 7, *descriptors)
    raw += struct.pack("<" + "I" * len(first), *first)
    count = -1 if negative and (version == 1 or version >= 10) else len(second)
    raw += struct.pack("<" + ("b" if version == 1 else "H" if version < 10 else "i"), count)
    raw += struct.pack("<" + "I" * len(second), *second)
    if version in (16, 17):
        raw += struct.pack("<H", 0x9876 ^ variant)
    return raw


def profile_blocks(version: int, profile: str) -> list[tuple[int, bytes]]:
    if profile in ("empty", "bitmap7", "bitmap8", "wrap"):
        return []
    if profile == "negative-local":
        return [(0x8000, b""), (0xffff, b""), (0, b"")]
    if profile == "negative-second":
        return [(1, entry_wire(version, 0, True))]
    pair = entry_wire(version, 0, False) + entry_wire(version, 1, False)
    if profile == "populated":
        return [(2, pair), (1, entry_wire(version, 2, False))]
    assert profile == "mixed"
    return [(0, b""), (2, pair), (0x8000, b"")]


def owned_case(version: int, profile: str, compressed: bool) -> dict:
    fluid, solid = {"bitmap7": (7, 0), "bitmap8": (3, 5), "wrap": (2147483647, 1)}.get(profile, (0, 0))
    blocks = profile_blocks(version, profile)
    raw = struct.pack("<3i", version, fluid, solid)
    total = fluid + solid
    bitmap_count = total // 8 + 1 if version >= 8 and 0 < total <= 2147483647 else 0
    raw += bytes([0xaa, 0x55][:bitmap_count])
    for index, (count, entries) in enumerate(blocks):
        raw += struct.pack("<BHH", (0xab, 0, 255)[index], 0x1234 + index, count) + entries
    return dict(name=f"{profile}-v{version}", compressed=compressed, leaf_count=len(blocks),
                stored=zlib.compress(raw, 9) if compressed else raw, decoded=raw)


def owned_cases() -> list[dict]:
    profiles = ("empty", "bitmap7", "bitmap8", "wrap", "negative-local", "negative-second", "populated", "mixed")
    return [owned_case(version, profile, compressed) for version in range(1, 18)
            for profile in profiles for compressed in (False, True)]


def entry_packet(entry: dict) -> bytes:
    raw = struct.pack("<IIi", entry["start"], entry["end"], entry["identifier_i32"])
    raw += optional(entry["legacy_index_i32"], "i") + optional(entry["member_20_i32"], "i")
    raw += struct.pack("<i", entry["member_28_i32"])
    raw += optional(entry["discarded_prefix_byte"], "B") + bytes([entry["state_byte"]])
    raw += optional(entry["flag_2_byte"], "B") + optional(entry["flag_4_byte"], "B")
    raw += struct.pack("<7H", *entry["descriptors"]) + words(entry["first_words"])
    raw += struct.pack("<i", entry["second_count_i32"]) + optional(entry["second_count_raw_small"], "H")
    return raw + words(entry["second_words"]) + optional(entry["discarded_suffix_word"], "H")


def block_packet(block: dict) -> bytes:
    raw = struct.pack("<3IBHHhB", block["ordinal"], block["start"], block["end"], block["block_byte"],
                      block["word_10_u16"], block["count_12_raw_u16"], block["count_12_i16"],
                      block["native_block_byte_operation"] == "assign")
    return raw + struct.pack("<I", len(block["entries"])) + b"".join(map(entry_packet, block["entries"]))


def case_packet(case: dict) -> bytes:
    parsed = flowblockdata.read_decoded(case["decoded"], leaf_count=case["leaf_count"])
    raw = blob(case["name"].encode()) + struct.pack("<BI", case["compressed"], case["leaf_count"])
    raw += blob(case["stored"]) + blob(case["decoded"])
    raw += struct.pack("<3iQi", parsed["version"], parsed["g_fluid"], parsed["g_solid"],
                       parsed["g_total_mathematical"], parsed["g_total_native_i32"])
    raw += blob(parsed["bitmap_bytes"])
    raw += struct.pack("<3IBI", parsed["entry_count"], parsed["first_integer_count"], parsed["second_integer_count"],
                       parsed["entry_total_equals_g_total_observed"], len(parsed["blocks"]))
    return raw + b"".join(map(block_packet, parsed["blocks"]))


def freeze(output_root: Path = ROOT) -> None:
    cases = native_cases() + owned_cases()
    packet = b"FBD1" + struct.pack("<I", len(cases)) + b"".join(map(case_packet, cases))
    destination = output_root / "tests/fixtures/flow-block-data/blocks.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(cases), native_payloads=1, native_decoded_bytes=352725,
                 bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowblockdata.py").read_bytes()).hexdigest(),
                 count_reference_sha256=hashlib.sha256((SOURCE / "flowcontextcounts.py").read_bytes()).hexdigest(),
                 scope="Complete legacy versions 1..17; explicit leaf context; no block/tree association or native flags")
    report = output_root / "docs/features/flow-block-data/fixture-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
