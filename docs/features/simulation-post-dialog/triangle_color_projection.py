"""Full parent/style C889 inputs with canonical presence and untouched archive bytes (#166)."""
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts

TRIANGLE_COLOR_PATHS = ('style.word_1f0',) + tuple(f'style.color_1f4.component_{i}' for i in range(4)) + ('style.word_21c', 'style.word_230') + tuple(f'style.color_220.component_{i}' for i in range(4))


def triangle_color_view_context(family: int, version: int, raw: bytes) -> tuple[bytes, bool]:
    """Retain exact complete archive bodies and every supplied control bit; e.g. triangle_color_view_context(0, 890620, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw)
    spans = {span['label']: span for span in cast(list[dict[str, object]], parsed['spans'])}
    present = [path in spans for path in TRIANGLE_COLOR_PATHS]
    assert all(present) or not any(present), (family, version, present)
    header = struct.pack('<4I', family, version, len(raw), int(all(present))) + raw
    if not all(present): return header, False
    controls = bytearray()
    for path in TRIANGLE_COLOR_PATHS:
        offset = cast(int, spans[path]['offset'])
        word = bytes.fromhex(cast(str, spans[path]['raw_hex']))
        assert len(word) == 4 and raw[offset:offset + 4] == word
        controls.extend(word)
    return header + controls, True


def verify_triangle_color_projection() -> None:
    """Keep all425 owned and276 native contexts including20 missing older groups; e.g. verify_triangle_color_projection()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    contexts = [[triangle_color_view_context(*case) for case in cases] for cases in (owned, native)]
    counts = [sum(present for _, present in rows) for rows in contexts]
    assert [len(rows) for rows in contexts] == [425, 276] and counts == [405, 276]
    packets = [struct.pack('<2I', 1, len(rows)) + b''.join(packet for packet, _ in rows) for rows in contexts]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'triangle-color-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-triangle-color-projection-native.bin': packets[1]}))
    dialog_baseline('triangle-color-projection-reference.json', dict(owned_contexts=425, owned_present=405,
        native_contexts=276, native_present=276, packets=summaries, additional_stored_byte_credit=0))
