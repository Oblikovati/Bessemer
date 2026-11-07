"""Independent sketch dimension and pattern bodies with tagged/direct collection counts."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_views import REFERENCE, WORKSPACE, object_tag, view_oracle
from freeze_blank_part import reference_snapshot
from sketch_dimension_fixture_bytes import G1, G2, OFFSETS, tagged, valued_collections, logical_collections, isoparam
from sketch_pattern_fixture_bytes import LOADED, constraint, linear, following, tagged as pattern_tagged
import sketchprefix
import sketchdimensions
import sketchpatterns
import modelheader
from mfcobjects import ObjectArchive


def reference_value(source: ObjectArchive, mode: int) -> object:
    if mode == 0: return source.object()
    value = {}
    readers = [sketchdimensions.first_collection, sketchdimensions.second_collection,
               sketchdimensions.third_collection, sketchdimensions.offset_collection]
    if mode in (1, 6):
        for reader in readers: value = reader(source, value)
    if mode in (2, 6): value = sketchdimensions.logical_collections(source, value)
    if mode in (3, 6): value = sketchdimensions.isoparam_collection(source, value)
    if mode in (4, 6): value = sketchpatterns.linear_collection(source, value)
    if mode in (5, 6): value = sketchpatterns.following_collections(source, value)
    value.pop('logicals', None)
    value.pop('pattern_constraints', None)
    return value


def freeze(mode: int, version: int, raw: bytes) -> tuple[bytes, int]:
    loaded = LOADED if mode & 0x80 else []
    source = ObjectArchive(raw, version, inherited_map=[None]+[{'_class': name} for name in loaded])
    source.feature_context = 'ordinary_external_xml'
    sketchprefix.install_scoped_readers(source)
    source.readers['su_CObArray'] = modelheader.object_list
    value = reference_value(source, mode & 0x7f)
    if source.p != len(raw): raise ValueError(f'sketch dimensions end {source.p}; expected {len(raw)}')
    expected = view_oracle('root.object' if mode & 0x7f == 0 else 'root', value)
    return struct.pack('<BIII', mode, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def body_cases(version: int) -> list[tuple[bytes, int]]:
    names = ['sgDim', 'sgDimEntityHelpData_c', *G1, *G2, *OFFSETS, 'sgValG3Dim',
             'sgLogDim', 'sgLogG1Dim', 'sgLogG2Dim', 'sgLogG3Dim', 'sgIsoParam_c']
    records = [freeze(0, version, tagged(name, version, rich)) for name in names for rich in [False, True]]
    records += [freeze(0x80 if loaded else 0, version, pattern_tagged(name, version, rich, loaded))
                for name in ['sgLinearPattern', 'sgCircularPattern', 'sgLinearPattCon', 'sgCircularPattCon']
                for rich in [False, True] for loaded in [False, True]]
    return records


def composition(version: int) -> bytes:
    out = valued_collections(version)+logical_collections(version, False)
    out += struct.pack('<H', 1)+isoparam(version, True)
    out += struct.pack('<H', 1)+linear(version, False, True, 3)
    return out+following(version, False, True)


def collection_cases(version: int) -> list[tuple[bytes, int]]:
    records = [freeze(1, version, valued_collections(version, group)) for group in [None, 0, 1, 2, 3]]
    records += [freeze(2, version, logical_collections(version, rich)) for rich in [False, True]]
    records += [freeze(3, version, struct.pack('<H', 1)+isoparam(version, rich)) for rich in [False, True]]
    records += [freeze(0x84 if loaded else 4, version, struct.pack('<H', 1)+linear(version, rich, loaded, 3 if loaded else 0))
                for rich in [False, True] for loaded in [False, True]]
    records += [freeze(0x85 if loaded else 5, version, following(version, rich, loaded))
                for rich in [False, True] for loaded in [False, True]]
    records += [freeze(mode, version, bytes(length)) for mode, length in [(1, 8), (2, 6), (3, 2), (4, 2), (5, 6)]]
    return records+[freeze(0x86, version, composition(version))]


def owned_wires() -> dict[str, bytes]:
    result = {f'sketch-pattern-{name}.bin': object_tag('sgLinearPattCon')+constraint(19000, True, 2, value)
              for name, value in [('original', -2**31), ('edited', 12345)]}
    for raw in result.values(): freeze(0, 19000, raw)
    return result


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    versions = [0x27cc, 0x2f79, 0x2f7a, 0x2f9f, 0x2fa0, 0x46cb, 0x46cc, 19000]
    records = [record for version in versions for record in body_cases(version)+collection_cases(version)]
    wires = owned_wires()
    if before != reference_snapshot(REFERENCE): raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in records)
    wires['sketch-dimension-pattern-cases.bin'] = packet
    for name, raw in wires.items(): (WORKSPACE/'tests/fixtures/model-archives'/name).write_bytes(raw)
    report = {'cases': len(records), 'fields': sum(fields for _, fields in records),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'loaded_target_scope': 'explicit loaded note and pattern-count identities; no body-reader substitution',
              'excluded_metadata': ['derived logicals and pattern_constraints completion flags'],
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('sketch-dimension-pattern-cases.json').write_bytes((json.dumps(report)+'\n').encode())
    print({key: value for key, value in report.items() if key not in ['source_hashes', 'loaded_target_scope', 'excluded_metadata']})


if __name__ == '__main__':
    generate()
