"""Freeze Source projections for all seven labels and explicit prerequisite states."""
from __future__ import annotations
import gzip
import hashlib
import json
import math
import struct
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(SOURCE))
import flowresults
import flowsettingsfields


def frame(identifier: int, payload: bytes) -> bytes:
    return struct.pack("<IQ", identifier, len(payload)) + payload


def owned_wire(bits: int, guard: int | None) -> bytes:
    numeric = [0] * 320
    for index in flowsettingsfields.SLOTS.values():
        numeric[index] = bits
    wire = b"\x08\0" + frame(102, struct.pack("<320Q", *numeric))
    if guard is None:
        return wire
    integers = [0] * 100
    integers[88] = guard
    return wire + frame(7, struct.pack("<100i", *integers))


def numeric_patterns() -> list[int]:
    values = [0.0, -0.0, 0.25, -0.5, 0.9, -0.9, 2.9, -3.9,
              2147483647.0, 2147483647.5, 2147483648.0, -2147483648.0,
              -2147483648.5, -2147483649.0, math.inf, -math.inf,
              math.nextafter(2147483648.0, 0), math.nextafter(-2147483649.0, 0)]
    bits = [struct.unpack("<Q", struct.pack("<d", value))[0] for value in values]
    return bits + [1, 0x8000000000000001, 0x7ff8000000000042, 0xfff8000000000042, 0x7ff0000000000001]


def native_wires() -> list[bytes]:
    corpus = json.loads((SOURCE / "research/flow-settings-fields-corpus.json").read_text())
    wires = []
    for sample in corpus:
        original = Path(sample["path"]).read_bytes()
        assert hashlib.sha256(original).hexdigest() == sample["source_sha256"]
        document = flowresults.read(original)
        assert flowsettingsfields.read(document) == sample["projection"]
        wire = b"\x07\0"
        for identifier in (102, 7):
            selected = flowresults.latest(document, identifier)
            if selected is not None:
                wire += frame(identifier, selected["payload"])
        wires.append(wire)
    return wires


def expected_bytes(wire: bytes) -> bytes:
    projection = flowsettingsfields.read(flowresults.read(wire))
    if projection is None:
        return b"\0"
    guard = projection["integer_slot_88"]
    result = b"\1" + bytes([guard is not None]) + struct.pack("<i", guard or 0)
    status = {"derived": 0, "requires_integer_slot_88": 1, "requires_fp_exception_context": 2}
    for field in projection["fields"].values():
        result += struct.pack("<IQBi", field["index"], field["stored_bits"],
                              status[field["status"]], field["value"] or 0)
    return result


def freeze(output_root: Path = ROOT) -> None:
    guards = [None, 0, 1, -1, -2147483648, 2147483647]
    wires = native_wires() + [owned_wire(bits, guard) for bits in numeric_patterns() for guard in guards]
    packet = b"FSF1" + struct.pack("<I", len(wires))
    packet += b"".join(struct.pack("<I", len(wire)) + wire + expected_bytes(wire) for wire in wires)
    fixture = output_root / "tests/fixtures/flow-settings-fields/fields.bin.gz"
    fixture.parent.mkdir(parents=True, exist_ok=True)
    fixture.write_bytes(gzip.compress(packet, mtime=0))
    report = dict(cases=len(wires), native_cases=3, numeric_patterns=len(numeric_patterns()), guards=6,
                  bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                  reference_sha256=hashlib.sha256((SOURCE / "flowsettingsfields.py").read_bytes()).hexdigest(),
                  scope="Pure source-bound projection; no native execution or stored-array normalization")
    destination = output_root / "docs/features/flow-settings-fields/fixture-verification.json"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
