"""C846 number-format inputs retain raw integers and all original replay scopes (#166)."""
from collections.abc import Callable
from typing import cast
import importlib
import json
import unittest
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline


def legend_format_report() -> dict[str, object]:
    """Retain all284 original successful/rejected rows; e.g. legend_format_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-legend-number-format-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-846' and report['target_executed'] is False
    assert report['coverage_credit'] == 0 and len(cast(list[object], report['rows'])) == 284
    dialog_baseline('legend-format-c846-reference.json', report)
    return report


def verify_legend_format_sources() -> None:
    """Admit only the exact historical color-report transition beside complete row replay; e.g. verify_legend_format_sources()."""
    transitions: list[dict[str, str]] = []
    for original, expected in cast(dict[str, str], legend_format_report()['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/research/'), original
        actual = digest((SOURCE / relative).read_bytes())
        if actual == expected: continue
        assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json', relative
        assert (expected, actual) == ('a1a672e78546d9dbddc7ee55b6315916003c8776dfe7acfd7ea1907495ccb67c', '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d'), (expected, actual)
        transitions.append(dict(path=relative, original=expected, current=actual))
    assert len(transitions) == 1, transitions
    dialog_baseline('legend-format-source-transitions.json', dict(transitions=transitions))


def legend_format_functions() -> tuple[Callable[[], None], ...]:
    """Keep full source/binary and complete native/owned obligations together; e.g. legend_format_functions()."""
    from legend_format_bindings import verify_legend_format_bindings, verify_legend_format_rea_capture
    from legend_format_projection import verify_legend_format_native, verify_legend_format_owned
    from legend_format_packets import verify_legend_format_operation_packets, verify_legend_format_projection_packets
    return (verify_legend_format_sources, verify_legend_format_bindings, verify_legend_format_rea_capture,
        verify_legend_format_native, verify_legend_format_owned, verify_legend_format_operation_packets,
        verify_legend_format_projection_packets)


def run_legend_format_checks() -> unittest.TestResult:
    """Retain original number-format tests and parent/catalog tests; e.g. run_legend_format_checks()."""
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    checks = (verify_dialog_sources, verify_dialog_field_sources) + legend_format_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style_field_catalog', 'test_cosworks_post_style', 'test_cosworks_post_color_fields', 'test_cosworks_legend_number_format'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
