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
from dialog_operation_packets import verify_dialog_operation_packets
from dialog_edit_oracle import verify_dialog_edit_matrix
from dialog_resource_oracle import verify_dialog_resources
from dialog_cap_oracle import verify_dialog_caps
from dialog_reference_cap_oracle import verify_reference_caps
from dialog_projection_shapes import verify_dialog_scalar_shapes
from dialog_parent_views import (verify_dialog_parent_sources, verify_dialog_parent_native, verify_dialog_parent_bindings,
                                 verify_dialog_parent_owned, verify_dialog_parent_branches)


def run_dialog_checks(pin: Path) -> unittest.TestResult:
    """Preserve all original C-770 and integrated legacy-view tests beside fresh full replay; e.g. run_dialog_checks(pin)."""
    if '--parent-views-only' in sys.argv: return run_parent_view_checks()
    if any(flag in sys.argv for flag in ('--resources-only', '--caps-only', '--reference-caps-only')): return run_resource_checks()
    replay = update_wrapper(partial(replay_dialog_native, pin), replay_dialog_native)
    additional = update_wrapper(partial(replay_additional_legacy_dialogs, pin), replay_additional_legacy_dialogs)
    checks = (verify_dialog_sources, verify_dialog_schema, verify_dialog_installed_bindings,
              verify_dialog_inventory, replay, verify_dialog_legacy_view, additional,
              verify_dialog_field_sources, verify_dialog_catalog_provenance, verify_dialog_field_bindings,
              verify_postinfo_field_views, verify_rgba_native_views, verify_dialog_role_catalog, verify_dialog_owned,
              verify_owned_postinfo_field_views, verify_dialog_operation_packets, verify_dialog_scalar_shapes, verify_dialog_parent_sources,
              verify_dialog_parent_native, verify_dialog_parent_bindings,
              verify_dialog_parent_owned, verify_dialog_parent_branches, verify_dialog_edit_matrix, verify_dialog_resources, verify_dialog_caps, verify_reference_caps)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for suffix in ('dialog_layout', 'dialog_prefix', 'info', 'style', 'planes', 'v7_composed_view',
                   'info_fields', 'cutprim_types', 'contour_policy', 'color_fields', 'style_field_catalog'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module('test_cosworks_post_' + suffix)))
    for name in ('test_cosworks_legend_range_fields', 'test_cosworks_deform_scale_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)



def run_resource_checks() -> unittest.TestResult:
    """Keep targeted resource initialization explicit without replacing the full default gate; e.g. run_resource_checks()."""
    boundary = verify_dialog_caps if '--caps-only' in sys.argv else verify_dialog_resources
    if '--reference-caps-only' in sys.argv: boundary = verify_reference_caps
    checks = (verify_dialog_sources, verify_dialog_field_sources, boundary)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run_parent_view_checks() -> unittest.TestResult:
    """Replay the view helpers and full owned denominators without replacing the default gate; e.g. run_parent_view_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources,
              verify_dialog_parent_owned, verify_dialog_parent_branches)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    return unittest.TextTestRunner(verbosity=2).run(suite)


def dialog_snapshot_readers() -> tuple[str, ...]:
    """Require exactly the readers exercised by the selected gate; e.g. dialog_snapshot_readers()."""
    if '--parent-views-only' in sys.argv:
        return ('cosworks_legend_range_fields', 'cosworks_deform_scale_fields')
    readers = DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout')
    if any(flag in sys.argv for flag in ('--resources-only', '--caps-only', '--reference-caps-only')):
        return readers
    return readers + ('cosworks_post_v7_composed_view', 'cfb', 'swcontainer')


def dialog_snapshot_filename() -> str:
    """Keep partial replay evidence separate from the mandatory full snapshot; e.g. dialog_snapshot_filename()."""
    modes = (('--resources-only', 'resource-source-snapshot.json'),
             ('--caps-only', 'cap-source-snapshot.json'),
             ('--reference-caps-only', 'reference-cap-source-snapshot.json'),
             ('--parent-views-only', 'parent-view-source-snapshot.json'))
    for flag, filename in modes:
        if flag in sys.argv: return filename
    return 'source-snapshot.json'


def run() -> bool:
    """Require every reader/delegate import to originate from an immutable copy; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-post-dialog-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_dialog_checks(pin)
        record_snapshot(pin, hashes, result, FEATURE, dialog_snapshot_readers(), ())
        name = dialog_snapshot_filename()
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / name)
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
