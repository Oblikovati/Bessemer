"""Bind software-profile expectations to exact rational arithmetic and frozen readers (#101)."""
from pathlib import Path
import hashlib
import json
import random
import struct
import sys
import tempfile
import unittest
from fractions import Fraction
from types import ModuleType

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot, SOURCE


def retain(path: Path, raw: bytes) -> None:
    """Require explicit baseline replacement; e.g. retain(path, expected_bytes)."""
    if '--write' in sys.argv:
        path.write_bytes(raw)
        return
    assert path.read_bytes() == raw, f'{path}; expected reviewed baseline bytes'


def retain_report(name: str, report: object) -> None:
    """Retain readable oracle facts alongside binary cases, e.g. retain_report('owned.json', [])."""
    retain(FEATURE / name, (json.dumps(report, indent=2) + '\n').encode())


def value_packet(wire: bytes, count: int, dimensions: int, expected: bytes) -> bytes:
    """Keep original wire/context separate from expected numeric patterns."""
    return struct.pack('<BIII', dimensions, count, len(wire), len(expected)) + wire + expected


def quantized_case(width: int, codes: list[int], first: int, second: int) -> bytes:
    """Use the independent exact-ratio oracle, then require actual profile agreement."""
    import composerscene
    import composervectors
    import test_composervectors as oracle
    mode = b'c' if width == 1 else b'b'
    wire = mode + struct.pack('<IIII', first, second, first, second)
    wire += b''.join(code.to_bytes(width, 'little') * 2 for code in codes)
    expected = b''.join(struct.pack('<II', *([oracle.exact_decode(code, width, first, second)] * 2)) for code in codes)
    block = composerscene.read_vector(wire, count=len(codes), dimensions=2)
    projected = composervectors.reconstruct(block, version=7.9, fp_profile=composervectors.PROFILE)
    assert projected['patterns_le'] == expected
    return value_packet(wire, len(codes), 2, expected)


def exhaustive_weights() -> list[bytes]:
    """Pin all 65,792 code weights using integer-ratio rounding, without floating casts."""
    import test_composervectors as oracle
    packets = []
    for width in (1, 2):
        codes = list(range(1 << (width * 8)))
        for code in codes:
            assert oracle.exact_decode(code, width, 0, 0x3f800000) == oracle.exact_word(Fraction(code, len(codes) - 1))
        packets.append(quantized_case(width, codes, 0, 0x3f800000))
    return packets


def endpoint_cases() -> list[bytes]:
    """Exercise all byte codes and word boundaries across reviewed finite degeneracies."""
    pairs = [(0xbf800000, 0x3f800000), (1, 0x00800000), (0x3f800001, 0x3f800002),
             (0x80800000, 1), (0x80000000, 0x80000000), (0, 0x80000000), (0x80000000, 0),
             (0x7f7fffff, 0x7f7fffff), (0xff7fffff, 0xff7fffff), (0x7f7fffff, 0xff7fffff),
             (0xff7fffff, 0x7f7fffff), (0x7f7fffff, 1), (1, 0x7f7fffff),
             (0x7f7fffff, 0x80000001), (0x007fffff, 0x00800000)]
    packets = []
    for width, codes in ((1, list(range(256))), (2, [0, 1, 2, 127, 128, 255, 256, 32767, 32768, 65534, 65535])):
        packets.extend(quantized_case(width, codes, first, second) for first, second in pairs)
    return packets


def generated_cases() -> list[bytes]:
    """Use a fixed seed to cover finite exponents, signs and cancellation independently."""
    generator = random.Random(0xC473)
    packets = []
    for _ in range(8192):
        first, second = generator.getrandbits(32), generator.getrandbits(32)
        first = finite_pattern(first)
        second = finite_pattern(second)
        width = generator.choice((1, 2))
        packets.append(quantized_case(width, [generator.randrange(1 << (width * 8))], first, second))
    return packets


def finite_pattern(word: int) -> int:
    """Map only exponent-255 random words into finite endpoint space."""
    if word & 0x7f800000 == 0x7f800000:
        return word ^ 0x00800000
    return word


def owned_values() -> None:
    """Retain exact-rational expected output and full actual-reader comparison."""
    packets = exhaustive_weights() + endpoint_cases() + generated_cases()
    raw = b'VVAL' + struct.pack('<I', len(packets)) + b''.join(packets)
    retain(ROOT / 'tests/fixtures/composer-vectors/values.bin', raw)
    retain_report('owned-reference.json', dict(cases=len(packets), bytes=len(raw),
                  sha256=hashlib.sha256(raw).hexdigest(), exhaustive_weights=65792,
                  endpoint_pairs=15, generated_pairs=8192, oracle='exact integer ratios / nearest-even quotient-remainder rounding'))


