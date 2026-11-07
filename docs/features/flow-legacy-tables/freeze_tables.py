"""Freeze complete legacy 401/405 layouts, with optional context kept distinct from native population."""
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
import flowresults

PATTERNS = (0, 0x8000000000000000, 0x7ff8000000001234, 0x7ff0000000000000,
            0xfff0000000000000, 1, 0x7ff0000000000001, 0xffffffffffffffff)


def blob(raw: bytes) -> bytes:
    return struct.pack("<I", len(raw)) + raw


def optional(value: int | None, shape: str) -> bytes:
    return b"\0" if value is None else b"\1" + struct.pack("<" + shape, value)


def counted_words(values: tuple, shape: str) -> bytes:
    return struct.pack("<I", len(values)) + struct.pack("<" + shape * len(values), *values)


def native_cases() -> list[dict]:
    row = json.loads((SOURCE / "research/flow-result-carrier-corpus.json").read_text())[1]
    raw = Path(row["path"]).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == row["source_sha256"]
    document = flowresults.read(raw)
    cases = []
    for identifier in (401, 405):
        record = flowresults.latest(document, identifier)
        expected = next(item for item in row["records"] if item["identifier"] == identifier)
        assert hashlib.sha256(record["payload"]).hexdigest() == expected["payload_sha256"]
        for pointer_count in ((None, 5952) if identifier == 401 else (None,)):
            cases.append(dict(name=f"native-cpt-{identifier}", identifier=identifier, compressed=True,
                pointer_count=pointer_count, legacy=None, stored=record["payload"], decoded=zlib.decompress(record["payload"])))
    return cases


def group_pairs(profile: str, ordinal: int) -> list[tuple[int, int]]:
    if profile == "empty" or profile == "first" and ordinal == 1 or profile == "second" and ordinal == 0:
        return []
    if profile == "negative-pairs":
        return [(-1, -2147483648), (2147483647, 0)]
    if profile == "duplicates":
        return [(0, 7), (0, 7), (7, 0)]
    if profile == "full":
        return [(ordinal, 7 - ordinal), (7 - ordinal, ordinal)]
    return [(ordinal, ordinal)]


def group_header(version: int, ordinal: int, count: int, lengths: tuple[int, int, int]) -> bytes:
    raw = struct.pack("<i", count)
    if version == 1:
        raw += struct.pack("<i", -17 - ordinal)
    if count == 0:
        return raw
    raw += struct.pack("<6i", -2147483648 + ordinal, 0, 2147483647 - ordinal, *lengths)
    if ordinal == 1 and version < 2:
        raw += struct.pack("<3i", 33, -44, 55)
    if version == 1:
        raw += struct.pack("<6i", -1, 0, 2147483647, -2147483648, 7, -9)
    return raw


def group_wire(version: int, ordinal: int, profile: str) -> bytes:
    pairs = group_pairs(profile, ordinal)
    lengths = (0, 0, 0) if profile == "zero-arrays" else (5, 0, 3) if ordinal == 0 else (0, 3, 2)
    raw = group_header(version, ordinal, len(pairs), lengths)
    if not pairs:
        return raw
    for index, count in enumerate(lengths):
        words = [PATTERNS[(slot + index * 3 + ordinal) % 8] for slot in range(count)]
        raw += struct.pack("<" + "Q" * count, *words)
    if version == 1:
        raw += struct.pack("<i", -19 - ordinal)
    return raw + struct.pack("<" + "i" * (2 * len(pairs)), *(n for pair in pairs for n in pair))


def owned_pairs() -> list[dict]:
    cases = []
    for version in range(4):
        for profile in ("empty", "first", "second", "full", "zero-arrays", "duplicates", "negative-pairs"):
            decoded = struct.pack("<i", version) + b"".join(group_wire(version, ordinal, profile) for ordinal in range(2))
            contexts = (None,) if profile == "negative-pairs" else (None, 0 if profile == "empty" else 8, 2147483647)
            for pointer_count in contexts:
                for compressed in (False, True):
                    cases.append(dict(name=f"pairs-v{version}-{profile}", identifier=401, compressed=compressed,
                        pointer_count=pointer_count, legacy=None, decoded=decoded,
                        stored=zlib.compress(decoded, 9) if compressed else decoded))
    return cases


