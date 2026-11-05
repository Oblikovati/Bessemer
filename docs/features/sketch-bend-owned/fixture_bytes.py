"""Complete explicitly declared canonical kind-7 graphs and raw frame suffixes (#219)."""
from pathlib import Path
import struct
import sys
import hashlib
import json

ROOT = Path(__file__).resolve().parents[3]
for folder in ('model-archives', 'reference-axes', 'sweeps', 'circular-patterns', 'resolved-features',
               'edge-flanges', 'sketch-bends', 'sheet-metal', 'base-flanges', 'weld-corners', 'weldments'):
    sys.path.append(str(ROOT / 'docs/features' / folder))
from sketch_bend_bytes import SketchBendGraph
from kernel_geometry_fixture import frame

WORDS = (-2147483648, -1, 0, 1, 65535, 2147483647)
BITS = (0, 0x8000000000000000, 1, 0x8000000000000001,
    0x7fefffffffffffff, 0xffefffffffffffff, 0x7ff0000000000000, 0xfff0000000000000,
    0x7ff8000000001234, 0xfff8000000004321, 0x7ff0000000000001, 0xfff0000000000001,
    0x0010000000000000, 0x8010000000000000, 0x3ff0000000000000, 0xbff0000000000000)
CONTEXTS = (('empty', 0, False), ('rich', 3, False), ('aliases', 5, False),
    ('helper-empty', 0, True), ('helper-rich', 3, True), ('helper-aliases', 5, True))


def owned_array(items: list[bytes]) -> bytes:
    """Retain direct standard counts and map references; e.g. owned_array([bytes(2)])."""
    prefix = struct.pack('<HI', 65535, len(items)) if items else bytes(2)
    return prefix + b''.join(items)


def owned_grid(graph: SketchBendGraph, owner: int, index: int, first: int, last: int, edit: int) -> bytes:
    """Keep the grid-array138 and later face-word138 distinct; e.g. owned_grid(g, 2, 0, -1, 1, 0)."""
    floats = [BITS[(index + offset) % 16] for offset in range(8)]
    raw = struct.pack('<i6Q', first, *floats[:6])
    items = [struct.pack('<H', owner), graph.marker(), bytes(2), graph.marker()] if graph.style else []
    raw += owned_array(items)
    raw += owned_array([graph.curve(), graph.marker()] if graph.style else [])
    raw += struct.pack('<2Qi', 0x7ff0000000000001 if edit == 1 else floats[6], floats[7],
        12345 if edit == 2 else last)
    return raw


def owned_feature(graph: SketchBendGraph, key: str, index: int, first: int, last: int, edit: int) -> bytes:
    """Use the full inherited base, direct owned data, ordinary suffix and mandatory raw frame; e.g. owned_feature(g, 'first', 0, -1, 1, 0)."""
    def contents() -> bytes:
        raw = graph.one_bend(False, kind=7)
        raw += owned_grid(graph, graph.ids[key], index, first, last, edit)
        raw += graph.marker() + graph.tagged_handle(True) + graph.tagged_handle(False) + graph.tagged_handle(False)
        raw += graph.curve() + struct.pack('<i', -19)
        return raw + frame(-7)[1:]
    return graph.shared(key, 'moSketchBend_c', contents)


def owned_graph(context: tuple[str, int, bool], version: int = 19000, first_slot: int = 0,
                edit: int = 0, index: int = 0, first: int = -2147483648, last: int = 2147483647) -> tuple[bytes, int]:
    """Reserve real owner identities before generic array back-references; e.g. owned_graph(CONTEXTS[1])."""
    _, style, extended = context
    graph = SketchBendGraph(version, style, first_slot, extended)
    graph.mapping.schema = 1
    graph.curve_class = 'moLineRef_w'
    graph.curve_mode = 0
    def contents() -> bytes:
        left = owned_feature(graph, 'first', index, first, last, edit)
        right = owned_feature(graph, 'second', index, first, last, 0)
        return struct.pack('<H', 3) + left + right + owned_feature(graph, 'first', index, first, last, 0)
    raw = graph.mapping.new('suObArray', contents) if style == 5 else owned_feature(graph, 'first', index, first, last, edit)
    return raw, graph.mapping.slot + 1


def owned_packet() -> bytes:
    """Cross sixteen double patterns and both full signed-word domains; e.g. owned_packet()."""
    records = [owned_graph(CONTEXTS[0], index=index, first=first, last=last)[0]
        for index in range(16) for first in WORDS for last in WORDS]
    return b'SK71' + struct.pack('<I', len(records)) + b''.join(
        struct.pack('<I', len(raw)) + raw for raw in records)


def owned_cases() -> dict[str, tuple[bytes, int]]:
    """Keep both saved helper layouts and independent edits; e.g. owned_cases()['rich.bin']."""
    cases = {f'{context[0]}.bin': owned_graph(context) for context in CONTEXTS[:3]}
    cases['minimum-rich.bin'] = owned_graph(CONTEXTS[1], 0x27cc)
    for context in CONTEXTS:
        for edit in range(3):
            raw, slots = owned_graph(context, first_slot=96, edit=edit)
            cases[f'owned-{context[0]}-{edit}.bin'] = struct.pack('<iH', -123, 1) + raw + (bytes(4) if context[2] else b''), slots
    cases['domains.bin'] = owned_packet(), 7
    return cases


def verify_owned_cases() -> None:
    """Check every stored literal without the production encoder; e.g. python fixture_bytes.py."""
    ref = json.loads(Path(__file__).with_name('fixture-reference.json').read_text())
    cases = owned_cases()
    assert set(ref['files']) == set(cases)
    for name, (raw, _) in cases.items():
        assert raw == (ROOT / 'tests/fixtures/sketch-bend-owned' / name).read_bytes(), name
        assert hashlib.sha256(raw).hexdigest() == ref['files'][name], name


if __name__ == '__main__':
    verify_owned_cases()
