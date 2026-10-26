"""Complete ordinary boundary and direct entity-ID records with aliases."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_views import REFERENCE, WORKSPACE, object_tag, view_oracle
from freeze_blank_part import reference_snapshot
import modelboundary
from mfcobjects import ObjectArchive

CLASSES = ['moBEntRef_c', 'moEdgeRef_c', 'moFaceRef_c', 'moVertexRef_c', 'moEntityId_c', 'moEntityIdRep_c']


def representation(rich: bool) -> bytes:
    return struct.pack('<HH', 2 if rich else 0, 0)


def base(version: int, rich: bool) -> bytes:
    out = struct.pack('<5I2dH', 0xffffffff, 0x89abcdef, 2 if rich else 0, 3, 5, -0.0, 1.25, 0)
    if rich:
        out += object_tag('moEntityIdRep_c')+b'\0\0\0\0'+struct.pack('<H', 4)
    out += struct.pack('<Ii', 0xfedcba98, 0)
    if version > 0x332c:
        out += struct.pack('<ii', -7, 0)
    return out


def wire(name: str, version: int, rich: bool) -> bytes:
    if name == 'moEntityIdRep_c':
        return object_tag(name)+representation(rich)
    if name == 'moEntityId_c':
        return object_tag(name)+object_tag('moEntityIdRep_c')+b'\0\0\0\0'
    out = object_tag(name)+base(version, rich)
    if name == 'moFaceRef_c':
        out += struct.pack('<i', 2)+struct.pack('<iIIi', -2**31, 0xffffffff, 0x89abcdef, 2**31-1)*2
    if name == 'moVertexRef_c':
        out += struct.pack('<i', 255 if rich else 0)+(struct.pack('<3d', -0.0, 1.25, -2.5) if rich else b'')
    return out


def freeze(raw: bytes, version: int) -> tuple[bytes, int]:
    archive = ObjectArchive(raw, version)
    archive.feature_context = 'ordinary_external_xml'
    value = archive.object()
    if archive.p != len(raw):
        raise ValueError(f'boundary end {archive.p}; expected {len(raw)}')
    expected = view_oracle('root.object', value)
    return struct.pack('<BIII', 0, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    cases = [freeze(wire(name, version, rich), version)
             for name in CLASSES for version in [0x27cc, 0x332c, 0x332d, 19000] for rich in [False, True]]
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in cases)
    (WORKSPACE/'tests/fixtures/model-archives/boundary-reference-cases.bin').write_bytes(packet)
    report = {'cases': len(cases), 'fields': sum(fields for _, fields in cases),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('boundary-reference-cases.json').write_text(json.dumps(report)+'\n')
    print({key: value for key, value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
