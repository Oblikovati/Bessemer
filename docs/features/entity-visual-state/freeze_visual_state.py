"""Freeze unchanged visual-related Source tests before adding stored-role operations (#181)."""
from pathlib import Path
import importlib
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
sys.path.insert(0, str(FEATURE.parent / 'dimension-stored-state'))
sys.path.insert(0, str(FEATURE.parent / 'resolved-features'))
sys.path.insert(0, str(FEATURE.parents[2] / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from dimension_state_evidence import stage_dimension_assets
from visual_state_evidence import verify_visual_readers, verify_visual_clauses, verify_visual_reports
from visual_native import verify_visual_installed
from visual_owned_carriers import verify_owned_visual_model_carriers
from visual_sketch_carriers import verify_owned_visual_sketch_carriers
from visual_display_carriers import verify_owned_visual_display_carriers

VISUAL_TEST_MODULES = ('test_feature_update_time', 'test_modelfeatures', 'test_node_state_fields',
    'test_node_tooltip', 'test_sketchprefix', 'test_modelboundary', 'test_camerafeatures',
    'test_modelcomponents', 'test_entity_visual_fields', 'test_dimensions', 'test_geometric_dimension_transform')


def run_visual_checks() -> unittest.TestResult:
    """Run every original related test beside separate evidence checks; e.g. run_visual_checks()."""
    checks = (verify_visual_readers, verify_visual_clauses, verify_visual_reports,
              verify_visual_installed, verify_owned_visual_model_carriers, verify_owned_visual_sketch_carriers,
              verify_owned_visual_display_carriers)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in VISUAL_TEST_MODULES:
        original = unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name))
        assert original.countTestCases() > 0, f'{name}; expected the complete original test module'
        suite.addTests(original)
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Record immutable reader bytes and origins before cleanup; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-visual-state-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        stage_dimension_assets(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_visual_checks()
        record_snapshot(pin, hashes, result, FEATURE, ('dl_light', 'dl_ui', 'modelfeatures',
            'sketchprefix', 'test_entity_visual_fields'), ('mfcobjects', 'swarchive'))
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
