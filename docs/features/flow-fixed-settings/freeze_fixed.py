"""Freeze specification 14.34 raw slots without applying runtime normalization."""
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
import flowfixedsettings
import flowresults


def native_cases() -> list[dict]:
    corpus = json.loads((SOURCE / "research/flow-fixed-settings-corpus.json").read_text())
    cases = []
    for sample in corpus:
        wire = Path(sample["path"]).read_bytes()
        assert hashlib.sha256(wire).hexdigest() == sample["source_sha256"]
        document = flowresults.read(wire)
        for expected in sample["records"]:
            selected = flowfixedsettings.read_record(document, expected["identifier"])
            assert list(selected["raw_bits"]) == expected["raw_bits"]
            record = flowresults.latest(document, expected["identifier"])
            assert hashlib.sha256(record["payload"]).hexdigest() == expected["payload_sha256"]
            cases.append(dict(name=Path(sample["path"]).suffix, identifier=expected["identifier"],
                              payload=record["payload"], bits=selected["raw_bits"]))
    return cases


def owned_cases() -> list[dict]:
    words = [0, 1, 0x7fffffff, 0x80000000, 0xffffffff]
    wides = [0, 0x8000000000000000, 0x3ff0000000000000, 0x7ff0000000000000,
             0xfff0000000000000, 0x7ff8000000000042, 0x7ff0000000000001, 1]
    cases = []
    for identifier, count, values, code in [(7, 100, words, "I"), (102, 320, wides, "Q")]:
        bits = tuple(values[index % len(values)] for index in range(count))
        payload = struct.pack("<" + code * count, *bits)
        parsed = flowfixedsettings.read_payload(identifier, payload)
        assert parsed["raw_bits"] == bits
        cases.append(dict(name="owned", identifier=identifier, payload=payload, bits=bits))
    return cases


def packet_case(case: dict) -> bytes:
    name = case["name"].encode()
    prefix = struct.pack("<I", len(name)) + name
    prefix += struct.pack("<III", case["identifier"], len(case["payload"]), len(case["bits"]))
    return prefix + case["payload"] + struct.pack("<" + "Q" * len(case["bits"]), *case["bits"])


def freeze(output_root: Path = ROOT) -> None:
    cases = native_cases() + owned_cases()
    packet = b"FFS1" + struct.pack("<I", len(cases)) + b"".join(map(packet_case, cases))
    fixture = output_root / "tests/fixtures/flow-fixed-settings/slots.bin.gz"
    fixture.parent.mkdir(parents=True, exist_ok=True)
    fixture.write_bytes(gzip.compress(packet, mtime=0))
    report = dict(cases=len(cases), native_payloads=4, native_bytes=5920,
                  slots=sum(len(case["bits"]) for case in cases), bytes=len(packet),
                  sha256=hashlib.sha256(packet).hexdigest(),
                  reference_sha256=hashlib.sha256((SOURCE / "flowfixedsettings.py").read_bytes()).hexdigest(),
                  scope="Strict raw slots only; no native execution, field semantics or normalization")
    destination = output_root / "docs/features/flow-fixed-settings/fixture-verification.json"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
