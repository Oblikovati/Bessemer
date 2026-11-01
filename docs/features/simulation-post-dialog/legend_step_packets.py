"""C851 exact runtime outcomes and complete versioned FC parent packets (#166)."""
from collections import Counter
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from legend_step_projection import legend_step_observation
from legend_step_cases import STEP_RUNTIME_CONTEXTS, step_operation_cases, expected_step_words, step_case_packet


def verify_step_operation_packets() -> None:
    """Freeze102272 exact runtime decisions, raw BEC results and reached-read masks; e.g. verify_step_operation_packets()."""
    cases = list(step_operation_cases())
    assert len(cases) == 102272
    masks = Counter(str(expected_step_words(case)[1]) for case in cases)
    assert set(masks) == {'3', '11', '15', '31', '63'}, masks
    packet = struct.pack('<II', 1, len(cases)) + b''.join(map(step_case_packet, cases))
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-step-operations.bin': packet})
    dialog_baseline('legend-step-operation-packets-reference.json', dict(cases=len(cases), reached_masks=dict(masks), packets=summaries,
        oracle='Original copied C851 explicit signed32 Source policy; independent native reached-read predicates',
        native_pointer_accessed=False, stored_fc_consumed=False, raw_bec_normalized=False))


def step_fc_patterns() -> list[int]:
    """Keep signed extrema and256 generated FC patterns without positivity rules; e.g. step_fc_patterns()."""
    words = [-2147483648, -2147483647, -65536, -2, -1, 0, 1, 2, 20, 65536, 2147483646, 2147483647]
    seed = 0x851fc
    for _ in range(256):
        seed = (1664525 * seed + 1013904223) % (1 << 32)
        words.append(struct.unpack('<i', struct.pack('<I', seed))[0])
    return words


def step_pattern_contexts(owned: list[tuple[int, int, bytes]]) -> list[tuple[int, int, bytes]]:
    """Mutate only actual FC bytes in268 complete standalone parents; e.g. step_pattern_contexts(owned)."""
    family, version, raw = next(context for context in owned if context[0] == 2 and legend_step_observation(*context))
    fields = legend_step_observation(family, version, raw)
    assert len(fields) == 1
    offset = cast(int, fields[0]['offset'])
    return [(family, version, raw[:offset] + struct.pack('<i', word) + raw[offset + 4:]) for word in step_fc_patterns()]


def step_projection_context(context: tuple[int, int, bytes]) -> bytes:
    """Retain complete parent bytes/FC presence and eight independent hypothetical runtime policies; e.g. step_projection_context(context)."""
    family, version, raw = context
    fields = legend_step_observation(family, version, raw)
    assert len(fields) <= 1
    present = int(bool(fields))
    fc = cast(int, fields[0]['signed_value']) if fields else 0
    header = struct.pack('<III', family, version, len(raw)) + raw
    return header + struct.pack('<III', present, fc & 0xffffffff, len(STEP_RUNTIME_CONTEXTS)) + b''.join(map(step_case_packet, STEP_RUNTIME_CONTEXTS))


def verify_step_projection_packets() -> None:
    """Freeze425 owned,268 complete FC mutants and276 native parents without inferring runtime facts; e.g. verify_step_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert (len(owned), len(native)) == (425, 276)
    mutants = step_pattern_contexts(owned)
    assert len(mutants) == 268
    packets = [struct.pack('<II', 1, len(contexts)) + b''.join(map(step_projection_context, contexts)) for contexts in (owned + mutants, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-step-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-legend-step-projection-native.bin': packets[1]}))
    dialog_baseline('legend-step-projection-packets-reference.json', dict(owned_contexts=425, native_contexts=276,
        complete_fc_mutants=268, hypothetical_contexts_per_parent=8, packets=summaries,
        runtime_words_inferred=False, fc_used_as_availability=False, native_target_executed=False))
