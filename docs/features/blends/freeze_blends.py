"""Independent complete blend/cut numeric-column and conditional-reference fixtures (#54)."""
from dataclasses import replace
from pathlib import Path
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
from object_fixture_observer import object_cursor
from primitive_inventory import observe_primitives, verify_primitive_coverage
from sheet_reference_observer import inventory_bytes
from sheet_owned_fixtures import SavedSheetMetalDocument, saved_manager
from freeze_owned_features import STREAM
from owned_circular_fixtures import write_observation
from blend_fixture_bytes import CLASSES, BlendFixtureOptions, blend_object

VERSIONS = (0x27cc, 19000, 0xffffffff)
DIRECTIONS = ((-2147483648, 2147483647), (-254, 0), (0, -254), (2, 2),
              (0x102, -254), (3, 3), (0x202, 0x102), (2147483647, -2147483648))
RICH = BlendFixtureOptions(True, 3, 3, 0x102, -254, 255, 3)


def verify_blend(root: dict[str, object], kind: int, options: BlendFixtureOptions) -> None:
    """Keep literal columns, declared count and conditional identities independent; e.g. verify_blend(root, 0, options)."""
    if root['_class'] != CLASSES[kind]:
        raise ValueError(f'blend class {root["_class"]}; expected {CLASSES[kind]}')
    payload = cast(dict[str, object], root['payload']);data = cast(dict[str, object], payload['data'])
    if data['declared_count_418'] != options.declared or len(cast(list[object], data['array_330'])) != options.count:
        raise ValueError(f'blend counts {data}; expected independent {options.declared}/{options.count}')
    verify_columns(data, options)
    expected = (options.first in (-254, 2, 0x102, 0x202), options.second in (-254, 2, 0x102, 0x202))
    if tuple(key in data for key in ('direction_68', 'direction_70')) != expected:
        raise ValueError(f'blend directions {data}; expected explicit control table {expected}')
    dimensions = (options.dimensions != 0, options.dimensions != 0 and options.selector == 3)
    if tuple(key in payload for key in ('dimension_c70', 'dimension_c78')) != dimensions:
        raise ValueError(f'blend dimensions {payload}; expected full-word equality {dimensions}')
    if all(expected) and data['direction_68'] is not data['direction_70']:
        raise ValueError('blend direction identity; expected one shared direction object')
    if all(dimensions) and payload['dimension_c70'] is not payload['dimension_c78']:
        raise ValueError('blend dimension identity; expected one shared dimension object')


def verify_columns(data: dict[str, object], options: BlendFixtureOptions) -> None:
    """Check column ordering and signed/unsigned widths against literal values; e.g. verify_columns(data, options)."""
    first = [0xffffffff, 0x80000000, 0x12345678][:options.count]
    second = [0x89abcdef, 0, 0xfedcba98][:options.count]
    signed = [-2147483648, 12345 if options.edited else 2147483647, -1][:options.count]
    if (data['raw_words_2f0'], data['raw_words_2f8'], data['raw_words_2e8']) != (first, second, signed):
        raise ValueError(f'blend numeric columns {data}; expected distinct unsigned columns and signed {signed}')
    points = cast(list[tuple[float, float, float]], data['raw_points_2d8'])
    if len(points) != options.count:
        raise ValueError(f'blend points {points}; expected {options.count} complete triples')
    if points and (struct.pack('<d', points[0][0]).hex() != '0000000000000080' or struct.pack('<d', points[0][1]).hex() != '341200000000f87f'):
        raise ValueError(f'blend first point {points[0]}; expected preserved negative zero and NaN payload bits')


