"""Freeze Source table fields and count-presence distinctions before the Rust API."""
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
import flowsimpletables

WIDE = (0, 0x8000000000000000, 0x7ff8000000001234, 0x7ff0000000000001,
        0x7ff0000000000000, 0xfff0000000000000, 1, 0xffffffffffffffff)
WORDS = (0, 0xffffffff, 0x80000000, 0x7fc01234, 0x7f800001, 1, 0x7f800000, 0xff800000)


def numbers(values: tuple | list, shape: str) -> bytes:
    return struct.pack("<" + shape * len(values), *values)


def blob(value: bytes) -> bytes:
    return struct.pack("<I", len(value)) + value


def optional(value: int | None, shape: str = "I") -> bytes:
    return b"\0" if value is None else b"\1" + struct.pack("<" + shape, value)


def counted(values: tuple | list, shape: str) -> bytes:
    return struct.pack("<I", len(values)) + numbers(values, shape)


def transports(name: str, identifier: int, raw: bytes, leaf: int | None = None, legacy: int | None = None) -> list[dict]:
    return [dict(name=name, identifier=identifier, decoded=raw, leaf=leaf, legacy=legacy,
                 compressed=compressed, stored=zlib.compress(raw, 9) if compressed else raw)
            for compressed in (False, True)]


def parallel_wire(counts: tuple[int, int], presence: tuple[int, int]) -> bytes:
    raw = struct.pack("<3i", 0, *counts)
    for group, (count, present) in enumerate(zip(counts, presence)):
        raw += struct.pack("<i", present)
        if present:
            for array in range(4):
                raw += numbers([WIDE[(group + array * 2 + index) % len(WIDE)] for index in range(count)], "Q")
    return raw


def parallel_cases() -> list[dict]:
    profiles = [((0, 0), (0, 0)), ((0, 0), (-1, 2)), ((2, 3), (1, -3)),
                ((2147483647, 7), (0, 0)), ((0, 3), (1, 0)), ((1, 0), (0, -2147483648)),
                ((1, 0), (1, 0)), ((0, 1), (0, 1))]
    cases = []
    for index, (counts, presence) in enumerate(profiles):
        cases += transports(f"parallel-v0-{index}", 407, parallel_wire(counts, presence))
    for version in (1, 2, 99, 2147483647):
        cases += transports(f"parallel-v{version}-empty", 407, struct.pack("<3i", version, 0, 0))
    return cases


def leaf_cases() -> list[dict]:
    cases = []
    for count in (0, 1, 2, 7, 255):
        raw = struct.pack("<i", 0) + numbers([WIDE[index % len(WIDE)] for index in range(2 * count)], "Q")
        cases += transports(f"leaf-pairs-{count}", 437, raw, leaf=count)
    return cases


def five_word_cases() -> list[dict]:
    cases = []
    for version in range(3):
        for context in (None, 0, 1, 2147483647):
            cases += transports(f"five-v{version}-context-{context}", 411, struct.pack("<i", version), legacy=context)
    for version in (3, 4):
        for count in (0, 1, 2, 7, 255):
            raw = struct.pack("<i", version) + (struct.pack("<i", count) if version == 4 else b"")
            raw += numbers([WORDS[index % len(WORDS)] for index in range(5 * count)], "I")
            cases += transports(f"five-v{version}-{count}", 411, raw, legacy=count if version == 3 else None)
    for version in (5, 6, 99, 2147483647):
        cases += transports(f"five-v{version}-empty", 411, struct.pack("<2i", version, 0))
    return cases


def byte_cases() -> list[dict]:
    cases = []
    for version in (0, 1):
        for count in (0, 1, 3, 256, 1024):
            raw = struct.pack("<i", version) + (struct.pack("<i", count) if version else b"")
            raw += bytes(index % 256 for index in range(count))
            cases += transports(f"bytes-v{version}-{count}", 413, raw, legacy=count if version == 0 else None)
    for version in (2, 3, 99, 2147483647):
        cases += transports(f"bytes-v{version}-empty", 413, struct.pack("<2i", version, 0))
    return cases


