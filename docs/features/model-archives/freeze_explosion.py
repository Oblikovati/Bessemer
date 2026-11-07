"""Owned explosion records checked against the approved scoped reference reader."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_environment import feature
from freeze_component_references import wire as component_wire, base as component_base
from freeze_node_units import node_wire
from freeze_views import REFERENCE, WORKSPACE, object_tag, transform, view_oracle
from freeze_blank_part import reference_snapshot
import partresolved
import bodyfeatures
import modelheader
import modelcomponents
from mfcobjects import ObjectArchive


def objects(extended: bool = False) -> bytes:
    count = struct.pack('<HI', 65535, 2) if extended else struct.pack('<H', 2)
    return count+struct.pack('<HH', 0, 2)


def direction(flags: int) -> bytes:
    out = struct.pack('<i', -1 if flags & 1 else 0)
    if flags & 1:
        out += objects(True)
    out += struct.pack('<iB', -3, 255 if flags & 2 else 0)
    if flags & 2:
        out += struct.pack('<3dB', -0.0, 1.25, -2.5, 254 if flags & 4 else 0)
        if flags & 4:
            out += struct.pack('<3d', 3.5, -4.75, 5)
        out += bytes([253 if flags & 8 else 0])
        if flags & 8:
            out += struct.pack('<3d', -6, 7.25, -8.5)
    return object_tag('moDirectionSpec_c')+out+struct.pack('<i', 11)


def step(version: int, rich: bool) -> bytes:
    out = feature(version, rich)+direction(15 if rich else 0)+struct.pack('<d', -0.0)
    out += object_tag('suObArray')+objects(rich)+struct.pack('<ii3d', -3, 5, -0.0, 1.25, -2.5)
    out += transform(rich, 0x1a if rich else 7)
    out += (struct.pack('<HI', 65535, 3) if rich else struct.pack('<H', 3))
    return out+struct.pack('<3I', 0, 0xffffffff, 0x89abcdef)


def exploder(version: int, rich: bool, word_360: int = 11) -> bytes:
    out = feature(version, rich)+struct.pack('<iii', -7, word_360, -13)
    for _ in range(2):
        out += object_tag('suObArray')+objects(rich)
    out += struct.pack('<d', -0.0)+b'\0\0'+struct.pack('<10d', *[index/4 for index in range(10)])
    return object_tag('moPrtExploder_c')+out+objects(rich)


def wire(name: str, version: int, rich: bool) -> bytes:
    if name == 'moPrtExploder_c':
        return exploder(version, rich)
    if name == 'moBodyXform_c':
        return object_tag(name)+transform(rich, 7)+component_wire('moCompSolidBody_c', version, False, 0)
    if name == 'moPrtExplodeLine_c':
        return line(version, rich)
    out = object_tag(name)+step(version, rich)
    if name == 'moBodyExplodeStep_c':
        out += component_wire('moCompSolidBody_c', version, False, 0)+b'\0\0'
    if name == 'moPrtChainExplodeStep_c':
        out += objects(rich)+struct.pack('<i', -17)
    return out


def line(version: int, rich: bool) -> bytes:
    out = object_tag('moPrtExplodeLine_c')+node_wire(version)
    out += object_tag('moPrtExplodeStep_c')+step(version, rich)
    return out+struct.pack('<8d3iHH', -0.0, 1, 2, 3, 4, 5, -6, 7, -19, 23, -29, 0, 0)


def readers() -> dict[str, object]:
    return dict(moPrtExploder_c=partresolved.exploder, moPrtExplodeStep_c=partresolved.explode_step,
                moBodyExplodeStep_c=partresolved.body_explode_step, moPrtChainExplodeStep_c=partresolved.chain_explode_step,
                moPrtExplodeLine_c=partresolved.explode_line, moBodyXform_c=partresolved.body_transform,
                moDirectionSpec_c=bodyfeatures.direction_spec, suObArray=modelheader.object_list,
                moCompHolder_c=modelcomponents.component_reference)


def freeze(version: int, raw: bytes) -> tuple[bytes, int]:
    archive = ObjectArchive(raw, version)
    archive.feature_context = 'ordinary_external_xml'
    archive.readers = dict(archive.readers, **readers())
    value = archive.object()
    if archive.p != len(raw):
        raise ValueError(f'explosion end {archive.p}; expected {len(raw)}')
    expected = view_oracle('root.object', value)
    return struct.pack('<BIII', 0, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    classes = [name for name in readers() if name not in ('moDirectionSpec_c', 'suObArray', 'moCompHolder_c')]
    cases = [freeze(version, wire(name, version, rich)) for name in classes
             for version in [0x27cc, 19000] for rich in [False, True]]
    cases += [freeze(19000, direction(flags)) for flags in range(16)]
    cases += [freeze(version, object_tag('moCompHolder_c')+component_base(version, rich))
              for version in [0x27cc, 19000] for rich in [False, True]]
    edited = exploder(0x27cc, False, 12345)
    freeze(0x27cc, edited)
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in cases)
    (WORKSPACE/'tests/fixtures/model-archives/explosion-cases.bin').write_bytes(packet)
    (WORKSPACE/'tests/fixtures/model-archives/explosion-edited.bin').write_bytes(edited)
    report = {'cases': len(cases), 'fields': sum(fields for _, fields in cases),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('explosion-cases.json').write_bytes((json.dumps(report)+'\n').encode())
    print({key: value for key, value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
