"""Independent point-family role regression fixtures through complete plane data (#46)."""
from pathlib import Path
from functools import partial
from typing import cast
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
for folder in ('model-archives', 'reference-axes', 'sweeps', 'sheet-metal', 'resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import ReferenceObjectCursor, object_cursor
from primitive_inventory import observe_primitives, verify_primitive_coverage
from plane_point_fixture_bytes import PLANES, POINTS, role_object, plane_feature
from sheet_owned_fixtures import SavedSheetMetalDocument, saved_manager
from freeze_owned_features import STREAM
from owned_circular_fixtures import write_observation

VERSIONS = (0x27cc, 0x2b83, 0x2b84, 0x2ba5, 0x2ba6, 19000)


def install_planes(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install the same copied plane readers used by resolved streams; e.g. install_planes(cursor, source)."""
    source_module('referenceplanes', reference).install_scoped(cursor)


def verify_role(root: dict[str, object], plane: int, point: int, cached: bool) -> None:
    """Check actual class and shared identity independently of assignability; e.g. verify_role(root, 0, 1, True)."""
    items = cast(list[dict[str, object] | None], root['items'])
    selected = cast(dict[str, object], items[1])
    target = cast(dict[str, object], selected['point_reference_88'])
    if selected['_class'] != PLANES[plane] or target['_class'] != POINTS[point]:
        raise ValueError(f'plane/point classes {selected}; expected {PLANES[plane]}/{POINTS[point]}')
    if cached and target is not items[0]:
        raise ValueError('cached plane point; expected the exact earlier complete object')
    if not cached and items[0] is not None:
        raise ValueError(f'fresh plane point prefix {items[0]}; expected a null first array item')


def observe_role(reference: Path, plane: int, point: int, version: int, cached: bool) -> tuple[bytes, dict[str, object]]:
    """Keep byte validity separate from the finite role membership rule; e.g. observe_role(source, 0, 3, 19000, True)."""
    raw, slots = role_object(plane, point, version, cached)
    cursor = object_cursor(reference, raw, version, partial(install_planes, reference=reference))
    with observe_primitives(source_module('swarchive', reference).Archive, raw) as reads:
        verify_role(cast(dict[str, object], cursor.object()), plane, point, cached)
    if (cursor.p, len(cursor.map)) != (len(raw), slots):
        raise ValueError(f'plane role end/map {(cursor.p, len(cursor.map))}; expected {(len(raw), slots)}')
    verify_primitive_coverage(raw, reads)
    oracle = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in reads if read.end > read.start).encode()
    kind = plane+4*point+16*int(cached)
    return struct.pack('<BIII', kind, version, len(raw), len(oracle))+raw+oracle, dict(kind=kind, version=version,
        bytes=len(raw), slots=slots, reads=len(oracle.splitlines()), assignable=point < 3,
        sha256=hashlib.sha256(raw).hexdigest(), oracle_sha256=hashlib.sha256(oracle).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind every new/cached positive and unrelated-target fixture to stable readers; e.g. freeze(source, output)."""
    before = reference_snapshot(reference);sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    cases = [observe_role(reference, plane, point, version, cached) for plane in range(3)
             for point in range(4) for version in VERSIONS for cached in (False, True)]
    packet = b'PPR1'+struct.pack('<I', len(cases))+b''.join(struct.pack('<I', row['slots']) for _, row in cases)+b''.join(raw for raw, _ in cases)
    (destination/'cases.bin').write_bytes(packet)
    owned = [freeze_owned(reference, destination, plane, point, False) for plane in range(3) for point in range(3)]
    owned += [freeze_owned(reference, destination, 0, 1, True), freeze_owned(reference, destination, 0, 3, False)]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    scripts = sorted(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[row for _, row in cases], owned=owned, source_hashes=before, packet_sha256=hashlib.sha256(packet).hexdigest(),
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in scripts},
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


def owned_plane(reference: Path, plane: int, point: int, edited: bool) -> tuple[SavedSheetMetalDocument, bytes, int]:
    """Resolve persisted context and actual inherited map slots first; e.g. owned_plane(source, 0, 1, False)."""
    container, prefix = (source_module(name, reference) for name in ('swcontainer', 'modelprefix'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, _ = saved_manager(reference, original, 0)
    probe = SavedSheetMetalDocument(original, manager, b'')
    _, context = prefix.read(probe, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    feature, slots = plane_feature(plane, point, len(context.map)-1, edited)
    raw = struct.pack('<iH', -123, 1)+feature
    return SavedSheetMetalDocument(original, manager, raw), raw, slots


def freeze_owned(reference: Path, destination: Path, plane: int, point: int, edited: bool) -> dict[str, object]:
    """Observe complete documents without conflating byte framing with role validity; e.g. freeze_owned(source, output, 0, 3, False)."""
    features, archive = (source_module(name, reference) for name in ('resolvedfeatures', 'swarchive'))
    document, raw, slots = owned_plane(reference, plane, point, edited)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(document, STREAM)
        data = parsed['items'][0]['plane_data']
        if data['_class'] != PLANES[plane] or data['point_reference_88']['_class'] != POINTS[point]:
            raise ValueError(f'owned plane point classes {data}; expected explicit constructors')
        expected = (123.5 if edited else 1.0, 2.0, -0.0 if point == 0 else 3.0)
        if point < 3 and data['point_reference_88']['point_a8'] != expected:
            raise ValueError(f'owned plane point {data}; expected independent coordinates {expected}')
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'owned plane end/tail/map; expected {(len(raw), b"", slots)}')
    label = f'owned-{plane}-{point}'+('-edited' if edited else '')
    return write_observation(destination, label, raw, [read for read in reads if read.end > read.start]) | dict(slots=slots,
        imported_slots=97, assignable=point < 3, manager_sha256=hashlib.sha256(document.manager).hexdigest())


if __name__ == '__main__':
    facts = freeze(REFERENCE, WORKSPACE/'tests/fixtures/plane-point-roles')
    (Path(__file__).parent/'reference.json').write_text(json.dumps(facts, indent=2)+'\n', encoding='utf-8')
