"""Independent ordinary dimensions, attachments, parameters and loaded-target handles."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_views import REFERENCE, WORKSPACE, object_tag, view_oracle
from freeze_blank_part import reference_snapshot
from dimension_fixture_bytes import attachment, dimension, dimension_base, handle, integer_parameter, tolerance
import bodyfeatures
import camerafeatures
import dimensionhandles
import dimensions
from mfcobjects import ObjectArchive

GEOMETRIC = ['moNumberDim_c', 'moScalerDim_c', 'ParallelPlaneDistanceDim_c', 'ThreeDRadiusDim_c', 'ThreeDdiameterDim_c', 'AngleDim_c', 'moRevolvedRadiusDim_c']
HANDLES = ['moFeatureDimHandle_c', 'moRefDimHandle_c', 'moSkDimHandleValG2_c', 'moSkDimHandleValG3_c', 'moSkDimHandleRadial_c', 'moSkDimHandleOffset_c']


def freeze(version: int, raw: bytes, loaded: str | None = None) -> tuple[bytes, int]:
    archive = ObjectArchive(raw, version, inherited_map=[None]+([{'_class': loaded}] if loaded else []))
    archive.feature_context, archive.model_root_class = 'ordinary_external_xml', 'moPart_c'
    bodyfeatures.install_scoped(archive)
    archive.readers = dict(archive.readers, moAttachPoint_c=camerafeatures.attach_point)
    value = archive.object()
    if archive.p != len(raw):
        raise ValueError(f'dimension end {archive.p}; expected {len(raw)}')
    expected = view_oracle('root.object', value)
    mode = {None: 0, 'moGFR_c': 1, 'sgSketch': 2}[loaded]
    return struct.pack('<BIII', mode, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def dimension_cases() -> list[tuple[bytes, int]]:
    versions = [0x27cc, 0x42ee, 0x42ef, 19000]
    cases = [freeze(version, dimension('Dimension_c', version, flags)) for version in versions for flags in [0, 1]]
    cases += [freeze(version, dimension(name, version, flags)) for name in GEOMETRIC for version in [0x27cc, 19000] for flags in range(32)]
    cases += [freeze(version, tolerance(version)) for version in [0x27cc, 0x2b6d, 0x2b6e, 0x2b76, 0x2b77, 19000]]
    cases += [freeze(version, integer_parameter(version, present)) for version in [0x27cc, 19000] for present in [0, 1, 255]]
    cases += [freeze(19000, attachment())]
    return cases


def handle_cases() -> list[tuple[bytes, int]]:
    return [freeze(version, handle(name, version, external, flags),
                   'moGFR_c' if external else ('sgSketch' if 'moSk' in name else None))
            for name in HANDLES for version in [0x27cc, 19000] for external in [False, True] for flags in range(8)]


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    cases = dimension_cases()+handle_cases()
    edited = object_tag('Dimension_c')+dimension_base(19000, True, 12345)
    freeze(19000, edited)
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in cases)
    (WORKSPACE/'tests/fixtures/model-archives/dimension-cases.bin').write_bytes(packet)
    (WORKSPACE/'tests/fixtures/model-archives/dimension-edited.bin').write_bytes(edited)
    report = {'cases': len(cases), 'fields': sum(fields for _, fields in cases),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'loaded_target_scope': 'explicit inherited moGFR_c/sgSketch identities; no claim to parse their bodies',
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('dimension-cases.json').write_bytes((json.dumps(report)+'\n').encode())
    print({key: value for key, value in report.items() if key not in ('source_hashes', 'loaded_target_scope')})


if __name__ == '__main__':
    generate()
