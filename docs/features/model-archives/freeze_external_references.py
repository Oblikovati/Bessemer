"""Model external-file bodies, sharing and modern header-schema boundaries."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_views import REFERENCE, WORKSPACE, object_tag, view_oracle
from freeze_blank_part import reference_snapshot
from owned_name_edit import short_wide_name
import modelheader
from mfcobjects import ObjectArchive


def names(mode: int) -> bytes:
    if mode == 0:
        return b'\0\0\0\0'
    path = object_tag('moCStringHandle_c')+short_wide_name('Ω/path')
    return path+(struct.pack('<H', 4) if mode == 2 else struct.pack('<H', 0x8003)+b'\x01x')


def wire(version: int, mode: int) -> bytes:
    out = object_tag('moExtObject_c')+names(mode)+struct.pack('<HBI', 65530, 255, 0xffffffff)
    out += short_wide_name('Ω shared')+b'\x01i'
    if version > 0x2f82:
        out += b'\x01p'
    if version >= 0x3330:
        out += b'\xfe'
    out += struct.pack('<BI3i', 253, 0x89abcdef, -3, 5, -7)+short_wide_name('Ω config')
    out += struct.pack('<iiI', -2**31, 11, 0xfedcba98)
    if version > 0x3f12:
        out += struct.pack('<i', -13)
    return out


def freeze(version: int, mode: int) -> tuple[bytes, int]:
    raw = wire(version, mode)
    archive = ObjectArchive(raw, version)
    archive.feature_context = 'ordinary_external_xml'
    value = archive.object()
    if archive.p != len(raw):
        raise ValueError(f'external end {archive.p}; expected {len(raw)}')
    expected = view_oracle('root.object', value)
    return struct.pack('<BIII', 0, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    cases = [freeze(version, mode) for version in [0x27cc, 0x2f82, 0x2f83, 0x332f, 0x3330, 0x3f12, 0x3f13, 19000] for mode in range(3)]
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in cases)
    (WORKSPACE/'tests/fixtures/model-archives/external-reference-cases.bin').write_bytes(packet)
    report = {'cases': len(cases), 'fields': sum(fields for _, fields in cases),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('external-reference-cases.json').write_text(json.dumps(report)+'\n')
    print({key: value for key, value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
