"""Complete relation, equation-side and element bodies; no expression evaluation."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_component_references import base as component_base
from freeze_views import REFERENCE, WORKSPACE, object_tag, view_oracle
from freeze_blank_part import reference_snapshot
from owned_name_edit import short_wide_name
import modelrelations
from mfcobjects import ObjectArchive

ELEMENTS = ['moRelElement_c', 'moRelComment_c', 'moRelOperator_c', 'moRelValue_c', 'moRelFunction_c']
CLASSES = ELEMENTS+['moRelation_c', 'moRelEquationSide_c', 'moRelDimension_c', 'moRelGlobalVar_c', 'moCompGlobalVar_c']


def element(rich: bool) -> bytes:
    return (short_wide_name('Ω(x)') if rich else b'\x01x')+struct.pack('<4i', -2**31, 0, 1, 2**31-1)


def equation(rich: bool, parent: bool) -> bytes:
    out = object_tag('moRelEquationSide_c')+struct.pack('<H', 2 if rich else 0)
    if rich:
        out += object_tag('moRelValue_c')+element(True)+struct.pack('<H', 6 if parent else 4)
    return out+short_wide_name('Ω side')+struct.pack('<iiH', -3, 255, 2 if parent else 0)


def relation(rich: bool) -> bytes:
    out = object_tag('moRelation_c')+short_wide_name('Ω relation')+struct.pack('<Hi', 65530, -5)
    out += equation(True, True)+struct.pack('<H', 4) if rich else b'\0\0\0\0'
    return out+struct.pack('<ii', -7, 11)+b'\x01y'+struct.pack('<iii', -13, 17, -19)


def wire(name: str, version: int, rich: bool) -> bytes:
    if name == 'moRelation_c':
        return relation(rich)
    if name == 'moRelEquationSide_c':
        return equation(rich, False)
    if name == 'moCompGlobalVar_c':
        return object_tag(name)+component_base(version, rich)+struct.pack('<i', -23)
    prefix = b''
    if name == 'moRelDimension_c':
        prefix = struct.pack('<HHHHH', 2, 0, 0, 0, 0) if rich else struct.pack('<HHH', 0, 0, 0)
    if name == 'moRelGlobalVar_c':
        prefix = b'\0\0\0\0'
    return object_tag(name)+prefix+element(rich)


def freeze(version: int, raw: bytes) -> tuple[bytes, int]:
    archive = ObjectArchive(raw, version)
    archive.feature_context = 'ordinary_external_xml'
    value = archive.object()
    if archive.p != len(raw):
        raise ValueError(f'relation end {archive.p}; expected {len(raw)}')
    expected = view_oracle('root.object', value)
    return struct.pack('<BIII', 0, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    cases = [freeze(version, wire(name, version, rich))
             for name in CLASSES for version in [0x27cc, 19000] for rich in [False, True]]
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in cases)
    (WORKSPACE/'tests/fixtures/model-archives/relation-cases.bin').write_bytes(packet)
    report = {'cases': len(cases), 'fields': sum(fields for _, fields in cases),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('relation-cases.json').write_text(json.dumps(report)+'\n')
    print({key: value for key, value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
