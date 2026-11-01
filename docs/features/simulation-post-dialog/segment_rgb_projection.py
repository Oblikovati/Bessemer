"""Complete C892 archive inputs beside explicit hypothetical local runtime scenarios (#166)."""
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from segment_color_evidence import segment_color_observation
from segment_rgb_oracle import segment_rgb_oracle


def segment_rgb_input_words(family: int, version: int, raw: bytes) -> list[int]:
    """Preserve exact older absence separately from known zero RGBA; e.g. segment_rgb_input_words(2, 700005, raw)."""
    view = segment_color_observation(family, version, raw)
    rgba = bytes(16) if view is None else bytes.fromhex(cast(str, view['rgba_1bc_raw_hex']))
    return [int(view is not None), *struct.unpack('<4I', rgba)]


def segment_rgb_projection_context(family: int, version: int, raw: bytes) -> bytes:
    """Retain full original bodies and label local scenarios independently of native observations; e.g. segment_rgb_projection_context(0, 890620, raw)."""
    inputs = segment_rgb_input_words(family, version, raw)
    packet = struct.pack('<3I', family, version, len(raw)) + raw + struct.pack('<5I', *inputs)
    scenarios = ((0, 0, 0, 0, 0, 0, 0, 0), (1, 0, 0, 0, 0, 0, 0, 0),
        (1, 3, 1, 1, 1, 1, 1, 0), (1, 4, 1, 2, 1, 1, 0, 0))
    for runtime in scenarios:
        packet += struct.pack('<8I', *runtime) + segment_rgb_oracle([*runtime, *inputs])
    return packet


def verify_segment_rgb_projection_packets() -> None:
    """Project all425 owned and276 native contexts, including20 unavailable older named colors; e.g. verify_segment_rgb_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert len(owned) == 425 and len(native) == 276
    packets = [struct.pack('<2I', 1, len(cases)) + b''.join(segment_rgb_projection_context(*case) for case in cases) for cases in (owned, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'segment-rgb-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-segment-rgb-projection-native.bin': packets[1]}))
    dialog_baseline('segment-rgb-projection-reference.json', dict(owned_contexts=425, native_contexts=276, scenarios_per_context=4,
        scenarios='Unknown mode; known mode0 bypass; hypothetical mode3/read; hypothetical mode4/reversed range. No observed native invocation.',
        packets=summaries, additional_stored_byte_credit=0))