def observe_blend(reference: Path, kind: int, version: int, options: BlendFixtureOptions) -> tuple[bytes, dict[str, object]]:
    """Require exact EOF and primitive coverage for each complete class; e.g. observe_blend(source, 1, 19000, options)."""
    raw, slots = blend_object(kind, version, options)
    cursor = object_cursor(reference, raw, version, None)
    with observe_primitives(source_module('swarchive', reference).Archive, raw) as reads:
        verify_blend(cast(dict[str, object], cursor.object()), kind, options)
    if (cursor.p, len(cursor.map)) != (len(raw), slots):
        raise ValueError(f'blend end/map {(cursor.p, len(cursor.map))}; expected {(len(raw), slots)}')
    verify_primitive_coverage(raw, reads);oracle = inventory_bytes(reads)
    return struct.pack('<BIII', kind, version, len(raw), len(oracle))+raw+oracle, dict(kind=kind, version=version,
        bytes=len(raw), slots=slots, reads=len(oracle.splitlines()), sha256=hashlib.sha256(raw).hexdigest(),
        oracle_sha256=hashlib.sha256(oracle).hexdigest(), options=options.__dict__)


def records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Cover both classes, controls and load-only count mismatches; e.g. records(source)."""
    cases = [observe_blend(reference, kind, version, options) for kind in range(2) for version in VERSIONS
             for options in (BlendFixtureOptions(), RICH)]
    controls = [replace(RICH, first=first, second=second) for first, second in DIRECTIONS]
    controls += [replace(RICH, dimensions=flag, selector=selector) for flag in (0, 1, 255) for selector in (0, 3, 0x103, -2147483648, 2147483647)]
    controls += [replace(RICH, declared=value) for value in (0, 1, 2147483647)]
    controls += [replace(RICH, count=0, declared=2147483647), replace(RICH, count=2, declared=1)]
    return cases+[observe_blend(reference, kind, 19000, options) for kind in range(2) for options in controls]


def owned_blend(reference: Path, kind: int, options: BlendFixtureOptions) -> tuple[SavedSheetMetalDocument, bytes, int]:
    """Import the actual model object map before the complete blend; e.g. owned_blend(source, 0, options)."""
    container, prefix = (source_module(name, reference) for name in ('swcontainer', 'modelprefix'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, _ = saved_manager(reference, original, 0)
    probe = SavedSheetMetalDocument(original, manager, b'')
    _, context = prefix.read(probe, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    feature, slots = blend_object(kind, 19000, options, len(context.map)-1)
    raw = struct.pack('<iH', -123, 1)+feature
    return SavedSheetMetalDocument(original, manager, raw), raw, slots


def freeze_owned(reference: Path, destination: Path, kind: int, options: BlendFixtureOptions, label: str) -> dict[str, object]:
    """Observe whole documents and an independent signed-column edit; e.g. freeze_owned(source, output, 0, options, 'owned-rich')."""
    features, archive = (source_module(name, reference) for name in ('resolvedfeatures', 'swarchive'))
    document, raw, slots = owned_blend(reference, kind, options)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(document, STREAM)
        verify_blend(cast(dict[str, object], parsed['items'][0]), kind, options)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'owned blend end/tail/map; expected {(len(raw), b"", slots)}')
    return write_observation(destination, label, raw, [read for read in reads if read.end > read.start]) | dict(slots=slots,
        imported_slots=97, manager_sha256=hashlib.sha256(document.manager).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind each exact observation to stable sibling reader revisions; e.g. freeze(source, output)."""
    before = reference_snapshot(reference);sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    cases = records(reference)
    packet = b'BLN1'+struct.pack('<I', len(cases))+b''.join(struct.pack('<I', row['slots']) for _, row in cases)+b''.join(raw for raw, _ in cases)
    (destination/'cases.bin').write_bytes(packet)
    options = [(0, BlendFixtureOptions(), 'owned-empty'), (0, RICH, 'owned-blend'), (1, RICH, 'owned-cut'),
               (0, replace(RICH, edited=True), 'owned-blend-edited')]
    owned = [freeze_owned(reference, destination, kind, entry, label) for kind, entry, label in options]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    scripts = sorted(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[row for _, row in cases], owned=owned, source_hashes=before, packet_sha256=hashlib.sha256(packet).hexdigest(),
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in scripts},
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    facts = freeze(REFERENCE, WORKSPACE/'tests/fixtures/blends')
    (Path(__file__).parent/'reference.json').write_text(json.dumps(facts, indent=2)+'\n', encoding='utf-8')
