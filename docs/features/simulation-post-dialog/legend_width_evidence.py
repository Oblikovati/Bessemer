"""C854 B4 width policy with complete native consumers and original admission scope (#166)."""
from collections.abc import Callable
from typing import cast
import importlib
import json
import struct
import unittest
from native_carriers import RESEARCH
from dialog_evidence import dialog_baseline
from legend_word_sources import legend_word_source_transitions


def legend_width_report() -> dict[str, object]:
    """Retain all284 original rows and both complete consumer intervals; e.g. legend_width_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-legend-width-selector-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-854' and report['target_executed'] is False and report['coverage_credit'] == 0
    assert report['field_count'] == 276 and report['value_counts'] == {'1': 262, '2': 14}
    assert len(cast(list[object], report['rows'])) == 284
    assert cast(dict[str, object], report['repeated_tests'])['run'] == 4
    dialog_baseline('legend-width-c854-reference.json', report)
    return report


def verify_legend_width_sources() -> None:
    """Admit only C854's exact historical color-report transition beside complete replay; e.g. verify_legend_width_sources()."""
    historical = 'a1a672e78546d9dbddc7ee55b6315916003c8776dfe7acfd7ea1907495ccb67c'
    transitions = legend_word_source_transitions(legend_width_report(), historical)
    dialog_baseline('legend-width-source-transitions.json', dict(transitions=transitions))


def legend_width_fields(spans: list[dict[str, object]]) -> list[dict[str, object]]:
    """Repeat the raw signed B4 annotation without enum normalization or pixel units; e.g. legend_width_fields(spans)."""
    fields: list[dict[str, object]] = []
    for span in spans:
        if span['label'] != 'legend.word_b4': continue
        raw = bytes.fromhex(cast(str, span['raw_hex']))
        width = cast(int, span['end']) - cast(int, span['offset'])
        assert len(raw) == width == 4, (span, 'expected exactly four signed32 B4 bytes')
        value = struct.unpack('<i', raw)[0]
        fields.append(dict(name='edrawings_legend_width_selector_raw', offset=span['offset'], end=span['end'],
            raw_hex=raw.hex(), signed_value=value, base_width_multiplier=4 if value == 0 else 2 if value == 1 else 1))
    return fields


def require_width_word_rejection(span: dict[str, object]) -> None:
    """Reject truncated and mismatched B4 words; e.g. require_width_word_rejection(span)."""
    try:
        legend_width_fields([span])
    except AssertionError:
        return
    raise AssertionError((span, 'expected complete four-byte width selector rejection'))


def verify_width_word_boundaries() -> None:
    """Keep every raw signed branch, absence and malformed span independently checked; e.g. verify_width_word_boundaries()."""
    assert legend_width_fields([]) == []
    for value, multiplier in ((-2147483648, 1), (-1, 1), (0, 4), (1, 2), (2, 1), (42, 1), (2147483647, 1)):
        raw = struct.pack('<i', value).hex()
        span = dict(label='legend.word_b4', offset=7, end=11, raw_hex=raw)
        assert legend_width_fields([span]) == [dict(name='edrawings_legend_width_selector_raw', offset=7,
            end=11, raw_hex=raw, signed_value=value, base_width_multiplier=multiplier)]
        require_width_word_rejection(span | {'end': 10})
        require_width_word_rejection(span | {'raw_hex': raw[:-2]})
    assert legend_width_fields([dict(label='unrelated')]) == []


def legend_width_functions() -> tuple[Callable[[], None], ...]:
    """Keep full width/title provenance and every original scalar observation together; e.g. legend_width_functions()."""
    from legend_width_bindings import verify_legend_width_bindings, verify_legend_width_rea_captures
    from legend_width_projection import verify_legend_width_native, verify_legend_width_owned
    from legend_width_oracle import verify_width_rational_domain
    from legend_width_packets import verify_width_operation_packets, verify_width_projection_packets
    return (verify_legend_width_sources, verify_width_word_boundaries, verify_legend_width_bindings,
        verify_legend_width_rea_captures, verify_legend_width_native, verify_legend_width_owned,
        verify_width_rational_domain, verify_width_operation_packets, verify_width_projection_packets)


def run_legend_width_checks() -> unittest.TestResult:
    """Retain all eight original catalog/style/color tests beside complete C854 obligations; e.g. run_legend_width_checks()."""
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    checks = (verify_dialog_sources, verify_dialog_field_sources) + legend_width_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style_field_catalog', 'test_cosworks_post_style', 'test_cosworks_post_color_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
