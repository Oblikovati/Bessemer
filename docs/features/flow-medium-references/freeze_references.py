"""Freeze typed reference bodies with explicit existing first-table key contexts."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "flow-metadata"))
from wire_packet import ROOT, SOURCE, blob, optional, native_records
import flowmediumliquids as schema

KEYS = [bytes(16), bytes(range(16)), bytes(range(255, 239, -1)), bytes((128,)) * 16]


def key_list(keys: list[bytes]) -> bytes:
    return struct.pack("<I", len(keys)) + b"".join(blob(key) for key in keys)


def body(kind: int, state: int, references: list[bytes], key: bytes) -> bytes:
    raw = b"\0\0" + key + (bytes((state,)) if kind == 1 else b"")
    return raw + struct.pack("<i", len(references)) + b"".join(references)


def native_cases() -> list[dict]:
    rows = []
    for name, raw in native_records(424):
        profile = schema.read_record_424_liquid_profile(dict(identifier=424, size=len(raw), payload=raw), string_mode=0, fresh_backend=True)
        for entry in profile["second_table"]:
            kind = entry["kind"]
            keys = [first["key_raw"] for first in profile["first_table"] if first["kind"] == kind]
            start = entry["start"] + 4
            for suffix, end in (("tail", len(raw)), ("exact", entry["end"])):
                rows.append(dict(name=f"native-{name}-{suffix}", raw=raw[start:end], kind=kind, keys=keys))
    return rows


def profiles() -> list[list[bytes]]:
    return [[], [KEYS[0]], [KEYS[1], KEYS[1], KEYS[0]], KEYS[::-1],
            [KEYS[index % len(KEYS)] for index in range(128)],
            [KEYS[index % len(KEYS)] for index in range(1024)]]


def cases() -> list[dict]:
    result = native_cases()
    for kind in (1, 2):
        for state in range(256) if kind == 1 else (0,):
            for index, refs in enumerate(profiles()):
                if index == 5 and state not in (0, 255):
                    continue
                contexts = [[], KEYS] if not refs else [KEYS, KEYS[::-1] + KEYS]
                for context, keys in enumerate(contexts):
                    raw = body(kind, state, refs, KEYS[(state + index) % len(KEYS)])
                    result.append(dict(name=f"kind-{kind}-state-{state}-refs-{index}-context-{context}", raw=raw + b"tail\0\xff", kind=kind, keys=keys))
    ambiguous = body(1, 0, [], KEYS[1])
    for kind in (1, 2):
        result.append(dict(name=f"same-wire-kind-{kind}", raw=ambiguous, kind=kind, keys=[]))
    return result


def case_packet(case: dict) -> bytes:
    value = schema.read_reference_body(case["raw"], kind=case["kind"], first_keys=case["keys"])
    out = blob(case["name"].encode()) + blob(case["raw"]) + struct.pack("<i", case["kind"]) + key_list(case["keys"])
    out += blob(value["key_raw"]) + optional(value.get("state_byte"), "B") + optional(value.get("native_state_after_load"), "B")
    out += key_list(value["references_raw"]) + struct.pack("<I", value["body_end"]) + blob(value["opaque_tail"])
    return out + bytes((value["native_runtime_state_reproduced"],))


def freeze(output_root: Path = ROOT) -> None:
    rows = cases()
    packet = b"FMR1" + struct.pack("<I", len(rows)) + b"".join(map(case_packet, rows))
    destination = output_root / "tests/fixtures/flow-medium-references/bodies.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(rows), bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowmediumliquids.py").read_bytes()).hexdigest(),
                 scope="Complete kind-one/two reference bodies with explicit first-table-kind key context, raw keys and ordered duplicate references, optional stored/native state and exact following tails; no native map allocation or runtime acceptance")
    report = output_root / "docs/features/flow-medium-references/baseline-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
