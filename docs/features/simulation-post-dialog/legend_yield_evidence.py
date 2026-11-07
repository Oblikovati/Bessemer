"""C850 stored D8 identity and complete runtime yield/split consumer evidence (#166)."""
from collections.abc import Callable
from typing import cast
import importlib
import json
import unittest
from native_carriers import RESEARCH
from dialog_evidence import dialog_baseline
from legend_word_sources import legend_word_source_transitions


def legend_yield_report() -> dict[str, object]:
    """Retain all284 original rows and complete adjustment instructions; e.g. legend_yield_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-legend-yield-color-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-850' and report['target_executed'] is False
    assert report['coverage_credit'] == 0 and report['color_count'] == 212
    assert len(cast(list[object], report['rows'])) == 284
    dialog_baseline('legend-yield-c850-reference.json', report)
    return report


def verify_legend_yield_sources() -> None:
    """Admit only C850's exact historical color-input transition beside full view replay; e.g. verify_legend_yield_sources()."""
    historical = 'a1a672e78546d9dbddc7ee55b6315916003c8776dfe7acfd7ea1907495ccb67c'
    transitions = legend_word_source_transitions(legend_yield_report(), historical)
    dialog_baseline('legend-yield-source-transitions.json', dict(transitions=transitions))


def legend_yield_functions() -> tuple[Callable[[], None], ...]:
    """Keep both complete consumers and original raw color observations together; e.g. legend_yield_functions()."""
    from legend_yield_bindings import verify_legend_yield_bindings, verify_legend_yield_rea_capture
    from legend_yield_projection import verify_legend_yield_native, verify_legend_yield_owned
    from legend_yield_packets import verify_yield_operation_packets, verify_yield_projection_packets
    return (verify_legend_yield_sources, verify_legend_yield_bindings, verify_legend_yield_rea_capture,
        verify_legend_yield_native, verify_legend_yield_owned, verify_yield_operation_packets, verify_yield_projection_packets)


def run_legend_yield_checks() -> unittest.TestResult:
    """Preserve all four original style/color tests and complete catalog obligations; e.g. run_legend_yield_checks()."""
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    checks = (verify_dialog_sources, verify_dialog_field_sources) + legend_yield_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style_field_catalog', 'test_cosworks_post_style', 'test_cosworks_post_color_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
