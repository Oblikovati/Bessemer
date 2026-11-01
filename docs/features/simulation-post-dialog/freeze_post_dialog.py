"""Copied-source complete dialog/legacy foundation with retained field-view obligations (#166)."""
from pathlib import Path
from functools import partial, update_wrapper
import importlib
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for folder in ('simulation-binary-thermal', 'model-archives', 'simulation-solid-inventory', 'simulation-post-graph'):
    sys.path.insert(0, str(FEATURE.parent / folder))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import pinned_auxiliaries
from dialog_evidence import verify_dialog_sources, verify_dialog_schema
from dialog_bindings import verify_dialog_installed_bindings
from dialog_native import (DIALOG_READERS, verify_dialog_inventory, replay_dialog_native, verify_dialog_legacy_view)
from dialog_legacy_carriers import replay_additional_legacy_dialogs
from dialog_field_evidence import verify_dialog_field_sources, verify_dialog_catalog_provenance
from dialog_field_bindings import verify_dialog_field_bindings
from dialog_field_views import verify_postinfo_field_views, verify_rgba_native_views
from dialog_catalog import verify_dialog_role_catalog
from dialog_owned import verify_dialog_owned
from dialog_owned_field_views import verify_owned_postinfo_field_views


def run_dialog_checks(pin: Path) -> unittest.TestResult:
    """Preserve all original C-770 and integrated legacy-view tests beside fresh full replay; e.g. run_dialog_checks(pin)."""
    replay = update_wrapper(partial(replay_dialog_native, pin), replay_dialog_native)
    additional = update_wrapper(partial(replay_additional_legacy_dialogs, pin), replay_additional_legacy_dialogs)
    checks = (verify_dialog_sources, verify_dialog_schema, verify_dialog_installed_bindings,
              verify_dialog_inventory, replay, verify_dialog_legacy_view, additional,
              verify_dialog_field_sources, verify_dialog_catalog_provenance, verify_dialog_field_bindings,
              verify_postinfo_field_views, verify_rgba_native_views, verify_dialog_role_catalog, verify_dialog_owned,
              verify_owned_postinfo_field_views)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for suffix in ('dialog_layout', 'dialog_prefix', 'info', 'style', 'planes', 'v7_composed_view',
                   'info_fields', 'cutprim_types', 'contour_policy', 'color_fields', 'style_field_catalog'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module('test_cosworks_post_' + suffix)))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Require every reader/delegate import to originate from an immutable copy; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-post-dialog-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_dialog_checks(pin)
        readers = DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_v7_composed_view', 'cfb', 'swcontainer')
        record_snapshot(pin, hashes, result, FEATURE, readers, ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
