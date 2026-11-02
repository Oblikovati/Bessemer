"""Replay the original historical plot tests from immutable source copies (#174)."""
from pathlib import Path
from functools import partial, update_wrapper
import hashlib
import importlib
import json
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
for folder in ('simulation-binary-thermal', 'simulation-post-dialog', 'simulation-solid-inventory', 'simulation-post-graph'):
    sys.path.insert(0, str(FEATURE.parent / folder))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import pinned_auxiliaries
from legacy_plot_evidence import verify_legacy_plot_sources, verify_legacy_plot_schema, verify_legacy_fixed_sequence
from legacy_plot_bindings import verify_legacy_plot_installed, verify_legacy_wide_rea
from legacy_plot_native import verify_legacy_plot_inventory, replay_legacy_plot_native
from legacy_plot_owned import verify_legacy_owned_grammar, verify_legacy_owned_failures
from legacy_plot_packets import verify_legacy_plot_operation_packets

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


def run_legacy_acceptance_checks(pin: Path) -> unittest.TestResult:
    """Keep original tests beside strict source, installed and complete native checks; e.g. run_legacy_acceptance_checks(pin)."""
    replay = update_wrapper(partial(replay_legacy_plot_native, pin), replay_legacy_plot_native)
    checks = (verify_legacy_plot_sources, verify_legacy_plot_schema, verify_legacy_fixed_sequence,
        verify_legacy_plot_installed, verify_legacy_wide_rea, verify_legacy_plot_inventory, replay,
        verify_legacy_owned_grammar, verify_legacy_owned_failures, verify_legacy_plot_operation_packets)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    original = importlib.import_module('test_cosworks_legacy_plot_record')
    tests = unittest.defaultTestLoader.loadTestsFromModule(original)
    assert tests.countTestCases() == 4, f'{tests.countTestCases()}; expected four original checks'
    suite.addTests(tests)
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Keep models pinned through final origin and hash checks; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-legacy-plot-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        pin_legacy_fixed_reads(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_legacy_acceptance_checks(pin)
        readers = ('cosworks_legacy_plot_record', 'test_cosworks_legacy_plot_record')
        record_snapshot(pin, hashes, result, FEATURE, readers, ())
        assert hashlib.sha256((pin / 'research' / FIXED_READS).read_bytes()).hexdigest() == FIXED_SHA256
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
