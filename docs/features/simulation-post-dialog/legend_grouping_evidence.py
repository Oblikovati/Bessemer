"""C858 locale grouping evidence preserves conditional enablement and unknown locale state (#166)."""
from collections.abc import Callable
from typing import cast
import json
import unittest
import importlib
from native_carriers import RESEARCH
from dialog_evidence import dialog_baseline
from legend_word_sources import legend_word_source_transitions


def legend_grouping_report() -> dict[str, object]:
    """Retain the original C858 rows, exact converter and complete caller; e.g. legend_grouping_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-legend-grouping-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-858' and report['target_executed'] is False and report['coverage_credit'] == 0
    rows = cast(list[dict[str, object]], report['rows'])
    assert len(rows) == 284 and sum(row['selected_version'] is not None for row in rows) == 276
    assert report['field_count'] == 160 and report['value_counts'] == {'1': 84, '0': 76}
    assert cast(dict[str, object], report['repeated_tests'])['run'] == 4
    dialog_baseline('legend-grouping-c858-reference.json', report)
    return report


def verify_legend_grouping_sources() -> None:
    """Require every original pin apart from the exact shared color replay transition; e.g. verify_legend_grouping_sources()."""
    transitions = legend_word_source_transitions(legend_grouping_report(), 'a1a672e78546d9dbddc7ee55b6315916003c8776dfe7acfd7ea1907495ccb67c')
    dialog_baseline('legend-grouping-source-transitions.json', dict(transitions=transitions))


def legend_grouping_functions() -> tuple[Callable[[], None], ...]:
    """Keep complete native bindings beside complete parent replay; e.g. legend_grouping_functions()."""
    from legend_grouping_bindings import verify_grouping_bindings, verify_grouping_rea
    from legend_grouping_projection import verify_grouping_native, verify_grouping_owned
    from legend_grouping_fixtures import verify_grouping_fixture_projection
    return (verify_legend_grouping_sources, verify_grouping_bindings, verify_grouping_rea,
        verify_grouping_native, verify_grouping_owned, verify_grouping_fixture_projection)


def run_legend_grouping_checks() -> unittest.TestResult:
    """Keep the routing baseline and original style/color tests beside C858 checks; e.g. run_legend_grouping_checks()."""
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    from dialog_reader_selection_checks import verify_dialog_reader_selection
    checks = (verify_dialog_sources, verify_dialog_field_sources, verify_dialog_reader_selection) + legend_grouping_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style', 'test_cosworks_post_color_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
