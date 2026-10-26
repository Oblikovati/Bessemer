"""Independent direct collection oracles with explicit composition and shared MFC identities."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_views import REFERENCE, WORKSPACE, view_oracle
from freeze_blank_part import reference_snapshot
from sketch_geometry_fixture_bytes import composite, line, point, text, texts
import sketchprefix
from mfcobjects import ObjectArchive
from swarchive import ArchiveError


def archive(raw: bytes, version: int) -> ObjectArchive:
    value = ObjectArchive(raw, version)
    value.feature_context = 'ordinary_external_xml'
    sketchprefix.install_scoped_readers(value)
    return value


def freeze(mode: int, version: int, raw: bytes) -> tuple[bytes, int]:
    source = archive(raw, version)
    value = sketchprefix.read_points(source) if mode in (0, 3) else {}
    if mode in (1, 3): value = sketchprefix.read_lines(source, value)
    if mode in (2, 3): value = sketchprefix.read_texts(source, value)
    if source.p != len(raw):
        raise ValueError(f'direct sketch end {source.p}; expected {len(raw)}')
    expected = view_oracle('root', value)
    return struct.pack('<BIII', mode, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def cases() -> list[tuple[bytes, int]]:
    versions = [0x27cc, 19000]
    result = [freeze(0, version, struct.pack('<iH', 1, category)+point(mask, rich, handles))
              for version in versions for category in [0, 1, 2, 65535] for mask in [0, 1, 0x10, 0x11, 0x8000, 0x8010]
              for rich in [False, True] for handles in [False, True]]
    result += [freeze(1, version, struct.pack('<iH', 1, category)+line(rich, vector, escaped))
               for version in versions for category in [0, 1, 2, 4, 7, 65535]
               for rich in [False, True] for vector in [False, True] for escaped in [False, True]]
    result += [freeze(2, version, texts(version, rich, count)) for version in versions for rich in [False, True] for count in [0, 1, 2]]
    result += [freeze(3, version, composite(version, rich)) for version in versions for rich in [False, True]]
    return result+[freeze(mode, version, bytes(4)) for mode in [0, 1] for version in versions]


def verify_kernel_stop(raw: bytes) -> None:
    try:
        sketchprefix.read_texts(archive(raw, 19000), {})
    except ArchiveError as error:
        if 'sketch text kernel body is not implemented' in str(error): return
        raise ValueError(f'reference stop {error!s}; expected explicit sketch-text kernel stop') from error
    raise ValueError('reference completed; expected unsupported sketch-text kernel stop')


def owned_wires() -> dict[str, bytes]:
    edits = {f'sketch-line-{name}.bin': struct.pack('<iH', 1, 7)+line(True, True, True, value)
             for name, value in [('original', 65535), ('edited', 12345)]}
    for raw in edits.values(): freeze(1, 19000, raw)
    edits['sketch-text-kernel-unsupported.bin'] = struct.pack('<i', 1)+text(19000, False, curve=True)
    verify_kernel_stop(edits['sketch-text-kernel-unsupported.bin'])
    return edits


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    records, wires = cases(), owned_wires()
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in records)
    wires['sketch-geometry-cases.bin'] = packet
    for name, raw in wires.items(): (WORKSPACE/'tests/fixtures/model-archives'/name).write_bytes(raw)
    report = {'cases': len(records), 'fields': sum(fields for _, fields in records),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('sketch-geometry-cases.json').write_bytes((json.dumps(report)+'\n').encode())
    print({key: value for key, value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
