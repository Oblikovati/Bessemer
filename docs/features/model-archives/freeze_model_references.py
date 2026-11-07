"""Ordinary wrappers shared by relation, geometric and component references."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_component_references import base as component_base
from freeze_views import REFERENCE, WORKSPACE, object_tag, transform, view_oracle
from freeze_blank_part import reference_snapshot
from owned_name_edit import short_wide_name
import modelrelations
from mfcobjects import ObjectArchive

CLASSES = ['moRefWrapper_c', 'moDimRefWrapper_c', 'moGlobalVarRefWrapper_c', 'moCompXFormWrapper_c']


def wrapper(version: int, rich: bool) -> bytes:
    out = object_tag('moCompGlobalVar_c')+component_base(version, False)+struct.pack('<i', -3) if rich else b'\0\0'
    out += struct.pack('<H', 65530)
    out += object_tag('moUnitComponent_c')+b'\0\0\0' if rich else b'\0\0'
    out += struct.pack('<ii', -5, 7)
    for matrix, flags in [(False, 0x2b), (True, 0x4c)]:
        out += struct.pack('<H', 65535 if rich else 0)+(transform(matrix, flags) if rich else b'')
    out += struct.pack('<4i', -11, 13, -17, 19)
    if version > 0x2faf:
        out += struct.pack('<i', -23)
    out += struct.pack('<4i', 29, -31, 37, -41)
    if version >= 0x2b73:
        out += struct.pack('<i', 43)
    if version >= 0x3b02:
        out += struct.pack('<i', -47)
    return out


def wire(name: str, version: int, rich: bool) -> bytes:
    out = object_tag(name)
    if name == 'moDimRefWrapper_c':
        out += struct.pack('<d', -0.0)
    out += wrapper(version, rich)
    if name == 'moGlobalVarRefWrapper_c':
        out += short_wide_name('Ω global')
    if name == 'moCompXFormWrapper_c':
        out += transform(True, 0x1a)+transform(False, 7)
    return out


def freeze(version: int, raw: bytes) -> tuple[bytes, int]:
    archive = ObjectArchive(raw, version)
    archive.feature_context = 'ordinary_external_xml'
    value = archive.object()
    if archive.p != len(raw):
        raise ValueError(f'wrapper end {archive.p}; expected {len(raw)}')
    expected = view_oracle('root.object', value)
    return struct.pack('<BIII', 0, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    versions = [0x27cc, 0x2b72, 0x2b73, 0x2faf, 0x2fb0, 0x3b01, 0x3b02, 19000]
    cases = [freeze(version, wire(name, version, rich)) for name in CLASSES for version in versions for rich in [False, True]]
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in cases)
    (WORKSPACE/'tests/fixtures/model-archives/model-reference-cases.bin').write_bytes(packet)
    report = {'cases': len(cases), 'fields': sum(fields for _, fields in cases),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('model-reference-cases.json').write_text(json.dumps(report)+'\n')
    print({key: value for key, value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
