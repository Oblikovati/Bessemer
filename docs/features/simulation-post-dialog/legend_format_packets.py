"""Freeze bounded operations and their complete owned/native archive parents (#166)."""
from collections import Counter
from itertools import chain
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets
from dialog_owned import dialog_owned_cases
from style_preferences_projection import raw_style_native_contexts
from legend_format_projection import legend_format_observation
from legend_format_cases import (LegendFormatCase, expected_legend_format, legend_case_packet,
    legend_primary_cases, legend_alternate_contexts, legend_numeric_cases, legend_magnitude_patterns)
from legend_format_kernel import verify_legend_format_kernel


def verify_legend_format_operation_packets() -> None:
    """Retain independent expected results for every reached/missing guard and magnitude class; e.g. verify_legend_format_operation_packets()."""
    verify_legend_format_kernel()
    cases = list(chain(legend_primary_cases(), legend_alternate_contexts(), legend_numeric_cases()))
    raw = struct.pack('<II', 1, len(cases)) + b''.join(map(legend_case_packet, cases))
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-format-operations.bin': raw})
    statuses = Counter(str(expected_legend_format(case)[0]) for case in cases)
    dialog_baseline('legend-format-operation-packets-reference.json', dict(cases=len(cases),
        magnitude_patterns=len(legend_magnitude_patterns()), status_counts=dict(statuses), packets=summaries,
        oracle='Unchanged copied Source format constructor; independent binary32 widening and binary64 lower-bound comparison',
        native_execution=False, additional_stored_byte_credit=0))


def legend_projection_cases(view: dict[str, object]) -> list[LegendFormatCase]:
    """Keep each original raw word/presence and seven explicit hypothetical calls; e.g. legend_projection_cases(view)."""
    observed = {field['name']: field['signed_value'] for field in cast(list[dict[str, object]], view['fields'])}
    names = ('number_format_mode', 'number_format_precision', 'alternate_number_format_enabled_raw', 'alternate_number_format_selector_raw')
    mask = sum(1 << index for index, name in enumerate(names) if name in observed)
    mode, precision, enabled, selector = [cast(int, observed.get(name, 0)) for name in names]
    calls = [(0, 0, 0, 0), (1, 0, 1, 0x3f800000), (1, 1, 0, 0), (1, 2, 1, 0x3f800000),
        (1, 2, 1, 0x3a83126e), (1, 2, 1, 0x7fa00042), (1, 2, 1, 0xc47a0000)]
    return [(operation, mask, mode, precision, enabled, selector, pointer, present, bits)
        for operation, pointer, present, bits in calls]


def legend_projection_packet(context: tuple[int, int, bytes]) -> bytes:
    """Retain complete EOF parents, explicit absence and full expected calls; e.g. legend_projection_packet((2, 700006, raw))."""
    family, version, raw = context
    view = legend_format_observation(family, version, raw)
    cases = legend_projection_cases(view)
    return struct.pack('<III', family, version, len(raw)) + raw + struct.pack('<I', len(cases)) + b''.join(map(legend_case_packet, cases))


def verify_legend_format_projection_packets() -> None:
    """Project all425 owned and276 native contexts without inventing actual invocation facts; e.g. verify_legend_format_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert (len(owned), len(native)) == (425, 276)
    packets = [struct.pack('<II', 1, len(contexts)) + b''.join(map(legend_projection_packet, contexts)) for contexts in (owned, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-format-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-legend-format-projection-native.bin': packets[1]}))
    dialog_baseline('legend-format-projection-packets-reference.json', dict(owned_contexts=425, native_contexts=276,
        calls_per_context=7, packets=summaries, actual_invocation_known=False, missing_fields_defaulted=False))
