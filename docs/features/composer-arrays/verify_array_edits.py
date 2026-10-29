"""Reopen every Rust native array replacement through actual frozen reference readers (#101)."""
from pathlib import Path
from collections.abc import Iterator
import hashlib
import json
import struct
import sys
import tempfile
import unittest
import freeze_arrays as baseline


def packet_records(raw: bytes) -> Iterator[tuple[int, int, int, bytes]]:
    """Require exact retained packet framing and return explicit caller context."""
    assert raw[:4] == b'VARR'
    count = struct.unpack_from('<I', raw, 4)[0]
    cursor = 8
    for _ in range(count):
        dimensions, rows, domain, size = struct.unpack_from('<BIII', raw, cursor)
        cursor += 13
        wire = raw[cursor:cursor + size]
        assert len(wire) == size
        cursor += size
        yield dimensions, rows, domain, wire
    assert cursor == len(raw)


def verify_array(dimensions: int, rows: int, domain: int, wire: bytes, changed: bytes) -> None:
    """Check exact requested code replacement plus original constant/endpoint metadata."""
    import composerscene
    expected = bytearray(wire)
    if dimensions == 0:
        original = composerscene.read_indices(wire, count=rows, domain=domain)
        reopened = composerscene.read_indices(changed, count=rows, domain=domain)
        packed_start, packed_size = 0, len(wire)
    else:
        original = composerscene.read_vector(wire, count=rows, dimensions=dimensions)
        reopened = composerscene.read_vector(changed, count=rows, dimensions=dimensions)
        for name in ('mode', 'count', 'dimensions', 'constant_axis_flags', 'constant_patterns', 'endpoint_patterns'):
            assert original[name] == reopened[name]
        packed_start = original['packed_values']['start']
        packed_size = len(original['packed_values']['raw_bytes'])
    if packed_size:
        expected[packed_start] ^= 1
    assert changed == expected and reopened['end'] == len(changed)


def verify_replacements() -> None:
    """Require every original probe identity and every exact edited output byte."""
    probe = (baseline.ROOT / 'target/composer-arrays-native.bin').read_bytes()
    identity = json.loads((baseline.FEATURE / 'native-probe-reference.json').read_bytes())
    assert hashlib.sha256(probe).hexdigest() == identity['sha256']
    assert hashlib.sha256((baseline.FEATURE / 'native-reference.json').read_bytes()).hexdigest() == identity['native_reference_sha256']
    changed = (baseline.ROOT / 'target/composer-arrays-native-edited.bin').read_bytes()
    cursor = count = 0
    for dimensions, rows, domain, wire in packet_records(probe):
        verify_array(dimensions, rows, domain, wire, changed[cursor:cursor + len(wire)])
        cursor += len(wire)
        count += 1
    assert count == identity['records'] == 8196 and cursor == len(changed)
    baseline.retained('native-edit-reference.json', dict(records=count, bytes=len(changed),
                      sha256=hashlib.sha256(changed).hexdigest(), original_probe_sha256=identity['sha256']))


def main() -> None:
    """Use immutable actual reference readers, e.g. python verify_array_edits.py."""
    with tempfile.TemporaryDirectory(prefix='bessemer-array-edits-') as temporary:
        pin = Path(temporary)
        hashes = baseline.copy_sources(pin)
        sys.meta_path.insert(0, baseline.FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(verify_replacements))
        assert result.wasSuccessful() and result.testsRun == 1 and not result.skipped
        baseline.record_snapshot(pin, hashes, result, baseline.FEATURE, ('composerscene',), ())
        (baseline.FEATURE / 'fixture-reference-snapshot.json').replace(baseline.FEATURE / 'native-edit-source-snapshot.json')


if __name__ == '__main__':
    main()
