"""Permanent marks and complete annotation-view boundary cases."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_views import REFERENCE, WORKSPACE, object_tag, view, view_oracle
from freeze_blank_part import reference_snapshot
import bodyfeatures
import modelpostviews
from mfcobjects import ObjectArchive


def annotation(version: int, matrix: bool) -> bytes:
    ordinary = view(version, matrix, 0x2b)
    out = object_tag('moAnnotationView_c')+ordinary[len(object_tag('moView_c')):]
    out += struct.pack('<iHHi dHiiH', -3, 0, 0, 5, -0.0, 0, -7, 11, 0)
    for index, gate in enumerate([0x2b7e, 0x2b9a, 0x2ba0, 0x2bbb]):
        if version > gate:
            out += struct.pack('<i', -13-index)
    return out


def freeze(version: int, raw: bytes) -> tuple[bytes, int]:
    archive = ObjectArchive(raw, version)
    archive.feature_context = 'ordinary_external_xml'
    archive.readers = dict(archive.readers, moAnnotationView_c=modelpostviews.annotation_view,
                           moPMarkRecord_c=bodyfeatures.permanent_mark_record)
    value = archive.object()
    if archive.p != len(raw):
        raise ValueError(f'annotation end {archive.p}; expected {len(raw)}')
    expected = view_oracle('root.object', value)
    return struct.pack('<BIII', 0, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    versions = [0x27cc, 0x2b7e, 0x2b7f, 0x2b9a, 0x2b9b, 0x2ba0,
                0x2ba1, 0x2bbb, 0x2bbc, 0x3763, 0x3764, 19000]
    cases = [freeze(version, annotation(version, matrix)) for version in versions for matrix in [False, True]]
    cases += [freeze(version, object_tag('moPMarkRecord_c')+struct.pack('<i', value))
              for version in [0x27cc, 19000] for value in [-2**31, -1, 0, 2**31-1]]
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in cases)
    (WORKSPACE/'tests/fixtures/model-archives/annotation-view-cases.bin').write_bytes(packet)
    report = {'cases': len(cases), 'fields': sum(fields for _, fields in cases),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('annotation-view-reference.json').write_text(json.dumps(report)+'\n')
    print({key: value for key, value in report.items() if key != 'source_hashes'})


if __name__ == '__main__':
    generate()
