"""Freeze complete independent dimension reads, with no generated runtime field metadata (#54)."""
from functools import partial
from pathlib import Path
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
for folder in ('model-archives', 'reference-axes', 'sweeps', 'sheet-metal', 'resolved-features', 'plane-point-roles', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import ReferenceObjectCursor, observe_object_body
from reference_dimension_bytes import ReferenceOptions, reference_object
from sheet_owned_fixtures import SavedSheetMetalDocument, saved_manager
from freeze_owned_features import STREAM
from primitive_inventory import observe_primitives, verify_primitive_coverage
from owned_circular_fixtures import write_observation

VERSIONS = (0x27cc, 0x2f6e, 0x2f6f, 0x3747, 0x3748, 0x42ee, 0x42ef, 0x46c6, 0x46c7, 19000)


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install complete locally scoped sketch and plane readers; e.g. configure(cursor, source)."""
    source_module('sketchprefix', reference).install_scoped_readers(cursor)
    cursor.readers['sgSketch'] = source_module('assemblyprofiles', reference).sketch
    cursor.readers['moAttachPoint_c'] = source_module('camerafeatures', reference).attach_point
    cursor.readers['moDimPlaneEntTag_c'] = source_module('geometricfrontier', reference).dimension_plane


def observe(reference: Path, kind: int, version: int, style: int) -> tuple[bytes, dict[str, object]]:
    """Require independent EOF, primitive coverage and slot allocation; e.g. observe(source, 2, 19000, 3)."""
    raw, slots = reference_object(kind, version, ReferenceOptions(style))
    return observe_object_body(reference, raw, slots, kind, version, f'{kind}/{style}', partial(configure, reference=reference))


def owned(reference: Path, destination: Path, kind: int, options: ReferenceOptions, label: str) -> dict[str, object]:
    """Verify complete inherited-map resolved streams independently; e.g. owned(source, output, 2, options, 'owned-two')."""
    container, prefix, features, archive = (source_module(name, reference) for name in ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, _ = saved_manager(reference, original, 0)
    probe = SavedSheetMetalDocument(original, manager, b'')
    _, context = prefix.read(probe, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    feature, slots = reference_object(kind, 19000, options, len(context.map)-1)
    raw = struct.pack('<iH', -123, 1)+feature
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(SavedSheetMetalDocument(original, manager, raw), STREAM)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'owned reference end/tail/map; expected {(len(raw), b"", slots)}')
    verify_primitive_coverage(raw, reads)
    return write_observation(destination, label, raw, [read for read in reads if read.end > read.start]) | dict(slots=slots)


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind independently assembled cases to one stable source revision; e.g. freeze(source, fixture_dir)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    cases = [observe(reference, kind, version, style) for kind in range(5) for version in VERSIONS for style in range(5)]
    if before != reference_snapshot(reference):
        raise ValueError('reference sources changed; expected stable schema snapshot')
    packet = b''.join(frame for frame, _ in cases)
    destination.mkdir(parents=True, exist_ok=True)
    (destination/'cases.bin').write_bytes(packet)
    edited, _ = reference_object(2, 19000, ReferenceOptions(3, True))
    (destination/'edited.bin').write_bytes(edited)
    saved = [owned(reference, destination, kind, ReferenceOptions(style, edited), label) for kind, style, edited, label in
        [(0, 0, False, 'owned-base'), (1, 1, False, 'owned-one'), (2, 3, False, 'owned-two'),
         (2, 3, True, 'owned-two-edited'), (3, 2, False, 'owned-handle')]]
    if before != reference_snapshot(reference):
        raise ValueError('reference sources changed during owned reads; expected stable schema snapshot')
    return dict(cases=[facts for _, facts in cases], owned=saved, reads=sum(facts['reads'] for _, facts in cases),
        sha256=hashlib.sha256(packet).hexdigest(), source_hashes=before)


if __name__ == '__main__':
    facts = freeze(REFERENCE, WORKSPACE/'tests/fixtures/reference-dimensions')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts, indent=2)+'\n', encoding='utf-8')
