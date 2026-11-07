"""Independent literal parting-line graphs composed from reviewed native field order (#216)."""
from pathlib import Path
import struct
import hashlib
import json
from itertools import product
import sys

ROOT = Path(__file__).resolve().parents[3]
for folder in ('model-archives', 'reference-axes', 'sweeps', 'resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(ROOT / 'docs/features' / folder))
from fillet_fixture_bytes import WireMap, body
from kernel_geometry_fixture import frame
from body_feature_fixture_bytes import direction
from fixture_object_map import shared_object
from freeze_component_references import wire as component_wire
from display_dimension_fixture_bytes import body as strip_tag

FEATURE = 'moMoldPartingLine_c'
SETTINGS = 'moMoldPartingLineData_c'


def line_component(mapping: WireMap, ids: dict[str, int], name: str, version: int) -> bytes:
    """Reuse a complete typed component and preserve repeated identity; e.g. line_component(w, ids, 'moCompEdge_c', 19000)."""
    return shared_object(mapping, ids, name, name,
        lambda: strip_tag(component_wire(name, version, False, 0)))


def line_list(items: list[bytes]) -> bytes:
    """Retain explicit extended counts independently of object tags; e.g. line_list([bytes(2)])."""
    count = len(items)
    prefix = struct.pack('<HI', 65535, count) if count else bytes(2)
    return prefix + b''.join(items)


def split_info(mapping: WireMap, ids: dict[str, int], version: int) -> bytes:
    """Keep a complete split-info face and signed word; e.g. split_info(w, ids, 19000)."""
    face = line_component(mapping, ids, 'moCompFace_c', version)
    scalar = struct.pack('<i', -2147483648)
    return face + scalar


def edge_element(mapping: WireMap, ids: dict[str, int], version: int) -> bytes:
    """Pin generic CObject references rather than inventing component-only roles; e.g. edge_element(w, ids, 19000)."""
    edge = line_component(mapping, ids, 'moCompEdge_c', version)
    owner = struct.pack('<H', ids['feature'])
    atom = line_component(mapping, ids, 'moCompAtomBody_c', version)
    split = shared_object(mapping, ids, 'split', 'moMPLFaceSplitInfo_c',
        lambda: split_info(mapping, ids, version))
    return edge + owner + atom + bytes(2) + split + struct.pack('<ii', -20, 2147483647)


def settings_prefix(mapping: WireMap, ids: dict[str, int], version: int, rich: bool, angle: int) -> bytes:
    """Assemble the owner, first list, direction and raw Angle bits; e.g. settings_prefix(w, ids, 19000, True, 0)."""
    raw = struct.pack('<H', ids['feature'])
    items = [line_component(mapping, ids, 'moCompEdge_c', version), bytes(2),
        line_component(mapping, ids, 'moCompEdge_c', version)] if rich else []
    raw += line_list(items)
    raw += shared_object(mapping, ids, 'direction', 'moDirectionSpec_c',
        lambda: direction(mapping, rich)) if rich else bytes(2)
    raw += struct.pack('<Qi', angle, -17)
    raw += line_component(mapping, ids, 'moCompAtomBody_c', version) if rich else bytes(2)
    return raw


def line_settings(mapping: WireMap, ids: dict[str, int], version: int, rich: bool, angle: int, split_flag: int = -31) -> bytes:
    """Preserve the five direct lists and full signed flag words; e.g. line_settings(w, ids, 19000, True, 0)."""
    raw = settings_prefix(mapping, ids, version, rich, angle)
    raw += line_list([line_component(mapping, ids, 'moCompEdge_c', version), bytes(2)] if rich else [])
    raw += struct.pack('<ii', split_flag, -2147483648)
    element = shared_object(mapping, ids, 'element', 'moMPLEdgeElem',
        lambda: edge_element(mapping, ids, version)) if rich else b''
    raw += line_list([element] if rich else [])
    raw += line_list([struct.pack('<H', ids['element'])] * 2 if rich else [])
    raw += line_list([struct.pack('<H', ids['element'])] if rich else [])
    return raw + struct.pack('<iii', 2147483647, -37, 123)


def parting_line_graph(rich: bool, count: int = 0, angle: int = 0x8000000000000000,
                       version: int = 19000, first_slot: int = 0, serialized: bool = False, split_flag: int = -31) -> tuple[bytes, int, dict[str, int]]:
    """Reserve the parent before data/element back-references; e.g. parting_line_graph(True, 2)."""
    mapping = WireMap(1)
    mapping.slot = first_slot
    identities: dict[str, int] = {}
    def contents() -> bytes:
        inherited = body(mapping, version, rich, identities['feature'])
        settings = shared_object(mapping, identities, 'settings', SETTINGS,
            lambda: line_settings(mapping, identities, version, rich, angle, split_flag))
        frames = b''.join(frame(-7-i) if serialized else bytes([(195, 201)[i % 2]]) for i in range(max(count, 0)))
        return inherited + settings + struct.pack('<ii', -9, count) + frames + struct.pack('<ii', -11, -2147483648)
    raw = shared_object(mapping, identities, 'feature', FEATURE, contents)
    return raw, mapping.slot + 1, identities


LINE_CONTEXTS = (('empty', False, 0, False), ('suppressed', True, 2, False),
    ('serialized', True, 2, True), ('negative', True, -2147483648, False))
LINE_BASE_CASES = (('feature-0-0.bin', False, 0, 19000, False),
    ('feature-1-2.bin', True, 2, 19000, False),
    ('feature-1--2147483648.bin', True, -2147483648, 19000, False),
    ('feature-serialized.bin', True, 2, 19000, True),
    ('feature-minimum.bin', True, 2, 0x27cc, False))


def bare_line_cases() -> dict[str, tuple[bytes, int, dict[str, int]]]:
    """Reproduce the five independent bare graphs; e.g. bare_line_cases()['feature-minimum.bin']."""
    return {name: parting_line_graph(rich, count, version=version, serialized=serialized)
        for name, rich, count, version, serialized in LINE_BASE_CASES}


def owned_line_cases() -> dict[str, tuple[bytes, int, dict[str, int]]]:
    """Compose original and two expected edits for each complete context; e.g. owned_line_cases()['owned-serialized-0.bin']."""
    cases: dict[str, tuple[bytes, int, dict[str, int]]] = {}
    for (name, rich, count, serialized), edit in product(LINE_CONTEXTS, range(3)):
        raw, slots, ids = parting_line_graph(rich, count, first_slot=96, serialized=serialized,
            angle=0x7ff0000000000001 if edit == 1 else 0x8000000000000000,
            split_flag=12345 if edit == 2 else -31)
        cases[f'owned-{name}-{edit}.bin'] = struct.pack('<iH', -123, 1) + raw, slots, ids
    return cases


def verify_line_cases() -> None:
    """Compare every expected byte and hash without a Bessemer encoder; e.g. python fixture_bytes.py."""
    reference = json.loads(Path(__file__).with_name('fixture-reference.json').read_text(encoding='utf-8'))
    cases = bare_line_cases() | owned_line_cases()
    assert set(cases) == set(reference['files'])
    for name, (raw, _, _) in cases.items():
        assert raw == (ROOT / 'tests/fixtures/mold-parting-line' / name).read_bytes(), name
        assert hashlib.sha256(raw).hexdigest() == reference['files'][name], name


if __name__ == '__main__':
    verify_line_cases()
