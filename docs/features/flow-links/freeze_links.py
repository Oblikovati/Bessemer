"""Independent ordered lookup oracle from specification 14.33's Source replay."""
from __future__ import annotations
import gzip
import hashlib
import itertools
import json
import struct
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(SOURCE))
import flowresults
import flowlinks

NIL = bytes(16)
A, B, C = bytes(range(16)), bytes(range(1, 17)), bytes(range(2, 18))


def blob(value: bytes) -> bytes:
    return struct.pack("<I", len(value)) + value


def record(identifier: int, payload: bytes) -> bytes:
    return struct.pack("<IQ", identifier, len(payload)) + payload


def wire(first: bytes, second: bytes, third: bytes, version: int = 8) -> bytes:
    return struct.pack("<H", version) + record(123, first + second) + record(435, b"\1\0\0\0" + third)


def candidate(kind: int, raw: bytes, name: str) -> dict:
    return dict(result_type=kind, document=flowresults.read(raw), wire=raw, name=name)


def native_candidates() -> list[dict]:
    corpus = json.loads((SOURCE / "research/flow-guid-link-corpus.json").read_text())
    candidates = []
    for sample in corpus:
        raw = Path(sample["path"]).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == sample["source_sha256"]
        document = flowresults.read(raw)
        assert flowlinks.candidate_key(document, sample["result_type"]) == sample["lookup_key"]
        selected = [flowresults.latest(document, identifier) for identifier in (123, 435)]
        trimmed = raw[:2] + b"".join(record(item["identifier"], item["payload"]) for item in selected if item)
        candidates.append(candidate(sample["result_type"], trimmed, Path(sample["path"]).name))
    return candidates


def cases() -> list[tuple[bytes, list[dict]]]:
    native = native_candidates()
    rows = [(item["wire"], native) for item in native]
    for version in (7, 8):
        seed = wire(A, B, C, version)
        choices = [candidate(kind, seed, f"owned.{suffix}") for kind, suffix in [(1, "cpt"), (2, "fld"), (4, "cfld")]]
        rows += [(seed, list(order)) for order in itertools.permutations(choices)]
        rows += [(wire(NIL, NIL, NIL, version), choices), (seed, []), (seed, [choices[1]])]
    return rows


def queries() -> list[bytes]:
    return [NIL, A, B, C] + [bytes(value ^ (1 if index == position else 0)
            for index, value in enumerate(A)) for position in range(16)]


def lookup_row(candidates: list[dict], kind: int, query: bytes) -> bytes:
    selected = flowlinks.find_by_guid(candidates, kind, query)
    index = next((index for index, item in enumerate(candidates) if item is selected), -1)
    visited = len(candidates) if index == -1 else index + 1
    eligible = sum(item["result_type"] in flowlinks.eligible_types(kind) for item in candidates[:visited])
    return struct.pack("<I", kind) + query + struct.pack("<iII", index, visited, eligible)


def related_row(seed: bytes, candidates: list[dict]) -> bytes:
    related = flowlinks.related_carriers(candidates, flowresults.read(seed))
    indices = [next((index for index, item in enumerate(candidates) if item is related[key]), -1)
               for key in ("cpt", "static_settings")]
    key = flowlinks.read_lookup_guids(flowresults.read(seed))["static_settings_lookup_guid"]
    stage = 0 if indices[0] == -1 else 2 if key == NIL else 3
    return struct.pack("<iiB", *indices, stage)


def case_packet(seed: bytes, candidates: list[dict]) -> bytes:
    packet = blob(seed) + struct.pack("<I", len(candidates))
    for item in candidates:
        packet += struct.pack("<I", item["result_type"]) + blob(item["name"].encode()) + blob(item["wire"])
    requests = [(kind, query) for kind in (1, 2, 4) for query in queries()]
    packet += struct.pack("<I", len(requests))
    return packet + b"".join(lookup_row(candidates, kind, query) for kind, query in requests) + related_row(seed, candidates)


def freeze(output_root: Path = ROOT) -> None:
    rows = cases()
    packet = b"FLK1" + struct.pack("<I", len(rows)) + b"".join(case_packet(*row) for row in rows)
    fixture = output_root / "tests/fixtures/flow-links/links.bin.gz"
    fixture.parent.mkdir(parents=True, exist_ok=True)
    fixture.write_bytes(gzip.compress(packet, mtime=0))
    report = dict(cases=len(rows), requests=len(rows) * 60, related_selections=len(rows), bytes=len(packet),
                  sha256=hashlib.sha256(packet).hexdigest(),
                  reference_sha256=hashlib.sha256((SOURCE / "flowlinks.py").read_bytes()).hexdigest(),
                  scope="Ordered classified candidates and extracted native GUID headers; no filesystem/native execution")
    destination = output_root / "docs/features/flow-links/fixture-verification.json"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
