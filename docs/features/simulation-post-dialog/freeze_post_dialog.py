"""Copied-source complete dialog/legacy foundation with retained field-view obligations (#166)."""
from pathlib import Path
from functools import partial, update_wrapper
from collections.abc import Callable
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
from dialog_rgba_packets import verify_rgba_packets
from colorref_import_evidence import verify_colorref_import_sources, verify_colorref_import_bindings
from colorref_import_packets import verify_colorref_import_packets
from colorref_export_evidence import verify_colorref_export_sources, verify_colorref_export_bindings
from colorref_export_packets import verify_colorref_export_packets
from color_comparison_evidence import verify_color_comparison_sources, verify_color_comparison_bindings
from color_comparison_packets import verify_color_comparison_packets
from legend_comparison_evidence import verify_legend_comparison_sources, verify_legend_comparison_bindings
from legend_comparison_packets import verify_legend_comparison_packets, verify_legend_style_projection
from render_policy_evidence import verify_render_policy_sources, verify_render_policy_bindings
from render_policy_packets import verify_render_policy_packets
from render_policy_projection import verify_render_policy_native, verify_render_policy_projection
from offset_fringe_evidence import verify_offset_fringe_sources, verify_offset_fringe_bindings
from offset_fringe_packets import verify_offset_fringe_packets
from offset_fringe_projection import verify_offset_fringe_native, verify_offset_fringe_projection
from vector_evidence import verify_vector_sources, verify_vector_bindings
from vector_table_bindings import verify_vector_static_tables
from vector_table_constants import verify_vector_table_constants, pinned_vector_data
from vector_packets import verify_vector_packets
from vector_projection import verify_vector_native, verify_vector_projection
from unit_evidence import verify_unit_sources, verify_unit_bindings, verify_unit_flag_bindings
from unit_registry_bindings import verify_unit_registry_bindings
from unit_packets import verify_unit_packets
from unit_projection import verify_unit_native, verify_unit_projection
from local_render_evidence import verify_local_render_sources, verify_local_render_bindings
from local_render_packets import verify_local_render_packets
from local_render_projection import verify_local_render_native, verify_local_render_projection
from style_preferences_evidence import verify_style_preferences_sources
from style_preferences_bindings import verify_style_preferences_bindings
from style_preferences_projection import verify_style_preferences_native, verify_style_preferences_projection
from style_preferences_policy import verify_style_preferences_policy
from vector_style_evidence import verify_vector_style_sources
from vector_style_bindings import verify_vector_style_bindings
from vector_style_projection import verify_vector_style_native, verify_vector_style_projection
from vector_style_decoder_checks import verify_vector_style_decoder_spellings
from vector_style_packets import verify_vector_style_packets
from annotation_evidence import verify_annotation_sources, verify_annotation_catalog
from annotation_bindings import verify_annotation_bindings
from annotation_projection import verify_annotation_native, verify_annotation_projection
from annotation_packets import verify_annotation_packets
from triangle_selector_evidence import verify_triangle_selector_sources, verify_triangle_selector_native, verify_triangle_selector_owned
from triangle_selector_bindings import verify_triangle_selector_bindings, verify_triangle_selector_rea_capture
from triangle_color_packets import verify_triangle_color_packets
from triangle_color_projection import verify_triangle_color_projection
from text_control_evidence import verify_text_control_sources, verify_text_control_native, verify_text_control_owned
from text_control_bindings import verify_text_control_bindings, verify_text_control_rea_capture
from text_selection_packets import verify_post_text_operation_packets
from text_selection_projection import verify_post_text_projection_packets
from plot54_evidence import verify_plot54_sources, verify_plot54_native, verify_plot54_owned
from plot54_bindings import verify_plot54_bindings, verify_plot54_rea_capture
from plot54_plan_packets import verify_plot54_plan_packets
from plot54_projection import verify_plot54_projection_packets
from segment_color_evidence import segment_color_functions, run_segment_color_checks
from mode3_rgb_evidence import mode3_rgb_functions, run_mode3_rgb_checks
from preference_color_evidence import preference_color_functions, run_preference_color_checks
from aux_triangle_evidence import verify_aux_triangle_sources
from aux_triangle_bindings import verify_aux_triangle_bindings
from aux_triangle_projection import verify_aux_triangle_native, verify_aux_triangle_projection
from aux_triangle_packets import verify_aux_triangle_packets
from aux_triangle_plan_packets import verify_aux_triangle_plan_packets
from dialog_projection_shapes import verify_dialog_scalar_shapes
from dialog_parent_views import (verify_dialog_parent_sources, verify_dialog_parent_native, verify_dialog_parent_bindings,
                                 verify_dialog_parent_owned, verify_dialog_parent_branches)


