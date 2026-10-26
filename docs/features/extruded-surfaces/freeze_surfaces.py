"""Freeze complete extruded-surface, ordered-helper and keeper observations (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
for folder in ('model-archives', 'shell-features'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot
from object_fixture_observer import observe_object_body
from surface_fixture_bytes import surface_object, ordered_object, keeper_object, SurfaceMode
from owned_surface_fixtures import freeze_owned

MODES: tuple[SurfaceMode, ...] = ('null', 'empty', 'populated')
VERSIONS = (0x27cc, 0x3b00, 0x3b01, 19000)


def freeze_records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Observe 52 features, eight helpers and two empty keepers; e.g. freeze_records(source)."""
    records = [observe_object_body(reference, *surface_object(version, rich, mode, keeper), 0, version,
        f'surface-{version:04x}-{int(rich)}-{mode}-{int(keeper)}')
        for version in VERSIONS for rich in (False, True) for mode in MODES for keeper in (False, True)]
    for version in (0x27cc, 19000):
        records += [observe_object_body(reference, *ordered_object(version, rich, escape), 1, version,
            f'ordered-{version:04x}-{int(rich)}-{int(escape)}')
            for rich in (False, True) for escape in (False, True)]
        records.append(observe_object_body(reference, *keeper_object(), 2, version, f'keeper-{version:04x}'))
    return records+[observe_object_body(reference,
        *surface_object(version, rich, 'populated', True, header_keeper=True), 0, version,
        f'surface-header-keeper-{version:04x}-{int(rich)}')
        for version in (0x27cc, 19000) for rich in (False, True)]


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Require exact EOF, slots and complete byte coverage; e.g. freeze(source, fixture_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    destination.mkdir(parents=True, exist_ok=True)
    records = freeze_records(reference)
    slots = b''.join(struct.pack('<I', row['slots']) for _, row in records)
    packet = b'ESF1'+struct.pack('<I', len(records))+slots+b''.join(raw for raw, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    owned = freeze_owned(reference, destination)
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source reader hashes')
    scripts = list(Path(__file__).parent.glob('*.py'))
    builders = list((WORKSPACE/'docs/features/model-archives').glob('*.py'))
    builders += [WORKSPACE/'docs/features/shell-features/shell_fixture_bytes.py',
        WORKSPACE/'docs/features/circular-patterns/owned_circular_fixtures.py',
        WORKSPACE/'docs/features/circular-patterns/circular_fixture_bytes.py',
        WORKSPACE/'docs/features/resolved-features/freeze_owned_features.py']
    return dict(cases=[row for _, row in records], owned=owned, source_hashes=before,
                packet_sha256=hashlib.sha256(packet).hexdigest(),
                generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(scripts)},
                builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/extruded-surfaces')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
