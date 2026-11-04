"""Complete C893 file facts and separately declared group/array runtime scenarios (#166)."""
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from mode3_rgb_evidence import mode3_rgb_observation
from mode3_rgb_operation_oracle import mode3_operation_oracle
from mode3_rgb_operation_packets import mode3_operation_case


def mode3_operation_input_words(family: int, version: int, raw: bytes) -> list[int]:
    """Keep actual named-field and parent-code availability independent; e.g. mode3_operation_input_words(2, 700005, raw)."""
    view = mode3_rgb_observation(family, version, raw)
    mode_tag, mode, color_tag, rgba = 0, 0, 0, bytes(16)
    if view is not None:
        mode_tag, color_tag, mode = 1, 1, cast(int, view['mode_1b8']) & 0xffffffff
        rgba = bytes.fromhex(cast(str, view['rgba_1dc_raw_hex']))
    plot_tag, plot = 0, 0
    if family == 0:
        parsed = actual_post_record(family, version, raw)
        header = bytes.fromhex(cast(str, cast(list[dict[str, object]], parsed['spans'])[0]['raw_hex']))
        assert len(header) == 56
        plot_tag, plot = 1, struct.unpack_from('<I', header, 4)[0]
    return [mode_tag, mode, color_tag, *struct.unpack('<4I', rgba), plot_tag, plot]


def mode3_projection_scenarios(inputs: list[int]) -> list[list[int]]:
    """Distinguish unknown native runtime from explicit primary/reflected what-if contexts; e.g. mode3_projection_scenarios(inputs)."""
    unknown = mode3_operation_case(status=None, items=None, face=None, first=False, other=False, selector=None, pass_code=0)
    hidden = mode3_operation_case(status=1, items=None, face=None, first=False, other=False, selector=None, pass_code=0)
    primary = mode3_operation_case(pass_code=0, face=0x404, selector=1)
    reflected = mode3_operation_case(pass_code=1, face=0x405)
    for words in (unknown, hidden): words[28:40] = [0] * 12
    reflected[28:40] = [1, 5, 0, 0, 0, 8, 1, 0, 1, 6, 77, 8]
    scenarios = [unknown, hidden, primary, reflected]
    for words in scenarios: words[:9] = inputs
    return scenarios


def mode3_operation_projection_context(family: int, version: int, raw: bytes) -> bytes:
    """Retain full bodies beside four independently expected local contexts; e.g. mode3_operation_projection_context(0, 890620, raw)."""
    inputs = mode3_operation_input_words(family, version, raw)
    packet = struct.pack('<3I', family, version, len(raw)) + raw + struct.pack('<9I', *inputs)
    for words in mode3_projection_scenarios(inputs):
        packet += struct.pack('<31I', *words[9:]) + mode3_operation_oracle(words)
    return packet


def verify_mode3_operation_projection_packets() -> None:
    """Freeze all425 owned and276 native contexts with original bytes and unknown runtime preserved; e.g. verify_mode3_operation_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert len(owned) == 425 and len(native) == 276
    packets = [struct.pack('<2I', 1, len(cases)) + b''.join(mode3_operation_projection_context(*case) for case in cases) for cases in (owned, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'mode3-rgb-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-mode3-rgb-projection-native.bin': packets[1]}))
    dialog_baseline('mode3-rgb-operation-projection-reference.json', dict(owned_contexts=425, native_contexts=276, scenarios_per_context=4,
        scenarios='Unknown runtime; known hidden group; hypothetical primary face404/selector1/existing array; hypothetical reflected face405/accepted growth. No observed native invocation.',
        packets=summaries, additional_stored_byte_credit=0))