def run_dialog_checks(pin: Path) -> unittest.TestResult:
    """Preserve all original C-770 and integrated legacy-view tests beside fresh full replay; e.g. run_dialog_checks(pin)."""
    specialized = run_style_annotation_checks()
    if specialized is not None: return specialized
    if '--unit-only' in sys.argv: return run_unit_checks()
    if '--vector-only' in sys.argv: return run_vector_checks()
    if '--offset-fringe-only' in sys.argv: return run_offset_fringe_checks()
    if '--render-policy-only' in sys.argv: return run_render_policy_checks()
    if '--legend-comparison-only' in sys.argv: return run_legend_comparison_checks()
    if '--rgba-comparison-only' in sys.argv: return run_color_comparison_checks()
    if '--colorref-export-only' in sys.argv: return run_colorref_export_checks()
    if '--colorref-import-only' in sys.argv: return run_colorref_import_checks()
    if '--rgba-views-only' in sys.argv: return run_rgba_view_checks()
    if '--postinfo-views-only' in sys.argv: return run_postinfo_view_checks()
    if '--parent-views-only' in sys.argv: return run_parent_view_checks()
    if any(flag in sys.argv for flag in ('--resources-only', '--caps-only', '--reference-caps-only')): return run_resource_checks()
    checks = default_dialog_functions(pin)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    append_dialog_unit_tests(suite)
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run_style_annotation_checks() -> unittest.TestResult | None:
    """Dispatch the concrete stored-style modes without changing the full denominator; e.g. run_style_annotation_checks()."""
    modes = (('--preference-colors-only', run_preference_color_checks), ('--mode3-rgb-only', run_mode3_rgb_checks), ('--segment-color-only', run_segment_color_checks), ('--plot54-only', run_plot54_checks), ('--text-controls-only', run_text_control_checks), ('--triangle-selectors-only', run_triangle_selector_checks), ('--aux-triangle-only', run_aux_triangle_checks), ('--annotations-only', run_annotation_checks), ('--vector-style-only', run_vector_style_checks),
             ('--raw-preferences-only', run_raw_preferences_checks), ('--local-render-only', run_local_render_checks))
    for flag, check in modes:
        if flag in sys.argv: return check()
    return None


def append_dialog_unit_tests(suite: unittest.TestSuite) -> None:
    """Keep every original and integrated Source unit module in the default denominator; e.g. append_dialog_unit_tests(suite)."""
    for suffix in ('dialog_layout', 'dialog_prefix', 'info', 'style', 'planes', 'v7_composed_view',
                   'info_fields', 'cutprim_types', 'contour_policy', 'color_fields', 'style_field_catalog'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module('test_cosworks_post_' + suffix)))
    for name in ('test_cosworks_legend_range_fields', 'test_cosworks_deform_scale_fields',
                 'test_cosworks_colorref_import_view', 'test_cosworks_colorref_assignment', 'test_cosworks_colorref_view',
                 'test_cosworks_color_comparison', 'test_cosworks_legend_comparison',
                 'test_cosworks_legend_render_gate', 'test_cosworks_need_model_tess',
                 'test_cosworks_need_offset', 'test_cosworks_fringe_option',
                 'test_cosworks_vector_direction_branch', 'test_cosworks_component_vector_policy',
                 'test_cosworks_component_unit_type', 'test_cosworks_render_safety', 'test_cosworks_aux_triangle_gate', 'test_cosworks_post_style_text_view', 'test_cosworks_style_preference_colors'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))