def independent_samples(block: dict[str, object], patterns: bytes) -> int:
    """Compare selected installed components with exact rational arithmetic, not host floats."""
    import test_composervectors as oracle
    if block['mode'] == 'a' or block['count'] == 0:
        return 0
    varying = [axis for axis, flag in enumerate(block['constant_axis_flags']) if not flag]
    width = 1 if block['mode'] == 'c' else 2
    samples = 0
    for row in sorted({0, block['count'] // 2, block['count'] - 1}):
        for packed_axis, axis in enumerate(varying):
            offset = (row * len(varying) + packed_axis) * width
            code = int.from_bytes(block['packed_values']['raw_bytes'][offset:offset + width], 'little')
            expected = oracle.exact_decode(code, width, *block['endpoint_patterns'][axis])
            assert struct.unpack_from('<I', patterns, (row * block['dimensions'] + axis) * 4)[0] == expected
            samples += 1
    return samples


def native_vector(block: dict[str, object], decoded: bytes, expected: dict[str, object]) -> tuple[bytes, int, bytes]:
    """Bind a complete actual vector to the published reference output and independent samples."""
    import composervectors
    patterns = composervectors.reconstruct(block, version=7.9, fp_profile=composervectors.PROFILE)['patterns_le']
    assert hashlib.sha256(patterns).hexdigest() == expected['reconstructed_sha256']
    for name in ('count', 'dimensions', 'mode', 'constant_axis_flags', 'constant_patterns', 'endpoint_patterns'):
        assert json.loads(json.dumps(block[name])) == expected[name]
    assert hashlib.sha256(block['packed_values']['raw_bytes']).hexdigest() == expected['packed_sha256']
    wire = decoded[block['start']:block['end']]
    return value_packet(wire, block['count'], block['dimensions'], patterns), independent_samples(block, patterns), patterns


def native_file(row: dict[str, object], baseline: ModuleType) -> tuple[dict[str, object], list[bytes]]:
    """Require complete source scenes and every expected per-vector span exactly once."""
    encoded, decoded, scene = baseline.native_scene(dict(path=row['path'], sha256=row['source_sha256']))
    assert hashlib.sha256(encoded).hexdigest() == row['member_sha256']
    expected = {tuple(vector['wire_span']): vector for vector in row['vectors']}
    packets, samples, output_bytes = [], 0, 0
    output_hash = hashlib.sha256()
    for block in baseline.array_blocks(scene['entries']):
        if 'packed_values' not in block:
            continue
        packet, checked, patterns = native_vector(block, decoded, expected.pop((block['start'], block['end'])))
        packets.append(packet)
        samples += checked
        output_bytes += len(patterns)
        output_hash.update(patterns)
    assert not expected
    return dict(path=row['path'], source_sha256=row['source_sha256'], vectors=len(packets),
                output_bytes=output_bytes, independent_samples=samples, patterns_sha256=output_hash.hexdigest()), packets


def native_values() -> None:
    """Verify all installed arrays and optionally export exact full-pattern Rust expectations."""
    sys.path.insert(0, str(FEATURE.parent / 'composer-arrays'))
    import freeze_arrays
    source = (SOURCE / 'research/composer-vector-values-corpus.json').read_bytes()
    corpus = json.loads(source)
    records, packets = [], []
    for row in corpus['files']:
        record, file_packets = native_file(row, freeze_arrays)
        records.append(record)
        if '--export' in sys.argv:
            packets.extend(file_packets)
    report = native_report(records, source)
    retain_report('native-reference.json', report)
    if '--export' in sys.argv:
        export_native(packets)


def native_report(records: list[dict[str, object]], source: bytes) -> dict[str, object]:
    """Require the complete documented corpus and its independent sample count."""
    report = dict(files=len(records), vectors=sum(row['vectors'] for row in records),
                  output_bytes=sum(row['output_bytes'] for row in records),
                  independent_samples=sum(row['independent_samples'] for row in records),
                  source_corpus_sha256=hashlib.sha256(source).hexdigest(), records=records)
    assert (report['files'], report['vectors'], report['output_bytes'], report['independent_samples']) == (38, 4367, 66250232, 32286)
    return report


def export_native(packets: list[bytes]) -> None:
    """Retain the exact native probe identity; expected full values stay outside the repository."""
    raw = b'VVAL' + struct.pack('<I', len(packets)) + b''.join(packets)
    (ROOT / 'target/composer-vectors-native.bin').write_bytes(raw)
    retain_report('native-probe-reference.json', dict(cases=len(packets), bytes=len(raw),
                  sha256=hashlib.sha256(raw).hexdigest()))


def main() -> None:
    """Copy all sources before importing readers/oracles, e.g. python freeze_values.py."""
    with tempfile.TemporaryDirectory(prefix='bessemer-vector-values-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in (owned_values, native_values))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        assert result.wasSuccessful() and result.testsRun == 2 and not result.skipped
        record_snapshot(pin, hashes, result, FEATURE, ('composerscene', 'composervectors', 'test_composervectors'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'owned-source-snapshot.json')


if __name__ == '__main__':
    main()
