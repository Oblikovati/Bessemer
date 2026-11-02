"""Freeze unchanged current dimension tests before adding archive-backed aliases (#179)."""
from pathlib import Path
import importlib
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
sys.path.insert(0, str(FEATURE.parent / 'resolved-features'))
sys.path.insert(0, str(FEATURE.parents[2] / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from dimension_state_evidence import verify_dimension_readers, verify_dimension_clauses, verify_dimension_reports
from dimension_state_evidence import stage_dimension_assets
from dimension_state_native import verify_dimension_installed
from dimension_state_owned import verify_owned_dimension_archives

ORIGINAL_TESTS = ('test_node_state_fields', 'test_node_tooltip', 'test_sketchprefix',
    'test_modelboundary', 'test_camerafeatures', 'test_modelcomponents',
    'test_entity_visual_fields', 'test_dimensions', 'test_geometric_dimension_transform')


def run_dimension_checks() -> unittest.TestResult:
    """Keep every original test and separate provenance checks; e.g. run_dimension_checks()."""
    checks = (verify_dimension_readers, verify_dimension_clauses, verify_dimension_reports,
              verify_dimension_installed, verify_owned_dimension_archives)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ORIGINAL_TESTS:
        original = unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name))
        assert original.countTestCases() > 0, f'{name}; expected the complete original test module'
        suite.addTests(original)
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Record all source origins before removing the immutable copy; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-dimension-state-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        stage_dimension_assets(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_dimension_checks()
        record_snapshot(pin, hashes, result, FEATURE,
                        ('dimensions', 'test_dimensions', 'test_geometric_dimension_transform',
                         'resolvedfeatures', 'referencedimensions', 'sketchprefix', 'modelprefix', 'bodyfeatures'),
                        ('mfcobjects', 'swarchive'))
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
