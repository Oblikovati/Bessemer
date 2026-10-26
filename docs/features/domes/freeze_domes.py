"""Freeze complete dome fields, pointer aliases and independent edits (#54)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
for folder in ('model-archives', 'sweeps', 'resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import ReferenceObjectCursor, object_cursor, observe_object_body
from primitive_inventory import observe_primitives
from freeze_owned_features import ReplacedFeatureDocument, STREAM
from owned_circular_fixtures import write_observation
from dome_fixture_bytes import dome_object

VERSIONS = (0x27cc, 0x3b00, 0x3b01, 19000)


def verify_dome(root: dict[str, object], rich: bool, mask: int, edited: bool = False) -> None:
    """Retain independent presence, shared references and cycles; e.g. verify_dome(root, True, 15)."""
    payload = cast(dict[str, object], root['payload'])
    for key, bit in [('dimension_850', 1), ('faces_838', 2), ('reference_840', 4), ('reference_848', 8)]:
        if (payload[key] is not None) != bool(mask & bit):
            raise ValueError(f'dome pointer {key}; expected presence {bool(mask & bit)}')
    if mask & 12 == 12 and payload['reference_840'] is not payload['reference_848']:
        raise ValueError('dome wrapper identities; expected one shared wrapper')
    if mask & 2:
        items = cast(dict[str, list[object]], payload['faces_838'])['items']
        if len(items) != (3 if rich else 0):
            raise ValueError(f'dome list length {len(items)}; expected {3 if rich else 0}')
        if rich:
            verify_list(root, payload, items, mask)
    if payload['raw_860'] != (12345 if edited else -1):
        raise ValueError(f'dome final word {payload["raw_860"]}; expected {12345 if edited else -1}')


def verify_list(root: dict[str, object], payload: dict[str, object], items: list[object], mask: int) -> None:
    """Verify root self references and cross-field aliases; e.g. verify_list(root, payload, items, 15)."""
    if items[1] is not root or items[2] is not None:
        raise ValueError('dome list identities; expected root self reference and null')
    for key, bit in [('reference_840', 4), ('reference_848', 8)]:
        if mask & bit and items[0] is not payload[key]:
            raise ValueError(f'dome list/reference {key}; expected one shared wrapper')


def freeze_records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Observe four versions and every nullable combination; e.g. freeze_records(source)."""
    records: list[tuple[bytes, dict[str, object]]] = []
    for version in VERSIONS:
        for rich, mask in ((rich, mask) for rich in (False, True) for mask in range(16)):
            raw, slots = dome_object(version, rich, mask)
            cursor = object_cursor(reference, raw, version, None)
            verify_dome(cast(dict[str, object], cursor.object()), rich, mask)
            records.append(observe_object_body(reference, raw, slots, mask, version,
                f'dome-{version:04x}-{int(rich)}-{mask}'))
    return records


def freeze_owned_case(reference: Path, destination: Path, rich: bool, mask: int, edited: bool) -> dict[str, object]:
    """Require actual imported context and complete saved-state framing; e.g. freeze_owned_case(source, output, True, 15, False)."""
    container, prefix, features, archive = (source_module(name, reference) for name in
        ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, imported = prefix.read(document, 'Contents/Config-0', through_part_resolved=True)
    context = cast(ReferenceObjectCursor, imported)
    feature, slots = dome_object(19000, rich, mask, len(context.map)-1, edited)
    raw = struct.pack('<iH', -123, 1)+feature+bytes(4)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(ReplacedFeatureDocument(document, raw), STREAM)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'dome stream end/tail/map; expected {(len(raw), b"", slots)}')
    verify_dome(parsed['items'][0], rich, mask, edited)
    label = f'owned-{int(rich)}-{mask}'+('-edited' if edited else '')
    return write_observation(destination, label, raw, reads) | dict(slots=slots,
        imported_slots=len(context.map), baseline_sha256=hashlib.sha256(baseline).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Keep source readers stable across complete fixture observation; e.g. freeze(source, output)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    records = freeze_records(reference)
    packet = b'DOF1'+struct.pack('<I', len(records))
    packet += b''.join(struct.pack('<I', row['slots']) for _, row in records)+b''.join(raw for raw, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    owned = [freeze_owned_case(reference, destination, rich, mask, edited) for rich, mask, edited in
        [(False, 0, False), (False, 15, False), (True, 15, False), (True, 15, True)]]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    scripts = sorted(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[row for _, row in records], owned=owned, source_hashes=before,
        packet_sha256=hashlib.sha256(packet).hexdigest(),
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in scripts},
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/domes')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
