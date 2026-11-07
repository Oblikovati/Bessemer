"""Independent complete OneBend raw tails and containing documents (#220)."""
from pathlib import Path
import hashlib
import json
import struct
import sys

ROOT = Path(__file__).resolve().parents[3]
for folder in ('model-archives', 'reference-axes', 'sweeps', 'circular-patterns',
               'resolved-features', 'edge-flanges', 'sheet-metal', 'base-flanges', 'weld-corners', 'weldments'):
    sys.path.append(str(ROOT / 'docs/features' / folder))
from edge_fixture_bytes import EdgeGraph
from one_bend_bytes import raw_tail
from kernel_geometry_fixture import frame

CONTEXTS = (('empty', 0, False), ('rich', 3, False), ('aliases', 5, False),
    ('helper-empty', 0, True), ('helper-rich', 3, True), ('helper-aliases', 5, True))


def positive_tail(edit: int) -> bytes:
    """Keep two lists and signed nonzero flags with direct frames; e.g. positive_tail(0)."""
    body = frame(9 if edit == 1 else -7)[1:]
    raw = struct.pack('<i', 2) + body * 2 + struct.pack('<i', 1) + body
    raw += struct.pack('<i', -1 if edit == 2 else -2147483648) + body
    raw += struct.pack('<i', 2147483647) + body
    return raw + struct.pack('<4i', -1, -2147483648, 2147483647, -1)


def positive_feature(graph: EdgeGraph, key: str, edit: int) -> bytes:
    """Replace only the known empty tail of the established assembler; e.g. positive_feature(g, 'first', 0)."""
    def contents() -> bytes:
        original = graph.one_bend(False)
        empty = raw_tail(graph.version, graph.style)
        assert original.endswith(empty)
        return original[:-len(empty)] + positive_tail(edit)
    return graph.shared(key, 'moOneBend_c', contents)


def positive_graph(context: tuple[str, int, bool], edit: int) -> tuple[bytes, int]:
    """Allocate distinct roots and a cached third occurrence in wire order; e.g. positive_graph(CONTEXTS[0], 0)."""
    _, style, extended = context
    graph = EdgeGraph(19000, style, 96, extended)
    def contents() -> bytes:
        return struct.pack('<H', 3) + positive_feature(graph, 'first', edit) + (
            positive_feature(graph, 'second', edit) + positive_feature(graph, 'first', edit))
    raw = graph.mapping.new('suObArray', contents) if style == 5 else positive_feature(graph, 'first', edit)
    suffix = bytes(4) if extended else b''
    return struct.pack('<iH', -123, 1) + raw + suffix, graph.mapping.slot + 1


def positive_cases() -> dict[str, tuple[bytes, int]]:
    """Provide eighteen independent original and edited streams; e.g. positive_cases()."""
    return {f'owned-{context[0]}-{edit}.bin': positive_graph(context, edit)
        for context in CONTEXTS for edit in range(3)}


def verify_positive_cases() -> None:
    """Compare every frozen byte and identity count; e.g. python fixture_bytes.py."""
    reference = json.loads(Path(__file__).with_name('fixture-reference.json').read_text())
    cases = positive_cases()
    assert set(reference['files']) == set(cases)
    for name, (raw, slots) in cases.items():
        assert raw == (ROOT / 'tests/fixtures/one-bend-raw-bodies' / name).read_bytes(), name
        assert reference['files'][name] == {'sha256': hashlib.sha256(raw).hexdigest(), 'slots': slots}


if __name__ == '__main__':
    verify_positive_cases()