def default_dialog_functions(pin: Path) -> tuple[Callable[[], None], ...]:
    """Retain the complete ordered default denominator beside targeted replay modes; e.g. default_dialog_functions(pin)."""
    replay = update_wrapper(partial(replay_dialog_native, pin), replay_dialog_native)
    additional = update_wrapper(partial(replay_additional_legacy_dialogs, pin), replay_additional_legacy_dialogs)
    return (verify_dialog_sources, verify_dialog_schema, verify_dialog_installed_bindings,
              verify_dialog_inventory, replay, verify_dialog_legacy_view, additional,
              verify_dialog_field_sources, verify_dialog_catalog_provenance, verify_dialog_field_bindings,
              verify_postinfo_field_views, verify_rgba_native_views, verify_dialog_role_catalog, verify_dialog_owned,
              verify_owned_postinfo_field_views, verify_dialog_operation_packets, verify_dialog_scalar_shapes, verify_dialog_parent_sources,
              verify_dialog_parent_native, verify_dialog_parent_bindings,
              verify_dialog_parent_owned, verify_dialog_parent_branches, verify_dialog_edit_matrix, verify_dialog_resources, verify_dialog_caps, verify_reference_caps, verify_rgba_packets,
              verify_colorref_import_sources, verify_colorref_import_bindings, verify_colorref_import_packets,
              verify_colorref_export_sources, verify_colorref_export_bindings, verify_colorref_export_packets,
              verify_color_comparison_sources, verify_color_comparison_bindings, verify_color_comparison_packets,
              verify_legend_comparison_sources, verify_legend_comparison_bindings, verify_legend_comparison_packets,
              verify_legend_style_projection, verify_render_policy_sources, verify_render_policy_bindings,
              verify_render_policy_native, verify_render_policy_packets, verify_render_policy_projection,
              verify_offset_fringe_sources, verify_offset_fringe_bindings, verify_offset_fringe_native,
              verify_offset_fringe_packets, verify_offset_fringe_projection,
              verify_vector_sources, verify_vector_bindings, verify_vector_static_tables, verify_vector_table_constants,
              verify_vector_packets, verify_vector_native, verify_vector_projection) + unit_model_functions() + local_render_functions() + raw_preferences_functions() + vector_style_functions() + annotation_functions() + aux_triangle_functions() + triangle_selector_functions() + text_control_functions() + plot54_functions() + segment_color_functions() + mode3_rgb_functions() + preference_color_functions()


def plot54_functions() -> tuple[Callable[[], None], ...]:
    """Keep complete C891 sources, installed bindings and all archived inputs together; e.g. plot54_functions()."""
    return (verify_plot54_sources, verify_plot54_bindings, verify_plot54_rea_capture, verify_plot54_native, verify_plot54_owned,
            verify_plot54_plan_packets, verify_plot54_projection_packets)


def run_plot54_checks() -> unittest.TestResult:
    """Retain all original catalog and repeated framing/color tests beside C891 replay; e.g. run_plot54_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources) + plot54_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style_field_catalog', 'test_cosworks_post_style', 'test_cosworks_post_color_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def text_control_functions() -> tuple[Callable[[], None], ...]:
    """Retain complete source/native text and earlier assignment obligations together; e.g. text_control_functions()."""
    return (verify_text_control_sources, verify_text_control_bindings, verify_text_control_rea_capture,
            verify_text_control_native, verify_text_control_owned, verify_post_text_operation_packets, verify_post_text_projection_packets)


def run_text_control_checks() -> unittest.TestResult:
    """Keep all original text-view and current catalog tests beside full native replay; e.g. run_text_control_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources) + text_control_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style_text_view', 'test_cosworks_post_style_field_catalog'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def triangle_selector_functions() -> tuple[Callable[[], None], ...]:
    """Retain the original C889 provenance and complete native selector replay; e.g. triangle_selector_functions()."""
    return (verify_triangle_selector_sources, verify_triangle_selector_bindings, verify_triangle_selector_rea_capture,
            verify_triangle_selector_native, verify_triangle_selector_owned, verify_triangle_color_packets, verify_triangle_color_projection)


def run_triangle_selector_checks() -> unittest.TestResult:
    """Keep historical source checks and the full catalog beside selector evidence; e.g. run_triangle_selector_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources) + triangle_selector_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module('test_cosworks_post_style_field_catalog')))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def aux_triangle_functions() -> tuple[Callable[[], None], ...]:
    """Keep complete caller/helper bindings and all archived groups beside the Source gate; e.g. aux_triangle_functions()."""
    return (verify_aux_triangle_sources, verify_aux_triangle_bindings, verify_aux_triangle_native,
            verify_aux_triangle_projection, verify_aux_triangle_packets, verify_aux_triangle_plan_packets)


def run_aux_triangle_checks() -> unittest.TestResult:
    """Preserve all original gate and current catalog unit tests in the targeted mode; e.g. run_aux_triangle_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources, verify_vector_style_decoder_spellings) + aux_triangle_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_aux_triangle_gate', 'test_cosworks_post_style_field_catalog'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def annotation_functions() -> tuple[Callable[[], None], ...]:
    """Keep full consumer bindings and all annotation cohorts together; e.g. annotation_functions()."""
    return (verify_annotation_sources, verify_annotation_catalog, verify_annotation_bindings,
            verify_annotation_native, verify_annotation_projection, verify_annotation_packets)


