"""Independent flat-pattern EOF, raw-kernel and direct-box observations (#54)."""
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
for folder in ('model-archives', 'reference-axes', 'sweeps', 'sheet-metal', 'base-flanges', 'resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from sheet_reference_observer import observe_sheet_object
from freeze_base_flanges import install_flange_dimensions
from sheet_primitive_projection import dynamic_payload_ranges, project_payload_words
from sheet_owned_fixtures import SavedSheetMetalDocument, saved_manager
from freeze_owned_features import STREAM
from primitive_inventory import observe_primitives
from owned_circular_fixtures import write_observation
from flat_fixture_bytes import FlatFixtureOptions, flat_object

VERSIONS = (0x27cc, 0x2baf, 0x2bc4, 0x2bc5, 0x3364, 0x3365, 0x372b, 0x372c, 0x3761, 0x3762, 0x46b9, 19000)


def verify_flat(root: dict[str, object], version: int, options: FlatFixtureOptions) -> None:
    """Pin independently expected fields and aliases before byte projection; e.g. verify_flat(root, 19000, options)."""
    payload = cast(dict[str, object], root['payload'])
    expected = (version > 0x372b, version > 0x3364, version > 0x3761, version > 0x3761 and options.rich, options.extended, options.present)
    actual = tuple(key in payload for key in ('raw_9b4', 'face_920', 'raw_928', 'string_930', 'raw_a80', 'body_a10'))
    if actual != expected:
        raise ValueError(f'flat gates {actual}; expected {expected}')
    pairs = cast(list[dict[str, int]], payload['pairs_ac0'])
    expected_pairs = [dict(first=7, second=8), dict(first=7, second=-9), dict(first=-2147483648, second=2147483647)] if options.rich else []
    if pairs != expected_pairs:
        raise ValueError(f'flat pairs {pairs}; expected ordered duplicate keys {expected_pairs}')
    if options.rich:
        first = cast(list[object], payload['array_948'])
        if first[0] is not first[2] or first[1] is not None or payload['reference_8f8'] is not payload['reference_908']:
            raise ValueError('flat identities; expected shared/null/shared array and shared face references')
    box = cast(dict[str, dict[str, object]], payload['box_918'])['data_10']
    if struct.pack('<d', cast(float, box['raw_48'])) != struct.pack('<d', 123.5 if options.edited else -0.0):
        raise ValueError(f'flat box extra {box["raw_48"]}; expected independent raw-double edit')


def records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Cover both saved states, present kernels and direct/tagged boxes; e.g. records(source)."""
    result = []
    for version in VERSIONS:
        for extended in (False, True):
            for rich in (False, True):
                for present in (False, True):
                    options = FlatFixtureOptions(extended, rich, present)
                    raw, slots = flat_object(version, options)
                    result.append(observe_sheet_object(reference, raw, slots, int(extended)+2*int(present), version, int(extended),
                        partial(verify_flat, version=version, options=options), partial(install_flange_dimensions, reference=reference)))
        for kind, name in ((4, 'moFlatPatBoundBox_c'), (8, 'moBoundBoxData_c')):
            for rich in (False, True):
                raw, slots = flat_object(version, FlatFixtureOptions(rich=rich), name=name)
                result.append(observe_sheet_object(reference, raw, slots, kind, version, 0, partial(verify_box, rich=rich)))
    return result


def verify_box(root: dict[str, object], rich: bool) -> None:
    """Use the same expected vector order through both tagged entry points; e.g. verify_box(root, True)."""
    contents = cast(dict[str, object], root.get('data_10', root))
    expected = (-0.0, 1.0, 2.0) if rich else (0.0, 0.0, 0.0)
    if contents['vector_50'] != expected or contents['raw_90'] != (-2147483648 if rich else 0):
        raise ValueError(f'box values {contents}; expected vector {expected} and preserved signed word')


def owned_flat(reference: Path, options: FlatFixtureOptions) -> tuple[SavedSheetMetalDocument, bytes, int]:
    """Use complete persisted CMgr state before resolving the feature stream; e.g. owned_flat(source, options)."""
    container, prefix = (source_module(name, reference) for name in ('swcontainer', 'modelprefix'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, _ = saved_manager(reference, original, int(options.extended))
    probe = SavedSheetMetalDocument(original, manager, b'')
    _, context = prefix.read(probe, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    feature, slots = flat_object(19000, options, len(context.map)-1)
    raw = struct.pack('<iH', -123, 1)+feature+(bytes(4) if options.extended else b'')
    return SavedSheetMetalDocument(original, manager, raw), raw, slots


def freeze_owned(reference: Path, destination: Path, options: FlatFixtureOptions, label: str) -> dict[str, object]:
    """Observe complete documents, including unsuppressed raw kernels; e.g. freeze_owned(source, output, options, 'owned-rich')."""
    features, archive = (source_module(name, reference) for name in ('resolvedfeatures', 'swarchive'))
    document, raw, slots = owned_flat(reference, options)
    with observe_primitives(archive.Archive, raw) as reads, dynamic_payload_ranges(reference, raw) as ranges:
        parsed = features.read_supported(document, STREAM)
        verify_flat(cast(dict[str, object], parsed['items'][0]), 19000, options)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'owned flat end/tail/map; expected {(len(raw), b"", slots)}')
    return write_observation(destination, label, raw, project_payload_words(raw, reads, ranges)) | dict(slots=slots, imported_slots=97,
        helper=int(options.extended), manager_sha256=hashlib.sha256(document.manager).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Reject mixed sibling reader revisions; e.g. freeze(source, output)."""
    before = reference_snapshot(reference);sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    cases = records(reference)
    packet = b'FLT1'+struct.pack('<I', len(cases))+b''.join(struct.pack('<I', row['slots']) for _, row in cases)+b''.join(frame for frame, _ in cases)
    (destination/'cases.bin').write_bytes(packet)
    rich = FlatFixtureOptions(rich=True)
    options = [(FlatFixtureOptions(), 'owned-empty'), (rich, 'owned-rich-0'),
        (replace(rich, extended=True, present=True), 'owned-rich-1'), (replace(rich, extended=True, present=True, edited=True), 'owned-rich-1-edited')]
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
    facts = freeze(REFERENCE, WORKSPACE/'tests/fixtures/flat-patterns')
    (Path(__file__).parent/'reference.json').write_text(json.dumps(facts, indent=2)+'\n', encoding='utf-8')
