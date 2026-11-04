"""Complete C851 scalar layout packets with actual FC spans and explicit helper-boundary assumptions (#166)."""
from collections import Counter
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from legend_step_projection import legend_step_observation
from legend_step_packets import step_pattern_contexts
from legend_step_layout_cases import LayoutCase, layout_operation_cases, expected_layout_words, layout_case_packet


def verify_layout_operation_packets() -> None:
    """Freeze35468 scalar layouts and every reached missing-input path; e.g. verify_layout_operation_packets()."""
    cases = list(layout_operation_cases())
    assert len(cases) == 35468
    statuses = Counter(str(expected_layout_words(case)[0]) for case in cases)
    assert set(statuses) == {'0', '1', *(str(value) for value in range(10, 21))}, statuses
    packet = struct.pack('<II', 1, len(cases)) + b''.join(map(layout_case_packet, cases))
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-step-layout-operations.bin': packet})
    dialog_baseline('legend-step-layout-operation-packets-reference.json', dict(cases=len(cases), status_counts=dict(statuses), packets=summaries,
        oracle='Independent unbounded signed arithmetic with modulo32 normalization, truncation toward zero and exact bound native read order',
        helper_completion_inferred=False, post_helper_fc_inferred=False, pixels_validated=False))


def layout_parent_cases(family: int, version: int, raw: bytes) -> list[LayoutCase]:
    """Supply eight explicit hypothetical local contexts beside actual stored FC presence; e.g. layout_parent_cases(2, 890103, raw)."""
    fields = legend_step_observation(family, version, raw)
    assert len(fields) <= 1
    fc = cast(int, fields[0]['signed_value']) if fields else 0
    present = int(bool(fields))
    base = (2046 | present, 1, fc, 17, 100, 20, 121, 100, 7, 0, 0, 0)
    cases = [(present, 0, fc, 0, 0, 0, 0, 0, 0, 0, 0, 0), base,
        base[:9] + (1, 0, -1), base[:10] + (42, 0)]
    no_second = list(cases[-1]); no_second[0] &= ~1024
    cases[-1] = tuple(no_second)
    no_after = list(base); no_after[0] &= ~1984
    no_fc = list(base); no_fc[0] &= ~256
    need_second = list(base); need_second[0] &= ~1024
    missing_initial = (present, 1, fc, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    return cases + [tuple(no_after), tuple(no_fc), tuple(need_second), missing_initial]


def layout_projection_context(context: tuple[int, int, bytes]) -> bytes:
    """Retain the entire parent and all eight independently expected projections; e.g. layout_projection_context(context)."""
    family, version, raw = context
    cases = layout_parent_cases(family, version, raw)
    assert len(cases) == 8
    return struct.pack('<III', family, version, len(raw)) + raw + struct.pack('<I', len(cases)) + b''.join(map(layout_case_packet, cases))


def verify_layout_projection_packets() -> None:
    """Freeze425 owned,268 complete FC mutants and276 native parents without guessing helper outcomes; e.g. verify_layout_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert (len(owned), len(native)) == (425, 276)
    mutants = step_pattern_contexts(owned)
    assert len(mutants) == 268
    packets = [struct.pack('<II', 1, len(contexts)) + b''.join(map(layout_projection_context, contexts)) for contexts in (owned + mutants, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-step-layout-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-legend-step-layout-projection-native.bin': packets[1]}))
    dialog_baseline('legend-step-layout-projection-packets-reference.json', dict(owned_contexts=425, native_contexts=276,
        complete_fc_mutants=268, hypothetical_contexts_per_parent=8, packets=summaries,
        helper_completion_inferred=False, after_helper_state_inferred=False, native_target_executed=False))
