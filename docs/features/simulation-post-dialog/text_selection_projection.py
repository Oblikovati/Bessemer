"""C890 complete archive/control snapshots with explicitly unknown runtime text (#166)."""
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from text_control_evidence import text_control_observation
from text_selection_oracle import post_text_frame, post_text_operation_oracle


def post_text_projection_context(family: int, version: int, raw: bytes) -> tuple[bytes, bool]:
    """Frame complete original bytes and explicit unknown-state operation expectations; e.g. post_text_projection_context(0, 890620, raw)."""
    view = text_control_observation(family, version, raw)
    packet = struct.pack('<4I', family, version, len(raw), int(view is not None)) + raw
    if view is None: return packet, False
    primary, lines = [int.from_bytes(bytes.fromhex(cast(str, view[name])), 'little')
        for name in ('primary_control_raw_hex', 'group_control_raw_hex')]
    wire = bytes.fromhex(cast(str, view['wire_text_raw_hex']))
    packet += struct.pack('<2I', primary, lines) + post_text_frame(wire)
    for mode, maximum in ((1, 0), (0, 512)):
        expected = post_text_operation_oracle(mode, primary, lines, wire, None, (None,) * 7, maximum)
        packet += struct.pack('<I', len(expected)) + expected
    return packet, True


def verify_post_text_projection_packets() -> None:
    """Keep every complete owned/native archive and require selected unknown strings instead of defaulting them; e.g. verify_post_text_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    contexts = [[post_text_projection_context(*case) for case in cases] for cases in (owned, native)]
    assert [len(rows) for rows in contexts] == [425, 276]
    assert [sum(present for _, present in rows) for rows in contexts] == [405, 276]
    packets = [struct.pack('<2I', 1, len(rows)) + b''.join(packet for packet, _ in rows) for rows in contexts]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'post-text-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-post-text-projection-native.bin': packets[1]}))
    dialog_baseline('text-selection-projection-reference.json', dict(owned_contexts=425, owned_present=405,
        native_contexts=276, native_present=276, packets=summaries,
        runtime_inputs='Explicitly unknown context; expectations do not assert observed native invocation or contents', additional_stored_byte_credit=0))