def run_annotation_checks() -> unittest.TestResult:
    """Retain current complete catalog tests and decoder rejection checks beside C887; e.g. run_annotation_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources, verify_vector_style_decoder_spellings) + annotation_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module('test_cosworks_post_style_field_catalog')))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def vector_style_functions() -> tuple[Callable[[], None], ...]:
    """Retain complete setter bindings and all stored presence contexts; e.g. vector_style_functions()."""
    return (verify_vector_style_decoder_spellings, verify_vector_style_sources, verify_vector_style_bindings,
            verify_vector_style_native, verify_vector_style_projection, verify_vector_style_packets)


def run_vector_style_checks() -> unittest.TestResult:
    """Keep the current full catalog unit module beside complete vector-group replay; e.g. run_vector_style_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources, verify_style_preferences_bindings) + vector_style_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module('test_cosworks_post_style_field_catalog')))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def raw_preferences_functions() -> tuple[Callable[[], None], ...]:
    """Share the complete raw initialization/preference evidence denominator; e.g. raw_preferences_functions()."""
    return (verify_style_preferences_sources, verify_style_preferences_bindings,
            verify_style_preferences_native, verify_style_preferences_projection, verify_style_preferences_policy)


def run_raw_preferences_checks() -> unittest.TestResult:
    """Require provenance and complete native/owned projections together; e.g. run_raw_preferences_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources) + raw_preferences_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    return unittest.TextTestRunner(verbosity=2).run(suite)


def local_render_functions() -> tuple[Callable[[], None], ...]:
    """Keep all original evidence and complete projections in both gates; e.g. local_render_functions()."""
    return (verify_local_render_sources, verify_local_render_bindings, verify_local_render_packets,
            verify_local_render_native, verify_local_render_projection)


def run_local_render_checks() -> unittest.TestResult:
    """Replay the qualified local predicate without replacing full verification; e.g. run_local_render_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources) + local_render_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module('test_cosworks_render_safety')))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def unit_model_functions() -> tuple[Callable[[], None], ...]:
    """Share the complete unit denominator between targeted and integrated gates; e.g. unit_model_functions()."""
    return (verify_unit_sources, verify_unit_bindings, verify_unit_flag_bindings,
            verify_unit_registry_bindings, verify_unit_packets, verify_unit_native, verify_unit_projection)


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


