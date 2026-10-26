"""Freeze complete mirror and plane-reference bodies from independent source reads (#54)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WORKSPACE/'docs/features/model-archives'))
from freeze_blank_part import reference_snapshot, source_module
from primitive_inventory import observe_primitives, verify_primitive_coverage
from mirror_fixture_bytes import mirror_object, plane_object, PlaneMode
from owned_mirror_fixtures import freeze_owned

VERSIONS = (0x27cc, 0x2f86, 0x2f87, 0x2f93, 0x2f94, 0x3745,
            0x3746, 0x42ce, 0x42cf, 0x42db, 0x42dc, 19000)


def mirror_variants() -> list[tuple[int, bool, bool, PlaneMode]]:
    """Keep distinct gate and identity cases; e.g. mirror_variants()."""
    cases = [(version, False, tagged, cast(PlaneMode, 'null'))
             for version in VERSIONS for tagged in ([False, True] if version > 0x3745 else [False])]
    cases += [(19000, True, tagged, 'null') for tagged in [False, True]]
    cases += [(19000, rich, True, 'shared') for rich in [False, True]]
    cases += [(19000, rich, tagged, 'direct') for rich in [False, True] for tagged in [False, True]]
    return cases


def freeze_record(reference: Path, raw: bytes, slots: int, kind: int, version: int,
                  label: str) -> tuple[bytes, dict[str, object]]:
    """Bind EOF, slots and primitive coverage; e.g. freeze_record(source, raw, 3, 1, 19000, 'plane')."""
    objects, archive, bodies = (source_module(name, reference) for name in
                               ('mfcobjects', 'swarchive', 'bodyfeatures'))
    cursor = objects.ObjectArchive(raw, version)
    cursor.feature_context = 'ordinary_external_xml'
    cursor.model_root_class = 'moPart_c'
    bodies.install_scoped(cursor, overwrite=True)
    with observe_primitives(archive.Archive, raw) as reads:
        cursor.object()
    if cursor.p != len(raw) or len(cursor.map) != slots:
        raise ValueError(f'{label} end/map {(cursor.p, len(cursor.map))}; expected {(len(raw), slots)}')
    reads = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, reads)
    oracle = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in reads).encode()
    frame = struct.pack('<BIII', kind, version, len(raw), len(oracle))+raw+oracle
    return frame, dict(label=label, kind=kind, version=version, bytes=len(raw), slots=slots, reads=len(reads),
        sha256=hashlib.sha256(raw).hexdigest(), oracle_sha256=hashlib.sha256(oracle).hexdigest())


def freeze_records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Observe full class bodies without Bessemer; e.g. freeze_records(source)."""
    records = [freeze_record(reference, *mirror_object(version, rich, tagged, mode), 0, version,
               f'mirror-{version:04x}-{int(rich)}-{int(tagged)}-{mode}')
               for version, rich, tagged, mode in mirror_variants()]
    records += [freeze_record(reference, *plane_object(19000, face), 1, 19000,
                f'plane-{int(face)}') for face in [False, True]]
    return records


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Reject reference changes during generation; e.g. freeze(source, owned_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    destination.mkdir(parents=True, exist_ok=True)
    records = freeze_records(reference)
    owned = freeze_owned(reference, destination)
    slots = b''.join(struct.pack('<I', row['slots']) for _, row in records)
    packet = b'MPF1'+struct.pack('<I', len(records))+slots+b''.join(raw for raw, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source reader hashes')
    scripts = list(Path(__file__).parent.glob('*.py'))
    builders = list((WORKSPACE/'docs/features/model-archives').glob('*.py'))
    return dict(cases=[row for _, row in records], owned=owned, source_hashes=before,
        packet_sha256=hashlib.sha256(packet).hexdigest(),
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(scripts)},
        builder_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/mirror-patterns')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
