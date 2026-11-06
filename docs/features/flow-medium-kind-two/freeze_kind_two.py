"""Freeze all six kind-two model layouts without engineering or installed-fixture credit."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "flow-medium-objects"))
import freeze_objects as common
from wire_packet import ROOT, SOURCE, blob, counted, packed
import flowmediumkind2 as schema

KIND_ONE_SUFFIX_BYTES = 4 * 8 + 1
WORD_PATTERNS = [0, 1, 0xffffffff, 0x80000000, *[1 << bit for bit in range(32)], *[0xffffffff ^ (1 << bit) for bit in range(32)]]


def model(kind: int, count: int, flag: int, seed: int, state: int, counts: tuple[int, int, int] | None = None) -> bytes:
    raw = b"\0"
    if kind in (0, 1, 2):
        raw += b"".join(common.curve(count, flag, seed + index) for index in range(2 if kind == 0 else 3))
        return raw + packed(common.patterns(1, seed), "Q") + (bytes((state, state ^ 255)) if kind == 1 else b"")
    if kind == 3:
        return raw + packed(common.patterns(7, seed), "Q")
    if kind == 4:
        return raw + packed(common.patterns(6, seed), "Q") + bytes((state,)) + packed(common.patterns(1, seed + 6), "Q")
    assert kind == 5
    return raw + table_model(count, seed, state, counts)


def table_model(count: int, seed: int, state: int, counts: tuple[int, int, int] | None = None) -> bytes:
    first, second, third = counts if counts is not None else (count, count // 2, count // 3)
    raw = struct.pack("<ii", first, second)
    raw += b"".join(packed(common.patterns(first, seed + index), "Q") for index in range(2))
    raw += packed(common.patterns(second, seed + 2), "Q")
    raw += packed([WORD_PATTERNS[(seed + index) % len(WORD_PATTERNS)] for index in range(second)], "I")
    raw += struct.pack("<i", third) + packed(common.patterns(third, seed + 3), "Q")
    member = (-2147483648, -1, 0, 2147483647)[seed % 4]
    return raw + struct.pack("<i", member) + packed(common.patterns(2, seed + 4), "Q") + bytes((state, state ^ 255))


def body(kind: int, count: int, flag: int, seed: int, state: int, name: bytes = b"\0\0", counts: tuple[int, int, int] | None = None) -> bytes:
    # Both documented body dialects share this prefix and differ at the final suffix.
    prefix = common.body(state, state ^ 165, count, flag, name, seed)[:-KIND_ONE_SUFFIX_BYTES]
    return prefix + packed(common.patterns(1, seed), "Q") + struct.pack("<i", kind) + model(kind, count, flag, seed, state, counts) + bytes((state ^ 165,))


def cases() -> list[tuple[str, bytes]]:
    rows = [(f"tag-{kind}-count-{count}-flag-{flag}", body(kind, count, flag, kind + count, flag))
            for kind in range(6) for count in (0, 1, 3, 32) for flag in (0, 1, 128, 255)]
    rows += [(f"base-state-{state}", body(state % 6, state % 3, state, state, state)) for state in range(256)]
    rows += [(f"model-{kind}-state-{state}", body(kind, 1, 255, state, state)) for kind in (1, 4, 5) for state in range(256)]
    rows += [(f"flag-{flag}-count-{count}", body(flag % 6, count, flag, flag, 128)) for flag in range(256) for count in (0, 1)]
    rows += [(f"model-{kind}-bits-{index}", body(kind, 1, 128, index, 255)) for kind in range(6) for index in range(len(common.PATTERNS))]
    names = [b"", b"\0\0", b"a\0\0\0b\0\0\xd8\0\0", b"\xff\xfe\xff\xff\x00\xd8\x00\xdc\0\0", b"x\0" * 65535 + b"\0\0"]
    rows += [(f"tag-{kind}-text-{index}", body(kind, 0, 0, index, 0, name)) for kind in range(6) for index, name in enumerate(names)]
    rows += [(f"tag-{kind}-large", body(kind, 1024, 255, kind, 255)) for kind in range(6)]
    independent = [(0, 1, 0), (1, 0, 3), (0, 0, 5), (3, 2, 1), (1, 7, 11), (1024, 0, 0), (0, 1024, 0), (0, 0, 1024)]
    rows += [(f"independent-counts-{index}", body(5, 0, 0, index, 128, counts=counts)) for index, counts in enumerate(independent)]
    return [(name, raw + (b"tail\0\xff" if index % 2 else b"")) for index, (name, raw) in enumerate(rows)]


def model_packet(value: dict) -> bytes:
    kind = value["kind"]
    raw = struct.pack("<iIIB", kind, value["start"], value["end"], value["version"])
    if kind in (0, 1, 2):
        raw += struct.pack("<I", len(value["curves"])) + b"".join(map(common.curve_packet, value["curves"]))
        return raw + counted(value["scalar_patterns"], "Q") + blob(value["state_bytes"])
    if kind == 3:
        return raw + counted(value["scalar_patterns"], "Q")
    if kind == 4:
        return raw + counted(value["first_scalar_patterns"], "Q") + bytes((value["state_byte"],)) + struct.pack("<Q", value["last_scalar_pattern"])
    raw += struct.pack("<III", value["first_count"], value["second_count"], len(value["first_arrays"]))
    raw += b"".join(counted(row, "Q") for row in value["first_arrays"])
    raw += counted(value["second_patterns"], "Q") + counted(value["second_word_patterns"], "I")
    raw += struct.pack("<I", value["third_count"]) + counted(value["third_patterns"], "Q") + struct.pack("<i", value["member_8c_i32"])
    return raw + counted(value["scalar_patterns"], "Q") + blob(value["state_bytes"])


def case_packet(name: str, raw: bytes) -> bytes:
    value = schema.read_kind2_body(raw, string_mode=0)
    out = blob(name.encode()) + blob(raw) + common.prefix_packet(value) + struct.pack("<Q", value["scalar_pattern"])
    out += model_packet(value["model"]) + bytes((value["state_byte"], value["native_state_after_load"]))
    out += struct.pack("<I", value["body_end"]) + blob(value["opaque_tail"])
    return out + bytes((value["body_framing_complete"], value["installed_fixture_validated"], value["native_runtime_state_reproduced"]))


def freeze(output_root: Path = ROOT) -> None:
    rows = cases()
    packet = b"FMO2" + struct.pack("<I", len(rows)) + b"".join(case_packet(*row) for row in rows)
    destination = output_root / "tests/fixtures/flow-medium-kind-two/bodies.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(rows), bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowmediumkind2.py").read_bytes()).hexdigest(),
                 model_tags=list(range(6)), installed_fixture_validated=False,
                 scope="Complete generated kind-two body fields and all six model layouts, raw 64-bit patterns/32-bit words and state bytes, lossless strings and exact following tails; no installed kind-two, engineering or runtime acceptance")
    report = output_root / "docs/features/flow-medium-kind-two/baseline-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
