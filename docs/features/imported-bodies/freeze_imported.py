"""Freeze independently observed complete imported and base-body features (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WORKSPACE/'docs/features/model-archives'))
from freeze_blank_part import reference_snapshot
from object_fixture_observer import observe_object_body
from imported_fixture_bytes import imported_object, ImportedClass, BodyMode
from owned_imported_fixtures import freeze_owned

CLASSES: tuple[ImportedClass, ...] = ('moImported_c', 'moBaseBody_c')
MODES: tuple[BodyMode, ...] = ('external', 'suppressed', 'compressed')
VERSIONS = (0x27cc, 0x2bcc, 0x2bcd, 19000)


def freeze_records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Observe external, suppressed and compressed variants; e.g. freeze_records(source)."""
    return [observe_object_body(reference, *imported_object(name, version, rich, mode),
        0 if name == 'moImported_c' else 1, version, f'{name}-{version:04x}-{int(rich)}-{mode}')
        for name in CLASSES for version in VERSIONS for rich in (False, True) for mode in MODES]


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind complete primitive coverage, exact EOF and slots; e.g. freeze(source, fixture_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    destination.mkdir(parents=True, exist_ok=True)
    records = freeze_records(reference)
    slots = b''.join(struct.pack('<I', row['slots']) for _, row in records)
    packet = b'IBF1'+struct.pack('<I', len(records))+slots+b''.join(raw for raw, _ in records)
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
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/imported-bodies')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
