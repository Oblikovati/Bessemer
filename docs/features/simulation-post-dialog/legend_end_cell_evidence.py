"""C853 versioned E8/EC end-cell roles without inferred activation or alpha behavior (#166)."""
from collections.abc import Callable
from typing import cast
import importlib
import json
import struct
import unittest
from native_carriers import RESEARCH
from dialog_evidence import dialog_baseline
from legend_word_sources import legend_word_source_transitions


def legend_end_cell_report() -> dict[str, object]:
    """Retain all284 original rows and the complete2D consumer interval; e.g. legend_end_cell_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-legend-end-cell-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-853' and report['target_executed'] is False and report['coverage_credit'] == 0
    assert report['flag_count'] == report['color_count'] == 109
    assert report['flag_values'] == {'0': 109}
    assert report['color_values'] == {'0000803f00000000000000000000803f': 101, '0000003f0000003f0000003f0000803f': 8}
    assert len(cast(list[object], report['rows'])) == 284
    assert cast(dict[str, object], report['repeated_tests'])['run'] == 4
    dialog_baseline('legend-end-cell-c853-reference.json', report)
    return report


def verify_legend_end_cell_sources() -> None:
    """Admit only C853's exact historical color-report transition beside complete replay; e.g. verify_legend_end_cell_sources()."""
    historical = 'b73b25360f0ddc45fe826d40a83f9ddc1faa714d82b444e4f475f7a29c16f860'
    transitions = legend_word_source_transitions(legend_end_cell_report(), historical)
    dialog_baseline('legend-end-cell-source-transitions.json', dict(transitions=transitions))


def end_cell_flag_fields(spans: list[dict[str, object]]) -> list[dict[str, object]]:
    """Repeat the literal signed E8 annotation without assigning a UI visibility label; e.g. end_cell_flag_fields(spans)."""
    fields: list[dict[str, object]] = []
    for span in spans:
        if span['label'] != 'legend.word_e8': continue
        raw = bytes.fromhex(cast(str, span['raw_hex']))
        width = cast(int, span['end']) - cast(int, span['offset'])
        assert len(raw) == width == 4, (span, 'expected exactly four signed32 E8 bytes')
        value = struct.unpack('<i', raw)[0]
        fields.append(dict(name='end_cell_extension_enabled_raw', offset=span['offset'], end=span['end'],
            raw_hex=raw.hex(), signed_value=value, consumer_nonzero=value != 0))
    return fields


def require_end_cell_flag_rejection(span: dict[str, object]) -> None:
    """Reject truncated and mismatched E8 words; e.g. require_end_cell_flag_rejection(span)."""
    try:
        end_cell_flag_fields([span])
    except AssertionError:
        return
    raise AssertionError((span, 'expected complete four-byte end-cell flag rejection'))


def verify_end_cell_flag_boundaries() -> None:
    """Preserve all nonboolean signed patterns and absence without defaults; e.g. verify_end_cell_flag_boundaries()."""
    assert end_cell_flag_fields([]) == []
    for value in (-2147483648, -1, 0, 1, 42, 2147483647):
        raw = struct.pack('<i', value).hex()
        span = dict(label='legend.word_e8', offset=7, end=11, raw_hex=raw)
        assert end_cell_flag_fields([span]) == [dict(name='end_cell_extension_enabled_raw', offset=7,
            end=11, raw_hex=raw, signed_value=value, consumer_nonzero=value != 0)]
        require_end_cell_flag_rejection(span | {'end': 10})
        require_end_cell_flag_rejection(span | {'raw_hex': raw[:-2]})
    assert end_cell_flag_fields([dict(label='unrelated')]) == []


def legend_end_cell_functions() -> tuple[Callable[[], None], ...]:
    """Keep full native provenance and every original flag/color interpretation together; e.g. legend_end_cell_functions()."""
    from legend_end_cell_bindings import verify_legend_end_cell_bindings, verify_legend_end_cell_rea_capture
    from legend_end_cell_projection import verify_legend_end_cell_native, verify_legend_end_cell_owned
    return (verify_legend_end_cell_sources, verify_end_cell_flag_boundaries, verify_legend_end_cell_bindings,
        verify_legend_end_cell_rea_capture, verify_legend_end_cell_native, verify_legend_end_cell_owned)


def run_legend_end_cell_checks() -> unittest.TestResult:
    """Retain all eight catalog/style/color tests beside C853's complete obligations; e.g. run_legend_end_cell_checks()."""
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    checks = (verify_dialog_sources, verify_dialog_field_sources) + legend_end_cell_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style_field_catalog', 'test_cosworks_post_style', 'test_cosworks_post_color_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
