"""Selected documented representation families with direct IDs and cyclic aliases."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_views import REFERENCE, WORKSPACE, object_tag, view_oracle
from freeze_blank_part import reference_snapshot
import modelboundary
from mfcobjects import ObjectArchive

RECIPES = {
    modelboundary.entity_representation: (False, 0),
    modelboundary.from_sketch_entity: (False, 1),
    modelboundary.end_face: (False, 2),
    modelboundary.three_integer_surface: (False, 3),
    modelboundary.mirror_pattern: (True, 0),
    modelboundary.derived_surface: (True, 1),
    modelboundary.linear_pattern: (True, 2),
    modelboundary.macro_feature_surface: (True, 3),
}


def feature_reference() -> bytes:
    return object_tag('moFR_c')+struct.pack('<HiI', 0, -2**31, 0xffffffff)


def wire(name: str, recipe: tuple[bool, int], rich: bool) -> bytes:
    source, words = recipe
    out = object_tag(name)+struct.pack('<H', 2 if rich else 0)
    out += feature_reference() if rich else b'\0\0'
    if source:
        out += struct.pack('<H', 2 if rich else 0)
    return out+struct.pack('<'+'i'*words, *[-3, 5, -7][:words])


def freeze(kind: int, version: int, raw: bytes) -> tuple[bytes, int]:
    archive = ObjectArchive(raw, version)
    archive.feature_context = 'ordinary_external_xml'
    if kind == 0:
        value = {'object': archive.object()}
    elif kind == 1:
        value = {'entity': modelboundary.entity_id(archive)}
    else:
        value = {'first': modelboundary.entity_id(archive), 'second': modelboundary.entity_id(archive)}
    if archive.p != len(raw):
        raise ValueError(f'representation end {archive.p}; expected {len(raw)}')
    expected = view_oracle('root', value)
    return struct.pack('<BIII', kind, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    classes = {name: RECIPES[reader] for name, reader in ObjectArchive.readers.items() if reader in RECIPES}
    if len(classes) != 40:
        raise ValueError(f'representation classes {len(classes)}; expected reviewed 40-class snapshot')
    cases = [freeze(kind, version, wire(name, recipe, rich)+(b'\x02\0' if kind == 2 else b''))
             for name, recipe in sorted(classes.items()) for version in [0x27cc, 19000]
             for rich in [False, True] for kind in range(3)]
    cases += [freeze(0, version, feature_reference()) for version in [0x27cc, 19000]]
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in cases)
    (WORKSPACE/'tests/fixtures/model-archives/entity-representation-cases.bin').write_bytes(packet)
    report = {'cases': len(cases), 'fields': sum(fields for _, fields in cases), 'classes': sorted(classes),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('entity-representation-cases.json').write_text(json.dumps(report)+'\n')
    print({key: value for key, value in report.items() if key not in ('source_hashes', 'classes')})


if __name__ == '__main__':
    generate()
