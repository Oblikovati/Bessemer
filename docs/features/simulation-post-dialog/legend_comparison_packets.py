"""Independent conditional legend comparisons and native report identity replay (#166)."""
from collections.abc import Iterator
from itertools import product
from typing import cast
import importlib
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_field_evidence import dialog_field_reports
from dialog_operation_packets import persist_dialog_packets
from legend_comparison_evidence import legend_comparison_report
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from dialog_operation_packets import accepted_dialog_native_cases
from color_comparison_packets import PATTERNS

WORD_OFFSETS = (8, 12, 16, 20, 0xa8, 0xb4, 0xb8, 0xbc, 0xc0, 0xc4, 0xc8, 0xd4, 0xe8, 0xfc, 0xac, 0xb0)
COLOR_OFFSETS = (0x18, 0x28, 0x38, 0x48, 0x58, 0x68, 0x78, 0x88, 0x98, 0xd8, 0xec, 0x100)
LoadedLegend = dict[str, bytes]


def baseline_legend(count: int, mode: int, flags: tuple[int, int, int]) -> LoadedLegend:
    """Start from a complete actual helper layout, then supply explicit runtime controls; e.g. baseline_legend(5, 0, (0, 0, 0))."""
    parser = importlib.import_module('cosworks_post_style_layout')
    helper = importlib.import_module('cosworks_legend_comparison')
    state = cast(LoadedLegend, helper.loaded_fields(parser.parse(bytes(2048), 890702, require_eof=False)['spans']))
    for offset, word in zip((0x10, 0xb8, 0xd4, 0xe8, 0xfc), (count, mode, *flags), strict=True):
        state[f'legend.word_{offset:x}'] = struct.pack('<i', word)
    return state


def legend_mutations(state: LoadedLegend) -> Iterator[LoadedLegend]:
    """Change every complete stored control and every RGBA component independently; e.g. list(legend_mutations(state))."""
    for path, raw in sorted(state.items()):
        for offset in range(0, len(raw), 4):
            replacement = bytearray(raw)
            struct.pack_into('<I', replacement, offset, struct.unpack_from('<I', raw, offset)[0] ^ 0x3f800000)
            yield state | {path: bytes(replacement)}


def owned_legend_pairs() -> Iterator[tuple[LoadedLegend, LoadedLegend]]:
    """Cross active-count, format and color flags, then retain signed and missing-state boundaries; e.g. list(owned_legend_pairs())."""
    for count, mode, flags in product((0, 1, 5, 9), (0, 1, -1), product((0, -1), repeat=3)):
        state = baseline_legend(count, mode, flags)
        yield state, state
        yield from ((state, changed) for changed in legend_mutations(state))
    yield from legend_boundary_pairs()
    yield from legend_float_pairs()


def legend_float_pairs() -> Iterator[tuple[LoadedLegend, LoadedLegend]]:
    """Exercise unordered, signed-zero, infinity and subnormal behavior through every float consumer; e.g. list(legend_float_pairs())."""
    state = baseline_legend(9, 0, (1, 1, 1))
    for offset, left, right in product((0xac, 0xb0), PATTERNS, PATTERNS):
        path = f'legend.word_{offset:x}'
        yield state | {path: struct.pack('<I', left)}, state | {path: struct.pack('<I', right)}
    for offset, channel, bits in product(COLOR_OFFSETS, range(4), PATTERNS):
        raw = bytearray(16)
        struct.pack_into('<I', raw, channel * 4, bits)
        yield state, state | {f'legend.color_{offset:x}': bytes(raw)}


def legend_boundary_pairs() -> Iterator[tuple[LoadedLegend, LoadedLegend]]:
    """Cover every palette activation boundary, missing group and signed count boundary; e.g. list(legend_boundary_pairs())."""
    for count in range(10):
        state = baseline_legend(count, 0, (1, 1, 1))
        yield from ((state, changed) for changed in legend_mutations(state))
    state = baseline_legend(5, 0, (1, 1, 1))
    for path in sorted(state):
        missing = {name: raw for name, raw in state.items() if name != path}
        yield missing, missing
        yield state, missing
        yield missing, state
    for count in (-2147483648, -1, 10, 2147483647):
        invalid = state | {'legend.word_10': struct.pack('<i', count)}
        yield invalid, invalid
        yield invalid, state
    yield {}, {}


