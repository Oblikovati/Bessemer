"""Feature folders and shared floating parameter bodies with noncanonical values."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_environment import feature, node_wire, short_wide_name
from freeze_views import REFERENCE, WORKSPACE, object_tag, view_oracle
from freeze_blank_part import reference_snapshot
import modelfeatures
import dimensions
from mfcobjects import ObjectArchive

FOLDERS = ['moMateReferenceGroupFolder_c', 'moFtrFolder_c', 'moNotesAreaFtrFolder_c']
PARAMETERS = ['moDensityParameter_c', 'moLengthParameter_c', 'moAngleParameter_c']


def folder(name: str, version: int, alias: bool, merged: bool) -> bytes:
    out = object_tag(name)+feature(version, merged)+struct.pack('<iiHii', -3, 5, 2 if alias else 0, -7, 11)
    if name == 'moNotesAreaFtrFolder_c':
        out += struct.pack('<i', -13)
    return out+struct.pack('<i', 17)


def parameter(name: str, version: int, presence: int) -> bytes:
    out = object_tag(name)+node_wire(version)
    out += struct.pack('<H', 4) if presence == 1 else struct.pack('<H', 0x8003)+short_wide_name('Ω parameter')
    out += struct.pack('<QB', 0x8000000000000000, presence)
    if presence:
        out += struct.pack('<Q', 0x7ff800000000002a)
    return out


def freeze(version: int, raw: bytes) -> tuple[bytes, int]:
    archive = ObjectArchive(raw, version)
    archive.feature_context = 'ordinary_external_xml'
    value = archive.object()
    if archive.p != len(raw):
        raise ValueError(f'folder/parameter end {archive.p}; expected {len(raw)}')
    expected = view_oracle('root.object', value)
    return struct.pack('<BIII', 0, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    versions = [0x27cc, 19000, 0xffffffff]
    cases = [freeze(version, folder(name, version, alias, merged))
             for name in FOLDERS for version in versions for alias in [False, True] for merged in [False, True]]
    cases += [freeze(version, parameter(name, version, presence))
              for name in PARAMETERS for version in versions for presence in [0, 1, 255]]
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in cases)
    (WORKSPACE/'tests/fixtures/model-archives/folder-parameter-cases.bin').write_bytes(packet)
    report = {'cases': len(cases), 'fields': sum(fields for _, fields in cases),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('folder-parameter-cases.json').write_text(json.dumps(report)+'\n')
    print({key: value for key, value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
