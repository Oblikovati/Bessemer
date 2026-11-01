"""Independent operation and complete-parent palette packets, without count-sized wire framing (#166)."""
from collections import Counter
from itertools import chain, product
from typing import cast
import importlib
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from stored_style_colors import canonical_style_spans, optional_style_rgba
from style_preferences_projection import raw_style_native_contexts
from legend_palette_projection import legend_palette_observation
from legend_palette_cases import (PaletteCase, PALETTE_MODES, PALETTE_REVERSE, PALETTE_COUNTS, PALETTE_GRAY,
    palette_case_packet, palette_context_cases, palette_diagnostic_cases, expected_palette_dispatch)

PALETTE_ROLES = ('color_mapping_mode_raw', 'reverse_base_palette_raw', 'base_palette_color_count', 'show_gray_above_yield_enabled_raw')
PALETTE_CALLS = ((0, 0, 0, 0, 0), (1, 0, 2, 1, 3), (1, -1, 0, 0, 0), (1, 0, 1, 1, 3),
    (1, 0, 0, 1, 3), (1, 0, 2, 0, 0), (1, 0, 2, 1, -2147483648))


def verify_palette_operation_packets() -> None:
    """Freeze all raw-diagnostic, guard-order and missing/unsupported-context results; e.g. verify_palette_operation_packets()."""
    cases = list(chain(palette_context_cases(), palette_diagnostic_cases()))
    packet = struct.pack('<II', 1, len(cases)) + b''.join(map(palette_case_packet, cases))
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-palette-operations.bin': packet})
    dialog_baseline('legend-palette-operation-packets-reference.json', dict(cases=len(cases),
        status_counts=dict(Counter(str(expected_palette_dispatch(case)[0]) for case in cases)), packets=summaries,
        oracle='Unchanged copied Source raw diagnostics plus independently specified complete dispatcher truth table',
        native_execution=False, count_diagnostic_is_native_validation=False))


def palette_control_mutants(owned: list[tuple[int, int, bytes]]) -> list[tuple[int, int, bytes]]:
    """Change all four raw words independently without changing the nine-slot wire layout; e.g. palette_control_mutants(owned)."""
    family, version, original = next(context for context in owned if context[:2] == (2, 810009))
    fields = legend_palette_observation(family, version, original)
    offsets = {field['name']: cast(int, field['offset']) for field in fields}
    assert set(offsets) == set(PALETTE_ROLES)
    contexts: list[tuple[int, int, bytes]] = []
    for values in product(PALETTE_MODES, PALETTE_REVERSE, PALETTE_COUNTS, PALETTE_GRAY):
        raw = bytearray(original)
        for role, value in zip(PALETTE_ROLES, values, strict=True):
            struct.pack_into('<i', raw, offsets[role], value)
        contexts.append((family, version, bytes(raw)))
    return contexts


def palette_projection_cases(fields: list[dict[str, object]]) -> list[PaletteCase]:
    """Preserve actual raw values and absence beside seven explicit hypothetical calls; e.g. palette_projection_cases(fields)."""
    observed = {field['name']: field['signed_value'] for field in fields}
    mask = sum(1 << index for index, role in enumerate(PALETTE_ROLES) if role in observed)
    mode, reverse, count, gray = [cast(int, observed.get(role, 0)) for role in PALETTE_ROLES]
    return [(mask, mode, reverse, count, gray, *runtime) for runtime in PALETTE_CALLS]


def palette_projection_context(context: tuple[int, int, bytes]) -> bytes:
    """Retain complete bytes, independent named-slot counts and raw/dispatch expectations; e.g. palette_projection_context(context)."""
    family, version, raw = context
    spans = canonical_style_spans(family, version, raw)
    helper = importlib.import_module('cosworks_legend_color_policy_fields')
    fields = cast(list[dict[str, object]], helper.decode_fields(list(spans.values())))
    colors = [optional_style_rgba(spans, raw, f'legend.color_{0x18 + 16 * index:x}') for index in range(9)]
    slots = sum(color is not None for color in colors)
    assert slots in (0, 9) and (slots != 0 or not fields), (version, slots, fields)
    cases = palette_projection_cases(fields)
    return struct.pack('<III', family, version, len(raw)) + raw + struct.pack('<II', slots, len(cases)) + b''.join(map(palette_case_packet, cases))


def verify_palette_projection_packets() -> None:
    """Retain425 owned/276 native parents and2205 independent count/flag/mode mutations; e.g. verify_palette_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert (len(owned), len(native)) == (425, 276)
    mutants = palette_control_mutants(owned)
    assert len(mutants) == 2205
    packets = [struct.pack('<II', 1, len(contexts)) + b''.join(map(palette_projection_context, contexts)) for contexts in (owned + mutants, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-palette-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-legend-palette-projection-native.bin': packets[1]}))
    dialog_baseline('legend-palette-projection-packets-reference.json', dict(owned_contexts=425, native_contexts=276,
        complete_control_mutants=len(mutants), calls_per_context=7, packets=summaries,
        count_resizes_wire_area=False, actual_runtime_plot_known=False, native_mapping_executed=False))
