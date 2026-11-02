"""Replay the original historical plot tests from immutable source copies (#174)."""
from pathlib import Path
import hashlib
import importlib
import json
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot

FIXED_READS = 'cosworks-legacy-plot-fixed-reads.json'
FIXED_SHA256 = 'f29d842bad49e163275d128c3d5e795d05cdfc27b8aaab531daffef94d5226d7'


def pin_legacy_fixed_reads(pin: Path) -> None:
    """Freeze the parser's non-Python layout dependency; e.g. pin_legacy_fixed_reads(pin)."""
    source = ROOT.parent / 'inventor-interface/solidworks/research' / FIXED_READS
    raw = source.read_bytes()
    assert hashlib.sha256(raw).hexdigest() == FIXED_SHA256, f'{source}; expected {FIXED_SHA256}'
    fields = json.loads(raw)['exact700003']
    assert len(fields) == 46 and sum(field['bytes'] for field in fields) == 1344, fields
    (pin / 'research' / FIXED_READS).write_bytes(raw)
    assert source.read_bytes() == raw, f'{source}; expected unchanged fixed-read source'


def run_legacy_source_tests() -> unittest.TestResult:
    """Run all four original framing and wide-view checks; e.g. run_legacy_source_tests()."""
    module = importlib.import_module('test_cosworks_legacy_plot_record')
    suite = unittest.defaultTestLoader.loadTestsFromModule(module)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    assert result.testsRun == 4, f'{result.testsRun}; expected all four original tests'
    return result


def run() -> bool:
    """Keep models pinned through final origin and hash checks; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-legacy-plot-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pin_legacy_fixed_reads(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_legacy_source_tests()
        readers = ('cosworks_legacy_plot_record', 'test_cosworks_legacy_plot_record')
        record_snapshot(pin, hashes, result, FEATURE, readers, ())
        assert hashlib.sha256((pin / 'research' / FIXED_READS).read_bytes()).hexdigest() == FIXED_SHA256
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
