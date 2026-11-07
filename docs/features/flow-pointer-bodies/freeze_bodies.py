"""Freeze legacy pool/body framing and explicit current-code context before the Rust API."""
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
import flowlegacytables
import flowlinkrecords
import flowpointerbodies
import flowresults

BITS = (0, 0x8000000000000000, 0x7ff8000000001234, 0x7ff0000000000001,
        0x7ff0000000000000, 0xfff0000000000000, 1, 0xffffffffffffffff)


def numbers(values: tuple | list, shape: str) -> bytes:
    return struct.pack("<" + shape * len(values), *values)


def blob(value: bytes) -> bytes:
    return struct.pack("<I", len(value)) + value


def optional(value: int | None, shape: str = "I") -> bytes:
    return b"\0" if value is None else b"\1" + struct.pack("<" + shape, value)


def counted(values: tuple | list, shape: str) -> bytes:
    return struct.pack("<I", len(values)) + numbers(values, shape)


def wide(count: int) -> bytes:
    return numbers([BITS[index % len(BITS)] for index in range(count)], "Q")


def vector(count: int) -> bytes:
    return bytes((count,)) + bytes(index % 256 for index in range(count))


def legacy_entry(mode: str, maximum: bool = False) -> bytes:
    if mode == "single":
        return vector(255 if maximum else 3)
    if mode == "wide":
        count = 255 if maximum else 3
        return bytes((count,)) + wide(3 * count)
    count = 255 if maximum else 3
    return bytes((count,)) + b"".join(vector(0 if index % 2 else 255 if maximum else 2) for index in range(count))


def tagged_entry(version: int, tag: int, count: int = 0) -> bytes:
    raw = bytes((tag,))
    if tag not in (1, 2):
        return raw
    raw += struct.pack("<i", count)
    if version == 3 and tag == 2:
        return raw + wide(3 * count + 1)
    return raw + bytes(index % 256 for index in range(count))


def transport_cases(name: str, raw: bytes, leaf: int, counts: tuple[int, int, int], codes: tuple[int, ...] | None) -> list[dict]:
    return [dict(name=name, decoded=raw, leaf=leaf, counts=counts, codes=codes, compressed=compressed,
                 stored=zlib.compress(raw, 9) if compressed else raw) for compressed in (False, True)]


def profile(version: int, name: str) -> tuple[bytes, int, tuple[int, int, int], tuple | None]:
    if name.startswith("empty"):
        codes = () if name == "empty-present" else (0, 255) if name == "empty-unused" else None
        return struct.pack("<i", version), 0, (0, 0, 0), codes
    maximum = name == "maximum"
    pool_count = 255 if maximum else 3 if name == "rich" else 0
    raw = struct.pack("<iB", version, pool_count) + wide(3 * pool_count) + b"\0"
    if version < 3:
        raw += legacy_entry("single", maximum) + b"\0" + legacy_entry("single")
        raw += legacy_entry("wide", maximum) + legacy_entry("multiple", maximum) + b"\0"
        return raw, 2, (2, 1, 3), (3, 0, 255)
    raw += tagged_entry(version, 1, 255 if maximum else 3) + tagged_entry(version, 2, 0)
    raw += tagged_entry(version, 2, 255 if maximum else 3)
    raw += tagged_entry(version, 0) + tagged_entry(version, 9) + tagged_entry(version, 255)
    return raw, 2, (2, 1, 3), None


def owned_cases() -> list[dict]:
    cases = []
    for version in range(6):
        names = ("empty-none", "rich", "minimum-pools", "maximum")
        if version < 3:
            names += ("empty-present", "empty-unused")
        for name in names:
            raw, leaf, counts, codes = profile(version, name)
            cases += transport_cases(f"v{version}-{name}", raw, leaf, counts, codes)
    for code in range(256):
        mode = "wide" if code in flowpointerbodies.WIDE_CODES else "multiple"
        raw = struct.pack("<i", 0) + legacy_entry(mode)
        cases += transport_cases(f"v0-code-{code}", raw, 0, (0, 0, 1), (code,))
    for version in (3, 4, 5):
        for tag in range(256):
            raw = struct.pack("<i", version) + tagged_entry(version, tag)
            cases += transport_cases(f"v{version}-tag-{tag}", raw, 0, (1, 0, 0), None)
    return cases


