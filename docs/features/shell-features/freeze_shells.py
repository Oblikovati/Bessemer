"""Freeze independently decoded complete ordinary part shells (#54)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WORKSPACE/'docs/features/model-archives'))
sys.path.insert(0, str(WORKSPACE/'docs/features/mirror-patterns'))
from freeze_blank_part import reference_snapshot
from freeze_fixtures import freeze_record
from shell_fixture_bytes import shell_object, ShellMode
from owned_shell_fixtures import freeze_owned


def shell_variants() -> list[tuple[int, bool, ShellMode, bool]]:
    """Cover both supported bounds and concrete edge families; e.g. shell_variants()."""
    modes = ('null', 'shared', 'unequal', 'populated')
    return [(version, rich, cast(ShellMode, mode), False)
            for version in (0x27cc, 19000) for rich in (False, True) for mode in modes] + [
            (19000, rich, 'populated', True) for rich in (False, True)]


def freeze_records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Observe each complete object independently; e.g. freeze_records(source)."""
    return [freeze_record(reference, *shell_object(version, rich, mode, silhouette=silh),
            0, version, f'shell-{version:04x}-{int(rich)}-{mode}-{int(silh)}')
            for version, rich, mode, silh in shell_variants()]


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind primitive coverage, exact EOF and final slots; e.g. freeze(source, fixture_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    destination.mkdir(parents=True, exist_ok=True)
    records = freeze_records(reference)
    slots = b''.join(struct.pack('<I', row['slots']) for _, row in records)
    packet = b'SHF1'+struct.pack('<I', len(records))+slots+b''.join(raw for raw, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    owned = freeze_owned(reference, destination)
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source reader hashes')
    scripts = list(Path(__file__).parent.glob('*.py'))
    builders = list((WORKSPACE/'docs/features/model-archives').glob('*.py'))
    builders.append(WORKSPACE/'docs/features/mirror-patterns/freeze_fixtures.py')
    builders.append(WORKSPACE/'docs/features/resolved-features/freeze_owned_features.py')
    return dict(cases=[row for _, row in records], owned=owned, source_hashes=before,
                packet_sha256=hashlib.sha256(packet).hexdigest(),
                generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(scripts)},
                builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/shell-features')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
