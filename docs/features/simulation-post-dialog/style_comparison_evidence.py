"""C863 complete conditional style-comparison dependencies and preserved control words (#166)."""
from collections.abc import Callable
from typing import cast
import importlib
import json
import unittest
from native_carriers import RESEARCH
from dialog_evidence import dialog_baseline
from legend_word_sources import legend_word_source_transitions

STYLE_COMPARISON_WORDS: tuple[str, ...] = tuple('style.word_' + offset for offset in ('1b8', '21c', '230', '1ec', '1f0', '204', '208', '234', '238'))
STYLE_COMPARISON_NATIVE_COUNTS: dict[str, dict[str, int]] = {
    'style.word_238': {'0': 250, '1': 26}, 'style.word_234': {'0': 252, '1': 24},
    'style.word_21c': {'0': 276}, 'style.word_1b8': {'1': 255, '0': 9, '2': 9, '3': 3},
    'style.word_1f0': {'0': 276}, 'style.word_230': {'1': 221, '0': 55},
    'style.word_1ec': {'0': 159, '1': 1}, 'style.word_204': {'0': 159, '1': 1}, 'style.word_208': {'0': 160},
}


def style_comparison_report() -> dict[str, object]:
    """Retain all284 original identities,including eight rejected candidates,and both methods; e.g. style_comparison_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-post-style-compare-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-863' and report['target_executed'] is False and report['coverage_credit'] == 0
    rows = cast(list[dict[str, object]], report['rows'])
    assert len(rows) == 284 and sum(row['selected_version'] is not None for row in rows) == 276
    assert report['word_value_counts'] == STYLE_COMPARISON_NATIVE_COUNTS
    assert cast(dict[str, object], report['repeated_tests'])['run'] == 4
    dialog_baseline('style-comparison-c863-reference.json', report)
    return report


def verify_style_comparison_sources() -> None:
    """Require both original probes/decompilations and the exact historical color-report transition; e.g. verify_style_comparison_sources()."""
    transitions = legend_word_source_transitions(style_comparison_report(), 'fb43aea838f7321ed97185507c51f7cff99a4e5ada77f476fc7ec22812332418')
    dialog_baseline('style-comparison-source-transitions.json', dict(transitions=transitions))


def style_comparison_functions() -> tuple[Callable[[], None], ...]:
    """Keep complete native control flow beside exact original/owned word observations; e.g. style_comparison_functions()."""
    from style_comparison_bindings import verify_style_comparison_bindings, verify_style_comparison_rea
    from style_comparison_projection import verify_style_comparison_native, verify_style_comparison_owned
    from style_comparison_packets import verify_style_comparison_operation_packets, verify_style_comparison_projection_packets
    return (verify_style_comparison_sources, verify_style_comparison_bindings, verify_style_comparison_rea,
        verify_style_comparison_native, verify_style_comparison_owned,
        verify_style_comparison_operation_packets, verify_style_comparison_projection_packets)


def run_style_comparison_checks() -> unittest.TestResult:
    """Retain routing provenance and all four original style/color tests with C863 checks; e.g. run_style_comparison_checks()."""
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    from dialog_reader_selection_checks import verify_dialog_reader_selection
    checks = (verify_dialog_sources, verify_dialog_field_sources, verify_dialog_reader_selection) + style_comparison_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style', 'test_cosworks_post_color_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
