"""Freeze the bounded metadata prefix while preserving every opaque backend byte."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
from pathlib import Path
from wire_packet import ROOT, SOURCE, blob, counted, native_records, optional, packed, flowmetadataprefix

TAIL = b"\0backend\xff" + bytes(range(256))
NAMES = ("empty", "zero-strings", "one-nul", "embedded-nul", "surrogates", "noncharacters", "many", "long-string")


def string_wire(units: list[int]) -> bytes:
    return struct.pack("<i", len(units)) + packed(units, "H")


def object_strings(name: str) -> list[list[int]]:
    if name == "zero-strings":
        return [[], [], [], []]
    if name == "one-nul":
        return [[0]] * 4
    if name == "embedded-nul":
        return [[65, 0, 66, 0xd800, 0], [0, 87, 0], [0, 0], []]
    if name == "surrogates":
        return [[0xd800, 0], [0xdfff, 0], [0xd800, 0xdc00, 0], [0xdc00, 0xd800, 0]]
    if name == "noncharacters":
        return [[0xfeff, 0xfffe, 0xffff, 0], [0xffff, 0], [], [0xfffe, 0]]
    first = [65, 0] + [66] * 65533 + [0] if name == "long-string" else [65, 0, 66, 0]
    return [first, [87, 97, 116, 101, 114, 0], [0xd800, 0], [90, 0]]


def object_wire(version: int, name: str, state: int) -> bytes:
    first, second, third, fourth = object_strings(name)
    raw = string_wire(first) + string_wire(second) + bytes((state,)) + struct.pack("<i", -(1 << 31)) + string_wire(third)
    return raw + (string_wire(fourth) if version >= 3 else b"")


def prefix(version: int, name: str, state: int = 128) -> bytes:
    if not 2 <= version <= 127:
        return bytes((version,))
    vectors = [] if name == "empty" else [[], [-(1 << 31), -1, 0, 1, (1 << 31) - 1], [7, 7, -1]]
    if name == "many":
        vectors = [[value, -value] for value in range(256)]
    raw = bytes((version,)) + struct.pack("<i", len(vectors)) + b"".join(counted(vector, "i") for vector in vectors)
    count = 0 if name == "empty" else 8 if name == "many" else 1
    return raw + struct.pack("<i", count) + b"".join(object_wire(version, name, state) for _ in range(count))


def cases() -> list[dict]:
    result = [dict(name=f"native-{name}-424", raw=raw) for name, raw in native_records(424)]
    for version in range(256):
        result.append(dict(name=f"version-byte-{version}", raw=prefix(version, "rich") + TAIL))
    for version in (2, 3, 127):
        for name in NAMES:
            result.append(dict(name=f"v{version}-{name}", raw=prefix(version, name) + (b"" if name in ("empty", "zero-strings") else TAIL)))
    for state in range(256):
        result.append(dict(name=f"v3-state-{state}", raw=prefix(3, "zero-strings", state) + TAIL))
    return result


def string_packet(value: dict) -> bytes:
    raw = packed([value[key] for key in ("start", "end", "stored_units", "text_start")], "I")
    return raw + blob(value["raw_bytes"]) + blob(value["text"].encode("utf-16le", "surrogatepass"))


def object_packet(value: dict) -> bytes:
    raw = packed([value[key] for key in ("ordinal", "start", "end")], "I")
    raw += string_packet(value["member_30_string"]) + string_packet(value["member_00_string"])
    raw += bytes((value["state_byte"], value["native_state_boolean"])) + struct.pack("<i", value["member_c4_i32"])
    raw += string_packet(value["member_60_string"])
    fourth = value["member_90_stored_string"]
    raw += bytes((fourth is not None,)) + (string_packet(fourth) if fourth is not None else b"")
    return raw + blob(value["member_90_native_text"].encode("utf-16le", "surrogatepass")) + bytes((value["member_90_derived_from_30"],))


def case_packet(case: dict) -> bytes:
    parsed = flowmetadataprefix.read_record_424_prefix(dict(identifier=424, size=len(case["raw"]), payload=case["raw"]), string_mode=0)
    raw = blob(case["name"].encode()) + blob(case["raw"]) + struct.pack("<iBbB", 0, parsed["version_byte"], parsed["version_i8"], parsed["body_active"])
    raw += struct.pack("<I", len(parsed["vectors"]))
    for vector in parsed["vectors"]:
        raw += packed([vector[key] for key in ("ordinal", "start", "end")], "I") + counted(vector["values_i32"], "i")
    raw += struct.pack("<I", len(parsed["objects"])) + b"".join(map(object_packet, parsed["objects"]))
    raw += packed([parsed[key] for key in ("prefix_end", "known_prefix_bytes", "unknown_bytes")], "I") + blob(parsed["opaque_tail"])
    return raw + bytes((parsed["stop_kind"] == "requires_backend_virtual_c8", parsed["full_payload_schema"], parsed["native_runtime_state_reproduced"]))


def freeze(output_root: Path = ROOT) -> None:
    views = cases()
    packet = b"FMP1" + struct.pack("<I", len(views)) + b"".join(map(case_packet, views))
    destination = output_root / "tests/fixtures/flow-metadata/prefix.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(views), bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowmetadataprefix.py").read_bytes()).hexdigest(),
                 native_payloads=len(native_records(424)), native_known_bytes=112, native_unknown_bytes=13079,
                 scope="Complete known raw metadata prefix and exact opaque tail, explicit UTF-16 mode zero, every version/state byte, raw code units and native text/derived-string projections; never full-record acceptance")
    report = output_root / "docs/features/flow-metadata/prefix-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