def legend_state_packet(state: LoadedLegend) -> bytes:
    """Encode independent presence plus all 64 raw input words without defaulting missing state; e.g. legend_state_packet(state)."""
    values = [state.get(f'legend.word_{offset:x}') for offset in WORD_OFFSETS]
    values.extend(None if (raw := state.get(f'legend.color_{offset:x}')) is None else raw[channel * 4:channel * 4 + 4]
                  for offset in COLOR_OFFSETS for channel in range(4))
    assert len(values) == 64 and all(raw is None or len(raw) == 4 for raw in values), values
    mask = sum(1 << index for index, raw in enumerate(values) if raw is not None)
    return struct.pack('<Q', mask) + b''.join(bytes(4) if raw is None else raw for raw in values)


def legend_pair_packet(left: LoadedLegend, right: LoadedLegend) -> bytes:
    """Preserve actual independent success, missing-input and invalid-count outcomes; e.g. legend_pair_packet(left, right)."""
    helper = importlib.import_module('cosworks_legend_comparison')
    try:
        outcome = helper.different(left, right, environment='masked_exceptions_daz_off')
        assert outcome in (0, 1), outcome
    except ValueError as error:
        message = str(error)
        assert 'explicit loaded' in message or 'palette count0..9' in message, message
        outcome = 3 if 'palette count0..9' in message else 2
    return legend_state_packet(left) + legend_state_packet(right) + struct.pack('<I', outcome)


def native_legend_states() -> list[LoadedLegend]:
    """Replay every original C-871 identity through the exact selected-version field observations; e.g. native_legend_states()."""
    original = cast(list[dict[str, object]], legend_comparison_report()['rows'])
    rows = cast(list[dict[str, object]], dialog_field_reports()[2]['rows'])
    available = {(row['path'], row['stream'], row['sha256']): row for row in rows}
    states: list[LoadedLegend] = []
    helper = importlib.import_module('cosworks_legend_comparison')
    for row in original:
        current = available[(row['path'], row['stream'], row['sha256'])]
        attempts = cast(list[dict[str, object]], current['attempts'])
        selected = [attempt for attempt in attempts if attempt['success'] and attempt['version'] == row['selected_version']]
        assert len(selected) == 1, (row, selected)
        parsed = cast(dict[str, object], selected[0]['result'])
        state = cast(LoadedLegend, helper.loaded_fields(parsed['spans']))
        assert helper.different(state, state, environment='masked_exceptions_daz_off') == row['model_self_difference'] == 0, row
        states.append(state)
    assert len(states) == 109, len(states)
    return states


def legend_pairs_packet(pairs: list[tuple[LoadedLegend, LoadedLegend]]) -> bytes:
    """Keep every ordered case and full independent paired result; e.g. legend_pairs_packet(pairs)."""
    return struct.pack('<II', 1, len(pairs)) + b''.join(legend_pair_packet(*pair) for pair in pairs)


def verify_legend_comparison_packets() -> None:
    """Freeze the full branch matrix plus every original native state and isolated mutation; e.g. verify_legend_comparison_packets()."""
    owned = list(owned_legend_pairs())
    states = native_legend_states()
    native = [(state, changed) for state in states for changed in (state, *legend_mutations(state))]
    packets = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-comparison.bin': legend_pairs_packet(owned)})
    packets.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-legend-comparison-native.bin': legend_pairs_packet(native)}))
    dialog_baseline('legend-comparison-packets-reference.json', dict(owned_pairs=len(owned), native_pairs=len(native),
        original_native_states=len(states), additional_stored_byte_credit=0, packets=packets))


def legend_style_projection(cases: list[tuple[int, bytes]]) -> bytes:
    """Project every actual complete style input into raw optional comparison state; e.g. legend_style_projection(cases)."""
    helper = importlib.import_module('cosworks_legend_comparison')
    rows: list[bytes] = []
    for version, raw in cases:
        parsed = actual_post_record(2, version, raw)
        assert parsed['success'] and parsed['end'] == len(raw), (version, parsed)
        state = helper.loaded_fields(parsed['spans'])
        rows.append(struct.pack('<II', version, len(raw)) + raw + legend_state_packet(state))
    return struct.pack('<II', 1, len(rows)) + b''.join(rows)


def verify_legend_style_projection() -> None:
    """Retain every complete owned/native style context, including missing older runtime fields; e.g. verify_legend_style_projection()."""
    owned = [(version, raw) for family, version, _, raw in dialog_owned_cases() if family == 2]
    original = (FEATURE.parents[2] / 'target/simulation-post-dialog-native.bin').read_bytes()
    native = [(version, raw) for family, version, raw in accepted_dialog_native_cases(original) if family == 2]
    assert (len(owned), len(native)) == (200, 52), (len(owned), len(native))
    packets = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-style-projection.bin': legend_style_projection(owned)})
    packets.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-legend-style-projection-native.bin': legend_style_projection(native)}))
    dialog_baseline('legend-style-projection-reference.json', dict(owned_styles=200, native_styles=52, additional_stored_byte_credit=0, packets=packets))
