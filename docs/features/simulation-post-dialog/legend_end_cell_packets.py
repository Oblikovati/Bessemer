"""C853 explicit read-time operation packets and complete stored parent projections (#166)."""
from collections import Counter
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from legend_end_cell_projection import end_cell_observation
from legend_end_cell_cases import EndCellCase, END_CELL_FLAGS, END_CELL_COLORS, end_cell_operation_cases, expected_end_cell_words, end_cell_case_packet


def verify_end_cell_operation_packets() -> None:
    """Freeze7232 independent boundary/presence/generated local outcomes; e.g. verify_end_cell_operation_packets()."""
    cases = list(end_cell_operation_cases()); assert len(cases) == 7232
    statuses = Counter(str(expected_end_cell_words(case)[0]) for case in cases)
    assert set(statuses) == {'1', '2', '10', '11', '12'}, statuses
    packet = struct.pack('<II', 1, len(cases)) + b''.join(map(end_cell_case_packet, cases))
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-end-cell-operations.bin': packet})
    dialog_baseline('legend-end-cell-operation-packets-reference.json', dict(cases=len(cases), status_counts=dict(statuses),
        packets=summaries, oracle='Independent two-predicate truth table and exact raw RGB copy identity',
        alpha_compositing_inferred=False, file_to_runtime_initialization_inferred=False))


def stored_end_cell_words(family: int, version: int, raw: bytes) -> tuple[int, ...]:
    """Encode exact optional flag/color presence without runtime defaults; e.g. stored_end_cell_words(2, 890103, raw)."""
    view = end_cell_observation(family, version, raw)
    fields, colors = (cast(list[dict[str, object]], view[key]) for key in ('fields', 'colors'))
    mask = int(bool(fields)) | (int(bool(colors)) << 1)
    flag = cast(int, fields[0]['signed_value']) & 0xffffffff if fields else 0
    color = struct.unpack('<4I', bytes.fromhex(cast(str, colors[0]['raw_hex']))) if colors else (0, 0, 0, 0)
    return (mask, flag, *color)


def parent_end_cell_cases(stored: tuple[int, ...]) -> list[EndCellCase]:
    """Keep eight explicitly hypothetical contexts, including changed post-helper colors; e.g. parent_end_cell_cases(stored)."""
    mask, flag, *color = stored
    parent_mask = 1 | ((mask & 1) << 1) | ((mask & 2) << 1)
    return [(0, 0, 0, *color), (1, 0, 0, *color), (1, 1, 0, *color), (3, 1, 0, *color),
        (parent_mask, 1, flag, *color), (1 | ((mask & 2) << 1) | 2, 0xffffffff, 1, *color),
        (7, 0x80000000, 0x80000000, *END_CELL_COLORS[4]), (7, 0x7fffffff, 1, *END_CELL_COLORS[7])]


def projection_end_cell_context(context: tuple[int, int, bytes]) -> bytes:
    """Preserve complete parent bytes, all optional stored words and eight independent contexts; e.g. projection_end_cell_context(context)."""
    family, version, raw = context
    stored = stored_end_cell_words(*context)
    cases = parent_end_cell_cases(stored); assert len(cases) == 8
    return struct.pack('<III', family, version, len(raw)) + raw + struct.pack('<7I', *stored, len(cases)) + b''.join(map(end_cell_case_packet, cases))


def end_cell_mutant_contexts(owned: list[tuple[int, int, bytes]]) -> list[tuple[int, int, bytes]]:
    """Retain complete framing while changing actual E8/EC spans across184 independent mutants; e.g. end_cell_mutant_contexts(owned)."""
    context = next(context for context in owned if context[0] == 2 and end_cell_observation(*context)['fields'])
    family, version, raw = context
    view = end_cell_observation(*context)
    offsets = [cast(int, cast(list[dict[str, object]], view[key])[0]['offset']) for key in ('fields', 'colors')]
    pairs = [(flag, color) for flag in END_CELL_FLAGS for color in END_CELL_COLORS]
    seed = 0x853ec
    for _ in range(128):
        words: list[int] = []
        for _ in range(5):
            seed = (1664525 * seed + 1013904223) % (1 << 32); words.append(seed)
        pairs.append((words[0], tuple(words[1:])))
    mutants: list[tuple[int, int, bytes]] = []
    for flag, color in pairs:
        mutated = bytearray(raw)
        for offset, payload in zip(offsets, (struct.pack('<I', flag), struct.pack('<4I', *color)), strict=True):
            mutated[offset:offset + len(payload)] = payload
        mutants.append((family, version, bytes(mutated)))
    return mutants


def verify_end_cell_projection_packets() -> None:
    """Freeze425 owned,184 complete raw-bit mutants and276 native parents; e.g. verify_end_cell_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts(); assert (len(owned), len(native)) == (425, 276)
    mutants = end_cell_mutant_contexts(owned); assert len(mutants) == 184
    packets = [struct.pack('<II', 1, len(contexts)) + b''.join(map(projection_end_cell_context, contexts)) for contexts in (owned + mutants, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-end-cell-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-legend-end-cell-projection-native.bin': packets[1]}))
    dialog_baseline('legend-end-cell-projection-packets-reference.json', dict(owned_contexts=425, complete_flag_color_mutants=184,
        native_contexts=276, hypothetical_contexts_per_parent=8, packets=summaries,
        unchanged_read_time_flag_or_color_inferred=False, native_target_executed=False))
