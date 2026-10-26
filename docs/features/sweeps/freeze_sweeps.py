"""Freeze complete sweep and dependency bodies against independent scoped readers (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WORKSPACE/'docs/features/model-archives'))
from freeze_blank_part import reference_snapshot
from object_fixture_observer import observe_object_body
from sweep_fixture_bytes import sweep_object, helper_object, SweepOptions
from owned_sweep_fixtures import freeze_owned

VERSIONS = (0x27cc, 0x27d9, 0x27da, 19000)
OPTIONS = (SweepOptions(), SweepOptions(255, 2, 1, 8, True, False),
    SweepOptions(1, 3, -1, 9, False, True), SweepOptions(255, 3, -2147483648, 8, True, True),
    SweepOptions(0, 2, -1, 9, True, False), SweepOptions(1, 2, 0, 7, False, True),
    SweepOptions(255, 3, 0, 7, True, True), SweepOptions(0, 3, 0, 8))
HELPERS = ('moWireContact_c', 'moEndPointRef_w', 'moLoftSynchRef_c', 'moCompProfile_c')


def freeze_records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Cover all independent predicates and complete nested helpers; e.g. freeze_records(source)."""
    records = [observe_object_body(reference, *sweep_object(version, rich, options), 0, version,
        f'sweep-{version:04x}-{int(rich)}-{index}')
        for version in VERSIONS for rich in (False, True) for index, options in enumerate(OPTIONS)]
    records += [observe_object_body(reference, *helper_object(name, version, rich), kind, version,
        f'{name}-{version:04x}-{int(rich)}') for kind, name in enumerate(HELPERS, 1)
        for version in (0x27cc, 0x2b73, 0x2fb0, 19000) for rich in (False, True)]
    return records+[observe_object_body(reference,
        *sweep_object(version, rich, options, name='moSweepCut_c'), 5, version,
        f'sweep-cut-{version:04x}-{int(rich)}-{index}')
        for version in VERSIONS for rich in (False, True) for index, options in enumerate(OPTIONS)]


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Require exact EOF, slots and primitive coverage; e.g. freeze(source, fixture_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    records = freeze_records(reference)
    slots = b''.join(struct.pack('<I', row['slots']) for _, row in records)
    packet = b'SWF1'+struct.pack('<I', len(records))+slots+b''.join(raw for raw, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    owned = freeze_owned(reference, destination)
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source reader hashes')
    scripts = list(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[row for _, row in records], owned=owned, source_hashes=before,
                packet_sha256=hashlib.sha256(packet).hexdigest(),
                generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(scripts)},
                builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/sweeps')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
