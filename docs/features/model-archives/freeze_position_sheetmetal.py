"""Position-group direction arrays and complete older sheet-metal bridge bodies."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_environment import feature, short_wide_name
from freeze_component_references import base as component_base
from freeze_views import REFERENCE, WORKSPACE, object_tag, view_oracle
from freeze_blank_part import reference_snapshot
import modelfeatures
import modelcomponents
import modelreftail
from mfcobjects import ObjectArchive


def position(version: int, count: int, extended: bool) -> bytes:
    out = object_tag('moPosGroupFolder_c')+feature(version)+struct.pack('<i', -3)+short_wide_name('Ω position')
    out += struct.pack('<H', count)+struct.pack('<'+'H'*count, *[65535, 0, 65534][:count])
    out += struct.pack('<'+'H'*count, *[65533, 2, 65532][:count])
    out += (struct.pack('<HI', 65535, 2) if extended else struct.pack('<H', 2))+struct.pack('<HHii', 0, 2, -5, 7)
    if version >= 0x3eeb:
        out += bytes([0, 255, 254, 253])
    return out


def component_feature(name: str, version: int) -> bytes:
    return object_tag(name)+component_base(version, False)+struct.pack('<iI', -11, 0xffffffff)


def sheet_metal(version: int, alias: bool) -> bytes:
    out = object_tag('moPerBodySheetMetalData_c')+component_feature('moCompFeature_c', version)+b'\0\0'
    out += struct.pack('<H', 4) if alias else component_feature('moCompBodyFeature_c', version)
    return out+b'\0\0'+struct.pack('<ii', -13, 17)


def freeze(version: int, raw: bytes) -> tuple[bytes, int]:
    archive = ObjectArchive(raw, version)
    archive.feature_context = 'ordinary_external_xml'
    archive.readers = dict(archive.readers, moPerBodySheetMetalData_c=modelreftail.per_body_sheet_metal)
    value = archive.object()
    if archive.p != len(raw):
        raise ValueError(f'position/sheet-metal end {archive.p}; expected {len(raw)}')
    expected = view_oracle('root.object', value)
    return struct.pack('<BIII', 0, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    versions = [0x27cc, 0x3eea, 0x3eeb, 19000]
    cases = [freeze(version, position(version, count, extended))
             for version in versions for count in range(4) for extended in [False, True]]
    cases += [freeze(version, sheet_metal(version, alias)) for version in [0x27cc, 19000] for alias in [False, True]]
    cases += [freeze(version, component_feature(name, version))
              for version in [0x27cc, 19000] for name in ['moCompFeature_c', 'moCompBodyFeature_c']]
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in cases)
    (WORKSPACE/'tests/fixtures/model-archives/position-sheetmetal-cases.bin').write_bytes(packet)
    report = {'cases': len(cases), 'fields': sum(fields for _, fields in cases),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('position-sheetmetal-cases.json').write_text(json.dumps(report)+'\n')
    print({key: value for key, value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
