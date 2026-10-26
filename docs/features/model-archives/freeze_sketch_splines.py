"""Independent complete direct splines and their interpolation/curve subrecords."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_views import REFERENCE, WORKSPACE, view_oracle
from freeze_blank_part import reference_snapshot
from oracle_projection import prune_compound_kernel
from sketch_spline_fixture_bytes import LISTS, alias_collection, collection
from spline_array_fixture_bytes import curve, interpolation, secondary
import sketchprefix
from mfcobjects import ObjectArchive


def freeze(mode: int, version: int, raw: bytes) -> tuple[bytes, int]:
    source = ObjectArchive(raw, version)
    source.feature_context = 'ordinary_external_xml'
    sketchprefix.install_scoped_readers(source)
    if mode == 0:
        value = sketchprefix.read_splines(source, {})
        for spline in value['splines']: spline.pop('interpolation_count')
        prune_compound_kernel(value, set())
    else:
        value = [sketchprefix.interpolation_data, sketchprefix.secondary_interpolation_data, sketchprefix.spline_curve][mode-1](source)
    if source.p != len(raw):
        raise ValueError(f'spline end {source.p}; expected {len(raw)}')
    expected = view_oracle('root', value)
    return struct.pack('<BIII', mode, version, len(raw), len(expected))+raw+expected, len(expected.splitlines())


def spline_cases(version: int) -> list[tuple[bytes, int]]:
    records = [freeze(0, version, collection(name, rich, flags, interpolation_mode, secondary_flag))
               for name in LISTS for rich in [False, True] for flags in [0, 1, 4, 5, 0x80000005]
               for interpolation_mode in range(5) for secondary_flag in [False, True]]
    records += [freeze(0, version, collection('sgSplineList_c', False, 0, 0, False, mask)) for mask in range(32) if not mask & ~0x1d]
    records += [freeze(0, version, collection(name, True, flags, 3, True, kernel=kernel))
                for name in LISTS[1:] for flags in [0, 1] for kernel in [1, 2]]
    return records+[freeze(0, version, alias_collection(name)) for name in LISTS[1:]]+[freeze(0, version, bytes(4))]


def array_cases() -> list[tuple[bytes, int]]:
    records = [freeze(1, 19000, interpolation(mode)) for mode in range(5)]
    records += [freeze(2, 19000, secondary(count, width)) for count in [0, 1, 3] for width in [-2**31, -1, 0, 2, 2**31-1]]
    records += [freeze(3, 19000, curve(0, 0, 0, 0, tolerance)) for tolerance in [0, 255]]
    records += [freeze(3, 19000, curve(dimension, knots, points, periodic, tolerance))
                for dimension in [3, 4] for knots in [0, 3] for points in [0, 1, 2]
                for periodic in [0, 65535] for tolerance in [0, 255]]
    return records


def owned_wires() -> dict[str, bytes]:
    result = {f'sketch-spline-{name}.bin': collection('cptsSplineList_c', True, 4, 3, True, final_word=value)
              for name, value in [('original', -31), ('edited', 12345)]}
    for raw in result.values(): freeze(0, 19000, raw)
    return result


def generate() -> None:
    before = reference_snapshot(REFERENCE)
    records = [record for version in [0x27cc, 19000] for record in spline_cases(version)]+array_cases()
    wires = owned_wires()
    if before != reference_snapshot(REFERENCE): raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in records)
    wires['sketch-spline-cases.bin'] = packet
    for name, raw in wires.items(): (WORKSPACE/'tests/fixtures/model-archives'/name).write_bytes(raw)
    report = {'cases': len(records), 'fields': sum(fields for _, fields in records),
              'sha256': hashlib.sha256(packet).hexdigest(), 'source_hashes': before,
              'excluded_metadata': ['derived interpolation_count duplicate', 'kernel nodes have a separate typed oracle; suppression remains included'],
              'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    Path(__file__).with_name('sketch-spline-cases.json').write_bytes((json.dumps(report)+'\n').encode())
    print({key: value for key, value in report.items() if key not in ['source_hashes', 'excluded_metadata']})


if __name__ == '__main__':
    generate()
