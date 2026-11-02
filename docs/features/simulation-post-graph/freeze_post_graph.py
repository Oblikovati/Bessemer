"""Copied-source complete graph evidence foundation without native target execution (#165)."""
from pathlib import Path
from functools import partial, update_wrapper
import importlib
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for folder in ('simulation-binary-thermal', 'model-archives', 'simulation-solid-inventory'):
    sys.path.insert(0, str(FEATURE.parent / folder))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import pinned_auxiliaries
from graph_evidence import READER, verify_graph_sources, verify_graph_schema
from graph_bindings import verify_graph_installed_bindings
from graph_native import verify_graph_inventory, replay_graph_native, verify_graph_path_guards
from graph_owned import verify_graph_owned
from graph_packets import verify_graph_packets
from graph_resources import verify_graph_resources, verify_graph_character_caps
from graph_malformed import verify_graph_malformed, verify_graph_extents


def run_graph_checks(pin: Path) -> unittest.TestResult:
    """Keep all three original tests beside full fresh native replay and bindings; e.g. run_graph_checks(pin)."""
    replay = update_wrapper(partial(replay_graph_native, pin), replay_graph_native)
    checks = (verify_graph_sources, verify_graph_schema, verify_graph_installed_bindings,
              verify_graph_inventory, verify_graph_path_guards, replay, verify_graph_owned, verify_graph_packets,
              verify_graph_resources, verify_graph_character_caps, verify_graph_malformed, verify_graph_extents)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module('test_cosworks_post_graph')))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Pin all imported readers/adapters and verify immutable origins before cleanup; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-post-graph-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_graph_checks(pin)
        record_snapshot(pin, hashes, result, FEATURE, (READER, 'cfb', 'swcontainer'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
