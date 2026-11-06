"""Freeze native root strides and local leaf paths without a global block assignment."""
from __future__ import annotations
import gzip
import hashlib
import itertools
import json
import struct
import sys
import zlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(SOURCE))
sys.path.insert(0, str(ROOT / "docs/features/flow-block-trees"))
import freeze_trees
import flowbasegrid
import flowblocktrees
import flowrootindices
import flowresults


def blob(raw: bytes) -> bytes:
    return struct.pack("<I", len(raw)) + raw


def native_cases() -> list[dict]:
    row = json.loads((SOURCE / "research/flow-block-data-corpus.json").read_text())[1]
    wire = Path(row["path"]).read_bytes()
    assert hashlib.sha256(wire).hexdigest() == row["source_sha256"]
    document = flowresults.read(wire)
    grid, trees = (flowresults.latest(document, identifier)["payload"] for identifier in (101, 400))
    assert hashlib.sha256(grid).hexdigest() == "d3a3a2b2c28f0e91a7c8a552cbca8e252377cac6e7c10110b572b84308b820f4"
    assert hashlib.sha256(trees).hexdigest() == json.loads((SOURCE / "research/flow-block-tree-corpus.json").read_text())[1]["payload_sha256"]
    return [dict(name="native-cpt", grid=grid, trees=trees, compressed=True, legacy=None, maximum_depth=64)]


def grid_wire(dimensions: tuple[int, int, int]) -> bytes:
    count = sum(n + 1 for n in dimensions)
    patterns = (0, 0x8000000000000000, 0x7ff8000000001234, 0x7ff0000000000001)
    return struct.pack("<3i", *dimensions) + struct.pack("<" + "Q" * count, *(patterns[i % 4] for i in range(count)))


def owned_case(dimensions: tuple[int, int, int], version: int, compressed: bool, depth: int | None) -> dict:
    bounds = tuple((0, n - 1) for n in dimensions)
    count = dimensions[0] * dimensions[1] * dimensions[2]
    body = b""
    for ordinal in range(count):
        body += bytes([(1, 255, 128)[ordinal % 3]]) if version >= 2 else b""
        body += freeze_trees.deep_tree(version, ordinal % 3 if depth is None else depth)
    raw = freeze_trees.header(version, bounds) + body
    return dict(name=f"grid-{dimensions}-v{version}-depth{depth}", grid=grid_wire(dimensions),
                trees=zlib.compress(raw, 9) if compressed else raw, compressed=compressed,
                legacy=bounds if version < 2 else None, maximum_depth=max(64, depth or 0))


def owned_cases() -> list[dict]:
    cases = [owned_case(dimensions, version, compressed, None)
             for dimensions in itertools.product((0, 1, 2), repeat=3)
             for version in range(4) for compressed in (False, True)]
    cases += [owned_case((1, 1, 1), version, compressed, depth)
              for depth in (255, 256) for version in range(4) for compressed in (False, True)]
    return cases


def project(case: dict) -> dict:
    grid = flowbasegrid.read_decoded(case["grid"])
    tree = zlib.decompress(case["trees"]) if case["compressed"] else case["trees"]
    parsed = flowblocktrees.read_decoded(tree, legacy_bounds=case["legacy"], max_depth=case["maximum_depth"])
    return flowrootindices.project(grid, parsed, fresh_context=True)


def root_packet(root: dict) -> bytes:
    raw = struct.pack("<4iQI", *root["indices"], root["native_root_index"], root["native_root_byte_offset"], len(root["leaves"]))
    for leaf in root["leaves"]:
        raw += struct.pack("<I", leaf["local_leaf_ordinal"]) + blob(bytes(leaf["child_path"]))
        raw += struct.pack("<II", leaf["wire_start"], leaf["wire_end"])
    return raw


def case_packet(case: dict) -> bytes:
    projected = project(case)
    raw = blob(case["name"].encode()) + struct.pack("<BH", case["compressed"], case["maximum_depth"])
    raw += bytes([case["legacy"] is not None])
    if case["legacy"] is not None:
        raw += freeze_trees.bounds_bytes(case["legacy"])
    raw += blob(case["grid"]) + blob(case["trees"])
    raw += struct.pack("<II", projected["leaf_count"], len(projected["roots"]))
    raw += b"".join(map(root_packet, projected["roots"]))
    flags = ("fresh_context_assumed", "native_context_executed", "global_block_ordinals_assigned", "parallel_scan_offsets_validated")
    return raw + bytes(projected[key] for key in flags)


def freeze(output_root: Path = ROOT) -> None:
    cases = native_cases() + owned_cases()
    packet = b"FRI1" + struct.pack("<I", len(cases)) + b"".join(map(case_packet, cases))
    destination = output_root / "tests/fixtures/flow-root-indices/indices.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(cases), native_roots=792, native_leaves=3984,
                 bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowrootindices.py").read_bytes()).hexdigest(),
                 scope="Root storage index/56-byte stride and local wire leaf paths; explicit fresh assumption; no global block ordinals")
    report = output_root / "docs/features/flow-root-indices/fixture-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
