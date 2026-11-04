"""C852 additional stored count/precision consumers without inferred color or contour identities (#166)."""
from collections.abc import Callable
from typing import cast
import importlib
import json
import struct
import unittest
from native_carriers import RESEARCH
from dialog_evidence import dialog_baseline
from legend_word_sources import legend_word_source_transitions

LEGEND_3D_LABELS = ('legend.word_14', 'legend.word_bc')


def legend_3d_report() -> dict[str, object]:
    """Retain every original C852 row and both complete consumer intervals; e.g. legend_3d_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-legend-3d-consumers-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-852' and report['target_executed'] is False
    assert report['coverage_credit'] == 0
    assert report['field_counts'] == {label: 276 for label in LEGEND_3D_LABELS}
    assert len(cast(list[object], report['rows'])) == 284
    assert cast(dict[str, object], report['repeated_tests'])['run'] == 4
    dialog_baseline('legend-3d-c852-reference.json', report)
    return report


def verify_legend_3d_sources() -> None:
    """Admit only the original C852 replay-input transition beside complete current replay; e.g. verify_legend_3d_sources()."""
    historical = '1d296569880aa715cf2a33301d586df68b4fc8d6d0de65398557ffbc2e83317e'
    transitions = legend_word_source_transitions(legend_3d_report(), historical)
    dialog_baseline('legend-3d-source-transitions.json', dict(transitions=transitions))


def legend_3d_fields(spans: list[dict[str, object]]) -> list[dict[str, object]]:
    """Repeat the original two raw signed-word annotations without rendering constraints; e.g. legend_3d_fields(spans)."""
    fields: list[dict[str, object]] = []
    for span in spans:
        if span['label'] not in LEGEND_3D_LABELS: continue
        raw = bytes.fromhex(cast(str, span['raw_hex']))
        width = cast(int, span['end']) - cast(int, span['offset'])
        assert len(raw) == width == 4, (span, 'expected exactly four signed32 count/precision bytes')
        fields.append(dict(label=span['label'], offset=span['offset'], end=span['end'],
            raw_hex=raw.hex(), signed_value=struct.unpack('<i', raw)[0]))
    return fields


def require_3d_word_rejection(span: dict[str, object]) -> None:
    """Reject truncated or mismatched scalar spans; e.g. require_3d_word_rejection(span)."""
    try:
        legend_3d_fields([span])
    except AssertionError:
        return
    raise AssertionError((span, 'expected complete four-byte count/precision rejection'))


def verify_3d_word_boundaries() -> None:
    """Preserve absence and signed extrema without count-safety or precision-range rules; e.g. verify_3d_word_boundaries()."""
    assert legend_3d_fields([]) == []
    for label in LEGEND_3D_LABELS:
        for value in (-2147483648, -1, 0, 1, 2147483647):
            raw = struct.pack('<i', value).hex()
            span = dict(label=label, offset=7, end=11, raw_hex=raw)
            assert legend_3d_fields([span]) == [dict(label=label, offset=7, end=11, raw_hex=raw, signed_value=value)]
            require_3d_word_rejection(span | {'end': 10})
            require_3d_word_rejection(span | {'raw_hex': raw[:-2]})
    assert legend_3d_fields([dict(label='unrelated')]) == []


def legend_3d_functions() -> tuple[Callable[[], None], ...]:
    """Keep full consumer provenance and original raw-word replay together; e.g. legend_3d_functions()."""
    from legend_3d_bindings import verify_legend_3d_bindings, verify_legend_3d_rea_capture
    from legend_3d_projection import verify_legend_3d_native, verify_legend_3d_owned
    from legend_3d_integer_bindings import verify_3d_integer_request_bindings
    from legend_3d_packets import verify_3d_operation_packets, verify_3d_projection_packets
    return (verify_legend_3d_sources, verify_3d_word_boundaries, verify_legend_3d_bindings,
        verify_legend_3d_rea_capture, verify_legend_3d_native, verify_legend_3d_owned,
        verify_3d_integer_request_bindings, verify_3d_operation_packets, verify_3d_projection_packets)


def run_legend_3d_checks() -> unittest.TestResult:
    """Retain all eight original catalog/style/color tests beside C852 obligations; e.g. run_legend_3d_checks()."""
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    checks = (verify_dialog_sources, verify_dialog_field_sources) + legend_3d_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style_field_catalog', 'test_cosworks_post_style', 'test_cosworks_post_color_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
