"""Independent complete base-flange fixtures with selected-slot and saved-state proofs (#54)."""
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
for folder in ('model-archives', 'reference-axes', 'sweeps', 'sheet-metal', 'resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from sheet_reference_observer import observe_sheet_object
from sheet_primitive_projection import dynamic_payload_ranges, project_payload_words
from sheet_owned_fixtures import SavedSheetMetalDocument, saved_manager
from freeze_owned_features import STREAM
from primitive_inventory import observe_primitives
from owned_circular_fixtures import write_observation
from flange_fixture_bytes import FlangeFixtureOptions, flange_object
from object_fixture_observer import ReferenceObjectCursor

VERSIONS = (0x27cc, 0x2b8e, 0x2b8f, 0x2bae, 0x2baf, 0x2bb9, 0x2bba, 0x3b0c, 0x3b0d, 0x46b9, 0x46ba, 19000)


def install_flange_dimensions(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install the scoped inherited dimension readers before the common bend; e.g. install_flange_dimensions(cursor, source)."""
    dimensions = source_module('displaydimensions', reference)
    dimensions.install_scoped(cursor)
    sketches = source_module('sketchprefix', reference)
    sketches.install_scoped_readers(cursor)
    profiles = source_module('assemblyprofiles', reference)
    cursor.readers['sgSketch'] = profiles.sketch


def verify_flange(root: dict[str, object], version: int, options: FlangeFixtureOptions) -> None:
    """Use explicit expected slot sets independent of the decoder predicate; e.g. verify_flange(root, 19000, options)."""
    payload = cast(dict[str, object], root['payload'])
    directions = cast(list[dict[str, object]], payload['directions'])
    expected = {0: [0, 1], 1: [0], 2: [1]}.get(options.mode, [])
    if [item['index'] for item in directions] != expected:
        raise ValueError(f'flange directions {directions}; expected slot order {expected} for raw mode {options.mode}')
    for item in directions:
        verify_direction(item, options)
    if ('raw_8cc' in payload, 'from_end_a18' in payload) != (version > 0x3b0c, version > 0x46b9):
        raise ValueError(f'flange version {version}; expected independent late-word and from-end gates')
    verify_handles(cast(dict[str, dict[str, object]], payload['handle_arrays']), options.handles)
    group = cast(dict[str, object], root['bend_group'])
    entries = cast(list[object], group['array_858'])
    if options.rich and (len(entries) != 3 or entries[0] is not entries[2] or entries[1] is not None):
        raise ValueError(f'bend-group array {entries}; expected original shared/null/shared order before runtime filtering')


def verify_direction(direction: dict[str, object], options: FlangeFixtureOptions) -> None:
    """Retain unknown types and pointer-presence distinctions; e.g. verify_direction(direction, options)."""
    index = cast(int, direction['index']);kind = (options.first_type, options.second_type)[index]
    if direction['type'] != kind:
        raise ValueError(f'flange direction type {direction["type"]}; expected raw {kind}')
    pointer = kind == 3 or kind in (4, 5) and options.reference_byte != 0
    if ('reference' in direction) != pointer or ('raw_value' in direction) != (kind in (0, 6)):
        raise ValueError(f'flange type/reference {(kind, options.reference_byte)}; expected only its documented payload branch')
    expected = struct.pack('<d', 123.5 if options.edited else -0.0)
    if struct.pack('<d', cast(float, direction['raw_extra'])) != expected:
        raise ValueError(f'flange extra {direction["raw_extra"]}; expected preserved negative zero or independent edit')


def verify_handles(arrays: dict[str, dict[str, object]], populated: bool) -> None:
    """Keep each collection's sketch and repeated/null handles; e.g. verify_handles(arrays, True)."""
    expected = (3, 0, 1, 2) if populated else (0, 0, 0, 0)
    if tuple(arrays[key]['count'] for key in ('958', '978', '9b8', '9d8')) != expected:
        raise ValueError(f'flange handle counts {arrays}; expected independent counts {expected}')
    if not populated:
        return
    first = arrays['958'];handles = cast(list[object], first['handles'])
    if handles[0] is not handles[1] or handles[2] is not None:
        raise ValueError('flange handles 958; expected shared/shared/null identities')
    for key in ('9b8', '9d8'):
        later = arrays[key]
        if later['sketch'] is not first['sketch'] or cast(list[object], later['handles'])[0] is not handles[0]:
            raise ValueError(f'flange handles {key}; expected the same complete sketch and point handle')


def observe_flange(reference: Path, version: int, options: FlangeFixtureOptions) -> tuple[bytes, dict[str, object]]:
    """Observe exact EOF and inherited-map allocation; e.g. observe_flange(source, 19000, options)."""
    raw, slots = flange_object(version, options)
    frame, facts = observe_sheet_object(reference, raw, slots, int(options.extended), version, int(options.extended),
        partial(verify_flange, version=version, options=options), partial(install_flange_dimensions, reference=reference))
    return frame, facts | dict(options=options.__dict__)


def freeze_records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Cover every branch without conflating mode, type and state; e.g. freeze_records(source)."""
    records = [observe_flange(reference, version, FlangeFixtureOptions(extended=extended, rich=rich, handles=rich,
        mode=0 if rich else 1, first_type=4 if rich else 0, second_type=5 if rich else 6, reference_byte=255 if rich else 0))
        for version in VERSIONS for extended in (False, True) for rich in (False, True)]
    records += [observe_flange(reference, 19000, FlangeFixtureOptions(mode=mode, first_type=kind, second_type=kind,
        reference_byte=flag, rich=kind in (3, 4, 5), handles=False))
        for mode in (-2147483648, -1, 0, 1, 2, 3, 2147483647)
        for kind in (-2147483648, -1, 0, 1, 2, 3, 4, 5, 6, 7, 2147483647) for flag in (0, 255)]
    return records


def owned_flange(reference: Path, options: FlangeFixtureOptions) -> tuple[SavedSheetMetalDocument, bytes, int]:
    """Select saved state from complete CMgr rather than the feature bytes; e.g. owned_flange(source, options)."""
    container, prefix = (source_module(name, reference) for name in ('swcontainer', 'modelprefix'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, _ = saved_manager(reference, original, int(options.extended))
    probe = SavedSheetMetalDocument(original, manager, b'')
    _, context = prefix.read(probe, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    feature, slots = flange_object(19000, options, len(context.map)-1)
    raw = struct.pack('<iH', -123, 1)+feature+(bytes(4) if options.extended else b'')
    return SavedSheetMetalDocument(original, manager, raw), raw, slots


def freeze_owned(reference: Path, destination: Path, options: FlangeFixtureOptions, label: str) -> dict[str, object]:
    """Bind complete documents and independent extra-value edits; e.g. freeze_owned(source, output, options, 'owned-rich')."""
    features, archive = (source_module(name, reference) for name in ('resolvedfeatures', 'swarchive'))
    document, raw, slots = owned_flange(reference, options)
    with observe_primitives(archive.Archive, raw) as reads, dynamic_payload_ranges(reference, raw) as ranges:
        parsed = features.read_supported(document, STREAM)
        verify_flange(cast(dict[str, object], parsed['items'][0]), 19000, options)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'owned flange end/tail/map; expected {(len(raw), b"", slots)}')
    projected = project_payload_words(raw, reads, ranges)
    return write_observation(destination, label, raw, projected) | dict(slots=slots, imported_slots=97,
        helper=int(options.extended), manager_sha256=hashlib.sha256(document.manager).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Keep all source modules fixed across fixture generation; e.g. freeze(source, output)."""
    before = reference_snapshot(reference);sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    records = freeze_records(reference)
    packet = b'BFL1'+struct.pack('<I', len(records))
    packet += b''.join(struct.pack('<I', facts['slots']) for _, facts in records)+b''.join(frame for frame, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    rich = FlangeFixtureOptions(rich=True, mode=2, second_type=5, reference_byte=255, handles=True)
    options = [(FlangeFixtureOptions(), 'owned-empty'), (rich, 'owned-rich-0'),
        (replace(rich, extended=True), 'owned-rich-1'), (replace(rich, extended=True, edited=True), 'owned-rich-1-edited')]
    owned = [freeze_owned(reference, destination, entry, label) for entry, label in options]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    return fixture_report(records, owned, before, packet)


def fixture_report(records: list[tuple[bytes, dict[str, object]]], owned: list[dict[str, object]],
                   hashes: dict[str, str], packet: bytes) -> dict[str, object]:
    """Bind every reused constructor and projected observation; e.g. fixture_report(records, owned, hashes, packet)."""
    scripts = sorted(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[facts for _, facts in records], owned=owned, source_hashes=hashes,
        packet_sha256=hashlib.sha256(packet).hexdigest(),
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in scripts},
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    facts = freeze(REFERENCE, WORKSPACE/'tests/fixtures/base-flanges')
    (Path(__file__).parent/'reference.json').write_text(json.dumps(facts, indent=2)+'\n', encoding='utf-8')
