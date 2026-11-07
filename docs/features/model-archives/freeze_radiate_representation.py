"""The exact scoped radiate-surface alias established by clause 11.115."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_entity_representations import wire
from freeze_views import REFERENCE, WORKSPACE, view_oracle
from freeze_blank_part import reference_snapshot
import modelboundary
from mfcobjects import ObjectArchive


def freeze(kind: int, version: int, rich: bool) -> tuple[bytes, int]:
    raw = wire('moRadiateSurfIdRep_c', (False, 2), rich)+(b'\x02\0' if kind == 2 else b'')
    archive = ObjectArchive(raw, version)
    archive.feature_context = 'ordinary_external_xml'
    archive.readers = dict(archive.readers, moRadiateSurfIdRep_c=modelboundary.end_face)
    if kind == 0:
        value = {'object': archive.object()}
    elif kind == 1:
        value = {'entity': modelboundary.entity_id(archive)}
    else:
        value = {'first': modelboundary.entity_id(archive), 'second': modelboundary.entity_id(archive)}
    if archive.p != len(raw):
        raise ValueError(f'radiate end {archive.p}; expected {len(raw)}')
    expected = view_oracle('root', value)
    return struct.pack('<BIII', kind, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    cases = [freeze(kind, version, rich) for version in [0x27cc, 19000] for rich in [False, True] for kind in range(3)]
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in cases)
    (WORKSPACE/'tests/fixtures/model-archives/radiate-representation-cases.bin').write_bytes(packet)
    evidence = REFERENCE/'research/radiate-surface-id-rtti.json'
    report = {'cases': len(cases), 'fields': sum(fields for _, fields in cases),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'native_identity_sha256': hashlib.sha256(evidence.read_bytes()).hexdigest(),
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('radiate-representation-cases.json').write_text(json.dumps(report)+'\n')
    print({key: value for key, value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
