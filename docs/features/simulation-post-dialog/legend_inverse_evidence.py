"""C860 A8/preference zero-test evidence, distinct from lossless inversion (#166)."""
from collections.abc import Callable
from typing import cast
import importlib
import json
import unittest
from native_carriers import RESEARCH
from dialog_evidence import dialog_baseline
from legend_word_sources import legend_word_source_transitions


def legend_inverse_report() -> dict[str, object]:
    """Retain both complete methods and all276 original A8 words; e.g. legend_inverse_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-legend-inverse-preference-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-860' and report['target_executed'] is False and report['coverage_credit'] == 0
    assert report['field_count'] == len(cast(list[object], report['rows'])) == 276
    assert report['stored_values'] == {'0': 252, '1': 24} and cast(dict[str, object], report['tests'])['run'] == 2
    dialog_baseline('legend-inverse-c860-reference.json', report)
    return report


def verify_legend_inverse_sources() -> None:
    """Require the original model, both probes/decompilations and one exact historical replay transition; e.g. verify_legend_inverse_sources()."""
    transitions = legend_word_source_transitions(legend_inverse_report(), 'f434779aed0aad1e2954e260c3beed66d5ac5d5d98eb541db09c72b15eec35ea')
    dialog_baseline('legend-inverse-source-transitions.json', dict(transitions=transitions))


def require_inverse_rejection(word: object) -> None:
    """Keep booleans, nonintegers and signed32 overflow outside the original relation; e.g. require_inverse_rejection(True)."""
    try:
        importlib.import_module('cosworks_legend_inverse_preference').zero_test(word)
    except ValueError:
        return
    raise AssertionError((word, 'expected explicit signed32 input rejection'))


def verify_inverse_relation() -> None:
    """Check zero/nonzero, noninvertibility and exact model input boundaries; e.g. verify_inverse_relation()."""
    model = importlib.import_module('cosworks_legend_inverse_preference')
    words = (-2147483648, -2, -1, 0, 1, 2, 2147483647)
    results = [model.zero_test(word) for word in words]
    assert results == [0, 0, 0, 1, 0, 0, 0]
    assert all(type(result) is int for result in results)
    assert model.zero_test(model.zero_test(2)) == 1 != 2
    for word in (True, False, None, '0', 0.0, -2147483649, 2147483648): require_inverse_rejection(word)
    dialog_baseline('legend-inverse-policy-reference.json', dict(words=words, results=results,
        bijective=False, bitwise_complement=False, raw_words_normalized=False, universal_setter_policy=False))


def legend_inverse_functions() -> tuple[Callable[[], None], ...]:
    """Keep original conversion tests beside full bindings and complete parent replay; e.g. legend_inverse_functions()."""
    from legend_inverse_bindings import verify_inverse_bindings, verify_inverse_rea
    from legend_inverse_projection import verify_inverse_native, verify_inverse_owned
    return (verify_legend_inverse_sources, verify_inverse_relation, verify_inverse_bindings,
        verify_inverse_rea, verify_inverse_native, verify_inverse_owned)


def run_legend_inverse_checks() -> unittest.TestResult:
    """Retain original two relation tests and four style/color checks with routing provenance; e.g. run_legend_inverse_checks()."""
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    from dialog_reader_selection_checks import verify_dialog_reader_selection
    checks = (verify_dialog_sources, verify_dialog_field_sources, verify_dialog_reader_selection) + legend_inverse_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_legend_inverse_preference', 'test_cosworks_post_style', 'test_cosworks_post_color_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
