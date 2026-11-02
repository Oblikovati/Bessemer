"""Freeze all original historical MaterialList tests before investigating body binding (#175)."""
from pathlib import Path
import importlib
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from legacy_material_evidence import verify_legacy_material_sources, verify_legacy_material_schema

READERS = ('cosworks_material_layout', 'cosworks_legacy_material_header', 'cosworks_legacy_material_candidate', 'cosworkspersist')
TESTS = ('test_cosworks_material', 'test_cosworks_legacy_material_header', 'test_cosworks_legacy_material_candidate')


def run_legacy_material_checks() -> unittest.TestResult:
    """Keep all nine actual original tests beside strict evidence checks; e.g. run_legacy_material_checks()."""
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in
        (verify_legacy_material_sources, verify_legacy_material_schema))
    original = unittest.TestSuite(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)) for name in TESTS)
    assert original.countTestCases() == 9, f'{original.countTestCases()}; expected all nine current original tests'
    suite.addTests(original)
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Keep all reader imports immutable through final hash/origin checks; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-legacy-material-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_legacy_material_checks()
        record_snapshot(pin, hashes, result, FEATURE, READERS, ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