def owned_words() -> list[dict]:
    cases = []
    profiles = [(0, pair) for pair in ((0, 0), (0, 3), (3, 0), (1, 2), (3, 5))]
    profiles += [(1, count) for count in (0, 1, 3, 8)]
    for version, context in profiles:
        count = sum(context) if version == 0 else context
        raw = struct.pack("<i", version) + (b"" if version == 0 else struct.pack("<i", count))
        words = [(0, 0xffffffff, 0x80000000, 1)[i % 4] for i in range(count)]
        raw += struct.pack("<" + "I" * count, *words)
        for compressed in (False, True):
            cases.append(dict(name=f"words-v{version}-{context}", identifier=405, compressed=compressed,
                pointer_count=None, legacy=context if version == 0 else None, decoded=raw,
                stored=zlib.compress(raw, 9) if compressed else raw))
    return cases


def group_packet(group: dict) -> bytes:
    raw = struct.pack("<4I", group["ordinal"], group["start"], group["end"], group["count"])
    raw += optional(group["discarded_count_word"], "i") + bytes([group["count"] > 0])
    if not group["count"]:
        return raw
    raw += struct.pack("<6i", *group["member_triplet"], *group["array_lengths"])
    raw += struct.pack("<I", len(group["discarded_triplets"]))
    raw += b"".join(struct.pack("<3i", *triplet) for triplet in group["discarded_triplets"])
    for array in group["arrays"]:
        raw += struct.pack("<II", array["start"], array["end"]) + counted_words(array["raw_bits"], "Q")
    raw += optional(group["discarded_pre_pairs_word"], "i") + struct.pack("<I", len(group["pairs"]))
    return raw + b"".join(struct.pack("<2i", *pair) for pair in group["pairs"])


def case_packet(case: dict) -> bytes:
    record = dict(identifier=case["identifier"], payload=case["stored"], size=len(case["stored"]))
    parsed = flowlegacytables.read_record(record, compression="zlib" if case["compressed"] else "none",
        legacy_counts=case["legacy"], global_pointer_count=case["pointer_count"])
    raw = blob(case["name"].encode()) + struct.pack("<IB", case["identifier"], case["compressed"])
    raw += optional(case["pointer_count"], "I") + bytes([case["legacy"] is not None])
    if case["legacy"] is not None:
        raw += struct.pack("<2I", *case["legacy"])
    raw += blob(case["stored"]) + blob(case["decoded"]) + struct.pack("<i", parsed["version"])
    if case["identifier"] == 401:
        return raw + bytes([parsed["global_pointer_indices_checked"]]) + b"".join(map(group_packet, parsed["groups"])) + bytes([parsed["native_pointer_population_reproduced"]])
    return raw + struct.pack("<IBI", parsed["count"], parsed["count_stored"], parsed["words_start"]) + counted_words(parsed["raw_words"], "I") + bytes([parsed["word_roles_resolved"]])


def freeze(output_root: Path = ROOT) -> None:
    cases = native_cases() + owned_pairs() + owned_words()
    packet = b"FLT1" + struct.pack("<I", len(cases)) + b"".join(map(case_packet, cases))
    destination = output_root / "tests/fixtures/flow-legacy-tables/tables.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(cases), pointer_tables=sum(case["identifier"] == 401 for case in cases),
                 word_tables=sum(case["identifier"] == 405 for case in cases), native_payloads=2,
                 native_pairs=15880, native_wide_words=7930, native_raw_words=5952,
                 bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowlegacytables.py").read_bytes()).hexdigest(),
                 scope="Exact legacy versions and context presence, raw arrays/pairs/discards; no native pointer population or word semantics")
    report = output_root / "docs/features/flow-legacy-tables/fixture-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
