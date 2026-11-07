"""Complete supporting sketch records checked by the approved reference readers."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_views import REFERENCE, WORKSPACE, view_oracle
from freeze_blank_part import reference_snapshot
from sketch_reference_fixture_bytes import (HANDLES, BACKUPS, backup, component_handle, component_vertex,
    external_entity, external_reference, handle, intersection_info, silhouette_vertex, slot, temporary_axis)
import sketchprefix
import sketchtail
import assemblyprofiles
from mfcobjects import ObjectArchive


def freeze(version: int, raw: bytes) -> tuple[bytes, int]:
    archive = ObjectArchive(raw, version)
    archive.feature_context = 'ordinary_external_xml'
    sketchprefix.install_scoped_readers(archive)
    archive.readers['moSkIntersectInfo_c'] = sketchtail.intersection_info
    archive.readers['mo3DPlaneBackedUpData_c'] = assemblyprofiles.plane_backup
    archive.readers['mo3DSphereBackedUpData_c'] = assemblyprofiles.sphere_backup
    value = archive.object()
    if archive.p != len(raw):
        raise ValueError(f'sketch supporting-record end {archive.p}; expected {len(raw)}')
    expected = view_oracle('root.object', value)
    return struct.pack('<BIII', 0, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def cases() -> list[tuple[bytes, int]]:
    versions = [0x27cc, 19000]
    indices = [0, 0x777e, 0x777f, 0x7780, 65535, -2**31, -1, 2**31-1]
    result = [freeze(version, handle(name, index)) for version in versions for name in HANDLES for index in indices]
    result += [freeze(version, backup(name, rich)) for version in versions for name in BACKUPS for rich in [False, True]]
    result += [freeze(version, slot(rich, name)) for version in versions for rich in [False, True] for name in HANDLES]
    result += [freeze(version, component_handle(version, name)) for version in versions for name in HANDLES]
    result += [freeze(version, external_reference(version, mode, rich)) for version in versions for mode in [-2, -1, 0] for rich in [False, True]]
    result += [freeze(version, temporary_axis(version, rich)) for version in [0x27cc, 0x371d, 0x371e, 19000] for rich in [False, True]]
    result += [freeze(version, reader(version, rich)) for version in versions for rich in [False, True]
               for reader in [external_entity, silhouette_vertex, component_vertex]]
    return result+[freeze(19000, intersection_info())]


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    records = cases()
    edited = handle('sgEntHandle', -2**31, 12345)
    freeze(19000, edited)
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in records)
    destination = WORKSPACE/'tests/fixtures/model-archives'
    (destination/'sketch-reference-cases.bin').write_bytes(packet)
    (destination/'sketch-handle-edited.bin').write_bytes(edited)
    report = {'cases': len(records), 'fields': sum(fields for _, fields in records),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('sketch-reference-cases.json').write_bytes((json.dumps(report)+'\n').encode())
    print({key: value for key, value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
