"""Immutable copied-source foundation for complete Results Property hierarchy operations (#164)."""
from pathlib import Path
from functools import partial, update_wrapper
import importlib
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for folder in ('simulation-binary-thermal', 'model-archives', 'simulation-solid-manager', 'simulation-solid-body',
               'simulation-shell-manager', 'simulation-shell-property', 'simulation-solid-inventory', 'simulation-sheet-property', 'simulation-material'):
    sys.path.insert(0, str(FEATURE.parent / folder))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import pinned_auxiliaries
from results_evidence import READERS, verify_results_sources, verify_results_schema, verify_results_level_guards
from results_bindings import verify_results_installed_bindings
from results_native import verify_results_inventory, replay_results_native
from results_owned import verify_results_owned, verify_results_writer_controls
from results_packets import verify_results_packets
from results_resources import verify_results_resources, verify_results_category_cap


def run_results_checks(pin: Path) -> unittest.TestResult:
    """Keep all nine original tests beside fresh inventory/binding/native checks; e.g. run_results_checks(pin)."""
    replay = update_wrapper(partial(replay_results_native, pin), replay_results_native)
    checks = (verify_results_sources, verify_results_schema, verify_results_level_guards,
              verify_results_installed_bindings, verify_results_inventory, replay,
              verify_results_owned, verify_results_writer_controls, verify_results_packets,
              verify_results_resources, verify_results_category_cap)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in READERS:
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module('test_' + name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Require every parser/helper import to originate from the immutable pin; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-results-headers-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_results_checks(pin)
        record_snapshot(pin, hashes, result, FEATURE, READERS + ('cfb', 'swcontainer'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
