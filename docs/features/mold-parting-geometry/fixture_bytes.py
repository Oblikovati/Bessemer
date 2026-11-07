"""Literal complete parting-geometry graphs from the retained native field order (#217)."""
from pathlib import Path
import hashlib
import json
import struct
import sys

ROOT = Path(__file__).resolve().parents[3]
for folder in ('model-archives', 'reference-axes', 'reference-axis-owners', 'sweeps',
               'circular-patterns', 'resolved-features', 'revolved-surfaces', 'thickening', 'weld-corners'):
    sys.path.append(str(ROOT / 'docs/features' / folder))
from fixture_object_map import WireMap, object_tag, shared_object
from axis_owner_bytes import geometry_header
from revolved_bytes import surface_wire
from kernel_geometry_fixture import frame
from body_feature_fixture_bytes import direction
from freeze_component_references import base as component_base, wire as component_wire
from display_dimension_fixture_bytes import body as strip_tag
from freeze_model_references import wrapper
from sweep_fixture_bytes import dimension, synchronization

CONTEXTS = (('empty', 2, 2, False, 0), ('rich', 2, 2, True, 3),
    ('allocated-skipped', 1, 2, True, 3), ('unallocated-selected', 2, 1, True, 3),
    ('child', 1, 1, True, 3), ('kernel-child', 1, 1, True, 5),
    ('empty-child', 1, 1, False, 0))


def geometry_list(items: list[bytes]) -> bytes:
    """Keep extended counts independent of pointer tags; e.g. geometry_list([bytes(2)])."""
    prefix = struct.pack('<HI', 65535, len(items)) if items else bytes(2)
    return prefix + b''.join(items)


def geometry_component(mapping: WireMap, ids: dict[str, int], version: int, name: str) -> bytes:
    """Reuse a complete component and its repeated identity; e.g. geometry_component(w, ids, 19000, 'moCompEdge_c')."""
    contents = lambda: (component_base(version, False) + struct.pack('<iI', -2147483648, 0x89abcdef)
        if name == 'moCompFeature_c' else strip_tag(component_wire(name, version, False, 0)))
    return shared_object(mapping, ids, name, name,
        contents)


def geometry_direction(mapping: WireMap, ids: dict[str, int]) -> bytes:
    """Retain shared complete direction bodies; e.g. geometry_direction(w, ids)."""
    return shared_object(mapping, ids, 'direction', 'moDirectionSpec_c',
        lambda: direction(mapping, True))


def geometry_edges(mapping: WireMap, ids: dict[str, int], version: int, rich: bool) -> bytes:
    """Retain duplicate and null generic collection elements; e.g. geometry_edges(w, ids, 19000, True)."""
    edge = geometry_component(mapping, ids, version, 'moCompEdge_c') if rich else b''
    return geometry_list([edge, bytes(2), struct.pack('<H', ids['moCompEdge_c'])] if rich else [])


def geometry_connected(mapping: WireMap, ids: dict[str, int], version: int) -> bytes:
    """Use direct object/DWORD arrays without runtime padding; e.g. geometry_connected(w, ids, 19000)."""
    raw = geometry_edges(mapping, ids, version, True)
    raw += geometry_list([struct.pack('<H', ids['feature']), bytes(2)])
    raw += struct.pack('<HI3Ii', 65535, 3, 0xffffffff, 0, 0xffffffff, -2147483648)
    return raw + synchronization(mapping, version, ids, False)


def geometry_child(mapping: WireMap, ids: dict[str, int], version: int, rich: bool) -> bytes:
    """Pin empty/populated child fields and generic aliases; e.g. geometry_child(w, ids, 19000, True)."""
    if not rich:
        return bytes(14)
    raw = geometry_edges(mapping, ids, version, True)
    raw += geometry_list([struct.pack('<H', ids['feature']), bytes(2)])
    raw += geometry_list([geometry_direction(mapping, ids)] * 2)
    raw += shared_object(mapping, ids, 'wrapper', 'moRefWrapper_c', lambda: wrapper(version, False))
    raw += synchronization(mapping, version, ids, False)
    raw += shared_object(mapping, ids, 'connected', 'moMoldPartingConnectedComponent_c',
        lambda: geometry_connected(mapping, ids, version))
    return raw + geometry_direction(mapping, ids)


def geometry_scalars(length: int) -> bytes:
    """Preserve exact IEEE bits and independent signed words; e.g. geometry_scalars(0x8000000000000000)."""
    raw = struct.pack('<4QiQiQ', length, 0x7ff8000000001234, 1, 0xfff0000000000000,
        -2147483648, 0x7ff0000000000001, 2147483647, 0x8000000000000000)
    return raw


