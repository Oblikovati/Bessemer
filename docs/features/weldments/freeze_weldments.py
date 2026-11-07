"""Independent weldment/property bodies with complete EOF and identity proofs (#54)."""
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
from weldment_fixture_bytes import CLASSES, EXPRESSION, weldment_object

VERSIONS = (0x27cc, 19000, 0xffffffff)


def verify_properties(root: dict[str, object], style: int, edited: bool) -> None:
    """Check direct record counts and preserve expression text before native rewriting; e.g. verify_properties(root, 2, False)."""
    properties = cast(list[dict[str, object]], root['properties'])
    expected = (0, 2, 3, 1)[style]
    if len(properties) != expected:
        raise ValueError(f'custom property count {len(properties)}; expected {expected}')
    if properties:
        verify_record(properties[0], style, edited)
    if style not in (1, 2):
        return
    value = 'Bessemer Ω' if edited else 'original' if style == 1 else 'Density@\n材料'
    if properties[0]['string_20'] != value:
        raise ValueError(f'custom property value {properties[0]}; expected original stored {value!r}')
    if style == 2 and (properties[0]['string_18'] != EXPRESSION or properties[1] is properties[2]):
        raise ValueError('custom property records; expected original newline/at-sign expressions and distinct direct identities')
    if style == 2 and properties[1] != properties[2]:
        raise ValueError('custom duplicate properties; expected equal values in independent direct records')


def verify_record(root: dict[str, object], style: int, edited: bool) -> None:
    """Pin native word widths and original stored expression text; e.g. verify_record(root, 2, False)."""
    actual = tuple(root[key] for key in ('raw_2c', 'raw_28', 'raw_34', 'raw_38', 'raw_3c'))
    expected = (-2147483648, 0xffffffff, -1, 0, 2147483647) if style in (1, 2) else (0, 0, 0, 0, 0)
    if actual != expected:
        raise ValueError(f'custom property words {actual}; expected signed/unsigned tuple {expected}')
    if style == 2 and root['string_18'] != EXPRESSION:
        raise ValueError(f'custom property expression {root["string_18"]}; expected original {EXPRESSION!r}')


def verify_weldment(root: dict[str, object], kind: int, style: int, edited: bool) -> None:
    """Distinguish the base, direct list and tagged alias graph; e.g. verify_weldment(root, 0, 2, False)."""
    if root['_class'] != CLASSES[kind]:
        raise ValueError(f'weldment class {root["_class"]}; expected {CLASSES[kind]}')
    if kind == 0:
        payload = cast(dict[str, object], root['payload'])
        if payload['raw_370'] != (0, 1, -2147483648, 2147483647)[style]:
            raise ValueError(f'weldment raw word {payload}; expected independent stored i32')
        verify_properties(cast(dict[str, object], payload['properties_360']), style, edited)
    if kind == 1:
        verify_properties(root, style, edited)
    if kind == 2:
        verify_record(root, style, edited)
    if kind == 3:
        items = cast(list[dict[str, object]], root['items'])
        if items[0] is not items[1] or items[2] is not items[3]:
            raise ValueError('tagged property graph; expected cached list and record identities')
        verify_properties(items[0], style, edited)


def observe_weldment(reference: Path, kind: int, version: int, style: int) -> tuple[bytes, dict[str, object]]:
    """Require complete primitive partitions and exact map allocation; e.g. observe_weldment(source, 0, 19000, 2)."""
    raw, slots = weldment_object(kind, version, style)
    cursor = object_cursor(reference, raw, version, None)
    with observe_primitives(source_module('swarchive', reference).Archive, raw) as reads:
        verify_weldment(cast(dict[str, object], cursor.object()), kind, style, False)
    if (cursor.p, len(cursor.map)) != (len(raw), slots):
        raise ValueError(f'weldment end/map {(cursor.p, len(cursor.map))}; expected {(len(raw), slots)}')
    verify_primitive_coverage(raw, reads);oracle = inventory_bytes(reads)
    return struct.pack('<BIII', kind, version, len(raw), len(oracle))+raw+oracle, dict(kind=kind, version=version, style=style,
        bytes=len(raw), slots=slots, reads=len(oracle.splitlines()), sha256=hashlib.sha256(raw).hexdigest(),
        oracle_sha256=hashlib.sha256(oracle).hexdigest())


def owned_weldment(reference: Path, style: int, edited: bool) -> tuple[SavedSheetMetalDocument, bytes, int]:
    """Use actual persisted manager context and inherited slots; e.g. owned_weldment(source, 2, False)."""
    container, prefix = (source_module(name, reference) for name in ('swcontainer', 'modelprefix'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, _ = saved_manager(reference, original, 0)
    probe = SavedSheetMetalDocument(original, manager, b'')
    _, context = prefix.read(probe, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    feature, slots = weldment_object(0, 19000, style, len(context.map)-1, edited)
    raw = struct.pack('<iH', -123, 1)+feature
    return SavedSheetMetalDocument(original, manager, raw), raw, slots


def freeze_owned(reference: Path, destination: Path, style: int, edited: bool) -> dict[str, object]:
    """Observe a complete property edit without changing duplicate neighbors; e.g. freeze_owned(source, output, 2, True)."""
    features, archive = (source_module(name, reference) for name in ('resolvedfeatures', 'swarchive'))
    document, raw, slots = owned_weldment(reference, style, edited)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(document, STREAM)
        verify_weldment(cast(dict[str, object], parsed['items'][0]), 0, style, edited)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'owned weldment end/tail/map; expected {(len(raw), b"", slots)}')
    label = f'owned-{style}'+('-edited' if edited else '')
    return write_observation(destination, label, raw, [read for read in reads if read.end > read.start]) | dict(slots=slots,
        imported_slots=97, manager_sha256=hashlib.sha256(document.manager).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind all cases to one stable approved reader revision; e.g. freeze(source, output)."""
    before = reference_snapshot(reference);sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    cases = [observe_weldment(reference, kind, version, style) for kind in (0, 1, 3) for version in VERSIONS for style in range(4)]
    cases += [observe_weldment(reference, 2, version, style) for version in VERSIONS for style in (0, 2)]
    packet = b'WLD1'+struct.pack('<I', len(cases))+b''.join(struct.pack('<I', row['slots']) for _, row in cases)+b''.join(raw for raw, _ in cases)
    (destination/'cases.bin').write_bytes(packet)
    owned = [freeze_owned(reference, destination, style, edited) for style, edited in ((0, False), (1, False), (2, False), (2, True))]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    scripts = sorted(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[row for _, row in cases], owned=owned, source_hashes=before, packet_sha256=hashlib.sha256(packet).hexdigest(),
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in scripts},
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    facts = freeze(REFERENCE, WORKSPACE/'tests/fixtures/weldments')
    (Path(__file__).parent/'reference.json').write_text(json.dumps(facts, indent=2)+'\n', encoding='utf-8')
