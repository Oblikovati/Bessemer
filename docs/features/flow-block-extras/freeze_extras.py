"""Freeze complete record-408 layouts independently of the Rust reader/editor."""
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
import flowbasegrid
import flowblockdata
import flowblockextras
import flowblocktrees
import flowresults

BITS = (0x8000000000000000, 0x7ff8000000001234, 0x7ff0000000000001,
        0x7ff0000000000000, 0xfff0000000000000, 1, 0xffffffffffffffff)


def numbers(values: tuple | list, shape: str) -> bytes:
    return struct.pack("<" + shape * len(values), *values)


def blob(raw: bytes) -> bytes:
    return struct.pack("<I", len(raw)) + raw


def optional(value: int | None, shape: str = "B") -> bytes:
    return b"\0" if value is None else b"\1" + struct.pack("<" + shape, value)


def counted(values: tuple | list, shape: str) -> bytes:
    return struct.pack("<I", len(values)) + numbers(values, shape)


def mixed_wire(version: int, count: int) -> bytes:
    words = (-2147483648, 0, 2147483647)[:count]
    wide = BITS[:count]
    values = numbers(words, "i") + numbers(wide, "Q") if version < 8 else b"".join(struct.pack("<iQ", a, b) for a, b in zip(words, wide))
    return struct.pack("<i", count) + values


def first_wire(version: int, profile: str) -> bytes:
    raw = b"\xff\0" + numbers((-2147483648, -1, 0, 1, 2147483647), "i") + mixed_wire(version, 3)
    raw += numbers((-1, -2147483648), "i") if version < 5 else numbers((BITS[0], BITS[6]), "Q")
    payload = b"" if profile == "bytes-empty" else bytes(range(255)) if profile == "bytes-max" else b"\0\xff"
    raw += bytes((len(payload),)) + payload
    if version < 7 or not payload:
        return raw
    presence = 0 if profile == "additional-zero" else 255 if profile == "flags-255" else 2
    raw += bytes((presence,))
    if presence:
        raw += numbers([(-2147483648, 0, 2147483647)[i % 3] for i in range(3 * len(payload))], "i")
    return raw


def nested_wire(version: int, profile: str, fourth: int) -> bytes:
    raw = b""
    for row in range(fourth):
        count = (255 if version < 5 else 300) if profile == "nested-max" else 2 if row == 0 else 0
        raw += bytes((count,)) if version < 5 else struct.pack("<i", count)
        for ordinal in range(count):
            size = 0 if profile == "nested-max" or ordinal == 1 else 2
            raw += bytes((size,)) + (struct.pack("<i", -2147483648 + ordinal) if version >= 6 else b"")
            raw += numbers((-1, 2147483647)[:size], "i") + numbers(BITS[:3 * size], "Q")
    return raw


def flags(profile: str) -> tuple[int, int, int, int]:
    if profile in ("flags-zero", "flags-two", "flags-255"):
        value = {"flags-zero": 0, "flags-two": 2, "flags-255": 255}[profile]
        return 1, value, value, value
    if profile in ("outer-two", "outer-255"):
        return (2 if profile == "outer-two" else 255), 0, 0, 0
    return 1, 1, 1, 2


def body_tail(version: int, profile: str, fourth: int) -> bytes:
    outer, nested, final, extra = flags(profile)
    raw = bytes((outer,))
    if outer == 1:
        raw += bytes((nested,))
        if nested == 1:
            raw += nested_wire(version, profile, fourth)
        raw += bytes((final,))
        if final == 1:
            raw += struct.pack("<i", -2147483648) + counted((-1, 0, 2147483647), "i")
    if version >= 3:
        raw += bytes((extra,))
        if extra:
            raw += struct.pack("<i", 2147483647) + mixed_wire(version, 2)
    return raw + (b"\xff" if version >= 4 else b"")


def body_wire(version: int, profile: str) -> bytes:
    if profile == "minimal":
        return b"\0\0\0\0\2" + (b"\0" if version >= 3 else b"") + (b"\0" if version >= 4 else b"")
    count = 255 if profile == "list-max" else 2
    raw = bytes((count,)) + first_wire(version, profile) * count
    second_count = 255 if profile == "list-max" else 1
    raw += bytes((second_count,)) + b"\xff\0\xff\3\0x\xff" * second_count
    raw += bytes((count,)) + b"".join(struct.pack("<i4Q", -2147483648 + index, *BITS[:4]) for index in range(count))
    fourth = 0 if profile == "fourth-zero" else 255 if profile == "list-max" else 2
    raw += bytes((fourth,)) + numbers([BITS[index % len(BITS)] for index in range(3 * fourth)], "Q")
    raw += bytes(index % 256 for index in range(fourth))
    return raw + body_tail(version, profile, fourth)


def owned_case(version: int, profile: str) -> tuple[bytes, int]:
    if profile == "empty-leaves":
        return struct.pack("<i", version), 0
    if profile == "absent":
        return struct.pack("<i", version) + b"".join(struct.pack("<i", token) + (b"\0" if version >= 3 else b"") for token in (-2, 0, 2147483647)), 3
    token, flag = (7, 255) if profile == "flag-body" and version >= 3 else (-1, 0)
    raw = struct.pack("<ii", version, token) + (bytes((flag,)) if version >= 3 else b"")
    raw += body_wire(version, profile)
    return raw + struct.pack("<i", 0) + (b"\0" if version >= 3 else b""), 2