def geometry_optional(mapping: WireMap, ids: dict[str, int], version: int, rich: bool) -> bytes:
    """Separate signed presence words from tagged pointers and signed vector counts; e.g. geometry_optional(w, ids, 19000, True)."""
    if not rich:
        return struct.pack('<3i', 0, 0, -2147483648)
    component = geometry_component(mapping, ids, version, 'moCompFeature_c')
    return struct.pack('<i', -1) + component + struct.pack('<iH', 2147483647,
        ids['moCompFeature_c']) + struct.pack('<4i', 3, -2147483648, 0, 2147483647)


def geometry_settings(mapping: WireMap, ids: dict[str, int], version: int, method: int,
                      selector: int, rich: bool, edit: int) -> bytes:
    """Keep the direct body and constructor/selector branch product; e.g. geometry_settings(w, ids, 19000, 1, 1, True, 0)."""
    raw = struct.pack('<H', ids['feature']) + geometry_edges(mapping, ids, version, rich)
    raw += dimension(mapping, version, ids, 'moDisplayDistanceDim_c') if rich else bytes(2)
    raw += geometry_scalars(0x7ff0000000000001 if edit == 1 else 0x8000000000000000)
    raw += geometry_optional(mapping, ids, version, rich)
    raw += struct.pack('<10Q3i', *([0x8000000000000000, 0x7ff8000000001234] * 5), -7, 1, -9)
    raw += geometry_direction(mapping, ids) if rich else bytes(2)
    raw += struct.pack('<3i', 12345 if edit == 2 else -31, 2046, selector)
    return raw + (geometry_child(mapping, ids, version, rich) if method == 1 and selector == 1 else b'')


def geometry_surface(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """Require the strict kernel body within a populated sheet; e.g. geometry_surface(w, ids, 19000, 5)."""
    if style != 5:
        return surface_wire(mapping, ids, version, style)
    raw = struct.pack('<2i', 1, 1) + frame(-7) + geometry_list([bytes(2)])
    raw += struct.pack('<iB', -1, 201)
    return raw + (struct.pack('<4i', 3, -2147483648, 0, 2147483647) if version >= 0x2bcd else b'')


def geometry_graph(context: tuple[str, int, int, bool, int], version: int = 19000,
                   first_slot: int = 0, edit: int = 0) -> tuple[bytes, int, dict[str, int]]:
    """Reserve the surface before its owner cycles; e.g. geometry_graph(CONTEXTS[4])."""
    _, method, selector, rich, style = context
    mapping = WireMap(1)
    mapping.slot = first_slot
    ids: dict[str, int] = {}
    def contents() -> bytes:
        raw = geometry_header(mapping, version, rich)
        raw += geometry_surface(mapping, ids, version, style)
        raw += geometry_edges(mapping, ids, version, rich) + struct.pack('<i', method)
        return raw + geometry_settings(mapping, ids, version, method, selector, rich, edit)
    raw = shared_object(mapping, ids, 'feature', 'moMoldPartingGeom_c', contents)
    return raw, mapping.slot + 1, ids


def geometry_cases() -> dict[str, tuple[bytes, int, dict[str, int]]]:
    """Generate bare/version-boundary and whole-carrier edit cases; e.g. geometry_cases()['child.bin']."""
    cases = {f'{context[0]}.bin': geometry_graph(context) for context in CONTEXTS}
    cases['minimum-child.bin'] = geometry_graph(CONTEXTS[4], 0x27cc)
    cases['component-child.bin'] = geometry_component_graph()
    for context in CONTEXTS:
        for edit in range(3):
            raw, slots, ids = geometry_graph(context, first_slot=96, edit=edit)
            cases[f'owned-{context[0]}-{edit}.bin'] = struct.pack('<iH', -123, 1) + raw, slots, ids
    return cases


def geometry_component_graph() -> tuple[bytes, int, dict[str, int]]:
    """Place complete geometry under the existing component-surface caller; e.g. geometry_component_graph()."""
    raw, slots, ids = geometry_graph(CONTEXTS[4], first_slot=2)
    prefix = object_tag('moCompRefSurface_c', 1) + component_base(19000, False)
    prefix += struct.pack('<iI', -2147483648, 0x89abcdef)
    return prefix + raw + struct.pack('<i', -37), slots, ids


def verify_geometry_cases() -> None:
    """Check every frozen byte without the Bessemer encoder; e.g. python fixture_bytes.py."""
    reference = json.loads(Path(__file__).with_name('fixture-reference.json').read_text())
    cases = geometry_cases()
    assert set(cases) == set(reference['files'])
    for name, (raw, _, _) in cases.items():
        assert raw == (ROOT / 'tests/fixtures/mold-parting-geometry' / name).read_bytes(), name
        assert hashlib.sha256(raw).hexdigest() == reference['files'][name], name


if __name__ == '__main__':
    verify_geometry_cases()
