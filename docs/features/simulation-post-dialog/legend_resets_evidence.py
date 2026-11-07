"""C857 explicit reader zero assignments, never blanket missing-field defaults (#166)."""
from collections.abc import Callable
from typing import cast
import importlib
import json
import unittest
from native_carriers import RESEARCH
from dialog_evidence import dialog_baseline
from legend_word_sources import legend_word_source_transitions

LEGEND_RESET_LABELS = ('legend.word_c0', 'legend.word_c4', 'legend.word_c8')


def legend_resets_report() -> dict[str, object]:
    """Retain all276 original accepted rows and the complete legend reader; e.g. legend_resets_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-legend-read-resets-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-857' and report['target_executed'] is False and report['coverage_credit'] == 0
    assert report['streams'] == len(cast(list[object], report['rows'])) == 276
    assert report['reset_counts'] == {label: 116 for label in LEGEND_RESET_LABELS}
    assert cast(dict[str, object], report['tests'])['run'] == 2
    dialog_baseline('legend-resets-c857-reference.json', report)
    return report


def verify_legend_resets_sources() -> None:
    """Admit only C857's exact historical color-report transition beside complete current replay; e.g. verify_legend_resets_sources()."""
    historical = 'a1a672e78546d9dbddc7ee55b6315916003c8776dfe7acfd7ea1907495ccb67c'
    transitions = legend_word_source_transitions(legend_resets_report(), historical)
    dialog_baseline('legend-resets-source-transitions.json', dict(transitions=transitions))


def require_reset_model_rejection(version: object) -> None:
    """Require explicit signed32 version context without raw-class inference; e.g. require_reset_model_rejection(True)."""
    model = importlib.import_module('cosworks_legend_read_resets')
    try:
        model.forced_zero_fields(version)
    except ValueError:
        return
    raise AssertionError((version, 'expected explicit integer version700006..2147483647 rejection'))


def verify_legend_reset_boundaries() -> None:
    """Check each independent version interval and all input-domain exclusions; e.g. verify_legend_reset_boundaries()."""
    model = importlib.import_module('cosworks_legend_read_resets')
    cases = ((700006, 7), (700007, 7), (840007, 7), (840008, 6), (840009, 6), (840010, 0), (2147483647, 0))
    for version, mask in cases:
        expected = {label: 0 for index, label in enumerate(LEGEND_RESET_LABELS) if mask & (1 << index)}
        actual = model.forced_zero_fields(version)
        assert actual == expected and all(type(value) is int for value in actual.values()), (version, actual, expected)
    for version in (0, -1, 700005, 2147483648, True, False, 840008.0, '840008', None):
        require_reset_model_rejection(version)
    dialog_baseline('legend-resets-policy-reference.json', dict(cases=[dict(version=version, mask=mask) for version, mask in cases],
        reset_fields=list(LEGEND_RESET_LABELS), blanket_missing_field_defaults=False,
        raw_class_conversion_inferred=False, target_executed=False))


def legend_resets_functions() -> tuple[Callable[[], None], ...]:
    """Keep full reader provenance, exact reset policy and original native rows together; e.g. legend_resets_functions()."""
    from legend_resets_bindings import verify_legend_resets_bindings, verify_legend_resets_rea_capture
    from legend_resets_projection import verify_legend_resets_native, verify_legend_resets_owned
    from legend_resets_packets import verify_reset_operation_packets, verify_reset_projection_packets
    return (verify_legend_resets_sources, verify_legend_resets_bindings, verify_legend_resets_rea_capture,
        verify_legend_reset_boundaries, verify_legend_resets_native, verify_legend_resets_owned,
        verify_reset_operation_packets, verify_reset_projection_packets)


def run_legend_resets_checks() -> unittest.TestResult:
    """Retain both original reset tests, reader-routing fixtures and eight catalog/style/color tests; e.g. run_legend_resets_checks()."""
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    from dialog_reader_selection_checks import verify_dialog_reader_selection
    checks = (verify_dialog_sources, verify_dialog_field_sources, verify_dialog_reader_selection) + legend_resets_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_legend_read_resets', 'test_cosworks_post_style_field_catalog', 'test_cosworks_post_style', 'test_cosworks_post_color_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
