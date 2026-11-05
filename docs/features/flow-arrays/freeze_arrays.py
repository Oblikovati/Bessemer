"""Freeze record-433 arrays from independent Python decoding and literal signed fixtures."""
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
sys.path.insert(0, str(ROOT / "docs/features/flow-carriers"))
import flowmaps
import flowresults
from freeze_carriers import blob, owned


def signed_array(values: list[int]) -> bytes:
    return struct.pack("<I", len(values)) + b"".join(struct.pack("<i", value) for value in values)


def owned_cases() -> list[tuple[str, bytes, int]]:
    cases = []
    for selector in [0, 1, 2, 0x7FFFFFFF]:
        for count in [0, 1, 3]:
            first = [-1, -0x80000000, 0x7FFFFFFF][:count] if selector == 0 else []
            second = [0x7FFFFFFF, -1, -0x80000000][:count]
            decoded = struct.pack("<ii", selector, count) + b"".join(
                struct.pack("<i", value) for value in first + second)
            payload = zlib.compress(decoded)
            raw = owned(8, [(123, bytes(32)), (433, payload), (19, b"opaque")])
            cases.append((f"selector-{selector}-count-{count}", raw, 1))
    return cases


def native_case() -> tuple[str, bytes, int]:
    corpus = json.loads((SOURCE / "research/flow-compressed-array-corpus.json").read_text())
    raw = Path(corpus["path"]).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == corpus["source_sha256"]
    record = flowresults.read(raw)["records"][2]
    assert hashlib.sha256(record["payload"]).hexdigest() == corpus["record_payload_sha256"]
    decoded = flowmaps.read_record(record)
    for key in ["first", "second"]:
        wire = b"".join(struct.pack("<i", value) for value in decoded[key])
        assert hashlib.sha256(wire).hexdigest() == corpus[key]["sha256"]
    return "native-cfld", raw, 2


def packet_case(name: str, raw: bytes, ordinal: int) -> bytes:
    record = flowresults.read(raw)["records"][ordinal]
    decoded = flowmaps.read_record(record)
    packet = blob(name.encode()) + blob(raw) + struct.pack("<Ii", ordinal, decoded["selector"])
    packet += bytes([decoded["first"] is not None])
    if decoded["first"] is not None:
        packet += signed_array(decoded["first"])
    packet += signed_array(decoded["second"])
    packet += blob(zlib.decompress(record["payload"]))
    return packet


def freeze(output_root: Path = ROOT) -> None:
    cases = [native_case()] + owned_cases()
    packet = b"FMA1" + struct.pack("<I", len(cases)) + b"".join(packet_case(*case) for case in cases)
    (output_root / "tests/fixtures/flow-arrays/arrays.bin.gz").write_bytes(gzip.compress(packet, mtime=0))
    wrong_wrapper = gzip.compress(struct.pack("<iiii", 0, 1, -1, 5), mtime=0)
    (output_root / "tests/fixtures/flow-arrays/wrong-wrapper.gz").write_bytes(wrong_wrapper)
    report = dict(cases=len(cases), native_elements_per_array=5952, bytes=len(packet),
                  sha256=hashlib.sha256(packet).hexdigest(), wrong_wrapper_sha256=hashlib.sha256(wrong_wrapper).hexdigest(),
                  references={name: hashlib.sha256((SOURCE / name).read_bytes()).hexdigest()
                              for name in ["flowresults.py", "flowmaps.py"]},
                  inputs=[dict(name=name, bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())
                          for name, raw, _ in cases])
    (output_root / "docs/features/flow-arrays/fixture-verification.json").write_text(
        json.dumps(report, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(dict(cases=len(cases), packet_bytes=len(packet)))


if __name__ == "__main__":
    freeze()
