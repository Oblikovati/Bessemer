"""Literal schema-1 conic data and complete OneBend carriers (#218)."""
from pathlib import Path
import hashlib
import json
import struct
import sys

ROOT = Path(__file__).resolve().parents[3]
for folder in ('model-archives', 'reference-axes', 'sweeps', 'circular-patterns',
               'resolved-features', 'edge-flanges', 'sheet-metal', 'base-flanges', 'weld-corners', 'weldments'):
    sys.path.append(str(ROOT / 'docs/features' / folder))
from fixture_object_map import object_tag
from edge_fixture_bytes import EdgeGraph

CONIC = 'moConicBendData_c'
BITS = (0, 0x8000000000000000, 1, 0x8000000000000001,
    0x7fefffffffffffff, 0xffefffffffffffff, 0x7ff0000000000000, 0xfff0000000000000,
    0x7ff8000000001234, 0xfff8000000004321, 0x7ff0000000000001, 0xfff0000000000001,
    0x0010000000000000, 0x8010000000000000, 0x3ff0000000000000, 0xbff0000000000000)
WORDS = (-2147483648, -1, 0, 1, 255, 2147483647)
CONTEXTS = (('empty', 0, False), ('rich', 3, False), ('aliases', 5, False),
    ('helper-empty', 0, True), ('helper-rich', 3, True), ('helper-aliases', 5, True))


def conic_payload(index: int, first: int, second: int, edit: int = 0) -> bytes:
    """Keep eight independent double bit patterns and two signed words; e.g. conic_payload(0, -1, 255)."""
    floats = [BITS[(index + offset) % len(BITS)] for offset in range(8)]
    if edit == 2:
        floats[6] = 0x7ff0000000000001
    return struct.pack('<8Qii', *floats, 12345 if edit == 1 else first, second)


def conic_pointer(graph: EdgeGraph, payload: bytes) -> bytes:
    """Keep the conic schema distinct from inherited fixture schemas; e.g. conic_pointer(g, bytes(72))."""
    previous = graph.mapping.schema
    graph.mapping.schema = 1
    raw = graph.shared('conic', CONIC, lambda: payload)
    graph.mapping.schema = previous
    return raw


def conic_feature(graph: EdgeGraph, key: str, payload: bytes) -> bytes:
    """Use the existing complete OneBend assembler with a real conic pointer; e.g. conic_feature(g, 'first', bytes(72))."""
    return graph.shared(key, 'moOneBend_c',
        lambda: graph.one_bend(False, lambda: conic_pointer(graph, payload)))


def conic_graph(context: tuple[str, int, bool], version: int = 19000,
                first_slot: int = 0, edit: int = 0) -> tuple[bytes, int]:
    """Compose separate features sharing one conic object; e.g. conic_graph(CONTEXTS[2])."""
    _, style, extended = context
    graph = EdgeGraph(version, style, first_slot, extended)
    payload = conic_payload(0, -2147483648, -1, edit)
    def contents() -> bytes:
        first = conic_feature(graph, 'first', payload)
        second = conic_feature(graph, 'second', payload)
        return struct.pack('<H', 3) + first + second + conic_feature(graph, 'first', payload)
    raw = graph.mapping.new('suObArray', contents) if style == 5 else conic_feature(graph, 'first', payload)
    return raw, graph.mapping.slot + 1


def conic_packet() -> bytes:
    """Pin 576 independent point/vector/scalar/state combinations; e.g. conic_packet()."""
    records = [object_tag(CONIC, 1) + conic_payload(index, first, second)
        for index in range(len(BITS)) for first in WORDS for second in WORDS]
    return b'CBN1' + struct.pack('<I', len(records)) + b''.join(
        struct.pack('<I', len(raw)) + raw for raw in records)


def conic_cases() -> dict[str, tuple[bytes, int]]:
    """Generate literal roots, helper-selected documents and independent edits; e.g. conic_cases()['rich.bin']."""
    cases = {f'{context[0]}.bin': conic_graph(context) for context in CONTEXTS[:3]}
    cases['minimum-rich.bin'] = conic_graph(CONTEXTS[1], 0x27cc)
    for context in CONTEXTS:
        for edit in range(3):
            raw, slots = conic_graph(context, first_slot=96, edit=edit)
            suffix = bytes(4) if context[2] else b''
            cases[f'owned-{context[0]}-{edit}.bin'] = struct.pack('<iH', -123, 1) + raw + suffix, slots
    cases['domains.bin'] = conic_packet(), 3
    return cases


def verify_conic_cases() -> None:
    """Check every frozen byte/hash without a production encoder; e.g. python fixture_bytes.py."""
    reference = json.loads(Path(__file__).with_name('fixture-reference.json').read_text())
    cases = conic_cases()
    assert set(cases) == set(reference['files'])
    for name, (raw, _) in cases.items():
        assert raw == (ROOT / 'tests/fixtures/conic-bend' / name).read_bytes(), name
        assert hashlib.sha256(raw).hexdigest() == reference['files'][name], name


if __name__ == '__main__':
    verify_conic_cases()
