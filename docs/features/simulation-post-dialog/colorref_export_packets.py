"""Independent projection status/packed bytes across exponent and conversion boundaries (#166)."""
from collections.abc import Iterator
from types import ModuleType
from typing import cast
import importlib
import struct
from colorref_export_evidence import colorref_export_report
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets


def single_channel_inputs() -> Iterator[bytes]:
    """Cross every exponent, both signs, representative significands and each RGB position; e.g. list(single_channel_inputs())."""
    for exponent in range(256):
        yield from signed_significand_inputs(exponent)


def signed_significand_inputs(exponent: int) -> Iterator[bytes]:
    """Retain both signs without relying on floating-point input generation; e.g. list(signed_significand_inputs(127))."""
    for fraction in (0, 1, 0x3fffff, 0x7ffffe, 0x7fffff):
        for sign in (0, 0x80000000):
            yield from positioned_channel_inputs(sign | exponent << 23 | fraction)


def positioned_channel_inputs(word: int) -> Iterator[bytes]:
    """Keep unrelated channels valid while exercising alpha independence; e.g. list(positioned_channel_inputs(1))."""
    for channel in range(3):
        words = [0, 0, 0, 0x7fa00042]
        words[channel] = word
        yield struct.pack('<4I', *words)


def owned_export_inputs() -> Iterator[bytes]:
    """Include the asymmetric signed32 boundary and independent mixed words; e.g. list(owned_export_inputs())."""
    yield from single_channel_inputs()
    for word in range(0x4b008040, 0x4b0080c1):
        yield from positioned_channel_inputs(word)
        yield from positioned_channel_inputs(word | 0x80000000)
    for index in range(2048):
        words = tuple((index * factor + offset) & 0xffffffff for factor, offset in
            ((0x9e3779b9, 0), (0x85ebca6b, 0x3f000000), (0xc2b2ae35, 0xbf000000), (0x27d4eb2f, 0x7fc00042)))
        yield struct.pack('<4I', *words)
    yield struct.pack('<4f', 0.5, -0.5, 2.0, 1.0)


def projection_record(helper: ModuleType, raw: bytes) -> bytes:
    """Record actual Source status without inventing packed values for excluded domains; e.g. projection_record(helper, raw)."""
    view = helper.project(raw, rounding_mode='nearest_even')
    assert view['raw_hex'] == raw.hex(), view
    accepted = view['status'] == 'projected'
    packed = cast(int, view['packed_colorref']) if accepted else 0
    assert accepted or view['packed_colorref'] is None, view
    return raw + struct.pack('<II', int(accepted), packed)


def export_packet(helper: ModuleType, inputs: list[bytes]) -> bytes:
    """Keep every complete raw input and its independent conditional result; e.g. export_packet(helper, inputs)."""
    return struct.pack('<II', 1, len(inputs)) + b''.join(projection_record(helper, raw) for raw in inputs)


def native_export_inputs(helper: ModuleType) -> list[bytes]:
    """Replay all original records under their retained qualifications, without new byte credit; e.g. native_export_inputs(helper)."""
    report = colorref_export_report()
    inputs: list[bytes] = []
    for row in cast(list[dict[str, object]], report['rows']):
        for color in cast(list[dict[str, object]], row['colors']):
            raw = bytes.fromhex(cast(str, color['raw_hex']))
            actual = helper.project(raw, rounding_mode='nearest_even')
            assert all(actual[key] == color[key] for key in actual), color
            inputs.append(raw)
    assert len(inputs) == 4836, len(inputs)
    return inputs


def verify_colorref_export_packets() -> None:
    """Freeze owned degenerate cases and the complete original native projection denominator; e.g. verify_colorref_export_packets()."""
    helper = importlib.import_module('cosworks_colorref_view')
    owned, native = list(owned_export_inputs()), native_export_inputs(helper)
    owned_packet, native_packet = export_packet(helper, owned), export_packet(helper, native)
    packets = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'colorref-export.bin': owned_packet})
    packets.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-colorref-export-native.bin': native_packet}))
    projected = sum(struct.unpack_from('<I', owned_packet, 8 + 24 * index + 16)[0] for index in range(len(owned)))
    dialog_baseline('rgba-colorref-export-packets-reference.json', dict(owned_cases=len(owned), owned_projected=projected,
        owned_outside_model=len(owned) - projected, native_cases=len(native), additional_stored_byte_credit=0, packets=packets))
