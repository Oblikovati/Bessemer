"""C851 stored FC layout role and separate explicit-word runtime policy (#166)."""
from collections.abc import Callable
from typing import cast
import importlib
import json
import struct
import unittest
from native_carriers import RESEARCH
from dialog_evidence import dialog_baseline
from legend_word_sources import legend_word_source_transitions


def legend_step_report() -> dict[str, object]:
    """Retain all284 original rows and five complete consumers; e.g. legend_step_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-legend-step-policy-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-851' and report['target_executed'] is False
    assert report['coverage_credit'] == 0 and report['field_count'] == 109
    assert report['value_counts'] == {'0': 109}
    assert len(cast(list[object], report['rows'])) == 284
    assert cast(dict[str, object], report['tests'])['run'] == 2
    dialog_baseline('legend-step-c851-reference.json', report)
    return report


def verify_legend_step_sources() -> None:
    """Admit only C851's exact historical replay-input transition; e.g. verify_legend_step_sources()."""
    historical = 'f453f208044bf122aead1896da9dedb54f44fb2d38a90df49ae5580e15f8cabc'
    transitions = legend_word_source_transitions(legend_step_report(), historical)
    dialog_baseline('legend-step-source-transitions.json', dict(transitions=transitions))


def legend_step_fields(spans: list[dict[str, object]]) -> list[dict[str, object]]:
    """Repeat the original signed FC annotation without assigning visibility or step number; e.g. legend_step_fields(spans)."""
    fields: list[dict[str, object]] = []
    for span in spans:
        if span['label'] != 'legend.word_fc': continue
        raw = bytes.fromhex(cast(str, span['raw_hex']))
        width = cast(int, span['end']) - cast(int, span['offset'])
        assert len(raw) == width == 4, (span, 'expected exactly four signed32 FC bytes')
        value = struct.unpack('<i', raw)[0]
        fields.append(dict(name='mode_shape_step_layout_switch_raw', offset=span['offset'], end=span['end'],
            raw_hex=raw.hex(), signed_value=value, consumer_nonzero=value != 0))
    return fields


def require_step_word_rejection(span: dict[str, object]) -> None:
    """Reject incomplete or mismatched FC words; e.g. require_step_word_rejection(span)."""
    try:
        legend_step_fields([span])
    except AssertionError:
        return
    raise AssertionError((span, 'expected complete four-byte FC-word rejection'))


def verify_step_word_boundaries() -> None:
    """Preserve signed extrema, nonzero patterns and absence without defaults; e.g. verify_step_word_boundaries()."""
    assert legend_step_fields([]) == []
    for value in (-2147483648, -1, 0, 1, 42, 2147483647):
        raw = struct.pack('<i', value).hex()
        span = dict(label='legend.word_fc', offset=7, end=11, raw_hex=raw)
        assert legend_step_fields([span]) == [dict(name='mode_shape_step_layout_switch_raw', offset=7,
            end=11, raw_hex=raw, signed_value=value, consumer_nonzero=value != 0)]
        require_step_word_rejection(span | {'end': 10})
        require_step_word_rejection(span | {'raw_hex': raw[:-2]})
    assert legend_step_fields([dict(label='unrelated')]) == []


def legend_step_functions() -> tuple[Callable[[], None], ...]:
    """Keep stored layout and runtime availability evidence separate within the full gate; e.g. legend_step_functions()."""
    from legend_step_bindings import verify_legend_step_bindings, verify_legend_step_rea_capture
    from legend_step_projection import verify_legend_step_native, verify_legend_step_owned
    return (verify_legend_step_sources, verify_step_word_boundaries, verify_legend_step_bindings,
        verify_legend_step_rea_capture, verify_legend_step_native, verify_legend_step_owned)


def run_legend_step_checks() -> unittest.TestResult:
    """Retain both original policy tests and all prior catalog/style/color tests; e.g. run_legend_step_checks()."""
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    checks = (verify_dialog_sources, verify_dialog_field_sources) + legend_step_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style_field_catalog', 'test_cosworks_post_style',
            'test_cosworks_post_color_fields', 'test_cosworks_legend_step_policy'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
