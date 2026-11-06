"""Freeze complete fresh-backend kind-one records and all physical reference parents."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import sys
from pathlib import Path
import freeze_references as references
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "flow-medium-objects"))
import freeze_objects as objects
import freeze_prefix as metadata
from wire_packet import ROOT, SOURCE, blob, counted, native_records


def profiles() -> dict:
    return dict(empty=([], [], [], []), first_only=([0], [], [], []),
                empty_references=([], [(1, [])], [1], [-1]),
                one=([0], [(1, [0])], [1], [-2147483648]),
                duplicate_keys=([0, 0, 1], [(2, [0, 0, 1]), (2, [1])], [2, 2], [-2147483648, 2147483647]),
                reordered=([3, 1, 0], [(0, [3, 0, 1]), (3, [1])], [3, 0, 3], [-1, 0, 2147483647]),
                many=([0, 1, 2, 3], [(key, [index % 4 for index in range(128)]) for key in range(4)],
                      [index % 4 for index in range(64)], [index - 32 for index in range(64)]))


def record(version: int, profile: str, state: int = 255, metadata_name: str = "empty") -> bytes:
    first, second, outer, attributes = profiles()[profile]
    raw = metadata.prefix(version, metadata_name, state) + b"\0\0" + struct.pack("<i", len(first))
    for ordinal, key in enumerate(first):
        raw += struct.pack("<i", 1) + references.KEYS[key]
        raw += objects.body(version, state, version % 3, state, b"\0\0", version + ordinal)
    raw += b"\0" + struct.pack("<i", len(second))
    for key, refs in second:
        raw += struct.pack("<i", 1) + references.body(1, state, [references.KEYS[index] for index in refs], references.KEYS[key])
    raw += struct.pack("<i", len(outer)) + b"".join(references.KEYS[key] for key in outer)
    return raw + b"".join(struct.pack("<i", value) for value in attributes) + bytes(8)


def cases() -> list[tuple[str, bytes]]:
    result = [(f"native-{name}", raw) for name, raw in native_records(424)]
    for version in range(2, 128):
        for profile in profiles():
            result.append((f"v{version}-{profile}", record(version, profile, version ^ 255)))
    for state in range(256):
        result.append((f"state-{state}", record(3, "one", state)))
    for version in (2, 3, 127):
        for name in metadata.NAMES:
            result.append((f"outer-v{version}-{name}", record(version, "one", 128, name)))
    return result


def first_packet(entry: dict, raw: bytes) -> bytes:
    body = raw[entry["start"] + 20:entry["end"]]
    header = struct.pack("<IIIi", entry["ordinal"], entry["start"], entry["end"], entry["kind"])
    return header + blob(entry["key_raw"]) + objects.case_packet(f"first-{entry['ordinal']}", body)


def second_packet(entry: dict) -> bytes:
    header = struct.pack("<IIIi", entry["ordinal"], entry["start"], entry["end"], entry["kind"])
    header += blob(entry["key_raw"]) + bytes((entry["state_byte"], entry["native_state_after_load"]))
    return header + references.key_list(entry["references_raw"])


def case_packet(name: str, raw: bytes) -> bytes:
    value = objects.schema.read_record_424_kind1_profile(dict(identifier=424, size=len(raw), payload=raw), string_mode=0, fresh_backend=True)
    out = blob(name.encode()) + blob(raw) + blob(raw[:value["backend_start"]])
    out += struct.pack("<II", value["backend_start"], value["backend_end"])
    out += struct.pack("<I", len(value["first_table"])) + b"".join(first_packet(entry, raw) for entry in value["first_table"])
    out += struct.pack("<I", len(value["second_table"])) + b"".join(map(second_packet, value["second_table"]))
    out += references.key_list(value["outer_reference_keys_raw"]) + counted(value["outer_attribute_i32"], "i")
    out += struct.pack("<III", *value["final_collection_counts"], value["end"])
    return out + bytes((value["complete_record_framing"], value["full_engineering_schema"], value["fresh_backend_assumed"], value["native_runtime_state_reproduced"]))


def freeze(output_root: Path = ROOT) -> None:
    rows = cases()
    packet = b"FCP1" + struct.pack("<I", len(rows)) + b"".join(case_packet(*row) for row in rows)
    destination = output_root / "tests/fixtures/flow-medium-references/profiles.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(rows), bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowmediumobjects.py").read_bytes()).hexdigest(),
                 scope="Complete fresh-backend kind-one record framing including all nested body fields, duplicate physical keys, typed references, outer keys and signed attributes, zero final counts and exact EOF; no engineering or native runtime acceptance")
    report = output_root / "docs/features/flow-medium-references/profile-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
