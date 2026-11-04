"""Exact C848 sign-extended immediate spelling boundary; global normalization stays strict (#172)."""
import pefile
from native_decoder_qualifications import (CapturedNativeText, qualify_captured_native_line,
    qualify_captured_native_method)
from style_preferences_disassembly import decoded_style_preference_method

PALETTE_SIGNED_IMMEDIATE: CapturedNativeText = {0x1800da9cc: (
    '1800da9cc add rcx, -0xc', '1800da9cc add rcx, 0xfffffffffffffff4',
    '4883c1f4', 'Exact sign-extended immediate-byte assembly spelling; #172')}


def decoded_palette_methods(image: pefile.PE, methods: list[dict[str, object]]) -> tuple[list[int], list[dict[str, str]]]:
    """Qualify one byte-checked spelling while retaining every complete method obligation; e.g. decoded_palette_methods(image, methods)."""
    verify_palette_qualification_boundaries()
    counts: list[int] = []
    qualifications: list[dict[str, str]] = []
    for method in methods:
        qualified, changes = qualify_captured_native_method(image, method, PALETTE_SIGNED_IMMEDIATE)
        counts.append(len(decoded_style_preference_method(image, qualified, normalize_address_spelling=True)))
        qualifications.extend(changes)
    assert counts == [36, 584] and len(qualifications) == 1, (counts, qualifications)
    return counts, qualifications


def require_palette_qualification_rejection(line: str, raw: bytes) -> None:
    """Reject changes to any captured instruction byte or operand; e.g. require_palette_qualification_rejection(line, raw)."""
    try:
        qualify_captured_native_line(line, raw, PALETTE_SIGNED_IMMEDIATE)
    except AssertionError:
        return
    raise AssertionError((line, raw.hex(), 'expected exact signed-immediate qualification rejection'))


def verify_palette_qualification_boundaries() -> None:
    """Pin one exact four-byte/address/text tuple and leave unknown locations unqualified; e.g. verify_palette_qualification_boundaries()."""
    original, canonical, encoded, _ = PALETTE_SIGNED_IMMEDIATE[0x1800da9cc]
    raw = bytes.fromhex(encoded)
    signed = int.from_bytes(raw[-1:], 'little', signed=True)
    assert signed == -12 and signed & 0xffffffffffffffff == 0xfffffffffffffff4
    assert qualify_captured_native_line(original, raw, PALETTE_SIGNED_IMMEDIATE)[0] == canonical
    for index in range(len(raw)):
        changed = bytearray(raw)
        changed[index] ^= 1
        require_palette_qualification_rejection(original, bytes(changed))
    for changed in (original.replace('rcx', 'rdx'), original.replace('-0xc', '-0xd'), original.replace('rcx', 'ecx')):
        require_palette_qualification_rejection(changed, raw)
    unknown = original.replace('1800da9cc', '1800da9cd')
    assert qualify_captured_native_line(unknown, raw, PALETTE_SIGNED_IMMEDIATE) == (unknown, None)
