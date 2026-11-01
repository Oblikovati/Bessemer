"""C854 operation expectations and complete optional-selector parent packets (#166)."""
from collections import Counter
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from legend_width_projection import legend_width_observation
from legend_width_cases import WidthCase, WIDTH_SELECTORS, width_font_boundaries, width_operation_cases, width_case_packet


def verify_width_operation_packets() -> None:
    """Freeze every independent rational width and missing-fact ordering; e.g. verify_width_operation_packets()."""
    cases = list(width_operation_cases())
    expected_count = 4 * len(width_font_boundaries()) * len(WIDTH_SELECTORS) + 4096
    assert len(cases) == expected_count
    rows = list(map(width_case_packet, cases))
    statuses = Counter(str(struct.unpack_from('<I', row, 12)[0]) for row in rows)
    assert set(statuses) == {'1', '2', '10', '11'}, statuses
    packet = struct.pack('<II', 1, len(cases)) + b''.join(rows)
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-width-operations.bin': packet})
    dialog_baseline('legend-width-operation-packets-reference.json', dict(cases=len(cases), font_boundaries=len(width_font_boundaries()),
        status_counts=dict(statuses), packets=summaries, oracle='Exact rational stages with monotonic neighboring-binary32 search and midpoint parity',
        ambient_float_state_used=False, native_sse_state_inferred=False))


def parent_width_cases(selector: int | None) -> list[WidthCase]:
    """Retain eight explicit hypothetical font/selector contexts, including changed entry reads; e.g. parent_width_cases(1)."""
    present = 3 if selector is not None else 1
    return [(0, 0, selector or 0), (2, 0, selector or 0), (1, 1, 0), (present, 0, selector or 0),
        (present, 1, selector or 0), (present, -2147483648, selector or 0), (3, 16777217, 0), (3, 16777219, 1)]


def projection_width_context(context: tuple[int, int, bytes]) -> bytes:
    """Keep complete parent bytes, optional stored selector and independent entry facts; e.g. projection_width_context(context)."""
    family, version, raw = context
    fields = legend_width_observation(*context)
    selector = cast(int, fields[0]['signed_value']) if fields else None
    cases = parent_width_cases(selector); assert len(cases) == 8
    prefix = struct.pack('<III', family, version, len(raw)) + raw
    return prefix + struct.pack('<III', int(selector is not None), (selector or 0) & 0xffffffff, len(cases)) + b''.join(map(width_case_packet, cases))


def width_mutant_contexts(owned: list[tuple[int, int, bytes]]) -> list[tuple[int, int, bytes]]:
    """Change only actual B4 bytes across135 complete standalone parents; e.g. width_mutant_contexts(owned)."""
    family, version, raw = next(context for context in owned if context[:2] == (2, 700006))
    offset = cast(int, legend_width_observation(family, version, raw)[0]['offset'])
    values = list(WIDTH_SELECTORS); seed = 0x854b4
    for _ in range(128):
        seed = (1664525 * seed + 1013904223) % (1 << 32); values.append(seed - (1 << 32) if seed >= 1 << 31 else seed)
    contexts: list[tuple[int, int, bytes]] = []
    for selector in values:
        mutated = bytearray(raw); mutated[offset:offset + 4] = struct.pack('<i', selector)
        contexts.append((family, version, bytes(mutated)))
    return contexts


def verify_width_projection_packets() -> None:
    """Freeze425 owned,135 complete selector mutants and276 native parents; e.g. verify_width_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts(); assert (len(owned), len(native)) == (425, 276)
    mutants = width_mutant_contexts(owned); assert len(mutants) == 135
    packets = [struct.pack('<II', 1, len(contexts)) + b''.join(map(projection_width_context, contexts)) for contexts in (owned + mutants, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-width-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-legend-width-projection-native.bin': packets[1]}))
    dialog_baseline('legend-width-projection-packets-reference.json', dict(owned_contexts=425, complete_selector_mutants=135,
        native_contexts=276, hypothetical_contexts_per_parent=8, packets=summaries, stored_to_entry_initialization_inferred=False))
