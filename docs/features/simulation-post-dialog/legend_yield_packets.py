"""Independent C850 call requests and complete owned/native parent projections (#166)."""
from collections import Counter
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from legend_yield_projection import legend_yield_observation
from legend_palette_projection import legend_palette_observation
from legend_yield_cases import YieldCase, yield_operation_cases, expected_yield_words, yield_case_packet


def verify_yield_operation_packets() -> None:
    """Freeze13632 independent signed-boundary/generated call plans and dependency failures; e.g. verify_yield_operation_packets()."""
    cases = list(yield_operation_cases())
    assert len(cases) == 13632
    counts = Counter(str(expected_yield_words(case)[0]) for case in cases)
    assert set(counts) == {'0', '1', '2', '10', '11', '12', '13'}, counts
    assert counts['2'] > 4106 and counts['0'] > 268, counts
    packet = struct.pack('<II', 1, len(cases)) + b''.join(map(yield_case_packet, cases))
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-yield-operations.bin': packet})
    dialog_baseline('legend-yield-operation-packets-reference.json', dict(cases=len(cases), status_counts=dict(counts), packets=summaries,
        oracle='Independent unbounded integer comparisons, modulo32 triple, signed64 byte offsets and byte-bound native call order',
        interpolation_modeled=False, runtime_split_derived=False, post_suffix_state_inferred=False))


def yield_parent_cases(family: int, version: int, raw: bytes) -> list[YieldCase]:
    """Read complete D8/reversal views, then supply eight explicitly hypothetical runtime contexts; e.g. yield_parent_cases(2, 810019, raw)."""
    colors = legend_yield_observation(family, version, raw)
    reverse = [field for field in legend_palette_observation(family, version, raw) if field['name'] == 'reverse_base_palette_raw']
    assert len(colors) <= 1 and len(reverse) <= 1
    bits = struct.unpack('<4I', bytes.fromhex(cast(str, colors[0]['raw_hex']))) if colors else (0, 0, 0, 0)
    word = cast(int, reverse[0]['signed_value']) if reverse else 0
    contexts = ((-2, -5, None), (-2, 10, None), (-1, 10, None), (0, 10, None),
        (0, 10, (0, 9, 0)), (2, 10, (42, -19, -71)),
        (715827883, 2147483647, (-2147483648, 2147483647, 2147483647)), (None, 10, None))
    cases: list[YieldCase] = []
    for index, count, after in contexts:
        mask = int(bool(colors)) | (int(bool(reverse)) << 1) | (int(index is not None) << 2) | (int(after is not None) << 3)
        cases.append((mask, *bits, word, index or 0, count, *(after or (0, 0, 0))))
    return cases


def yield_projection_context(context: tuple[int, int, bytes]) -> bytes:
    """Retain the complete parent beside every independent local call expectation; e.g. yield_projection_context(context)."""
    family, version, raw = context
    cases = yield_parent_cases(family, version, raw)
    assert len(cases) == 8
    return struct.pack('<III', family, version, len(raw)) + raw + struct.pack('<I', len(cases)) + b''.join(map(yield_case_packet, cases))


def yield_color_patterns() -> list[tuple[int, ...]]:
    """Cover exceptional/unequal channels and sixty-four generated four-word colors; e.g. yield_color_patterns()."""
    patterns = [(0, 0, 0, 0), (0x3f800000,) * 4, (0x7fc00001, 0x7f800001, 0x7f800000, 0xff800000),
        (0x80000000, 1, 0x7fffff, 0x7f7fffff), (0xffffffff, 0xbf800000, 0x3f000000, 0x3f800001),
        (0x3f800000, 0, 0x3f800000, 0)]
    seed = 0xd850
    for _ in range(64):
        color: list[int] = []
        for _ in range(4):
            seed = (1664525 * seed + 1013904223) % (1 << 32)
            color.append(seed)
        patterns.append(tuple(color))
    assert len(patterns) == 70
    return patterns


def yield_pattern_contexts(owned: list[tuple[int, int, bytes]]) -> list[tuple[int, int, bytes]]:
    """Mutate only D8 and reversal within210 complete standalone parents; e.g. yield_pattern_contexts(owned)."""
    family, version, original = next(context for context in owned if context[0] == 2 and legend_yield_observation(*context))
    color = legend_yield_observation(family, version, original)[0]
    reverse = next(field for field in legend_palette_observation(family, version, original) if field['name'] == 'reverse_base_palette_raw')
    color_start, reverse_start = cast(int, color['offset']), cast(int, reverse['offset'])
    contexts: list[tuple[int, int, bytes]] = []
    for bits in yield_color_patterns():
        for word in (-2147483648, 0, 2147483647):
            mutated = bytearray(original)
            mutated[color_start:color_start + 16] = struct.pack('<4I', *bits)
            mutated[reverse_start:reverse_start + 4] = struct.pack('<i', word)
            contexts.append((family, version, bytes(mutated)))
    return contexts


def verify_yield_projection_packets() -> None:
    """Freeze425 owned,210 complete mutants and276 native parents without deriving runtime state; e.g. verify_yield_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert (len(owned), len(native)) == (425, 276)
    mutants = yield_pattern_contexts(owned)
    assert len(mutants) == 210
    packets = [struct.pack('<II', 1, len(contexts)) + b''.join(map(yield_projection_context, contexts)) for contexts in (owned + mutants, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-yield-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-legend-yield-projection-native.bin': packets[1]}))
    dialog_baseline('legend-yield-projection-packets-reference.json', dict(owned_contexts=425, native_contexts=276,
        complete_color_reversal_mutants=210, hypothetical_contexts_per_parent=8, packets=summaries,
        runtime_split_derived=False, post_suffix_state_inferred=False, native_helper_executed=False))
