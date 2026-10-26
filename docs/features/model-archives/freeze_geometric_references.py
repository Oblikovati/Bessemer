"""Geometric reference wire layouts checked by the approved independent reader."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_model_references import wrapper
from freeze_views import REFERENCE, WORKSPACE, object_tag, view_oracle
from freeze_blank_part import reference_snapshot
from oracle_projection import physical, prune_compound_kernel
from kernel_geometry_fixture import compressed, frame, transmit
import modelreferences
import kernelarchive
from mfcobjects import ObjectArchive

CLASSES = ['moGeneralCurveRef_w', 'moSingleFaceRef_w', 'moCircleRef_w', 'moCylinderRef_w',
           'moSphereRef_w', 'moPointRef_w', 'moLineRef_w', 'moLoopRef_w']
BODY_CLASSES = {'moGeneralCurveRef_w', 'moSingleFaceRef_w', 'moCircleRef_w', 'moLoopRef_w'}


def optional_frame(mode: int) -> bytes:
    out = struct.pack('<H', mode)
    if mode:
        out += struct.pack('<3d', -0.0, 1.25, -2.5)
    if 1 < mode < 0x8000:
        out += struct.pack('<3d', 3, 4, 5)
    return out


def embedded(mode: int, byte_presence: bool) -> bytes:
    out = bytes([255 if mode else 0]) if byte_presence else struct.pack('<i', -1 if mode else 0)
    return out+(b'\xc3' if mode == 1 else frame() if mode == 2 else b'')


def curve(version: int, rich: bool, mode: int) -> bytes:
    return wrapper(version, rich)+struct.pack('<2d', -0.0, 1.25)+embedded(mode, True)+struct.pack('<IiHHH', 0xfedcba98, -3, 0, 2 if rich else 0, 0)


def surface(version: int, rich: bool) -> bytes:
    return wrapper(version, rich)+struct.pack('<4d', -0.0, 1, 2, 3)


def wire(name: str, version: int, rich: bool, body: int, mode: int = 2) -> bytes:
    if name in ('moGeneralCurveRef_w', 'moCircleRef_w'):
        out = curve(version, rich, body)
        if name == 'moCircleRef_w': out += struct.pack('<7di', 1, 2, 3, 4, 5, 6, -0.0, -5)+optional_frame(mode)
    elif name == 'moSingleFaceRef_w':
        out = surface(version, rich)+embedded(body, False)+(struct.pack('<HHH', 2, 0, 0) if body else b'')+struct.pack('<Ii', 0x89abcdef, -7)
    elif name == 'moCylinderRef_w':
        out = surface(version, rich)+struct.pack('<7d', 1, 2, 3, 4, 5, 6, -0.0)+optional_frame(mode)
    elif name == 'moSphereRef_w':
        out = surface(version, rich)+struct.pack('<4d', 1, 2, 3, -0.0)
    elif name == 'moPointRef_w': out = wrapper(version, rich)+struct.pack('<3d', 1, 2, -0.0)
    elif name == 'moLineRef_w': out = wrapper(version, rich)+struct.pack('<8dB', -0.0, 1, 2, 3, 4, 5, 6, 7, 255)
    else: out = wrapper(version, rich)+embedded(body, False)
    return object_tag(name)+out


def freeze(version: int, raw: bytes) -> tuple[bytes, int]:
    archive = ObjectArchive(raw, version)
    archive.feature_context = 'ordinary_external_xml'
    value = physical(archive.object())
    if archive.p != len(raw): raise ValueError(f'geometry end {archive.p}; expected {len(raw)}')
    prune_compound_kernel(value, set())
    expected = view_oracle('root.object', value)
    return struct.pack('<BIII', 0, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    cases = [freeze(version, wire(name, version, rich, body)) for name in CLASSES
             for version in [0x27cc, 19000] for rich in [False, True]
             for body in (range(3) if name in BODY_CLASSES else [0])]
    cases += [freeze(19000, wire(name, 19000, True, 0, mode)) for name in ['moCircleRef_w', 'moCylinderRef_w'] for mode in [0, 1, 0x7fff, 0x8000, 0xffff]]
    kernelarchive.compressed_parasolid(frame()[6:])
    if before != reference_snapshot(REFERENCE): raise ValueError('reference changed; expected stable source hashes')
    destination = WORKSPACE/'tests/fixtures/model-archives'
    packet = b''.join(raw for raw, _ in cases)
    (destination/'geometric-reference-cases.bin').write_bytes(packet)
    kernel_files(destination)
    report = {'cases': len(cases), 'fields': sum(fields for _, fields in cases), 'sha256': hashlib.sha256(packet).hexdigest(),
              'kernel_oracle_scope': 'compound frame validated separately as typed neutral partition with one synthetic BODY record and identifier -7; no topology claim',
              'source_hashes': before, 'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('geometric-reference-cases.json').write_text(json.dumps(report)+'\n')
    print({key: value for key, value in report.items() if key != 'source_hashes'})


def kernel_files(destination: Path) -> None:
    (destination/'kernel-frame-one.bin').write_bytes(frame())
    (destination/'kernel-transmit-one.bin').write_bytes(transmit())
    (destination/'kernel-transmit-edited.bin').write_bytes(transmit(9))
    unmarked = transmit(title=b': TRANSMIT FILE created by modeller version 3701250')
    kernelarchive.compressed_parasolid(compressed(unmarked))
    (destination/'kernel-transmit-unmarked.bin').write_bytes(unmarked)


if __name__ == '__main__': generate()
