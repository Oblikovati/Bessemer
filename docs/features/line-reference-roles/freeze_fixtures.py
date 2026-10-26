"""Freeze finite line-family regression payloads against independent readers (#46)."""
from pathlib import Path
from typing import Protocol
import hashlib
import json
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WORKSPACE/'docs/features/model-archives'))
from freeze_blank_part import reference_snapshot, source_module
from primitive_inventory import observe_primitives, verify_primitive_coverage
from owned_revolution_fixtures import freeze_owned

ROLES: dict[str, tuple[int, tuple[int, ...]]] = {
    'revolution': (24, (19,)), 'angle-plane': (365, (345,)),
    'curve-plane': (157, (147,)), 'sketch-pattern': (76, (60, 62)),
    'intersection': (61, (47,)),
}


class RoleCursor(Protocol):
    p: int
    map: list[object]
    feature_context: str
    def u16(self) -> int: ...


def role_wire(kind: str) -> bytes:
    """Use an independently located inherited tag; e.g. role_wire('revolution')."""
    width, offsets = ROLES[kind]
    raw = bytearray(width)
    for offset in offsets:
        raw[offset:offset+2] = b'\x01\x00'
    return bytes(raw)


def read_role(reference: Path, cursor: RoleCursor, kind: str) -> None:
    """Select the complete source body; e.g. read_role(source, cursor, 'intersection')."""
    readers = {
        'revolution': ('bodyfeatures', 'revolution_suffix'),
        'angle-plane': ('referenceplanes', 'at_angle_data'),
        'curve-plane': ('referenceplanes', 'line_point_data'),
        'sketch-pattern': ('sketchpatterns', 'linear'),
        'intersection': ('sketchintersections', 'full_body'),
    }
    if kind == 'intersection' and cursor.u16() != 0:
        raise ValueError('intersection marker; expected zero for the complete body')
    module, member = readers[kind]
    getattr(source_module(module, reference), member)(cursor)


def freeze_case(reference: Path, destination: Path, kind: str, runtime_class: str) -> dict[str, object]:
    """Bind a complete role read; e.g. freeze_case(source, output, 'revolution', 'moTempAxisRef_w')."""
    objects, archive = (source_module(name, reference) for name in ('mfcobjects', 'swarchive'))
    raw = role_wire(kind)
    cursor = objects.ObjectArchive(raw, 19000)
    cursor.feature_context = 'ordinary_external_xml'
    cursor.map.append({'_class': runtime_class})
    with observe_primitives(archive.Archive, raw) as reads:
        read_role(reference, cursor, kind)
    if cursor.p != len(raw) or len(cursor.map) != 2:
        raise ValueError(f'{kind} end/map {(cursor.p, len(cursor.map))}; expected {(len(raw), 2)}')
    reads = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, reads)
    oracle = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in reads).encode()
    (destination/(kind+'.bin')).write_bytes(raw)
    (destination/(kind+'.tsv')).write_bytes(oracle)
    return dict(kind=kind, runtime_class=runtime_class, bytes=len(raw), reads=len(reads),
                sha256=hashlib.sha256(raw).hexdigest(), oracle_sha256=hashlib.sha256(oracle).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Require source stability for both line classes; e.g. freeze(source, owned_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    destination.mkdir(parents=True, exist_ok=True)
    cases = [freeze_case(reference, destination, kind, runtime_class)
             for kind in ROLES for runtime_class in ['moLineRef_w', 'moTempAxisRef_w']]
    owned = freeze_owned(reference, destination)
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable independent reader hashes')
    return dict(cases=cases, owned=owned, source_hashes=before,
                owned_generator_sha256=hashlib.sha256((Path(__file__).parent/'owned_revolution_fixtures.py').read_bytes()).hexdigest(),
                generator_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest())


if __name__ == '__main__':
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/line-reference-roles')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
