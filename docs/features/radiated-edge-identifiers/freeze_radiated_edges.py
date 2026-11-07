"""Independent complete radiated-edge observations, including real saved-map documents (#54)."""
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
from object_fixture_observer import observe_object_body
from sheet_owned_fixtures import SavedSheetMetalDocument, saved_manager
from primitive_inventory import observe_primitives, verify_primitive_coverage
from freeze_owned_features import STREAM
from owned_circular_fixtures import write_observation
from radiated_edge_bytes import identifier_object


def owned(reference: Path, destination: Path, style: int, rich: bool, edited: bool, label: str) -> dict[str, object]:
    """Verify complete context import and both declared reference occurrences; e.g. owned(source, output, 2, True, False, 'owned-alias')."""
    container, prefix, features, archive = (source_module(name, reference) for name in ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, _ = saved_manager(reference, original, 0)
    probe = SavedSheetMetalDocument(original, manager, b'')
    _, context = prefix.read(probe, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    feature, slots = identifier_object(style, rich, edited, len(context.map)-1)
    raw = struct.pack('<iH', -123, 1)+feature
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(SavedSheetMetalDocument(original, manager, raw), STREAM)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'radiated identifier end/tail/map; expected {(len(raw), b"", slots)}')
    verify_primitive_coverage(raw, reads)
    return write_observation(destination, label, raw, [read for read in reads if read.end > read.start]) | dict(slots=slots)


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind complete observations to unchanged approved sources; e.g. freeze(source, fixture_dir)."""
    before = reference_snapshot(reference);sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    cases = [observe_object_body(reference, *identifier_object(style, rich), style, version, f'{style}/{rich}')
        for style in range(3) for version in (0x27cc, 19000, 0xffffffff) for rich in (False, True)]
    packet = b'REI1'+struct.pack('<I', len(cases))+b''.join(struct.pack('<I', row['slots']) for _, row in cases)+b''.join(frame for frame, _ in cases)
    (destination/'cases.bin').write_bytes(packet)
    saved = [owned(reference, destination, style, rich, edited, label) for style, rich, edited, label in
        [(0, False, False, 'owned-empty'), (1, True, False, 'owned-owner'), (2, True, False, 'owned-alias'), (2, True, True, 'owned-alias-edited')]]
    if before != reference_snapshot(reference):
        raise ValueError('reference source changed; expected stable reader hashes')
    return dict(cases=[row for _, row in cases], owned=saved, source_hashes=before,
        packet_sha256=hashlib.sha256(packet).hexdigest(), reads=sum(row['reads'] for _, row in cases))


if __name__ == '__main__':
    facts = freeze(REFERENCE, WORKSPACE/'tests/fixtures/radiated-edge-identifiers')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts, indent=2)+'\n', encoding='utf-8')
