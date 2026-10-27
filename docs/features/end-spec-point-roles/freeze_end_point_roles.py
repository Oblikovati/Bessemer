"""Independent complete end-specification point-role graphs and edits (#46)."""
from functools import partial
from pathlib import Path
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
for folder in ('model-archives', 'reference-axes', 'sweeps', 'sheet-metal', 'resolved-features', 'circular-patterns', 'plane-point-roles'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import ReferenceObjectCursor, observe_object_body
from sheet_owned_fixtures import SavedSheetMetalDocument, saved_manager
from primitive_inventory import observe_primitives, verify_primitive_coverage
from freeze_owned_features import STREAM
from owned_circular_fixtures import write_observation
from end_point_bytes import role_object

VERSIONS = (0x27cc, 0x2b83, 0x2b84, 0x3b00, 0x3b01, 19000)


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Keep registration local to each complete cursor; e.g. configure(cursor, source)."""
    bodies = source_module('bodyfeatures', reference)
    cursor.readers['moEndSpec_c'] = bodies.end_spec
    cursor.readers['moRevEndSpec_c'] = bodies.revolution_end_spec


def owned(reference: Path, destination: Path, owner: int, point: int, edited: bool, label: str) -> dict[str, object]:
    """Observe actual configuration-map imports and complete side references; e.g. owned(source, output, 0, 1, False, 'owned-endpoint')."""
    container, prefix, features, archive = (source_module(name, reference) for name in ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, _ = saved_manager(reference, original, 0)
    probe = SavedSheetMetalDocument(original, manager, b'')
    _, context = prefix.read(probe, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    feature, slots = role_object(owner, point, 19000, True, len(context.map)-1, edited, feature=True)
    raw = struct.pack('<iH', -123, 1)+feature
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(SavedSheetMetalDocument(original, manager, raw), STREAM)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'end-point end/tail/map; expected {(len(raw), b"", slots)}')
    verify_primitive_coverage(raw, reads)
    return write_observation(destination, label, raw, [read for read in reads if read.end > read.start]) | dict(slots=slots, valid=point < 3)


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind all graph/class/role cases to unchanged source readers; e.g. freeze(source, fixture_dir)."""
    before = reference_snapshot(reference);sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    cases = [observe_object_body(reference, *role_object(owner, point, version, cached), owner*8+point*2+cached,
        version, f'{owner}/{point}/{cached}', partial(configure, reference=reference))
        for owner in range(2) for point in range(4) for version in VERSIONS for cached in (False, True)]
    packet = b'ESP1'+struct.pack('<I', len(cases))+b''.join(struct.pack('<I', row['slots']) for _, row in cases)+b''.join(frame for frame, _ in cases)
    (destination/'cases.bin').write_bytes(packet)
    saved = [owned(reference, destination, owner, point, False, f'owned-{owner}-{point}') for owner in range(2) for point in range(4)]
    saved += [owned(reference, destination, owner, 1, True, f'owned-{owner}-1-edited') for owner in range(2)]
    if before != reference_snapshot(reference):
        raise ValueError('reference source changed; expected stable reader hashes')
    return dict(cases=[row for _, row in cases], owned=saved, source_hashes=before,
        packet_sha256=hashlib.sha256(packet).hexdigest(), reads=sum(row['reads'] for _, row in cases))


if __name__ == '__main__':
    facts = freeze(REFERENCE, WORKSPACE/'tests/fixtures/end-spec-point-roles')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts, indent=2)+'\n', encoding='utf-8')
