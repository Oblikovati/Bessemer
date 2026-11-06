"""Freeze Source record-400 forests in explicit wire preorder, without native vector ordering."""
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
import flowblocktrees
import flowresults
from research.flow_block_tree_replay import summarize

UNIT = ((0, 0), (0, 0), (0, 0))
EMPTY = ((1, 0), (-2147483648, 2147483647), (-2147483648, 2147483647))
FULL = ((-2147483648, 2147483647),) * 3


def blob(raw: bytes) -> bytes:
    return struct.pack("<I", len(raw)) + raw


def bounds_bytes(bounds: tuple) -> bytes:
    return struct.pack("<6i", *(value for pair in bounds for value in pair))


def header(version: int, bounds: tuple) -> bytes:
    return struct.pack("<i", version) + (bounds_bytes(bounds) if version >= 2 else b"")


def leaf(version: int, marker: int) -> bytes:
    return bytes([marker]) + (struct.pack("<I", 0xffffff00 + marker) if version == 2 else b"")


def split(version: int, children: list[bytes]) -> bytes:
    prefixes = [0, 1, 7, 127, 128, 254, 255, 33]
    return b"\xff" + b"".join((bytes([prefix]) if version in (2, 3) else b"") + child for prefix, child in zip(prefixes, children))


def deep_tree(version: int, depth: int) -> bytes:
    tree = leaf(version, 238)
    for _ in range(depth):
        tree = split(version, [tree] + [leaf(version, index) for index in range(1, 8)])
    return tree


def mixed_roots(version: int) -> bytes:
    flags = [0, 1, 255, 0, 7, 255] if version >= 2 else [1] * 6
    raw = b""
    for index, flag in enumerate(flags):
        raw += bytes([flag]) if version >= 2 else b""
        if flag:
            raw += deep_tree(version, 1) if index == 2 else leaf(version, [0, 1, 7, 127, 128, 254][index])
    return raw


def owned_decoded() -> list[tuple[str, bytes, tuple | None, int]]:
    cases = []
    mixed = ((-1, 1), (2, 3), (4, 4))
    for version in range(4):
        for label, depth in [("leaf", 0), ("split", 1), ("nested", 2), ("depth255", 255), ("depth256", 256)]:
            raw = header(version, UNIT) + (b"\xff" if version >= 2 else b"") + deep_tree(version, depth)
            cases.append((label, raw, UNIT if version < 2 else None, max(64, depth)))
        cases.append(("mixed", header(version, mixed) + mixed_roots(version), mixed if version < 2 else None, 64))
        cases.append(("zero-grid", header(version, EMPTY), EMPTY if version < 2 else None, 64))
    for version in (4, 5, 18, 2147483647):
        for bounds in (UNIT, EMPTY, FULL):
            cases.append(("inactive", header(version, bounds) + b"\0", None, 64))
    return cases


def owned_cases() -> list[dict]:
    cases = []
    for name, decoded, bounds, maximum_depth in owned_decoded():
        for compression in ("none", "zlib"):
            payload = decoded if compression == "none" else zlib.compress(decoded)
            cases.append(dict(name=name, payload=payload, decoded=decoded, bounds=bounds,
                              maximum_depth=maximum_depth, compression=compression))
    return cases


def native_cases() -> tuple[list[dict], list[dict]]:
    corpus = json.loads((SOURCE / "research/flow-block-tree-corpus.json").read_text())
    cases, audit = [], []
    for row in corpus:
        raw = Path(row["path"]).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == row["source_sha256"]
        record = flowresults.latest(flowresults.read(raw), 400)
        assert (record is not None) == row["record_400_present"]
        audit.append(dict(source_sha256=row["source_sha256"], present=record is not None))
        if record is None:
            continue
        assert hashlib.sha256(record["payload"]).hexdigest() == row["payload_sha256"]
        parsed = flowblocktrees.read_record(record, compression="zlib")
        assert summarize(parsed) == row["projection"]
        cases.append(dict(name="native-cpt-zlib", payload=record["payload"], decoded=zlib.decompress(record["payload"]),
                          bounds=None, maximum_depth=64, compression="zlib"))
    return cases, audit


