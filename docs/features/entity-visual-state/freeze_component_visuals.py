"""Run the complete component matrix against an immutable original Source copy (#181)."""
from pathlib import Path
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
for folder in ('model-archives', 'dimension-stored-state', 'resolved-features'):
    sys.path.insert(0, str(FEATURE.parent / folder))
sys.path.insert(0, str(FEATURE.parents[2] / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from dimension_state_evidence import stage_dimension_assets
from visual_component_corpus import verify_component_visual_matrix
from visual_state_evidence import verify_visual_readers, verify_visual_clauses


def run_component_visual_checks() -> unittest.TestResult:
    """Keep immutable-source/schema guards beside the complete component matrix; e.g. run_component_visual_checks()."""
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in
        (verify_visual_readers, verify_visual_clauses, verify_component_visual_matrix))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Bind the full case result to every copied Source file before cleanup; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-component-visual-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        stage_dimension_assets(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_component_visual_checks()
        record_snapshot(pin, hashes, result, FEATURE,
            ('displaylists', 'dl_light', 'dl_ui', 'facettess'), ('swcontainer', 'mfcobjects', 'swarchive'))
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'component-source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
