"""Freeze current original node-state tests before exposing class-scoped archive views (#178)."""
from pathlib import Path
import importlib
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
sys.path.insert(0, str(FEATURE.parent / 'resolved-features'))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from node_state_evidence import verify_node_state_sources, verify_node_state_schema, verify_node_state_reports
from node_state_native import verify_node_state_installed
from node_state_owned import verify_owned_node_archives


def run_node_state_checks() -> unittest.TestResult:
    """Keep every original focused test beside separate source/native evidence checks; e.g. run_node_state_checks()."""
    checks = (verify_node_state_sources, verify_node_state_schema, verify_node_state_reports,
              verify_node_state_installed, verify_owned_node_archives)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    original = unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module('test_node_state_fields'))
    assert original.countTestCases() == 5, f'{original.countTestCases()}; expected all five current original node-state tests'
    suite.addTests(original)
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Keep all imported reader bytes and origins immutable through completion; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-node-state-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_node_state_checks()
        record_snapshot(pin, hashes, result, FEATURE,
                        ('modelnode', 'test_node_state_fields', 'attributes', 'dimensions', 'partlightweight', 'rtti'),
                        ('mfcobjects', 'swarchive'))
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
