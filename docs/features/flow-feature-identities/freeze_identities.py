"""Freeze specification 14.37 wire fields without runtime matching or Unicode normalization."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(SOURCE))
import flowfeatureidentities as identities
import flowresults


def blob(raw: bytes) -> bytes:
    return struct.pack("<I", len(raw)) + raw


def object_wire(kind: int, units: list[int], state: int, index: int) -> bytes:
    return struct.pack("<IIIi", 0x80000000 + index, kind, 99 + index, len(units)) + struct.pack("<" + "H" * len(units), *units) + bytes([state])


def list_wire(version: int, objects: list[bytes], tail: tuple[int, ...]) -> bytes:
    prefix = struct.pack("<Ii", version, len(objects)) + b"".join(objects)
    return prefix if version < 4 else prefix + struct.pack("<i", len(tail)) + struct.pack("<" + "i" * len(tail), *tail)


def native_cases() -> list[tuple[str, int, bytes]]:
    rows = json.loads((SOURCE / "research/flow-feature-identity-corpus.json").read_text())
    cases = []
    for row in rows:
        raw = Path(row["path"]).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == row["source_sha256"]
        document = flowresults.read(raw)
        expected_ids = {record["identifier"] for record in row["records"]}
        for identifier in (143, 144):
            assert (flowresults.latest(document, identifier) is None) == (identifier not in expected_ids)
        for expected in row["records"]:
            selected = flowresults.latest(document, expected["identifier"])
            payload = selected["payload"]
            assert hashlib.sha256(payload).hexdigest() == expected["payload_sha256"]
            parsed = identities.read_record(document, expected["identifier"])
            assert parsed["end"] == expected["end"]
            cases.append(("native-fld", expected["identifier"], payload))
    return cases


def owned_cases() -> list[tuple[str, int, bytes]]:
    factory = [object_wire(kind, [65 + index, 0], state, index)
               for index, (kind, state) in enumerate(zip([0, 5, 9, 10, 39, 0xffffffff], [0, 1, 7, 127, 128, 255]))]
    texts = [[], [0], [0xd800, 0], [0xdc00, 0], [0xd83d, 0xde00, 0], [120, 0, 116, 97, 105, 108, 0]]
    edges = [object_wire(39, units, index, index) for index, units in enumerate(texts)]
    cases = []
    for version in range(7):
        for label, objects, tail in [("empty", [], ()), ("factory", factory, (-2147483648, -1, 0, 2147483647)), ("text", edges, (-1, 2))]:
            cases.append((label, 143, list_wire(version, objects, tail)))
        for count in (0, 3):
            cases.append(("nulls", 144, struct.pack("<Ii", version, count) + bytes(count)))
    return cases


def object_packet(obj: dict) -> bytes:
    words = [obj[key] for key in ("start", "end", "header_version", "type_code", "body_version", "text_count", "text_start")]
    logical = obj["text"].encode("utf-16le", "surrogatepass")
    return struct.pack("<7I", *words) + blob(obj["text_bytes"]) + blob(logical) + blob(obj["native_class"].encode()) + bytes([obj["state_byte"], obj["state_boolean"]])


def tail_packet(parsed: dict, payload: bytes) -> bytes:
    tail = parsed["tail_values"]
    if tail is not None:
        return b"\1" + struct.pack("<I", len(tail)) + struct.pack("<" + "i" * len(tail), *tail)
    contexts = [None, 0, 1, -1, -2147483648, 2147483647]
    rows = []
    for context in contexts:
        derived = identities.read143(payload, context)["derived_tail"]
        rows.append(struct.pack("<BiIB", context is not None, context or 0, derived["count"] or 0, context is not None))
    return b"\0" + struct.pack("<I", len(rows)) + b"".join(rows)


def case_packet(case: tuple[str, int, bytes]) -> bytes:
    name, identifier, payload = case
    parsed = identities.read143(payload) if identifier == 143 else identities.read144_nulls(payload)
    prefix = blob(name.encode()) + struct.pack("<I", identifier) + blob(payload)
    prefix += struct.pack("<II", parsed["version"], parsed["count"])
    if identifier == 144:
        return prefix + blob(bytes(parsed["presence_bytes"]))
    return prefix + b"".join(map(object_packet, parsed["objects"])) + tail_packet(parsed, payload)


def freeze(output_root: Path = ROOT) -> None:
    cases = native_cases() + owned_cases()
    packet = b"FFI1" + struct.pack("<I", len(cases)) + b"".join(map(case_packet, cases))
    fixture = output_root / "tests/fixtures/flow-feature-identities/identities.bin.gz"
    fixture.parent.mkdir(parents=True, exist_ok=True)
    fixture.write_bytes(gzip.compress(packet, mtime=0))
    report = dict(cases=len(cases), native_payloads=2, native_payload_bytes=219,
                  identity_cases=sum(case[1] == 143 for case in cases), null_list_cases=sum(case[1] == 144 for case in cases),
                  bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                  reference_sha256=hashlib.sha256((SOURCE / "flowfeatureidentities.py").read_bytes()).hexdigest(),
                  scope="Raw identity fields and null-list framing only; populated record 144 and runtime matching remain separate")
    destination = output_root / "docs/features/flow-feature-identities/fixture-verification.json"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
