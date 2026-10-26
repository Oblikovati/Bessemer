"""Ordinary component reference, boundary, edge and body records."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_views import REFERENCE, WORKSPACE, object_tag, transform, view_oracle
from freeze_blank_part import reference_snapshot
from owned_name_edit import short_wide_name
import modelcomponents
import partlightweight
from mfcobjects import ObjectArchive

CLASSES = ['moCompRef_c', 'moCompFace_c', 'moCompVertex_c', 'moCompEdge_c',
           'moCompLoop_c', 'moCompSilhEdge_c', 'moCompAtomBody_c',
           'moCompSolidBody_c', 'moCompSurfaceBody_c']


def base(version: int, rich: bool) -> bytes:
    out = struct.pack('<H7iiI5i', 2 if rich else 0, -3, 5, -7, 11, -13, 17, 0, -19, 0xfedcba98, -23, 29, -31, 37, -41)
    out += struct.pack('<i', -1 if rich else 0)+(b'\0\0' if rich else b'')
    out += struct.pack('<i', 255 if rich else 0)+(short_wide_name('Ω component') if rich else b'')
    if version > 0x2b60:
        out += struct.pack('<i', 0)
    if version > 0x2bb8:
        out += struct.pack('<i', -1 if rich else 0)+(transform(False, 0x2b) if rich else b'')
    if version > 0x2f55:
        out += struct.pack('<i', 255 if rich else 0)+(b'\x01x' if rich else b'')
    if version >= 0x3ef6:
        out += struct.pack('<I', 0x89abcdef)
    return out


def boundary(version: int, rich: bool) -> bytes:
    return base(version, rich)+bytes([255 if rich else 0])+(short_wide_name('Ω alias') if rich else b'')+struct.pack('<HH', 2 if rich else 0, 0)


def edge(version: int, rich: bool, flags: int) -> bytes:
    out = boundary(version, rich)+struct.pack('<i6dIiHH', flags, -0.0, 1, 2, 3, 4, 5, 0xffffffff, 2, 0, 2)
    if flags & 6 != 6 and flags & 8:
        out += struct.pack('<HH', 2, 0)
    return out


def wire(name: str, version: int, rich: bool, flags: int) -> bytes:
    if name == 'moCompRef_c':
        body = base(version, rich)
    elif name in ('moCompFace_c', 'moCompVertex_c'):
        body = boundary(version, rich)
    elif name in ('moCompEdge_c', 'moCompSilhEdge_c'):
        body = edge(version, rich, flags)
        if name == 'moCompSilhEdge_c':
            body += struct.pack('<i', 255 if rich else 0)+(transform(True, 7) if rich else b'')
    elif name == 'moCompLoop_c':
        body = boundary(version, rich)+b'\0\0'
    else:
        body = base(version, rich)+struct.pack('<Hi3i3i', 0, 3, -2**31, 0, 2**31-1, -43, 47, -53)
    return object_tag(name)+body


def freeze(raw: bytes, version: int) -> tuple[bytes, int]:
    archive = ObjectArchive(raw, version)
    archive.feature_context = 'ordinary_external_xml'
    value = archive.object()
    if archive.p != len(raw):
        raise ValueError(f'component end {archive.p}; expected {len(raw)}')
    expected = view_oracle('root.object', value)
    return struct.pack('<BIII', 0, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    versions = [0x27cc, 0x2b60, 0x2b61, 0x2bb8, 0x2bb9, 0x2f55, 0x2f56, 0x3ef5, 0x3ef6, 19000]
    cases = [freeze(wire(name, version, rich, 8), version)
             for name in CLASSES for version in versions for rich in [False, True]]
    cases += [freeze(wire(name, 19000, True, flags), 19000)
              for name in ['moCompEdge_c', 'moCompSilhEdge_c'] for flags in [0, 6, 14, -1]]
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in cases)
    (WORKSPACE/'tests/fixtures/model-archives/component-reference-cases.bin').write_bytes(packet)
    report = {'cases': len(cases), 'fields': sum(fields for _, fields in cases),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('component-reference-cases.json').write_text(json.dumps(report)+'\n')
    print({key: value for key, value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
