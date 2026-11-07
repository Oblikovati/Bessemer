"""Freeze complete independent revolved surface/data/keeper records and signed-word edits (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
for folder in ('model-archives', 'sweeps', 'sheet-metal', 'resolved-features', 'circular-patterns', 'reference-axes', 'reference-axis-owners', 'revolved-surfaces'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import observe_object_body
from sheet_owned_fixtures import SavedSheetMetalDocument, saved_manager
from primitive_inventory import observe_primitives, verify_primitive_coverage
from freeze_owned_features import STREAM
from owned_circular_fixtures import write_observation
from revolved_bytes import WORDS, VERSIONS, revolved_object


def owned(reference: Path, destination: Path, kind: int, style: int, edited: bool, label: str) -> dict[str, object]:
    """Use the actual complete inherited context before building aliases; e.g. owned(source, output, 0, 2, False, 'owned-item')."""
    container, prefix, features, archive = (source_module(name, reference) for name in ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, _ = saved_manager(reference, original, 0)
    probe = SavedSheetMetalDocument(original, manager, b'')
    _, context = prefix.read(probe, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    feature, slots = revolved_object(kind, 19000, style, WORDS[0], edited, len(context.map)-1)
    raw = struct.pack('<iH', -123, 1)+feature
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(SavedSheetMetalDocument(original, manager, raw), STREAM)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'revolved surface end/tail/map; expected {(len(raw), b"", slots)}')
    verify_primitive_coverage(raw, reads)
    return write_observation(destination, label, raw, [read for read in reads if read.end > read.start]) | dict(slots=slots)


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind independently assembled revolved graphs to stable approved source; e.g. freeze(source, fixtures)."""
    before = reference_snapshot(reference);sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    cases = [observe_object_body(reference, *revolved_object(kind, version, style, words), kind*4+style,
        version, f'{kind}/{style}/{words}') for kind in range(3) for version in VERSIONS
        for style in range(4) for words in WORDS]
    cases += [observe_object_body(reference, *revolved_object(0, 19000, 4, WORDS[0]), 12, 19000, 'kernel')]
    packet = b'RVS1'+struct.pack('<I', len(cases))+b''.join(struct.pack('<I', row['slots']) for _, row in cases)+b''.join(frame for frame, _ in cases)
    (destination/'cases.bin').write_bytes(packet)
    saved = [owned(reference, destination, kind, style, False, f'owned-{kind}-{style}') for kind in range(3) for style in range(4)]
    saved += [owned(reference, destination, kind, 2, True, f'owned-{kind}-2-edited') for kind in range(2)]
    saved += [owned(reference, destination, 0, 4, False, 'owned-kernel')]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    return dict(cases=[row for _, row in cases], owned=saved, source_hashes=before,
        packet_sha256=hashlib.sha256(packet).hexdigest(), reads=sum(row['reads'] for _, row in cases))


if __name__ == '__main__':
    facts = freeze(REFERENCE, WORKSPACE/'tests/fixtures/revolved-surfaces')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts, indent=2)+'\n', encoding='utf-8')
