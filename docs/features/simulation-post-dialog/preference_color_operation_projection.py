"""Complete C907 original route colors beside explicit hypothetical runtime preferences (#166)."""
from types import ModuleType
from typing import cast
import importlib
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from preference_color_evidence import PREFERENCE_COLOR_ROUTES, preference_color_observation
from preference_color_operation_packets import preference_operation_case
from preference_color_operation_oracle import preference_operation_oracle


def preference_projection_scenarios(route: tuple[int, int, int | None], original: tuple[int, ...]) -> list[list[int]]:
    """Keep unknown context, skipped/no-source, applied and truncated contexts distinct; e.g. preference_projection_scenarios(route, original)."""
    required = max(route[1], route[2] or 0) + 4
    contexts = (((0, 1, 73), None), ((2, 1, 73), None), ((1, 1, 73), required), ((2, 1, 70), required - 1))
    scenarios = [preference_operation_case(route, runtime, length, 0xaa123456, original[3], 0x7fa00042) for runtime, length in contexts]
    for words in scenarios: words[1:5] = original
    return scenarios


def preference_operation_projection_context(model: ModuleType, family: int, version: int, raw: bytes) -> bytes:
    """Retain full original bodies and every absent/present normalized route; e.g. preference_operation_projection_context(model, 2, 700005, raw)."""
    observed = preference_color_observation(family, version, raw)
    packet = struct.pack('<3I', family, version, len(raw)) + raw
    for route in PREFERENCE_COLOR_ROUTES:
        color = observed[hex(route[0])]
        original = struct.unpack('<4I', bytes(16) if color is None else bytes.fromhex(cast(str, color)))
        packet += struct.pack('<6I', route[0], int(color is not None), *original)
        if color is None: continue
        for words in preference_projection_scenarios(route, original):
            packet += struct.pack('<16I', *words) + preference_operation_oracle(model, words)
    return packet


def verify_preference_operation_projection_packets() -> None:
    """Freeze all425 owned and276 native whole bodies, including route-specific absences; e.g. verify_preference_operation_projection_packets()."""
    model = importlib.import_module('cosworks_style_preference_colors')
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert len(owned) == 425 and len(native) == 276
    packets = [struct.pack('<2I', 1, len(cases)) + b''.join(preference_operation_projection_context(model, *case) for case in cases) for cases in (owned, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'preference-color-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-preference-color-projection-native.bin': packets[1]}))
    dialog_baseline('preference-color-operation-projection-reference.json', dict(owned_contexts=425, native_contexts=276, routes_per_context=8,
        scenarios_per_present_route=4, scenarios='Unknown pointer despite supplied type73; known nonnull/type73 skipped with no source; known null/type73 with exact source; known nonnull/type70 with one-byte-short source. No observed native invocation.',
        packets=summaries, target_executed=False, additional_stored_byte_credit=0))
