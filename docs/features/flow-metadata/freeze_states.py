"""Freeze complete ID/state rows and ordered updates against explicit existing maps."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
from pathlib import Path
from wire_packet import ROOT, SOURCE, blob, counted, native_records, optional, packed, flowmetadataprefix

MINIMUM = -(1 << 31)
MAXIMUM = (1 << 31) - 1


def profiles() -> dict[str, list[tuple[int, int]]]:
    return dict(empty=[], single=[(0, 0)], duplicates=[(7, value) for value in (0, 1, 255, 2, 128)],
                boundaries=[(MINIMUM, 0), (-1, 128), (0, 255), (MAXIMUM, 2)],
                all_states_duplicate=[(7, value) for value in range(256)],
                all_states_distinct=[(value - 128, value) for value in range(256)],
                mixed=[(-2, 255), (-1, 0), (0, 128), (MAXIMUM, 7), (MINIMUM, 2), (-1, 255), (7, 3), (MAXIMUM, 0)])


def existing_maps() -> list[dict[int, int]]:
    return [{}, {7: 0, 42: 9, 410: 0, 413: 255},
            {MINIMUM: 255, -1: 3, 0: 7, MAXIMUM: 128},
            {value - 128: value ^ 85 for value in range(256)}]


def headers() -> list[int]:
    return sorted(set((0, 1, 2, MAXIMUM, 0x80000000, 0xffffffff, 0x12345678))
                  | {1 << bit for bit in range(32)} | {0xffffffff ^ (1 << bit) for bit in range(32)})


def wire(version: int, entries: list[tuple[int, int]]) -> bytes:
    return struct.pack("<Ii", version, len(entries)) + b"".join(struct.pack("<iB", identifier, state) for identifier, state in entries)


def cases() -> list[dict]:
    result = [dict(name=f"native-{name}-410-map-{ordinal}", raw=raw, existing=existing)
              for name, raw in native_records(410) for ordinal, existing in enumerate(existing_maps())]
    for header in headers():
        for name, entries in profiles().items():
            for ordinal, existing in enumerate(existing_maps()):
                result.append(dict(name=f"word-{header:08x}-{name}-map-{ordinal}", raw=wire(header, entries), existing=existing))
    return result


def map_packet(states: dict[int, int]) -> bytes:
    return struct.pack("<I", len(states)) + b"".join(struct.pack("<iB", key, value) for key, value in sorted(states.items()))


def assignment_packet(entries: list[dict], existing: dict[int, int], final: dict[int, int]) -> bytes:
    states = dict(existing)
    raw = struct.pack("<I", len(entries))
    for entry in entries:
        key, value = entry["identifier_i32"], entry["state_byte"]
        raw += struct.pack("<Ii", entry["ordinal"], key) + optional(states.get(key), "B") + bytes((value,))
        states[key] = value
    assert states == final
    return raw


def case_packet(case: dict) -> bytes:
    parsed = flowmetadataprefix.read_record_410(dict(identifier=410, size=len(case["raw"]), payload=case["raw"]))
    applied = flowmetadataprefix.apply_id_states(parsed, existing=case["existing"])
    raw = blob(case["name"].encode()) + blob(case["raw"]) + map_packet(case["existing"])
    raw += struct.pack("<II", parsed["version_word_u32"], parsed["count"])
    for entry in parsed["entries"]:
        raw += packed([entry[key] for key in ("ordinal", "start", "end")], "I")
        raw += struct.pack("<iB", entry["identifier_i32"], entry["state_byte"])
    raw += struct.pack("<I", parsed["end"]) + bytes((parsed["native_version_dispatch"],)) + map_packet(applied["states"])
    raw += assignment_packet(parsed["entries"], case["existing"], applied["states"])
    return raw + bytes((applied["unmentioned_existing_entries_preserved"], applied["native_hash_table_reproduced"]))


def freeze(output_root: Path = ROOT) -> None:
    views = cases()
    packet = b"FIS1" + struct.pack("<I", len(views)) + b"".join(map(case_packet, views))
    destination = output_root / "tests/fixtures/flow-metadata/states.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(views), bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowmetadataprefix.py").read_bytes()).hexdigest(),
                 header_patterns=len(headers()), native_payloads=len(native_records(410)),
                 scope="Complete raw ID/state wire rows and ordered last-wins updates on explicit existing maps, including independent assignment chains; no version dispatch, identifier roles or native hash-table reconstruction")
    report = output_root / "docs/features/flow-metadata/state-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
