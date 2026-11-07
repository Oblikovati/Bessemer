"""Independent complete direct curve/surface collection oracles and container aliases."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_views import REFERENCE, WORKSPACE, view_oracle
from freeze_blank_part import reference_snapshot
from sketch_curve_fixture_bytes import arc, composite, conic, ellipse, hatch, surfaces
import sketchprefix
import modelheader
import plines
from mfcobjects import ObjectArchive


def freeze(mode: int, version: int, raw: bytes) -> tuple[bytes, int]:
    source = ObjectArchive(raw, version)
    source.feature_context = 'ordinary_external_xml'
    sketchprefix.install_scoped_readers(source)
    source.readers.update(su_CObList=modelheader.object_list, suObList=modelheader.object_list, su_CDWordArray=plines.dword_array_object)
    value = {}
    readers = [sketchprefix.read_hatches, sketchprefix.read_arcs, sketchprefix.read_ellipses, sketchprefix.read_conics, sketchprefix.read_surfaces]
    for index, reader in enumerate(readers):
        if mode in (index, 5): value = reader(source, value)
    value.pop('surfaces', None)
    if source.p != len(raw):
        raise ValueError(f'curve collection end {source.p}; expected {len(raw)}')
    expected = view_oracle('root', value)
    return struct.pack('<BIII', mode, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def curved_cases(version: int) -> list[tuple[bytes, int]]:
    categories = [0, 1, 2, 4, 7, 65535]
    records = [freeze(1, version, struct.pack('<iH', 1, category)+arc(rich, endpoints, center))
               for category in categories for rich in [False, True] for endpoints in [False, True] for center in [False, True]]
    records += [freeze(2, version, struct.pack('<iH', 1, category)+ellipse(rich, endpoints, mask))
                for category in [0, 1, 4, 65535] for rich in [False, True] for endpoints in [False, True] for mask in range(32)]
    records += [freeze(3, version, struct.pack('<iH', 1, category)+conic(rich)) for category in categories for rich in [False, True]]
    return records


def remaining_cases(version: int) -> list[tuple[bytes, int]]:
    records = [freeze(0, version, struct.pack('<i', 1)+hatch(rich, flags, extended, name))
               for rich in [False, True] for flags in [0x8000, 0x8001, 0xfffe, 0xffff]
               for extended in [False, True] for name in ['su_CObList', 'suObList']]
    records += [freeze(4, version, surfaces(rich, active)) for rich in [False, True] for active in [None, 0, 1, 2, 3, 4]]
    records += [freeze(4, version, surfaces(True, 0, flags)) for flags in range(4)]
    records += [freeze(mode, version, bytes(4 if mode < 4 else 20)) for mode in range(5)]
    return records+[freeze(5, version, composite(rich)) for rich in [False, True]]


def edit_wires() -> dict[str, bytes]:
    result = {}
    for name, value in [('original', 65535), ('edited', 12345)]:
        raw = struct.pack('<iH', 1, value)+arc(True, True, True)
        freeze(1, 19000, raw)
        result[f'sketch-arc-{name}.bin'] = raw
    return result


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    records = [record for version in [0x27cc, 19000] for record in curved_cases(version)+remaining_cases(version)]
    wires = edit_wires()
    if before != reference_snapshot(REFERENCE):
        raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in records)
    wires['sketch-curve-cases.bin'] = packet
    for name, raw in wires.items(): (WORKSPACE/'tests/fixtures/model-archives'/name).write_bytes(raw)
    report = {'cases': len(records), 'fields': sum(fields for _, fields in records),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'excluded_metadata': ['derived surfaces-completion marker'],
              'container_scope': 'explicit source-documented suObList/su_CObList forwarding and su_CDWordArray grammar',
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('sketch-curve-cases.json').write_bytes((json.dumps(report)+'\n').encode())
    print({key: value for key, value in report.items() if key not in ['source_hashes', 'excluded_metadata', 'container_scope']})


if __name__ == '__main__':
    generate()
