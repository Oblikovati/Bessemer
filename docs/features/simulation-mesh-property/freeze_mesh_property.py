"""Freeze complete documented Mesh Property reference evidence before implementing its typed codec (#163)."""
from pathlib import Path
from functools import partial, update_wrapper
import importlib
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for folder in ('simulation-binary-thermal', 'model-archives', 'simulation-solid-manager', 'simulation-solid-body',
               'simulation-shell-manager', 'simulation-shell-property', 'simulation-solid-inventory', 'simulation-sheet-property',
               'simulation-material'):
    sys.path.insert(0, str(FEATURE.parent / folder))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import pinned_auxiliaries
from freeze_body import verify_unimported_boundary
from property_evidence import verify_property_sources, verify_property_refinement_guards, verify_property_schema
from property_bindings import verify_property_installed_bindings
from property_native import verify_property_inventory, verify_property_historical_replay, verify_property_historical_guards, replay_property_native
from property_owned_packets import verify_owned_property_packets, verify_property_literal_guards
from property_native_operations import replay_native_property_operations


def run_property_checks(pin: Path) -> unittest.TestResult:
    """Replay every actual original test alongside strict evidence checks; e.g. run_property_checks(pin)."""
    replay = update_wrapper(partial(replay_property_native, pin), replay_property_native)
    operations = update_wrapper(partial(replay_native_property_operations, pin), replay_native_property_operations)
    checks = (verify_property_sources, verify_property_refinement_guards, verify_property_schema,
              verify_property_installed_bindings, verify_property_inventory, verify_property_historical_replay, verify_property_historical_guards,
              replay, verify_owned_property_packets, verify_property_literal_guards, operations, verify_unimported_boundary)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_mesh', 'test_cosworks_material', 'test_cosworks_embedded_body_ids',
                 'test_cosworks_embedded_const_surface', 'test_cosworks_mesh_nested_record', 'test_cosworks_mesh_newest_set'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Require immutable copied imports through final origin/hash verification; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-mesh-property-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_property_checks(pin)
        verify_unimported_boundary()
        readers = ('cosworks_mesh_layout', 'cosworks_mesh_nested_record', 'cosworks_embedded_const_surface',
                   'cosworks_embedded_body_ids', 'cfb', 'swcontainer')
        record_snapshot(pin, hashes, result, FEATURE, readers, ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
