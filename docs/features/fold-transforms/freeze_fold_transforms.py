"""Independent complete fold-folder, transform and property records (#54)."""
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
from sheet_reference_observer import inventory_bytes
from sheet_owned_fixtures import SavedSheetMetalDocument, saved_manager
from freeze_owned_features import STREAM
from primitive_inventory import observe_primitives
from owned_circular_fixtures import write_observation
from fold_fixture_bytes import CLASSES, FoldFixtureOptions, fold_object
from fold_reference_observer import project_fold_bytes

VERSIONS = (0x27cc, 19000, 0xffffffff)
FLAGS = (0, 0x1f, 0x2f, 0x4f, 0xff)


def verify_fold(root: dict[str, object], kind: int, options: FoldFixtureOptions) -> list[bytes]:
    """Check semantic order and aliases before projecting any raw span; e.g. verify_fold(root, 0, options)."""
    if root['_class'] != CLASSES[kind]:
        raise ValueError(f'fold class {root["_class"]}; expected {CLASSES[kind]}')
    if kind == 3:
        verify_property(root, options)
        return []
    if kind == 0:
        verify_transform_list(cast(dict[str, object], root['feature_transform_570']), 'a0', options)
    if kind == 2:
        verify_transform_list(root, 'a0', options)
        return []
    sketch = cast(dict[str, object], root['sketch_transform_568']) if kind == 0 else root
    verify_transform_list(sketch, 'e8', options)
    verify_properties(sketch, options)
    expected = b'\x00\xff\x80\x7f' if options.rich else b''
    if sketch['bytes_108'] != expected:
        raise ValueError(f'fold bytes {sketch["bytes_108"]}; expected {expected}')
    return [expected] if expected else []


def verify_transform_list(root: dict[str, object], label: str, options: FoldFixtureOptions) -> None:
    """Pin literal decoded coordinates and independent extension presence; e.g. verify_transform_list(root, 'e8', options)."""
    transforms = cast(list[dict[str, object]], root[f'transforms_{label}'])
    if len(transforms) != (2 if options.rich else 0):
        raise ValueError(f'fold transforms {transforms}; expected two populated or zero empty entries')
    if not options.rich:
        return
    first = cast(dict[str, object], transforms[0]['raw_transform'])
    translation = (123.5 if options.transformed and label == 'e8' else -0.0, 1.25, -2.5)
    if first['translation'] != translation or first['scaling'] != 3.75 or first['flags'] != options.flags:
        raise ValueError(f'fold transform {first}; expected literal translation/scale and raw flag {options.flags}')
    expected = {0: (False, False), 0x1f: (True, False), 0x2f: (True, False), 0x4f: (False, True), 0xff: (False, False)}[options.flags]
    if ('matrix' in first, 'perspective_info' in first, 'matrix5' in first) != (options.matrix, *expected):
        raise ValueError(f'fold transform branches {first}; expected matrix {options.matrix} and extensions {expected}')


def verify_properties(sketch: dict[str, object], options: FoldFixtureOptions) -> None:
    """Keep ordered repeated keys, null pairs and shared properties; e.g. verify_properties(sketch, options)."""
    pairs = cast(list[dict[str, object]], sketch['properties_158'])
    if len(pairs) != (3 if options.rich else 0):
        raise ValueError(f'fold property pairs {pairs}; expected three populated or zero empty entries')
    if not options.rich:
        return
    if pairs[0]['feature'] is not pairs[1]['feature'] or pairs[0]['property'] is not pairs[1]['property']:
        raise ValueError('fold property identities; expected repeated feature and property objects')
    if pairs[2] != dict(feature=None, property=None):
        raise ValueError(f'fold final pair {pairs[2]}; expected two null pointers')
    verify_property(cast(dict[str, object], pairs[0]['property']), options)


def verify_property(root: dict[str, object], options: FoldFixtureOptions) -> None:
    """Keep four native widths and both CStrings; e.g. verify_property(root, options)."""
    expected = (-2147483648, 2147483647, -1, 0xfedcba98) if options.rich else (0, 0, 0, 0)
    actual = tuple(root[key] for key in ('raw_08', 'raw_0c', 'raw_10', 'raw_14'))
    strings = ('Fold Ω' if options.rich else '', 'Bessemer Ω' if options.edited else 'Property Δ' if options.rich else '')
    if actual != expected or (root['string_18'], root['string_20']) != strings:
        raise ValueError(f'fold property {(actual, root)}; expected {expected} and strings {strings}')


