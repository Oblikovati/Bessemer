"""Freeze independently observed complete Hole Wizard and slot-axis bodies (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WORKSPACE/'docs/features/model-archives'))
from freeze_blank_part import reference_snapshot
from object_fixture_observer import observe_object_body
from hole_fixture_bytes import hole_object, slot_object
from owned_hole_fixtures import freeze_owned

VERSIONS = (0x27cc, 0x2b5d, 0x2b5e, 0x2f45, 0x2f46, 0x2f47, 0x2f48,
            0x2f67, 0x2f68, 0x332e, 0x332f, 0x3355, 0x3356, 0x3f02, 0x3f03,
            0x42e9, 0x42ea, 19000)


def freeze_records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Observe all gate and direct/tagged cases; e.g. freeze_records(source)."""
    records = [observe_object_body(reference, *hole_object(version, rich, slot), 0, version,
        f'hole-{version:04x}-{int(rich)}-{int(slot)}')
        for version in VERSIONS for rich in (False, True) for slot in (False, True)]
    return records+[observe_object_body(reference, *slot_object(), 1, version,
        f'slot-{version:04x}') for version in (0x27cc, 19000)]


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind exact EOF, final slots and primitive coverage; e.g. freeze(source, fixture_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    destination.mkdir(parents=True, exist_ok=True)
    records = freeze_records(reference)
    slots = b''.join(struct.pack('<I', row['slots']) for _, row in records)
    packet = b'HWF1'+struct.pack('<I', len(records))+slots+b''.join(raw for raw, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    owned = freeze_owned(reference, destination)
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source reader hashes')
    scripts = list(Path(__file__).parent.glob('*.py'))
    builders = list((WORKSPACE/'docs/features/model-archives').glob('*.py'))
    builders += [WORKSPACE/'docs/features/circular-patterns/owned_circular_fixtures.py',
        WORKSPACE/'docs/features/circular-patterns/circular_fixture_bytes.py',
        WORKSPACE/'docs/features/resolved-features/freeze_owned_features.py']
    return dict(cases=[row for _, row in records], owned=owned, source_hashes=before,
                packet_sha256=hashlib.sha256(packet).hexdigest(),
                generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(scripts)},
                builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/hole-wizard')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