def node_shape(node: dict) -> dict:
    return dict(start=node["start"], end=node["end"], marker=node["marker_byte"], depth=node["depth"],
                level=node["native_level_byte"], word=node.get("discarded_leaf_word_bits"),
                children=[dict(prefix=child["discarded_prefix_byte"], index=None) for child in node.get("children", [])])


def append_tree(tree: dict, nodes: list[dict]) -> int:
    first = len(nodes)
    pending = [(tree, None, None)]
    while pending:
        node, parent, slot = pending.pop()
        index = len(nodes)
        nodes.append(node_shape(node))
        if parent is not None:
            nodes[parent]["children"][slot]["index"] = index
        children = node.get("children", [])
        for child in reversed(range(len(children))):
            pending.append((children[child]["node"], index, child))
    return first


def optional_byte(value: int | None) -> bytes:
    return bytes([value is not None]) + (b"" if value is None else bytes([value]))


def node_packet(node: dict) -> bytes:
    packet = struct.pack("<IIBHB", node["start"], node["end"], node["marker"], node["depth"], node["level"])
    packet += bytes([node["word"] is not None]) + (b"" if node["word"] is None else struct.pack("<I", node["word"]))
    packet += bytes([len(node["children"])])
    return packet + b"".join(optional_byte(child["prefix"]) + struct.pack("<I", child["index"]) for child in node["children"])


def forest_packet(parsed: dict) -> bytes:
    nodes, roots = [], []
    for root in parsed["roots"] or []:
        index = append_tree(root["tree"], nodes) if root["present"] else -1
        roots.append(struct.pack("<II3i", root["start"], root["end"], *root["indices"])
                     + optional_byte(root["presence_byte"]) + struct.pack("<i", index))
    packet = bytes([parsed["roots"] is not None]) + struct.pack("<I", len(roots)) + b"".join(roots)
    return packet + struct.pack("<I", len(nodes)) + b"".join(map(node_packet, nodes))


def case_packet(case: dict) -> bytes:
    record = dict(identifier=400, payload=case["payload"], size=len(case["payload"]))
    parsed = flowblocktrees.read_record(record, compression=case["compression"], legacy_bounds=case["bounds"], max_depth=case["maximum_depth"])
    packet = blob(case["name"].encode()) + bytes([case["compression"] == "zlib"]) + struct.pack("<H", case["maximum_depth"])
    packet += bytes([case["bounds"] is not None]) + (b"" if case["bounds"] is None else bounds_bytes(case["bounds"]))
    packet += blob(case["payload"]) + blob(case["decoded"]) + struct.pack("<i", parsed["version"]) + bounds_bytes(parsed["bounds"])
    packet += bytes([parsed["bounds_stored"]]) + parsed["grid_count"].to_bytes(16, "little") + bytes([parsed["body_present"]])
    return packet + optional_byte(parsed.get("body_presence_byte")) + forest_packet(parsed) + bytes([parsed["native_block_vector_order_proven"]])


def freeze(output_root: Path = ROOT) -> None:
    native, audit = native_cases()
    cases = native + owned_cases()
    packet = b"FBT1" + struct.pack("<I", len(cases)) + b"".join(map(case_packet, cases))
    fixture = output_root / "tests/fixtures/flow-block-trees/trees.bin.gz"
    fixture.parent.mkdir(parents=True, exist_ok=True)
    fixture.write_bytes(gzip.compress(packet, mtime=0))
    report = dict(cases=len(cases), native_payloads=len(native), native_stored_bytes=340, native_decoded_bytes=8908,
                  installed_sources=audit, bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                  reference_sha256=hashlib.sha256((SOURCE / "flowblocktrees.py").read_bytes()).hexdigest(),
                  scope="Wire preorder and explicit compression/bounds/depth only; no delegated active tree or native block-vector ordering")
    destination = output_root / "docs/features/flow-block-trees/fixture-verification.json"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