def native_cases() -> list[dict]:
    row = json.loads((SOURCE / "research/flow-result-carrier-corpus.json").read_text())[1]
    wire = Path(row["path"]).read_bytes()
    assert hashlib.sha256(wire).hexdigest() == row["source_sha256"]
    document = flowresults.read(wire)
    cases = []
    for identifier in (407, 437, 411, 413):
        record = flowresults.latest(document, identifier)
        expected = [item for item in row["records"] if item["identifier"] == identifier][-1]
        assert hashlib.sha256(record["payload"]).hexdigest() == expected["payload_sha256"]
        cases.append(dict(name=f"native-cpt-{identifier}", identifier=identifier, stored=record["payload"],
            decoded=zlib.decompress(record["payload"]), compressed=True, leaf=3984 if identifier == 437 else None, legacy=None))
    verify_member_relationships(document, cases)
    return cases


def parsed(case: dict) -> dict:
    return flowsimpletables.read_record(dict(identifier=case["identifier"], payload=case["stored"], size=len(case["stored"])),
        compression="zlib" if case["compressed"] else "none", leaf_count=case["leaf"], legacy_count=case["legacy"])


def verify_member_relationships(document: dict, cases: list[dict]) -> None:
    links = flowlinkrecords.read_record(flowresults.latest(document, 402), compression="zlib")
    values = {case["identifier"]: parsed(case) for case in cases}
    assert links["member_values"]["0x2d0"] == values[411]["count"] == 4056
    assert links["member_values"]["0x294"] == values[413]["count"] == 0
    assert [group["count"] for group in values[407]["groups"]] == [1944, 1960]


def parallel_packet(groups: list[dict]) -> bytes:
    raw = b""
    for group in groups:
        raw += numbers([group[key] for key in ("ordinal", "count", "start", "end")], "I")
        raw += optional(group["presence_word"], "i") + struct.pack("<I", len(group["arrays"]))
        for array in group["arrays"]:
            raw += numbers((array["start"], array["end"]), "I") + counted(array["raw_bits"], "Q")
    return raw


def expected_packet(value: dict) -> bytes:
    identifier = value["identifier"]
    if identifier == 407:
        return parallel_packet(value["groups"])
    if identifier == 437:
        raw = struct.pack("<I", value["count"])
        return raw + b"".join(numbers([row[key] for key in ("ordinal", "start", "end")], "I") + numbers(row["raw_bits"], "Q") for row in value["rows"])
    if identifier == 411:
        raw = optional(value["count"]) + bytes((value["count_stored"],)) + struct.pack("<I", value["words_start"]) + bytes((value["wire_rows_present"],))
        return raw + struct.pack("<I", len(value["rows"])) + b"".join(numbers(row, "I") for row in value["rows"])
    return struct.pack("<IBI", value["count"], value["count_stored"], value["bytes_start"]) + blob(value["raw_bytes"])


def case_packet(case: dict) -> bytes:
    value = parsed(case)
    raw = blob(case["name"].encode()) + struct.pack("<IB", case["identifier"], case["compressed"])
    raw += optional(case["leaf"]) + optional(case["legacy"]) + blob(case["stored"]) + blob(case["decoded"])
    raw += struct.pack("<i", value["version"]) + expected_packet(value)
    return raw + bytes((value["native_runtime_state_reproduced"], value["field_roles_resolved"])) + optional(value.get("block_to_tree_order_assigned"), "B")


def freeze(output_root: Path = ROOT) -> None:
    cases = native_cases() + parallel_cases() + leaf_cases() + five_word_cases() + byte_cases()
    packet = b"FST1" + struct.pack("<I", len(cases)) + b"".join(map(case_packet, cases))
    destination = output_root / "tests/fixtures/flow-simple-tables/tables.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(cases), bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowsimpletables.py").read_bytes()).hexdigest(),
                 native_payloads=4, native_parallel_counts=[1944, 1960], native_leaf_count=3984, native_five_rows=4056, native_byte_count=0,
                 scope="Complete raw values, count/context presence, stored counts versus inactive rows and empty later layouts; no runtime state, roles or tree ordering")
    report = output_root / "docs/features/flow-simple-tables/fixture-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