def run_postinfo_view_checks() -> unittest.TestResult:
    """Replay raw fixed slots and the bounded named policies through copied helpers; e.g. run_postinfo_view_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources, verify_owned_postinfo_field_views)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for suffix in ('info_fields', 'cutprim_types', 'contour_policy'):
        module = importlib.import_module('test_cosworks_post_' + suffix)
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(module))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run_rgba_view_checks() -> unittest.TestResult:
    """Check complete parent/style color denominators through actual copied parsers; e.g. run_rgba_view_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources, verify_rgba_packets)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run_colorref_import_checks() -> unittest.TestResult:
    """Keep original reports, installed leaves and independent integer inputs in the targeted gate; e.g. run_colorref_import_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources, verify_colorref_import_sources,
              verify_colorref_import_bindings, verify_colorref_import_packets)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_colorref_import_view', 'test_cosworks_colorref_assignment'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run_colorref_export_checks() -> unittest.TestResult:
    """Bind the conditional converter and independent projected/excluded cases; e.g. run_colorref_export_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources, verify_colorref_import_sources,
              verify_colorref_export_sources, verify_colorref_export_bindings, verify_colorref_export_packets)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module('test_cosworks_colorref_view')))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run_color_comparison_checks() -> unittest.TestResult:
    """Replay explicit comparison assumptions, complete leaves and paired raw colors; e.g. run_color_comparison_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources, verify_colorref_import_sources,
              verify_color_comparison_sources, verify_color_comparison_bindings, verify_color_comparison_packets)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module('test_cosworks_color_comparison')))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run_legend_comparison_checks() -> unittest.TestResult:
    """Verify complete dependencies, conditional branches and original native identities; e.g. run_legend_comparison_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources, verify_colorref_import_sources,
              verify_color_comparison_sources, verify_color_comparison_bindings,
              verify_legend_comparison_sources, verify_legend_comparison_bindings, verify_legend_comparison_packets,
              verify_legend_style_projection)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module('test_cosworks_legend_comparison')))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run_render_policy_checks() -> unittest.TestResult:
    """Replay all original identities, complete projections and explicit branch matrices; e.g. run_render_policy_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources, verify_colorref_import_sources,
              verify_render_policy_sources, verify_render_policy_bindings, verify_render_policy_native,
              verify_render_policy_packets, verify_render_policy_projection)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_legend_render_gate', 'test_cosworks_need_model_tess'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run_offset_fringe_checks() -> unittest.TestResult:
    """Check complete native input replay, optional state and explicit runtime call-order models; e.g. run_offset_fringe_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources,
              verify_offset_fringe_sources, verify_offset_fringe_bindings, verify_offset_fringe_native,
              verify_offset_fringe_packets, verify_offset_fringe_projection)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_need_offset', 'test_cosworks_fringe_option'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run_vector_checks() -> unittest.TestResult:
    """Check every static table/tail, complete native identity and explicit loaded-state branch; e.g. run_vector_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources, verify_vector_sources, verify_vector_bindings,
              verify_vector_static_tables, verify_vector_table_constants, verify_vector_packets,
              verify_vector_native, verify_vector_projection)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_vector_direction_branch', 'test_cosworks_component_vector_policy'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def render_snapshot_readers() -> tuple[str, ...]:
    """Require the actual complete parent/style and legacy conversion readers beside both decision models; e.g. render_snapshot_readers()."""
    return DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout',
        'cosworks_post_v7_composed_view', 'cosworks_legend_render_gate', 'cosworks_need_model_tess')


def run_unit_checks() -> unittest.TestResult:
    """Verify original unresolved states, explicit alternatives, complete code bindings and context matrices; e.g. run_unit_checks()."""
    checks = (verify_dialog_sources, verify_dialog_field_sources) + unit_model_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module('test_cosworks_component_unit_type')))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def offset_fringe_snapshot_readers() -> tuple[str, ...]:
    """Keep the exact full parent and offset/fringe model readers for targeted replay; e.g. offset_fringe_snapshot_readers()."""
    return DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout',
        'cosworks_need_offset', 'cosworks_fringe_option')


def unit_snapshot_readers() -> tuple[str, ...]:
    """Require the actual pure unit model and complete parent readers for unit-only replay; e.g. unit_snapshot_readers()."""
    return DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_component_unit_type')


def vector_snapshot_readers() -> tuple[str, ...]:
    """Require the real composed model and full parent readers for vector-only replay; e.g. vector_snapshot_readers()."""
    return DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout',
        'cosworks_vector_direction_branch', 'cosworks_component_vector_policy')


def dialog_snapshot_readers() -> tuple[str, ...]:
    """Require exactly the readers exercised by the selected gate; e.g. dialog_snapshot_readers()."""
    if '--preference-colors-only' in sys.argv: return DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_style_field_catalog', 'cosworks_post_color_fields', 'cosworks_style_preference_colors', 'cosworks_colorref_import_view')
    if any(flag in sys.argv for flag in ('--mode3-rgb-only', '--segment-color-only', '--plot54-only')): return DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_style_field_catalog', 'cosworks_post_color_fields')
    if '--text-controls-only' in sys.argv: return DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_style_field_catalog', 'cosworks_post_style_text_view')
    if '--aux-triangle-only' in sys.argv: return DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_style_field_catalog', 'cosworks_aux_triangle_gate')
    if any(flag in sys.argv for flag in ('--triangle-selectors-only', '--annotations-only')): return DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_style_field_catalog')
    if '--vector-style-only' in sys.argv: return DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_style_field_catalog', 'cosworks_colorref_import_view')
    if '--raw-preferences-only' in sys.argv: return DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout')
    if '--local-render-only' in sys.argv: return DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_render_safety')
    if '--unit-only' in sys.argv: return unit_snapshot_readers()
    if '--vector-only' in sys.argv: return vector_snapshot_readers()
    if '--offset-fringe-only' in sys.argv: return offset_fringe_snapshot_readers()
    if '--render-policy-only' in sys.argv: return render_snapshot_readers()
    if '--legend-comparison-only' in sys.argv: return ('cosworks_legend_comparison', 'cosworks_color_comparison', 'cosworks_post_style_layout', 'cosworks_post_color_fields')
    if '--rgba-comparison-only' in sys.argv: return ('cosworks_color_comparison',)
    if '--colorref-export-only' in sys.argv: return ('cosworks_colorref_view',)
    if '--colorref-import-only' in sys.argv: return ('cosworks_colorref_import_view', 'cosworks_colorref_view')
    if '--rgba-views-only' in sys.argv: return DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_color_fields')
    if '--postinfo-views-only' in sys.argv: return ('cosworks_post_info_fields', 'cosworks_post_contour_policy')
    if '--parent-views-only' in sys.argv: return ('cosworks_legend_range_fields', 'cosworks_deform_scale_fields')
    return default_snapshot_readers()


def default_snapshot_readers() -> tuple[str, ...]:
    """Keep resource-only and full replay readers separate from targeted view readers; e.g. default_snapshot_readers()."""
    readers = DIALOG_READERS + ('cosworks_post_info_layout', 'cosworks_post_style_layout')
    if any(flag in sys.argv for flag in ('--resources-only', '--caps-only', '--reference-caps-only')):
        return readers
    return readers + ('cosworks_post_v7_composed_view', 'cfb', 'swcontainer', 'cosworks_colorref_import_view', 'cosworks_colorref_view', 'cosworks_color_comparison', 'cosworks_legend_comparison', 'cosworks_legend_render_gate', 'cosworks_need_model_tess', 'cosworks_need_offset', 'cosworks_fringe_option', 'cosworks_vector_direction_branch', 'cosworks_component_vector_policy', 'cosworks_component_unit_type', 'cosworks_render_safety', 'cosworks_aux_triangle_gate', 'cosworks_post_style_text_view', 'cosworks_style_preference_colors')


def dialog_snapshot_filename() -> str:
    """Keep partial replay evidence separate from the mandatory full snapshot; e.g. dialog_snapshot_filename()."""
    modes = (('--preference-colors-only', 'preference-color-source-snapshot.json'), ('--mode3-rgb-only', 'mode3-rgb-source-snapshot.json'), ('--segment-color-only', 'segment-color-source-snapshot.json'), ('--plot54-only', 'plot54-source-snapshot.json'), ('--text-controls-only', 'text-control-source-snapshot.json'), ('--triangle-selectors-only', 'triangle-selector-source-snapshot.json'), ('--aux-triangle-only', 'aux-triangle-source-snapshot.json'), ('--annotations-only', 'annotation-source-snapshot.json'), ('--vector-style-only', 'vector-style-source-snapshot.json'), ('--raw-preferences-only', 'raw-preferences-source-snapshot.json'),
             ('--local-render-only', 'local-render-source-snapshot.json'),
             ('--resources-only', 'resource-source-snapshot.json'),
             ('--caps-only', 'cap-source-snapshot.json'),
             ('--reference-caps-only', 'reference-cap-source-snapshot.json'),
             ('--parent-views-only', 'parent-view-source-snapshot.json'),
             ('--postinfo-views-only', 'postinfo-view-source-snapshot.json'),
             ('--rgba-views-only', 'rgba-view-source-snapshot.json'),
             ('--colorref-import-only', 'colorref-import-source-snapshot.json'),
             ('--colorref-export-only', 'colorref-export-source-snapshot.json'),
             ('--rgba-comparison-only', 'rgba-comparison-source-snapshot.json'),
             ('--legend-comparison-only', 'legend-comparison-source-snapshot.json'),
             ('--render-policy-only', 'render-policy-source-snapshot.json'),
             ('--offset-fringe-only', 'offset-fringe-source-snapshot.json'),
             ('--vector-only', 'vector-source-snapshot.json'),
             ('--unit-only', 'unit-source-snapshot.json'))
    for flag, filename in modes:
        if flag in sys.argv: return filename
    return 'source-snapshot.json'


def run() -> bool:
    """Require every reader/delegate import to originate from an immutable copy; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-post-dialog-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        vector_data = '--vector-only' in sys.argv or dialog_snapshot_filename() == 'source-snapshot.json'
        if vector_data: pinned_vector_data(pin, copy=True)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_dialog_checks(pin)
        if vector_data: pinned_vector_data(pin, copy=False)
        record_snapshot(pin, hashes, result, FEATURE, dialog_snapshot_readers(), ())
        name = dialog_snapshot_filename()
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / name)
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