def observe_fold(reference: Path, kind: int, version: int, options: FoldFixtureOptions) -> tuple[bytes, dict[str, object]]:
    """Require complete byte coverage, EOF and exact map allocation; e.g. observe_fold(source, 0, 19000, options)."""
    raw, slots = fold_object(kind, version, options)
    cursor = object_cursor(reference, raw, version, None)
    with observe_primitives(source_module('swarchive', reference).Archive, raw) as reads:
        expected = verify_fold(cast(dict[str, object], cursor.object()), kind, options)
    if (cursor.p, len(cursor.map)) != (len(raw), slots):
        raise ValueError(f'fold end/map {(cursor.p, len(cursor.map))}; expected {(len(raw), slots)}')
    projected = project_fold_bytes(raw, reads, expected);oracle = inventory_bytes(projected)
    return struct.pack('<BIII', kind, version, len(raw), len(oracle))+raw+oracle, dict(kind=kind, version=version,
        bytes=len(raw), slots=slots, reads=len(projected), source_reads=sum(read.end > read.start for read in reads),
        sha256=hashlib.sha256(raw).hexdigest(), oracle_sha256=hashlib.sha256(oracle).hexdigest(), options=options.__dict__)


def records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Exercise every flag-selected extension in both matrix forms; e.g. records(source)."""
    options = [FoldFixtureOptions()]+[FoldFixtureOptions(True, matrix, flags) for matrix in (False, True) for flags in FLAGS]
    cases = [observe_fold(reference, kind, version, entry) for kind in range(3) for version in VERSIONS for entry in options]
    return cases+[observe_fold(reference, 3, version, FoldFixtureOptions(rich=rich)) for version in VERSIONS for rich in (False, True)]


def owned_fold(reference: Path, options: FoldFixtureOptions) -> tuple[SavedSheetMetalDocument, bytes, int]:
    """Import the real model map before constructing the complete folder; e.g. owned_fold(source, options)."""
    container, prefix = (source_module(name, reference) for name in ('swcontainer', 'modelprefix'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, _ = saved_manager(reference, original, 0)
    probe = SavedSheetMetalDocument(original, manager, b'')
    _, context = prefix.read(probe, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    feature, slots = fold_object(0, 19000, options, len(context.map)-1)
    raw = struct.pack('<iH', -123, 1)+feature
    return SavedSheetMetalDocument(original, manager, raw), raw, slots


def freeze_owned(reference: Path, destination: Path, options: FoldFixtureOptions, label: str) -> dict[str, object]:
    """Bind whole documents and a shared-property edit; e.g. freeze_owned(source, output, options, 'owned-rich')."""
    features, archive = (source_module(name, reference) for name in ('resolvedfeatures', 'swarchive'))
    document, raw, slots = owned_fold(reference, options)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(document, STREAM)
        expected = verify_fold(cast(dict[str, object], parsed['items'][0]), 0, options)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'owned fold end/tail/map; expected {(len(raw), b"", slots)}')
    return write_observation(destination, label, raw, project_fold_bytes(raw, reads, expected)) | dict(slots=slots, imported_slots=97,
        manager_sha256=hashlib.sha256(document.manager).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind all independently observed fields to stable sibling revisions; e.g. freeze(source, output)."""
    before = reference_snapshot(reference);sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    cases = records(reference)
    packet = b'FLD1'+struct.pack('<I', len(cases))+b''.join(struct.pack('<I', row['slots']) for _, row in cases)+b''.join(raw for raw, _ in cases)
    (destination/'cases.bin').write_bytes(packet)
    rich = FoldFixtureOptions(rich=True, matrix=True, flags=0x2f)
    owned = [freeze_owned(reference, destination, options, label) for options, label in
             ((FoldFixtureOptions(), 'owned-empty'), (rich, 'owned-rich'), (replace(rich, edited=True), 'owned-rich-edited'),
              (replace(rich, transformed=True), 'owned-transform-edited'))]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    scripts = sorted(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[row for _, row in cases], owned=owned, source_hashes=before, packet_sha256=hashlib.sha256(packet).hexdigest(),
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in scripts},
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    facts = freeze(REFERENCE, WORKSPACE/'tests/fixtures/fold-transforms')
    (Path(__file__).parent/'reference.json').write_text(json.dumps(facts, indent=2)+'\n', encoding='utf-8')