def native_case() -> dict:
    row = json.loads((SOURCE / "research/flow-result-carrier-corpus.json").read_text())[1]
    raw = Path(row["path"]).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == row["source_sha256"]
    document = flowresults.read(raw)
    pointers = flowlegacytables.read_record(flowresults.latest(document, 401), compression="zlib")
    links = flowlinkrecords.read_record(flowresults.latest(document, 402), compression="zlib")
    counts = tuple(group["count"] for group in pointers["groups"]) + (links["count"],)
    codes = tuple(entry["native_code_byte"] for entry in links["entries"])
    assert counts == (9338, 6542, 5532)
    record = flowresults.latest(document, 421)
    expected = [item for item in row["records"] if item["identifier"] == 421][-1]
    assert hashlib.sha256(record["payload"]).hexdigest() == expected["payload_sha256"]
    return dict(name="native-cpt-421", stored=record["payload"], decoded=zlib.decompress(record["payload"]),
                compressed=True, leaf=3984, counts=counts, codes=codes)


def pool_packet(value: dict) -> bytes:
    return numbers([value[key] for key in ("ordinal", "start", "end", "count")], "I") + numbers(value["wide_bits"], "Q")


def tagged_packet(value: dict) -> bytes:
    raw = bytes((value["tag_byte"],)) + optional(value.get("stored_count"))
    raw += bytes(("wide_bits" in value,))
    if "wide_bits" in value:
        raw += counted(value["wide_bits"], "Q")
    raw += bytes(("raw_payload" in value,))
    if "raw_payload" in value:
        raw += blob(value["raw_payload"])
    return raw + optional(value.get("packed_payload_decoded"), "B")


def entry_packet(value: dict) -> bytes:
    mode = ("single", "wide", "multiple", "tagged").index(value["mode"])
    raw = numbers([value[key] for key in ("ordinal", "start", "end")], "I") + bytes((mode,))
    raw += optional(value.get("runtime_code_context"), "B")
    if mode == 0:
        return raw + blob(value["byte_vector"])
    if mode == 1:
        return raw + struct.pack("<I", value["count"]) + numbers(value["wide_bits"], "Q")
    if mode == 2:
        return raw + struct.pack("<I", value["count"]) + b"".join(map(blob, value["byte_vectors"]))
    return raw + tagged_packet(value)


def group_packet(value: dict) -> bytes:
    return numbers([value[key] for key in ("ordinal", "start", "end", "count")], "I") + b"".join(map(entry_packet, value["entries"]))


def case_packet(case: dict) -> bytes:
    parsed = flowpointerbodies.read_record(dict(identifier=421, payload=case["stored"], size=len(case["stored"])),
        compression="zlib" if case["compressed"] else "none", leaf_count=case["leaf"], group_counts=case["counts"], entry_codes=case["codes"])
    raw = blob(case["name"].encode()) + struct.pack("<BI3I", case["compressed"], case["leaf"], *case["counts"])
    raw += bytes((case["codes"] is not None,))
    if case["codes"] is not None:
        raw += blob(bytes(case["codes"]))
    raw += blob(case["stored"]) + blob(case["decoded"]) + struct.pack("<i", parsed["version"])
    raw += b"".join(map(pool_packet, parsed["pools"])) + b"".join(map(group_packet, parsed["groups"]))
    return raw + bytes(parsed[key] for key in ("native_runtime_state_reproduced", "orientation_postprocessing_reproduced", "block_to_tree_order_assigned", "engineering_roles_resolved"))


def freeze(output_root: Path = ROOT) -> None:
    cases = [native_case()] + owned_cases()
    packet = b"FPB1" + struct.pack("<I", len(cases)) + b"".join(map(case_packet, cases))
    destination = output_root / "tests/fixtures/flow-pointer-bodies/bodies.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(cases), bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowpointerbodies.py").read_bytes()).hexdigest(),
                 native_leaf_count=3984, native_group_counts=[9338, 6542, 5532], native_pool_triplets=10144,
                 scope="Complete serialized pools/body entries and explicit runtime-code context; no orientation, runtime reconstruction, engineering roles or tree ordering")
    report = output_root / "docs/features/flow-pointer-bodies/fixture-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
