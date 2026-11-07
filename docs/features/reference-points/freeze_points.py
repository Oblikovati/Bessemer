"""Freeze complete reference-point records against the independent scoped readers (#54)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
for folder in ('model-archives', 'reference-axes', 'reference-axis-owners', 'sweeps', 'resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import ReferenceObjectCursor, object_cursor, observe_object_body
from primitive_inventory import observe_primitives
from freeze_owned_features import ReplacedFeatureDocument, STREAM
from owned_circular_fixtures import write_observation
from point_fixture_bytes import KINDS, point_object

VERSIONS = (0x27cc, 0x2f85, 0x2f86, 19000)


def install_references(cursor: ReferenceObjectCursor) -> None:
    """Match the resolved reader's local geometric registrations; e.g. install_references(cursor)."""
    import referenceplanes, sketchprefix
    referenceplanes.install_scoped(cursor)
    sketchprefix.install_scoped_readers(cursor)


def verify_point(root: dict[str, object], kind: int, rich: bool, standalone: bool,
                 component: bool, edited: bool = False) -> None:
    """Check exact subtype, required data, shared curves and empty bases; e.g. verify_point(root, 5, True, False, False)."""
    point = cast(dict[str, object], root['reference_geometry']) if component else root
    payload = point if standalone else cast(dict[str, object], point['data_6f8'])
    if payload['_class'] != KINDS[kind]:
        raise ValueError(f'point data class {payload["_class"]}; expected {KINDS[kind]}')
    if kind == 0 and set(payload) != {'_class'}:
        raise ValueError(f'base point fields {list(payload)}; expected only the tagged class')
    if rich and kind == 5 and payload['curve_reference_10'] is not payload['curve_reference_18']:
        raise ValueError('two-edge point curves; expected shared identity')
    if kind == 6 and payload['raw_40'] != (12345 if edited else -2147483648):
        raise ValueError(f'along-curve final word {payload["raw_40"]}; expected selected signed value')
    if not standalone:
        verify_root(point, payload, kind, rich)


def verify_root(point: dict[str, object], payload: dict[str, object], kind: int, rich: bool) -> None:
    """Keep optional profile/surface tags separate and share existing faces; e.g. verify_root(point, payload, 1, True)."""
    if (point['profile_6f0'] is not None) != rich or (point['surface_reference_700'] is not None) != rich:
        raise ValueError('reference-point optional pointers; expected independent complete profile/surface tags')
    profile = cast(dict[str, object] | None, point['profile_6f0'])
    if profile is not None and profile['_class'] != 'moProfileFeature_c':
        raise ValueError(f'point profile class {profile["_class"]}; expected moProfileFeature_c')
    if rich and kind in (1, 4):
        face = payload['face_reference_10' if kind == 1 else 'face_reference_18']
        if face is not point['surface_reference_700']:
            raise ValueError('point-data face/surface identities; expected the same loaded face')


def observe_case(reference: Path, version: int, kind: int, rich: bool, standalone: bool,
                 style: int = 0, point_style: int = 0, component: bool = False) -> tuple[bytes, dict[str, object]]:
    """Require complete root and subtype serializers; e.g. observe_case(source, 19000, 6, True, False, 2)."""
    raw, slots = point_object(version, kind, rich, standalone, style, point_style, component=component)
    cursor = object_cursor(reference, raw, version, install_references)
    verify_point(cast(dict[str, object], cursor.object()), kind, rich, standalone, component)
    code = kind+(14 if component else 7 if standalone else 0)
    return observe_object_body(reference, raw, slots, code, version,
        f'point-{version:04x}-{kind}-{int(rich)}-{int(standalone)}-{style}-{point_style}-{int(component)}', install_references)


def freeze_records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Cover every data class, curve family and point-wrapper extension; e.g. freeze_records(source)."""
    records = [observe_case(reference, version, kind, rich, standalone)
        for version in VERSIONS for kind in range(7) for rich in (False, True) for standalone in (False, True)]
    records += [observe_case(reference, 19000, kind, True, True, style)
        for kind in (2, 5, 6) for style in range(4)]
    records += [observe_case(reference, 19000, 4, True, standalone, point_style=style)
        for standalone in (False, True) for style in (1, 2)]
    return records+[observe_case(reference, 19000, kind, True, False, 2, component=True) for kind in (0, 1, 4, 6)]


def freeze_owned_case(reference: Path, destination: Path, kind: int, rich: bool,
                      edited: bool, component: bool) -> dict[str, object]:
    """Require actual saved context and independent expected edits; e.g. freeze_owned_case(source, output, 6, True, False, False)."""
    container, prefix, features, archive = (source_module(name, reference) for name in
        ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, imported = prefix.read(document, 'Contents/Config-0', through_part_resolved=True)
    context = cast(ReferenceObjectCursor, imported)
    feature, slots = point_object(19000, kind, rich, style=2, first_slot=len(context.map)-1, edited=edited, component=component)
    raw = struct.pack('<iH', -123, 1)+feature+bytes(4)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(ReplacedFeatureDocument(document, raw), STREAM)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'point end/tail/map; expected {(len(raw), b"", slots)}')
    verify_point(parsed['items'][0], kind, rich, False, component, edited)
    label = f'owned-{kind}-{int(rich)}-{int(component)}'+('-edited' if edited else '')
    return write_observation(destination, label, raw, reads) | dict(slots=slots,
        imported_slots=len(context.map), baseline_sha256=hashlib.sha256(baseline).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Freeze byte-complete records while keeping source hashes stable; e.g. freeze(source, output)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    records = freeze_records(reference)
    packet = b'RPF1'+struct.pack('<I', len(records))
    packet += b''.join(struct.pack('<I', row['slots']) for _, row in records)+b''.join(raw for raw, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    owned = [freeze_owned_case(reference, destination, kind, rich, edited, component)
        for kind, rich, edited, component in [(0, False, False, False), (4, True, False, False),
            (5, True, False, False), (6, True, False, False), (6, True, True, False), (6, True, False, True)]]
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
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/reference-points')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
