"""Freeze complete independent cut-list folder layers and direct property edits (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
for folder in ('model-archives', 'sweeps', 'sheet-metal', 'resolved-features', 'circular-patterns', 'weldments', 'cut-list-folders'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import observe_object_body
from sheet_owned_fixtures import SavedSheetMetalDocument, saved_manager
from primitive_inventory import observe_primitives, verify_primitive_coverage
from freeze_owned_features import STREAM
from owned_circular_fixtures import write_observation
from cut_list_bytes import VERSIONS, cut_list_object
from folder_identity_checks import verify_aliases


def owned(reference: Path, destination: Path, kind: int, style: int, edited: bool, label: str) -> dict[str, object]:
    """Use the actual complete inherited context before building aliases; e.g. owned(source, output, 0, 2, False, 'owned-draft')."""
    container, prefix, features, archive = (source_module(name, reference) for name in ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, _ = saved_manager(reference, original, 0)
    probe = SavedSheetMetalDocument(original, manager, b'')
    _, context = prefix.read(probe, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    feature, slots = cut_list_object(kind, 19000, style, edited, len(context.map)-1)
    raw = struct.pack('<iH', -123, 1)+feature
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(SavedSheetMetalDocument(original, manager, raw), STREAM)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'cut-list folder end/tail/map; expected {(len(raw), b"", slots)}')
    verify_primitive_coverage(raw, reads)
    return write_observation(destination, label, raw, [read for read in reads if read.end > read.start]) | dict(slots=slots)


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind independently assembled cut-list graphs to stable approved source; e.g. freeze(source, fixtures)."""
    before = reference_snapshot(reference);sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    cases = [bare_case(reference, kind, version, style) for kind in range(14)
        for version in VERSIONS for style in range(6)]
    packet = b'CLF1'+struct.pack('<I', len(cases))+b''.join(struct.pack('<I', row['slots']) for _, row in cases)+b''.join(frame for frame, _ in cases)
    (destination/'cases.bin').write_bytes(packet)
    saved = [owned(reference, destination, kind, style, False, f'owned-{kind}-{style}') for kind in range(14) for style in range(6)]
    saved += [owned(reference, destination, kind, 2, True, f'owned-{kind}-2-edited') for kind in (4, 5)]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    return dict(cases=[row for _, row in cases], owned=saved, source_hashes=before,
        packet_sha256=hashlib.sha256(packet).hexdigest(), reads=sum(row['reads'] for _, row in cases))


def bare_case(reference: Path, kind: int, version: int, style: int) -> tuple[bytes, dict[str, object]]:
    """Check typed targets and the independent primitive partition; e.g. bare_case(source, 5, 19000, 2)."""
    raw, slots = cut_list_object(kind, version, style)
    verify_aliases(reference, raw, version, kind, style)
    return observe_object_body(reference, raw, slots, kind*6+style, version, f'{kind}/{style}')


if __name__ == '__main__':
    facts = freeze(REFERENCE, WORKSPACE/'tests/fixtures/cut-list-folders')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts, indent=2)+'\n', encoding='utf-8')
