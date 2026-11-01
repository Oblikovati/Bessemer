"""C847 raw position-factor roles and qualified native pixel-consumer evidence (#166)."""
from collections.abc import Callable
from typing import cast
import importlib
import json
import unittest
from native_carriers import RESEARCH
from dialog_evidence import dialog_baseline
from legend_word_sources import legend_word_source_transitions


def legend_position_report() -> dict[str, object]:
    """Retain all284 original accepted/rejected position-view rows; e.g. legend_position_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-legend-position-fields-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-847' and report['target_executed'] is False
    assert report['coverage_credit'] == 0 and len(cast(list[object], report['rows'])) == 284
    dialog_baseline('legend-position-c847-reference.json', report)
    return report


def verify_legend_position_sources() -> None:
    """Admit only C847's exact historical color-report transition beside complete replay; e.g. verify_legend_position_sources()."""
    report = legend_position_report()
    historical = '71db7fc57456b4661b5c22b256e61a32dc0f6a043abcba387f22491df0cc4bda'
    transitions = legend_word_source_transitions(report, historical)
    dialog_baseline('legend-position-source-transitions.json', dict(transitions=transitions))


def legend_position_functions() -> tuple[Callable[[], None], ...]:
    """Keep full native consumer bindings and all complete-body observations together; e.g. legend_position_functions()."""
    from legend_position_bindings import verify_legend_position_bindings, verify_legend_position_rea_capture
    from legend_position_projection import verify_legend_position_native, verify_legend_position_owned
    from legend_position_packets import verify_legend_position_projection_packets
    return (verify_legend_position_sources, verify_legend_position_bindings, verify_legend_position_rea_capture,
        verify_legend_position_native, verify_legend_position_owned, verify_legend_position_projection_packets)


def run_legend_position_checks() -> unittest.TestResult:
    """Retain both original position tests beside parent/catalog/color obligations; e.g. run_legend_position_checks()."""
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    checks = (verify_dialog_sources, verify_dialog_field_sources) + legend_position_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style_field_catalog', 'test_cosworks_post_style', 'test_cosworks_post_color_fields', 'test_cosworks_legend_position_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
