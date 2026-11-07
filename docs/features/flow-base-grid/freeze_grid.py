"""Freeze record-101 dimensions and raw boundary words before adding its API."""
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
import flowbasegrid
import flowresults


def blob(raw: bytes) -> bytes:
    return struct.pack("<I", len(raw)) + raw


def native_cases() -> list[tuple[str, bytes]]:
    rows = json.loads((SOURCE / "research/flow-base-grid-corpus.json").read_text())
    cases = []
    for row in rows:
        wire = Path(row["path"]).read_bytes()
        assert hashlib.sha256(wire).hexdigest() == row["source_sha256"]
        record = flowresults.latest(flowresults.read(wire), 101)
        assert (record is not None) == row["record_101_present"]
        if record is None:
            continue
        raw = record["payload"]
        parsed = flowbasegrid.read_decoded(raw)
        assert hashlib.sha256(raw).hexdigest() == row["payload_sha256"]
        assert list(parsed["dimensions"]) == row["dimensions"]
        assert [list(a["raw_bits"]) for a in parsed["boundary_arrays"]] == [a["raw_bits"] for a in row["boundary_arrays"]]
        cases.append(("native-cpt", raw))
    return cases


def owned_cases() -> list[tuple[str, bytes]]:
    patterns = [0, 0x8000000000000000, 0x7ff0000000000000, 0xfff0000000000000,
                0x7ff8000000001234, 0x7ff0000000000001, 1, 0xffffffffffffffff]
    cases = []
    for dimensions in itertools.product((0, 1, 3), repeat=3):
        count = sum(n + 1 for n in dimensions)
        words = [patterns[index % len(patterns)] for index in range(count)]
        raw = struct.pack("<3i", *dimensions) + struct.pack("<" + "Q" * count, *words)
        cases.append(("owned-" + "-".join(map(str, dimensions)), raw))
    return cases


def case_packet(case: tuple[str, bytes], compressed: bool) -> bytes:
    import zlib
    name, raw = case
    stored = zlib.compress(raw, 9) if compressed else raw
    parsed = flowbasegrid.read_record(dict(identifier=101, size=len(stored), payload=stored), compression="zlib" if compressed else "none")
    packet = blob(name.encode()) + bytes([compressed]) + blob(stored) + blob(raw)
    packet += struct.pack("<3i", *parsed["dimensions"])
    packet += struct.pack("<6i", *(n for pair in parsed["default_tree_bounds"] for n in pair))
    packet += parsed["root_grid_count_mathematical"].to_bytes(16, "little")
    for array in parsed["boundary_arrays"]:
        packet += struct.pack("<3I", array["start"], array["end"], array["count"])
        packet += struct.pack("<" + "Q" * array["count"], *array["raw_bits"])
    return packet


def freeze(output_root: Path = ROOT) -> None:
    cases = native_cases() + owned_cases()
    packets = [case_packet(case, compressed) for case in cases for compressed in (False, True)]
    packet = b"FBG1" + struct.pack("<I", len(packets)) + b"".join(packets)
    destination = output_root / "tests/fixtures/flow-base-grid/grid.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    report = dict(cases=len(packets), native_payloads=1, native_payload_bytes=308,
                  bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                  reference_sha256=hashlib.sha256((SOURCE / "flowbasegrid.py").read_bytes()).hexdigest(),
                  scope="Exact dimensions, bounds, boundary word bits and framing; no refinement or native indexing")
    proof = output_root / "docs/features/flow-base-grid/fixture-verification.json"
    proof.parent.mkdir(parents=True, exist_ok=True)
    proof.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