def owned_cases() -> list[dict]:
    cases = []
    profiles = ("empty-leaves", "absent", "minimal", "rich", "flags-zero", "flags-two", "flags-255",
                "outer-two", "outer-255", "bytes-empty", "bytes-max", "additional-zero", "nested-max", "fourth-zero", "flag-body", "list-max")
    for version in range(9):
        for profile in profiles:
            raw, leaves = owned_case(version, profile)
            for compressed in (False, True):
                cases.append(dict(name=f"v{version}-{profile}", decoded=raw, leaves=leaves,
                    compressed=compressed, stored=zlib.compress(raw, 9) if compressed else raw))
    return cases


def native_case() -> dict:
    row = json.loads((SOURCE / "research/flow-result-carrier-corpus.json").read_text())[1]
    raw = Path(row["path"]).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == row["source_sha256"]
    document = flowresults.read(raw)
    grid = flowbasegrid.read_record(flowresults.latest(document, 101), compression="none")
    trees = flowblocktrees.read_record(flowresults.latest(document, 400), compression="zlib")
    context = flowblockdata.derive_fresh_leaf_count(grid, trees, fresh_context=True)
    assert context["leaf_count"] == 3984
    record = flowresults.latest(document, 408)
    expected = next(item for item in row["records"] if item["identifier"] == 408)
    assert hashlib.sha256(record["payload"]).hexdigest() == expected["payload_sha256"]
    return dict(name="native-cpt-408", decoded=zlib.decompress(record["payload"]), leaves=3984,
                compressed=True, stored=record["payload"])


def mixed_packet(value: dict) -> bytes:
    return numbers((value["start"], value["end"]), "I") + bytes((value["layout"] == "interleaved",)) + counted(value["words_i32"], "i") + numbers(value["wide_bits"], "Q")


def first_packet(value: dict, version: int) -> bytes:
    raw = numbers((value["start"], value["end"]), "I") + value["pair_bytes"] + numbers(value["words_i32"], "i")
    raw += mixed_packet(value["mixed"]) + numbers(value["reference_words"], "i" if version < 5 else "Q")
    return raw + blob(value["byte_array"]) + optional(value["additional_presence_byte"]) + counted(value["additional_i32"], "i")


def second_packet(value: dict) -> bytes:
    return numbers((value["start"], value["end"]), "I") + value["pair_bytes"] + bytes((value["tag_byte"],)) + blob(value["byte_array"])


def third_packet(value: dict) -> bytes:
    return numbers((value["start"], value["end"]), "I") + struct.pack("<i", value["word_i32"]) + numbers(value["wide_bits"], "Q")


def nested_packet(value: dict) -> bytes:
    raw = numbers((value["start"], value["end"]), "I") + optional(value["stored_index"], "i") + struct.pack("<i", value["native_index"])
    return raw + counted(value["words_i32"], "i") + numbers(value["wide_bits"], "Q")


def optional_nested(value: list | None) -> bytes:
    if value is None:
        return b"\0"
    return b"\1" + struct.pack("<I", len(value)) + b"".join(struct.pack("<I", len(row)) + b"".join(map(nested_packet, row)) for row in value)


def optional_final(value: dict | None) -> bytes:
    return b"\0" if value is None else b"\1" + struct.pack("<i", value["word_i32"]) + counted(value["words_i32"], "i")


def optional_extra(value: dict | None) -> bytes:
    return b"\0" if value is None else b"\1" + struct.pack("<i", value["word_i32"]) + mixed_packet(value["mixed"])


def body_packet(value: dict | None, version: int) -> bytes:
    if value is None:
        return b"\0"
    raw = b"\1" + numbers((value["start"], value["end"]), "I")
    for key, pack in (("first", lambda item: first_packet(item, version)), ("second", second_packet), ("third", third_packet)):
        raw += struct.pack("<I", len(value[key])) + b"".join(map(pack, value[key]))
    raw += counted(value["fourth_wide_bits"], "Q") + blob(value["fourth_bytes"])
    raw += bytes((value["outer_presence_byte"],)) + optional(value["nested_presence_byte"]) + optional_nested(value["nested"])
    raw += optional(value["final_presence_byte"]) + optional_final(value["final"])
    return raw + optional(value["extra_presence_byte"]) + optional_extra(value["extra"]) + optional(value["last_byte"])


def case_packet(case: dict) -> bytes:
    parsed = flowblockextras.read_record(dict(identifier=408, payload=case["stored"], size=len(case["stored"])),
        compression="zlib" if case["compressed"] else "none", leaf_count=case["leaves"], enabled=True)
    raw = blob(case["name"].encode()) + bytes((case["compressed"],)) + struct.pack("<I", case["leaves"]) + b"\1"
    raw += blob(case["stored"]) + blob(case["decoded"]) + struct.pack("<iI", parsed["version"], len(parsed["blocks"]))
    for block in parsed["blocks"]:
        raw += numbers([block[key] for key in ("ordinal", "start", "end")], "I") + struct.pack("<i", block["token_i32"])
        raw += optional(block["body_flag_byte"]) + body_packet(block["body"], parsed["version"])
    return raw + bytes((parsed["block_to_tree_order_assigned"], parsed["native_object_references_resolved"], parsed["native_runtime_state_reproduced"]))


def freeze(output_root: Path = ROOT) -> None:
    cases = [native_case()] + owned_cases()
    packet = b"FBE1" + struct.pack("<I", len(cases)) + b"".join(map(case_packet, cases))
    destination = output_root / "tests/fixtures/flow-block-extras/extras.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(cases), bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowblockextras.py").read_bytes()).hexdigest(),
                 native_leaf_context=3984, native_bodies=1936, native_lists=[4636, 19880, 3904], native_fourth=25632,
                 scope="Complete legacy per-leaf bodies and explicit enabled/count contexts; no tree-order assignment, native references or runtime reconstruction")
    report = output_root / "docs/features/flow-block-extras/fixture-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
