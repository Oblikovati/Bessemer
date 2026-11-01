"""Owned exhaustive channels and original native constructor/assignment expectations (#166)."""
from collections import Counter
from collections.abc import Iterator
from types import ModuleType
from typing import cast
import importlib
import struct
from colorref_import_evidence import colorref_import_reports
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets

ALPHA_BITS = (0, 0x80000000, 1, 0x7f800000, 0xff800000, 0x7fc00042, 0x7fa00042, 0xffffffff, 0x3f800000, 0x3f000000)


def owned_colorref_cases() -> Iterator[tuple[int, bytes]]:
    """Exercise every byte in every RGB position, all high bytes and independent mixed words; e.g. list(owned_colorref_cases())."""
    for byte in range(256):
        for channel in range(3):
            packed = (byte << (channel * 8)) | (byte << 24)
            alpha = ALPHA_BITS[(byte + channel) % len(ALPHA_BITS)]
            yield packed, struct.pack('<4I', 0x12345678, 0x87654321, 0x7fa00042, alpha)
    for index in range(257):
        packed = index * 0x9e3779b9 & 0xffffffff
        yield packed, struct.pack('<4I', 0x80000000, 0x7fa00042, 0xff800000, ALPHA_BITS[index % len(ALPHA_BITS)])


def colorref_case_packet(helper: ModuleType, packed: int, original: bytes) -> bytes:
    """Keep original bits beside separately modeled constructor and assignment outputs; e.g. colorref_case_packet(helper, 0, original)."""
    constructed = helper.unpack_colorref(packed, rounding_mode='nearest_even')
    assigned = helper.assign_colorref(original, packed, rounding_mode='nearest_even')
    imported = bytes.fromhex(constructed['rgba_raw_hex'])
    replacement = bytes.fromhex(assigned['rgba_raw_hex'])
    assert imported[12:] == bytes.fromhex('0000803f'), constructed
    assert replacement[:12] == imported[:12] and replacement[12:] == original[12:], assigned
    return struct.pack('<I', packed) + original + imported + replacement


def native_colorref_cases(helper: ModuleType) -> list[tuple[int, bytes]]:
    """Replay every original constructor record and preserve all native alpha observations; e.g. native_colorref_cases(helper)."""
    constructor, assignment = colorref_import_reports()
    cases: list[tuple[int, bytes]] = []
    alphas: Counter[str] = Counter()
    for row in cast(list[dict[str, object]], constructor['rows']):
        for color in cast(list[dict[str, object]], row['colors']):
            original = bytes.fromhex(cast(str, color['original_rgba_raw_hex']))
            packed = cast(int, color['packed_colorref'])
            actual = helper.unpack_colorref(packed, rounding_mode='nearest_even')
            assert actual['rgba_raw_hex'] == color['reconstructed_rgba_raw_hex'], color
            cases.extend(((packed, original), (0, original)))
            alphas[original[12:].hex()] += 1
    assert sum(alphas.values()) == assignment['native_colors_checked'] == 4836, alphas
    assert dict(alphas) == assignment['original_alpha_counts'], alphas
    return cases


def colorref_import_packet(helper: ModuleType, cases: list[tuple[int, bytes]]) -> bytes:
    """Retain one explicit case count and every complete input/output frame; e.g. colorref_import_packet(helper, cases)."""
    return struct.pack('<II', 1, len(cases)) + b''.join(colorref_case_packet(helper, *case) for case in cases)


def verify_colorref_import_packets() -> None:
    """Freeze exact independent rounding and alpha-preservation results without DLL execution; e.g. verify_colorref_import_packets()."""
    helper = importlib.import_module('cosworks_colorref_import_view')
    owned, native = list(owned_colorref_cases()), native_colorref_cases(helper)
    assert (len(owned), len(native)) == (1025, 9672), (len(owned), len(native))
    packets = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'colorref-import.bin': colorref_import_packet(helper, owned)})
    packets.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-colorref-import-native.bin': colorref_import_packet(helper, native)}))
    dialog_baseline('rgba-colorref-import-packets-reference.json', dict(owned_cases=len(owned), native_cases=len(native),
        native_original_colors=4836, alpha_patterns=len(ALPHA_BITS), additional_stored_byte_credit=0, packets=packets))
