"""Independent complete geometry-backup observations and declared-owner edits (#54)."""
from functools import partial
from pathlib import Path
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
for folder in ('model-archives', 'sweeps', 'sheet-metal', 'resolved-features', 'circular-patterns', 'shared-sketch-handles'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import ReferenceObjectCursor, observe_object_body
from sheet_owned_fixtures import SavedSheetMetalDocument, saved_manager
from primitive_inventory import observe_primitives, verify_primitive_coverage
from freeze_owned_features import STREAM
from owned_circular_fixtures import write_observation
from backup_fixture_bytes import CLASSES, backup_object

VERSIONS = (0x27cc, 19000, 0xffffffff)


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Keep all registrations local to one independent complete cursor; e.g. configure(cursor, source)."""
    sketches = source_module('sketchprefix', reference);sketches.install_scoped_readers(cursor)
    backups = source_module('backupfrontier', reference)
    cursor.readers.update({CLASSES[0]: backups.parabola, CLASSES[1]: backups.cylinder,
        CLASSES[2]: sketches.handle, CLASSES[3]: sketches.handle})


def observe(reference: Path, kind: int, version: int, style: int, mode: int = 0, index_case: int = 0) -> tuple[bytes, dict[str, object]]:
    """Require exact endpoints, primitive coverage and real map allocations; e.g. observe(source, 0, 19000, 2, -1)."""
    raw, slots = backup_object(kind, version, style, mode, index_case=index_case)
    label = f'{kind}/{style}/{mode}/{index_case}'
    return observe_object_body(reference, raw, slots, kind*3+style, version, label, partial(configure, reference=reference))


def owned(reference: Path, destination: Path, kind: int, style: int, edited: bool, label: str) -> dict[str, object]:
    """Resolve actual saved context before constructing complete owners; e.g. owned(source, output, 0, 2, False, 'owned-parabola')."""
    container, prefix, features, archive = (source_module(name, reference) for name in ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, _ = saved_manager(reference, original, 0)
    probe = SavedSheetMetalDocument(original, manager, b'')
    _, context = prefix.read(probe, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    feature, slots = backup_object(kind, 19000, style, -1, edited, len(context.map)-1, 7)
    raw = struct.pack('<iH', -123, 1)+feature
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(SavedSheetMetalDocument(original, manager, raw), STREAM)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'geometry-backup end/tail/map; expected {(len(raw), b"", slots)}')
    verify_primitive_coverage(raw, reads)
    return write_observation(destination, label, raw, [read for read in reads if read.end > read.start]) | dict(slots=slots)


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind complete cases and independent edits to stable approved sources; e.g. freeze(source, fixture_dir)."""
    before = reference_snapshot(reference);sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    cases = [observe(reference, kind, version, style) for kind in range(4) for version in VERSIONS for style in range(3)]
    cases += [observe(reference, kind, version, style, index_case=index) for kind in (2, 3) for version in VERSIONS for style in range(3) for index in range(1, 8)]
    cases += [observe(reference, kind, version, 2, mode) for kind in (0, 1) for version in VERSIONS for mode in (-2, -1)]
    packet = b'GBC1'+struct.pack('<I', len(cases))+b''.join(struct.pack('<I', row['slots']) for _, row in cases)+b''.join(frame for frame, _ in cases)
    (destination/'cases.bin').write_bytes(packet)
    saved = [owned(reference, destination, kind, style, False, f'owned-{kind}-{style}') for kind in range(4) for style in (0, 2)]
    saved += [owned(reference, destination, kind, 2, True, f'owned-{kind}-2-edited') for kind in range(4)]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    return dict(cases=[row for _, row in cases], owned=saved, source_hashes=before,
        packet_sha256=hashlib.sha256(packet).hexdigest(), reads=sum(row['reads'] for _, row in cases))


if __name__ == '__main__':
    facts = freeze(REFERENCE, WORKSPACE/'tests/fixtures/sketch-geometry-backups')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts, indent=2)+'\n', encoding='utf-8')
