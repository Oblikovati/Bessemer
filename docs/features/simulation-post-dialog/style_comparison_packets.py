"""C863 independent native-program outcomes, ordered checks and complete parent views (#166)."""
from collections import Counter
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from style_preferences_projection import raw_style_native_contexts
from style_comparison_inputs import StyleFrame, loaded_style_frame, style_state_packet, STYLE_INPUT_PATHS
from style_comparison_machine import StyleProgram, comparison_native_program, style_comparison_expected
from style_comparison_cases import owned_style_pairs


def style_pair_packet(left: StyleFrame, right: StyleFrame, program: StyleProgram, coverage: set[int]) -> bytes:
    """Keep every original optional fact beside independent completion and paired-check order; e.g. style_pair_packet(left, right, program, coverage)."""
    result = style_comparison_expected(left, right, program, coverage)
    return style_state_packet(left) + style_state_packet(right) + struct.pack('<25I', *result)


def native_style_pairs() -> list[tuple[StyleFrame, StyleFrame]]:
    """Project276 original native parents as stored facts and isolated mutations without runtime defaults; e.g. native_style_pairs()."""
    pairs: list[tuple[StyleFrame, StyleFrame]] = []
    for family, version, raw in raw_style_native_contexts():
        parsed = actual_post_record(family, version, raw); assert parsed['success'] and parsed['end'] == len(raw)
        state = loaded_style_frame(cast(list[dict[str, object]], parsed['spans']))
        pairs.append((state, state))
        for path in STYLE_INPUT_PATHS:
            original = state.get(path)
            if original is None: continue
            value = struct.unpack('<I', original)[0] ^ 0x3f800000
            pairs.append((state, state | {path: struct.pack('<I', value)}))
    return pairs


def style_pair_corpus(pairs: list[tuple[StyleFrame, StyleFrame]], program: StyleProgram, coverage: set[int]) -> bytes:
    """Preserve exact input order and all independent result words; e.g. style_pair_corpus(pairs, program, coverage)."""
    return struct.pack('<II', 1, len(pairs)) + b''.join(style_pair_packet(left, right, program, coverage) for left, right in pairs)


def verify_style_comparison_operation_packets() -> None:
    """Freeze all outer gates, delegated cases, partial facts and IEEE boundaries against original instructions; e.g. verify_style_comparison_operation_packets()."""
    program = comparison_native_program(); coverage: set[int] = set()
    owned = list(owned_style_pairs()); native = native_style_pairs()
    packets = [style_pair_corpus(pairs, program, coverage) for pairs in (owned, native)]
    required = {address for address in program if 0x180044034 <= address < 0x180044203}
    assert coverage == required, (sorted(required - coverage), sorted(coverage - required))
    statuses = Counter(str(struct.unpack_from('<I', packet, offset + 856)[0]) for packet in packets for offset in range(8, len(packet), 956))
    assert set(statuses) == {'0', '1', '2', '3'}, statuses
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'style-comparison-operations.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-style-comparison-operations-native.bin': packets[1]}))
    dialog_baseline('style-comparison-operation-packets-reference.json', dict(owned_pairs=len(owned), native_pairs=len(native),
        visited_semantic_instructions=len(coverage), semantic_addresses=[hex(address) for address in sorted(coverage)],
        status_counts=dict(statuses), packets=summaries, native_target_executed=False,
        oracle='Restricted exact native-body interpreter with independent original C864/C871 callees', stored_facts_as_actual_loaded_state=False))


def style_projection_parent(context: tuple[int, int, bytes], program: StyleProgram) -> bytes:
    """Keep full mandatory bytes, all optional facts and independent self-comparison result; e.g. style_projection_parent(context, program)."""
    family, version, raw = context; parsed = actual_post_record(*context)
    assert parsed['success'] and parsed['end'] == len(raw)
    state = loaded_style_frame(cast(list[dict[str, object]], parsed['spans']))
    return struct.pack('<III', family, version, len(raw)) + raw + style_state_packet(state) + struct.pack('<25I', *style_comparison_expected(state, state, program))


def verify_style_comparison_projection_packets() -> None:
    """Freeze425 owned and276 native complete optional-state views without applying migrations; e.g. verify_style_comparison_projection_packets()."""
    program = comparison_native_program()
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts(); assert (len(owned), len(native)) == (425, 276)
    packets = [struct.pack('<II', 1, len(contexts)) + b''.join(style_projection_parent(context, program) for context in contexts) for contexts in (owned, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'style-comparison-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-style-comparison-projection-native.bin': packets[1]}))
    dialog_baseline('style-comparison-projection-packets-reference.json', dict(owned_contexts=425, native_contexts=276,
        packets=summaries, scalar22c_alias='style.color_220.component_3', scalar2b8_alias='style.color_2ac.component_3',
        original_wire_words_preserved=True, missing_runtime_facts_defaulted=False))
