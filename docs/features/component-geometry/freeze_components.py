"""Freeze component geometry against complete ordinary reference readers (#54)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
for folder in ('model-archives', 'reference-axes', 'resolved-features', 'circular-patterns', 'reference-axis-owners'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import ReferenceObjectCursor, object_cursor, observe_object_body
from primitive_inventory import observe_primitives
from freeze_owned_features import ReplacedFeatureDocument, STREAM
from owned_circular_fixtures import write_observation
from component_geometry_bytes import NAMES, component_object

VERSIONS = (0x27cc, 0x2b60, 0x2b61, 0x2bb8, 0x2bb9, 0x2f55, 0x2f56,
            0x2f85, 0x2f86, 0x3ef5, 0x3ef6, 19000)


def install_geometry(cursor: ReferenceObjectCursor) -> None:
    """Use the same local plane registration as resolved lists; e.g. install_geometry(cursor)."""
    import referenceplanes
    referenceplanes.install_scoped(cursor)


def verify_component(root: dict[str, object], name: str, kind: int, wrapped: bool, edited: bool = False) -> None:
    """Require the actual derived class and absence of a plane-only word; e.g. verify_component(root, NAMES[0], 2, True)."""
    component = cast(dict[str, object], root['component_reference']) if wrapped else root
    expected_id = 12345 if edited else -2147483648
    if component['_class'] != name or component['feature_id'] != expected_id or component['feature_time'] != 0xfedcba98:
        raise ValueError(f'component identity/feature words; expected {(name, expected_id, 0xfedcba98)}')
    if 'word_100' in component:
        raise ValueError('component axis/point suffix; expected no plane-only word_100')
    geometry = cast(dict[str, object] | None, component['reference_geometry'])
    expected = (None, 'moRefPlane_c', 'moRefAxis_c')[kind]
    if (None if geometry is None else geometry['_class']) != expected:
        raise ValueError(f'component geometry {geometry}; expected class {expected}')


def observe_case(reference: Path, name: str, version: int, rich: bool, kind: int, wrapped: bool) -> tuple[bytes, dict[str, object]]:
    """Verify complete bodies and exact map counts independently; e.g. observe_case(source, NAMES[0], 19000, True, 2, False)."""
    raw, slots = component_object(name, version, rich, kind, wrapped=wrapped)
    cursor = object_cursor(reference, raw, version, install_geometry)
    verify_component(cast(dict[str, object], cursor.object()), name, kind, wrapped)
    code = NAMES.index(name)+2*int(wrapped)
    return observe_object_body(reference, raw, slots, code, version,
        f'component-{version:04x}-{name}-{int(rich)}-{kind}-{int(wrapped)}', install_geometry)


def freeze_owned_case(reference: Path, destination: Path, name: str, rich: bool, kind: int,
                      wrapped: bool, edited: bool) -> dict[str, object]:
    """Use the real model/CMgr state and independent edit construction; e.g. freeze_owned_case(source, output, NAMES[0], True, 2, False, False)."""
    container, prefix, features, archive = (source_module(name, reference) for name in
        ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, imported = prefix.read(document, 'Contents/Config-0', through_part_resolved=True)
    context = cast(ReferenceObjectCursor, imported)
    feature, slots = component_object(name, 19000, rich, kind, len(context.map)-1, wrapped, edited)
    raw = struct.pack('<iH', -123, 1)+feature+bytes(4)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(ReplacedFeatureDocument(document, raw), STREAM)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'component end/tail/map; expected {(len(raw), b"", slots)}')
    verify_component(parsed['items'][0], name, kind, wrapped, edited)
    label = f'owned-{NAMES.index(name)}-{int(rich)}-{kind}-{int(wrapped)}'+('-edited' if edited else '')
    return write_observation(destination, label, raw, reads) | dict(slots=slots,
        imported_slots=len(context.map), baseline_sha256=hashlib.sha256(baseline).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Cover inherited gates and both classes in wrapper contexts; e.g. freeze(source, output)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    records = [observe_case(reference, name, version, rich, kind, False) for version in VERSIONS
        for name in NAMES for rich in (False, True) for kind in (0, 1, 2)]
    records += [observe_case(reference, name, 19000, rich, kind, True)
        for name in NAMES for rich in (False, True) for kind in (0, 2)]
    packet = b'CGF1'+struct.pack('<I', len(records))
    packet += b''.join(struct.pack('<I', row['slots']) for _, row in records)+b''.join(raw for raw, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    owned = [freeze_owned_case(reference, destination, *args) for args in
        [(NAMES[0], False, 0, False, False), (NAMES[1], False, 0, False, False),
         (NAMES[0], True, 2, False, False), (NAMES[1], True, 1, False, False),
         (NAMES[0], True, 2, True, False), (NAMES[1], True, 2, True, False),
         (NAMES[0], True, 2, True, True)]]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    scripts = sorted(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[row for _, row in records], owned=owned, source_hashes=before,
        packet_sha256=hashlib.sha256(packet).hexdigest(),
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in scripts},
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/component-geometry')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
