"""C848 raw palette controls retain original native and reader qualifications (#166)."""
from collections.abc import Callable
from typing import cast
import importlib
import json
import unittest
from native_carriers import RESEARCH
from dialog_evidence import dialog_baseline
from legend_word_sources import legend_word_source_transitions


def legend_palette_report() -> dict[str, object]:
    """Retain every original C848 observation and rejected stream; e.g. legend_palette_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-legend-color-policy-fields-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-848' and report['target_executed'] is False
    assert report['coverage_credit'] == 0 and len(cast(list[object], report['rows'])) == 284
    dialog_baseline('legend-palette-c848-reference.json', report)
    return report


def verify_legend_palette_sources() -> None:
    """Admit only C848's exact historical replay-input transition beside complete views; e.g. verify_legend_palette_sources()."""
    historical = 'f434779aed0aad1e2954e260c3beed66d5ac5d5d98eb541db09c72b15eec35ea'
    transitions = legend_word_source_transitions(legend_palette_report(), historical)
    dialog_baseline('legend-palette-source-transitions.json', dict(transitions=transitions))


def legend_palette_functions() -> tuple[Callable[[], None], ...]:
    """Keep complete consumers and every original stored-control observation together; e.g. legend_palette_functions()."""
    from legend_palette_bindings import verify_legend_palette_bindings, verify_legend_palette_rea_capture
    from legend_palette_projection import verify_legend_palette_native, verify_legend_palette_owned
    return (verify_legend_palette_sources, verify_legend_palette_bindings, verify_legend_palette_rea_capture,
        verify_legend_palette_native, verify_legend_palette_owned)


def run_legend_palette_checks() -> unittest.TestResult:
    """Retain both original raw-control tests and parent/catalog/color checks; e.g. run_legend_palette_checks()."""
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    checks = (verify_dialog_sources, verify_dialog_field_sources) + legend_palette_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style_field_catalog', 'test_cosworks_post_style', 'test_cosworks_post_color_fields', 'test_cosworks_legend_color_policy_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
