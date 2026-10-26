"""Freeze independently decoded complete circular patterns and inherited streams (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
for folder in ('model-archives', 'mirror-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import observe_object_body, ReferenceObjectCursor
from circular_fixture_bytes import circular_object, LineMode
from owned_circular_fixtures import freeze_owned


def circular_variants() -> list[tuple[int, bool, bool, bool, LineMode]]:
    """Cover the geometry gate, empty maps and both line families; e.g. circular_variants()."""
    variants: list[tuple[int, bool, bool, bool, LineMode]] = [
        (version, rich, alternate, True, 'null') for version in (0x27cc, 0x3efa, 0x3efb, 19000)
        for rich in (False, True) for alternate in (False, True)]
    variants += [(19000, rich, alternate, False, 'line')
                 for rich in (False, True) for alternate in (False, True)]
    return variants+[(19000, rich, True, True, 'temporary') for rich in (False, True)]


def freeze_records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Observe full bodies without Bessemer; e.g. freeze_records(source)."""
    return [observe_object_body(reference, *circular_object(version, rich, alternate, populated, line),
        0, version, f'circular-{version:04x}-{int(rich)}-{int(alternate)}-{int(populated)}-{line}',
        lambda cursor: configure_cursor(cursor, reference))
        for version, rich, alternate, populated, line in circular_variants()]


def configure_cursor(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install the complete documented temporary-axis reader locally; e.g. configure_cursor(cursor, source)."""
    cursor.readers['moTempAxisRef_w'] = source_module('sketchprefix', reference).temporary_axis_reference


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind EOF, every primitive and final map slots; e.g. freeze(source, fixture_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    destination.mkdir(parents=True, exist_ok=True)
    records = freeze_records(reference)
    slots = b''.join(struct.pack('<I', row['slots']) for _, row in records)
    packet = b'CPF1'+struct.pack('<I', len(records))+slots+b''.join(raw for raw, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    owned = freeze_owned(reference, destination)
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source reader hashes')
    scripts = list(Path(__file__).parent.glob('*.py'))
    builders = list((WORKSPACE/'docs/features/model-archives').glob('*.py'))
    builders += [WORKSPACE/'docs/features/mirror-patterns/mirror_fixture_bytes.py',
                 WORKSPACE/'docs/features/resolved-features/freeze_owned_features.py']
    return dict(cases=[row for _, row in records], owned=owned, source_hashes=before,
                packet_sha256=hashlib.sha256(packet).hexdigest(),
                generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(scripts)},
                builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/circular-patterns')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
