"""C849 cell-count role and complete height/title/tick consumer provenance (#166)."""
from collections.abc import Callable
from typing import cast
import importlib
import json
import struct
import unittest
from native_carriers import RESEARCH
from dialog_evidence import dialog_baseline
from legend_word_sources import legend_word_source_transitions


def legend_height_report() -> dict[str, object]:
    """Retain all284 original rows and every native consumer qualification; e.g. legend_height_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-legend-height-fields-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-849' and report['target_executed'] is False
    assert report['coverage_credit'] == 0 and report['field_count'] == 276
    assert len(cast(list[object], report['rows'])) == 284
    dialog_baseline('legend-height-c849-reference.json', report)
    return report


def verify_legend_height_sources() -> None:
    """Admit only the exact historical color-input transition with complete fresh observations; e.g. verify_legend_height_sources()."""
    historical = 'f434779aed0aad1e2954e260c3beed66d5ac5d5d98eb541db09c72b15eec35ea'
    transitions = legend_word_source_transitions(legend_height_report(), historical)
    dialog_baseline('legend-height-source-transitions.json', dict(transitions=transitions))


def legend_height_fields(spans: list[dict[str, object]]) -> list[dict[str, object]]:
    """Repeat C849's literal raw signed-word annotation without calling its audit; e.g. legend_height_fields(spans)."""
    fields: list[dict[str, object]] = []
    for span in spans:
        if span['label'] != 'legend.word_14': continue
        raw = bytes.fromhex(cast(str, span['raw_hex']))
        width = cast(int, span['end']) - cast(int, span['offset'])
        assert len(raw) == width == 4, (span, 'expected exactly four signed32 bytes')
        fields.append(dict(name='legend_height_cell_count', offset=span['offset'], end=span['end'],
            raw_hex=raw.hex(), signed_value=struct.unpack('<i', raw)[0]))
    return fields


def require_height_word_rejection(span: dict[str, object]) -> None:
    """Keep incomplete words and mismatched spans invalid; e.g. require_height_word_rejection(span)."""
    try:
        legend_height_fields([span])
    except AssertionError:
        return
    raise AssertionError((span, 'expected complete four-byte height-word rejection'))


def verify_height_word_boundaries() -> None:
    """Preserve all signed extremes and explicit absence without positivity/default rules; e.g. verify_height_word_boundaries()."""
    assert legend_height_fields([]) == []
    for value in (-2147483648, -1, 0, 10, 12, 2147483647):
        raw = struct.pack('<i', value).hex()
        span = dict(label='legend.word_14', offset=7, end=11, raw_hex=raw)
        assert legend_height_fields([span]) == [dict(name='legend_height_cell_count', offset=7, end=11, raw_hex=raw, signed_value=value)]
        require_height_word_rejection(span | {'end': 10})
        require_height_word_rejection(span | {'raw_hex': raw[:-2]})
    assert legend_height_fields([dict(label='unrelated')]) == []


def legend_height_functions() -> tuple[Callable[[], None], ...]:
    """Keep complete native provenance and all raw field observations together; e.g. legend_height_functions()."""
    from legend_height_bindings import verify_legend_height_bindings, verify_legend_height_rea_capture
    from legend_height_projection import verify_legend_height_native, verify_legend_height_owned
    from legend_height_packets import verify_height_operation_packets, verify_height_projection_packets
    return (verify_legend_height_sources, verify_legend_height_bindings, verify_legend_height_rea_capture,
        verify_height_word_boundaries, verify_legend_height_native, verify_legend_height_owned,
        verify_height_operation_packets, verify_height_projection_packets)


def run_legend_height_checks() -> unittest.TestResult:
    """Retain all four original style/color tests plus catalog and height obligations; e.g. run_legend_height_checks()."""
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    checks = (verify_dialog_sources, verify_dialog_field_sources) + legend_height_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style_field_catalog', 'test_cosworks_post_style', 'test_cosworks_post_color_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
