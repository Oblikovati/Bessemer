"""Independent complete dimension-text records, including nested point and curve arrays."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_views import REFERENCE, WORKSPACE, object_tag, view_oracle
from freeze_blank_part import reference_snapshot
from freeze_detail_prefix import character
from owned_name_edit import short_wide_name
import sketchprefix
from mfcobjects import ObjectArchive

SHAPES = [([], []), ([0, 0], []), ([1, 2], [1, 0, 2]), ([2, 1], [0, 2, 1])]


def vectors(count: int) -> bytes:
    return b''.join(struct.pack('<3d', -0.0, index+.25, -index-.5) for index in range(count))


def nested(shape: int) -> bytes:
    groups, points = SHAPES[shape]
    out = struct.pack('<i', len(groups))+struct.pack('<'+'i'*len(groups), *groups)
    out += struct.pack('<'+'i'*len(points), *[-1, 2**31-1, -2**31][:len(points)])
    return out+struct.pack('<'+'i'*len(points), *points)+vectors(sum(points))


def curves(mode: int) -> bytes:
    if mode == 0:
        return struct.pack('<i', 0)
    count = mode-1
    out = struct.pack('<ii', [255, -1, 1][count], count)+vectors(count)
    out += struct.pack('<'+'i'*count, *[-9, -2**31][:count])
    return out+vectors(count)+struct.pack('<i', -10)


def wire(version: int, shape: int, curve: int, font: bool, final_word: int = -42) -> bytes:
    out = object_tag('moDimText_c')+short_wide_name('<RTL>Ω sketch text')+vectors(1)+struct.pack('<i', -5)
    out += object_tag('utCharFormat_c')+character(version) if font else b'\0\0'
    out += struct.pack('<i', 7)+vectors(1)+struct.pack('<d', -0.0)+nested(shape)
    out += struct.pack('<i', -8)+curves(curve)+struct.pack('<iddi', 17, 1.5, -0.0, 18)
    out += struct.pack('<HI', 65535, 2) if font else struct.pack('<H', 2)
    out += short_wide_name('Ω first')+b'\x06second'
    return out+struct.pack('<iiidi', 19, 20, 21, 3.5, final_word)


def freeze(version: int, raw: bytes) -> tuple[bytes, int]:
    archive = ObjectArchive(raw, version)
    archive.feature_context = 'ordinary_external_xml'
    sketchprefix.install_scoped_readers(archive)
    value = archive.object()
    if archive.p != len(raw):
        raise ValueError(f'dimension-text end {archive.p}; expected {len(raw)}')
    expected = view_oracle('root.object', value)
    return struct.pack('<BIII', 0, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    cases = [freeze(version, wire(version, shape, curve, font))
             for version in [0x27cc, 0x3eec, 0x3eed, 19000] for shape in range(4)
             for curve in range(4) for font in [False, True]]
    edits = [wire(19000, 2, 3, True, final) for final in [-42, 12345]]
    for raw in edits: freeze(19000, raw)
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    destination = WORKSPACE/'tests/fixtures/model-archives'
    packet = b''.join(raw for raw, _ in cases)
    (destination/'sketch-text-cases.bin').write_bytes(packet)
    for name, raw in zip(['original', 'edited'], edits): (destination/f'sketch-text-{name}.bin').write_bytes(raw)
    report = {'cases': len(cases), 'fields': sum(fields for _, fields in cases),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('sketch-text-cases.json').write_bytes((json.dumps(report)+'\n').encode())
    print({key: value for key, value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
