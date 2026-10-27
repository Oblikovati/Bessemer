"""Independent complete structural-member and nested contour/index observations (#54)."""
from dataclasses import replace
from functools import partial
from pathlib import Path
from typing import cast
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
for folder in ('model-archives', 'reference-axes', 'sweeps', 'sheet-metal', 'resolved-features', 'circular-patterns', 'plane-point-roles', 'weldments'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import object_cursor, ReferenceObjectCursor
from primitive_inventory import observe_primitives, verify_primitive_coverage
from sheet_reference_observer import inventory_bytes
from sheet_owned_fixtures import SavedSheetMetalDocument, saved_manager
from freeze_owned_features import STREAM
from owned_circular_fixtures import write_observation
from member_reference_bytes import MemberFixtureOptions
from member_fixture_bytes import CLASSES, structural_object

VERSIONS = (0x27cc, 19000, 0xffffffff)
MODES = (-2147483648, -1, 0, 1, 2, 2147483647)
RICH = MemberFixtureOptions(rich=True, mode=1)


def install_member_context(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install the same scoped plane dependencies as complete resolved readers; e.g. install_member_context(cursor, source)."""
    source_module('referenceplanes', reference).install_scoped(cursor)


def verify_aliases(items: list[object], rich: bool) -> None:
    """Keep each serialized packed/signed collection's shared/null/shared pattern; e.g. verify_aliases(items, True)."""
    if not rich and items != []:
        raise ValueError(f'member collection {items}; expected empty list')
    if rich and (len(items) != 3 or items[0] is not items[2] or items[1] is not None):
        raise ValueError(f'member collection {items}; expected shared/null/shared order')


def verify_member(root: dict[str, object], options: MemberFixtureOptions) -> None:
    """Use explicit mode cases and literal full-width point metadata; e.g. verify_member(root, options)."""
    payload = cast(dict[str, object], root['payload'])
    if payload['raw_9c8'] != options.mode:
        raise ValueError(f'member mode {payload["raw_9c8"]}; expected raw {options.mode}')
    expected_contour = options.rich and options.mode in (1, 2, 2147483647)
    if (payload['contours_9c0'] is not None) != expected_contour:
        raise ValueError(f'member contours {payload}; expected positive-mode-only pointer {expected_contour}')
    for label in ('curves_890', 'curves_8b0', 'array_9f0'):
        verify_aliases(cast(list[object], payload[label]), options.rich)
    points = cast(list[dict[str, object]], payload['points_830'])
    expected = [(12345 if options.edited else -2147483648, 2147483647), (-1, 65536), (-32769, 32768)] if options.rich else []
    if [(entry['raw_8f0'], entry['raw_8d0']) for entry in points] != expected:
        raise ValueError(f'member point metadata {points}; expected full signed words {expected}')
    if options.rich and (points[0]['point'] is not points[1]['point'] or points[2]['point'] is not None or points[0]['point'] is not payload['point_950']):
        raise ValueError('member point identities; expected shared/shared/null and repeated point_950')


def verify_object(root: dict[str, object], kind: int, options: MemberFixtureOptions) -> None:
    """Check complete class identity and each new container's direct order; e.g. verify_object(root, 0, options)."""
    if root['_class'] != CLASSES[kind]:
        raise ValueError(f'structural class {root["_class"]}; expected {CLASSES[kind]}')
    if kind == 0:
        verify_member(root, options)
    if kind == 1 and root['raw_a8'] != 0xfedcba98:
        raise ValueError(f'profile word {root}; expected preserved u32 0xfedcba98')
    if kind == 2:
        verify_aliases(cast(list[object], root['contours']), options.rich)
    if kind in (3, 4):
        verify_aliases(cast(list[object], root['items']), options.rich)
    if kind == 5 and tuple(root[key] for key in ('raw_08', 'raw_0c', 'raw_10')) != (-2147483648, 2147483647, -1):
        raise ValueError(f'index fields {root}; expected three complete signed words')
    if kind == 6 and root['point_10'] is not root['point_108']:
        raise ValueError('orientation point identity; expected repeated pointer')
    if kind in (7, 8):
        verify_aliases(cast(list[object], root['array_28' if kind == 7 else 'array_50']), options.rich)


def observe_structural(reference: Path, kind: int, version: int, options: MemberFixtureOptions) -> tuple[bytes, dict[str, object]]:
    """Require exact EOF, a complete primitive partition and map allocation; e.g. observe_structural(source, 0, 19000, options)."""
    raw, slots = structural_object(kind, version, options)
    cursor = object_cursor(reference, raw, version, partial(install_member_context, reference=reference))
    with observe_primitives(source_module('swarchive', reference).Archive, raw) as reads:
        verify_object(cast(dict[str, object], cursor.object()), kind, options)
    if (cursor.p, len(cursor.map)) != (len(raw), slots):
        raise ValueError(f'structural end/map {(cursor.p, len(cursor.map))}; expected {(len(raw), slots)}')
    verify_primitive_coverage(raw, reads);oracle = inventory_bytes(reads)
    return struct.pack('<BIII', kind, version, len(raw), len(oracle))+raw+oracle, dict(kind=kind, version=version,
        bytes=len(raw), slots=slots, reads=len(oracle.splitlines()), options=options.__dict__,
        sha256=hashlib.sha256(raw).hexdigest(), oracle_sha256=hashlib.sha256(oracle).hexdigest())


def records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Cover every class and independently vary mode, point and curve subtypes; e.g. records(source)."""
    cases = [observe_structural(reference, kind, version, options) for kind in range(9) for version in VERSIONS
             for options in (MemberFixtureOptions(), RICH)]
    cases += [observe_structural(reference, 0, 19000, replace(RICH, point_style=point, curve_style=curve, mode=mode))
              for point in range(3) for curve in range(2) for mode in MODES]
    return cases


def owned_structural(reference: Path, options: MemberFixtureOptions) -> tuple[SavedSheetMetalDocument, bytes, int]:
    """Resolve the real saved-state context before constructing the complete member; e.g. owned_structural(source, options)."""
    container, prefix = (source_module(name, reference) for name in ('swcontainer', 'modelprefix'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, _ = saved_manager(reference, original, 0)
    probe = SavedSheetMetalDocument(original, manager, b'')
    _, context = prefix.read(probe, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    feature, slots = structural_object(0, 19000, options, len(context.map)-1)
    raw = struct.pack('<iH', -123, 1)+feature
    return SavedSheetMetalDocument(original, manager, raw), raw, slots


def freeze_owned(reference: Path, destination: Path, options: MemberFixtureOptions, label: str) -> dict[str, object]:
    """Verify an independent metadata edit without native truncation or mode rewrites; e.g. freeze_owned(source, output, options, 'owned-rich')."""
    features, archive = (source_module(name, reference) for name in ('resolvedfeatures', 'swarchive'))
    document, raw, slots = owned_structural(reference, options)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(document, STREAM)
        verify_object(cast(dict[str, object], parsed['items'][0]), 0, options)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'owned structural end/tail/map; expected {(len(raw), b"", slots)}')
    return write_observation(destination, label, raw, [read for read in reads if read.end > read.start]) | dict(slots=slots,
        imported_slots=97, manager_sha256=hashlib.sha256(document.manager).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind every fixture to stable approved source and builder revisions; e.g. freeze(source, output)."""
    before = reference_snapshot(reference);sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    cases = records(reference)
    packet = b'STM1'+struct.pack('<I', len(cases))+b''.join(struct.pack('<I', row['slots']) for _, row in cases)+b''.join(raw for raw, _ in cases)
    (destination/'cases.bin').write_bytes(packet)
    rich = replace(RICH, point_style=2)
    options = [(MemberFixtureOptions(), 'owned-empty'), (replace(RICH, mode=0, point_style=1, curve_style=1), 'owned-zero'),
        (rich, 'owned-positive'), (replace(RICH, mode=2, curve_style=1), 'owned-two'),
        (replace(RICH, mode=-1), 'owned-negative'), (replace(rich, edited=True), 'owned-positive-edited')]
    owned = [freeze_owned(reference, destination, entry, label) for entry, label in options]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    scripts = sorted(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[row for _, row in cases], owned=owned, source_hashes=before, packet_sha256=hashlib.sha256(packet).hexdigest(),
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in scripts},
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    facts = freeze(REFERENCE, WORKSPACE/'tests/fixtures/structural-members')
    (Path(__file__).parent/'reference.json').write_text(json.dumps(facts, indent=2)+'\n', encoding='utf-8')
