"""Independent complete-parent factor views, including every binary32 exponent/sign class (#166)."""
from collections.abc import Iterator
from typing import cast
import importlib
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_projection import actual_post_record, dialog_fields_from_record
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from legend_position_projection import legend_position_observation

POSITION_ROLES = (('horizontal_position_factor', 'legend.word_ac'), ('vertical_position_factor', 'legend.word_b0'))


def position_float_patterns() -> list[int]:
    """Retain both signs, every exponent, boundary mantissas and observed adjacent words; e.g. position_float_patterns()."""
    patterns = {sign | exponent << 23 | mantissa for sign in (0, 0x80000000)
        for exponent in range(256) for mantissa in (0, 1, 0x400000, 0x7fffff)}
    patterns.update((0x3f4ccccc, 0x3f4ccccd, 0x3dcccccc, 0x3f3f5f9b, 0x3f4add0f, 0x7fa01234, 0xffa01234))
    return sorted(patterns)


def position_pattern_contexts(owned: list[tuple[int, int, bytes]]) -> Iterator[tuple[int, int, bytes]]:
    """Patch only actual factor spans in a complete explicit-field style body; e.g. list(position_pattern_contexts(owned))."""
    family, version, original = next(context for context in owned if context[:2] == (2, 700006))
    observed = legend_position_observation(family, version, original)
    assert len(observed) == 2
    offsets = {field['name']: cast(int, field['offset']) for field in observed}
    for bits in position_float_patterns():
        raw = bytearray(original)
        struct.pack_into('<I', raw, offsets[POSITION_ROLES[0][0]], bits)
        struct.pack_into('<I', raw, offsets[POSITION_ROLES[1][0]], bits ^ 0x80000000)
        yield family, version, bytes(raw)


def position_expected_words(family: int, version: int, raw: bytes) -> bytes:
    """Use actual copied Source float views and full primitive-field ordinals; e.g. position_expected_words(2, 700006, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw)
    helper = importlib.import_module('cosworks_legend_position_fields')
    observed = {field['name']: field for field in helper.decode_fields(parsed['spans'])}
    fields = dialog_fields_from_record(raw, parsed)
    paths = [field[4:4 + struct.unpack_from('<I', field)[0]].decode('utf-8') for field in fields]
    output = bytearray()
    for role, path in POSITION_ROLES:
        field = observed.get(role)
        if field is None:
            output.extend(bytes(28))
            continue
        bits = struct.unpack('<I', bytes.fromhex(field['raw_hex']))[0]
        finite = field['finite_value']
        numeric = 0 if finite is None else struct.unpack('<I', struct.pack('<f', finite))[0]
        output.extend(struct.pack('<7I', 1, bits, field['offset'], field['end'], int(finite is not None), numeric, paths.index(path)))
    return bytes(output)


def position_context_packet(context: tuple[int, int, bytes]) -> bytes:
    """Retain complete source bytes and both optional expected factor records; e.g. position_context_packet(context)."""
    family, version, raw = context
    expected = position_expected_words(family, version, raw)
    assert len(expected) == 56
    return struct.pack('<III', family, version, len(raw)) + raw + expected


def verify_legend_position_projection_packets() -> None:
    """Freeze all complete owned/native contexts plus independent raw float-class mutations; e.g. verify_legend_position_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert (len(owned), len(native)) == (425, 276)
    patterns = list(position_pattern_contexts(owned))
    contexts = (owned + patterns, native)
    packets = [struct.pack('<II', 1, len(group)) + b''.join(map(position_context_packet, group)) for group in contexts]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-position-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-legend-position-projection-native.bin': packets[1]}))
    dialog_baseline('legend-position-projection-packets-reference.json', dict(owned_complete_contexts=425,
        native_complete_contexts=276, pattern_contexts=len(patterns), packets=summaries,
        actual_runtime_dimensions_known=False, native_pixel_results_emulated=False, additional_stored_byte_credit=0))
