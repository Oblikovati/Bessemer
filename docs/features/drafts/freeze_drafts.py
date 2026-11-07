"""Freeze complete independent draft/keeper observations and actual dynamic-payload word spans (#54)."""
from functools import partial
from pathlib import Path
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
for folder in ('model-archives', 'sweeps', 'sheet-metal', 'resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from sheet_reference_observer import observe_sheet_object
from sheet_owned_fixtures import SavedSheetMetalDocument, saved_manager
from sheet_primitive_projection import dynamic_payload_ranges, project_payload_words
from primitive_inventory import observe_primitives
from freeze_owned_features import STREAM
from owned_circular_fixtures import write_observation
from draft_reference_bytes import DraftOptions
from draft_fixture_bytes import CLASSES, draft_object

VERSIONS = (0x27cc, 0x3746, 0x3747, 19000, 0xffffffff)


def verify_class(root: dict[str, object], kind: int) -> None:
    """Require the actual complete constructor identity; e.g. verify_class(root, 0)."""
    if root['_class'] != CLASSES[kind]:
        raise ValueError(f'draft class {root["_class"]}; expected {CLASSES[kind]}')


def owned(reference: Path, destination: Path, kind: int, options: DraftOptions, label: str) -> dict[str, object]:
    """Use actual saved context before constructing the full draft or keeper; e.g. owned(source, output, 0, options, 'owned-rich')."""
    container, prefix, features, archive = (source_module(name, reference) for name in ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, _ = saved_manager(reference, original, 0)
    probe = SavedSheetMetalDocument(original, manager, b'')
    _, context = prefix.read(probe, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    feature, slots = draft_object(kind, 19000, options, len(context.map)-1)
    raw = struct.pack('<iH', -123, 1)+feature
    with observe_primitives(archive.Archive, raw) as reads, dynamic_payload_ranges(reference, raw) as ranges:
        parsed = features.read_supported(SavedSheetMetalDocument(original, manager, raw), STREAM)
        verify_class(parsed['items'][0], kind)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'draft end/tail/map; expected {(len(raw), b"", slots)}')
    projected = project_payload_words(raw, reads, ranges)
    return write_observation(destination, label, raw, projected) | dict(slots=slots, source_reads=sum(read.end > read.start for read in reads), raw_payload_ranges=[list(span) for span in ranges])


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Retain complete source observations and independent field-edit goldens; e.g. freeze(source, fixtures)."""
    before = reference_snapshot(reference);sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    cases = [observe_sheet_object(reference, *draft_object(kind, version, DraftOptions(style)), kind*6+style,
        version, 0, partial(verify_class, kind=kind)) for kind in range(3) for version in VERSIONS for style in range(6)]
    packet = b'DRF1'+struct.pack('<I', len(cases))+b''.join(struct.pack('<I', row['slots']) for _, row in cases)+b''.join(frame for frame, _ in cases)
    (destination/'cases.bin').write_bytes(packet)
    entries = [(0, DraftOptions(style), f'owned-{style}') for style in range(6)]
    entries += [(0, DraftOptions(1, True), 'owned-1-edited'), (1, DraftOptions(5), 'owned-keeper-1'), (2, DraftOptions(5), 'owned-keeper-2')]
    saved = [owned(reference, destination, kind, options, label) for kind, options, label in entries]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    return dict(cases=[row for _, row in cases], owned=saved, source_hashes=before,
        packet_sha256=hashlib.sha256(packet).hexdigest(), reads=sum(row['reads'] for _, row in cases))


if __name__ == '__main__':
    facts = freeze(REFERENCE, WORKSPACE/'tests/fixtures/drafts')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts, indent=2)+'\n', encoding='utf-8')
