"""Complete C891 file facts, including older missing pairs and unknown standalone parent code (#166)."""
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_projection import actual_post_record
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from plot54_evidence import plot54_observation
from plot54_plan_oracle import plot54_plan_oracle


def plot54_input_words(family: int, version: int, raw: bytes) -> list[int]:
    """Keep parent-code knowledge independent of the optional serialized pair; e.g. plot54_input_words(2, 820005, raw)."""
    view = plot54_observation(family, version, raw)
    plot_tag, plot = 0, 0
    if family == 0:
        parsed = actual_post_record(family, version, raw)
        header = bytes.fromhex(cast(str, cast(list[dict[str, object]], parsed['spans'])[0]['raw_hex']))
        assert len(header) == 56
        plot_tag, plot = 1, struct.unpack_from('<I', header, 4)[0]
    colors = bytes(32) if view is None else bytes.fromhex(cast(str, view['rgba_2ac_raw_hex']) + cast(str, view['rgba_2bc_raw_hex']))
    return [plot_tag, plot, int(view is not None), *struct.unpack('<8I', colors)]


def plot54_projection_context(family: int, version: int, raw: bytes) -> bytes:
    """Frame actual archive facts beside separately labeled unknown and what-if runtime scenarios; e.g. plot54_projection_context(0, 890620, raw)."""
    inputs = plot54_input_words(family, version, raw)
    packet = struct.pack('<3I', family, version, len(raw)) + raw + struct.pack('<11I', *inputs)
    scenarios = ((0, 0, 0, 0), (1, 0, 0, 0), (1, 3, 2, 2), (1, 4, 2, 1))
    for mode_tag, mode, core, callback in scenarios:
        words = [mode_tag, mode, *inputs, core, callback]
        packet += struct.pack('<4I', mode_tag, mode, core, callback) + plot54_plan_oracle(words)
    return packet


def verify_plot54_projection_packets() -> None:
    """Preserve every425 owned/276 native full body and every unavailable older/standalone fact; e.g. verify_plot54_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert len(owned) == 425 and len(native) == 276
    packets = [struct.pack('<2I', 1, len(cases)) + b''.join(plot54_projection_context(*case) for case in cases) for cases in (owned, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'plot54-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-plot54-projection-native.bin': packets[1]}))
    dialog_baseline('plot54-projection-reference.json', dict(owned_contexts=425, native_contexts=276, scenarios_per_context=4,
        scenarios='Explicit unknown mode, known bypass, hypothetical mode3/read and mode4/null callback; not observed native invocation',
        packets=summaries, additional_stored_byte_credit=0))
