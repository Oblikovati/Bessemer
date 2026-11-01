"""Exact signed32 sentinel spelling for C850 without global immediate normalization (#172)."""
import pefile
from native_decoder_qualifications import (CapturedNativeText, qualify_captured_native_line,
    qualify_captured_native_method)
from style_preferences_disassembly import decoded_style_preference_method

YIELD_SIGNED_SENTINEL: CapturedNativeText = {0x1800db110: ('1800db110 cmp r9d, -2',
    '1800db110 cmp r9d, 0xfffffffe', '4183f9fe', 'Exact sign-extended immediate-byte sentinel spelling; #172')}


def decoded_yield_methods(image: pefile.PE, methods: list[dict[str, object]]) -> tuple[list[int], list[dict[str, str]]]:
    """Qualify one exact sentinel tuple and retain all complete-body obligations; e.g. decoded_yield_methods(image, methods)."""
    verify_yield_qualification_boundaries()
    counts: list[int] = []
    qualifications: list[dict[str, str]] = []
    for method in methods:
        qualified, changes = qualify_captured_native_method(image, method, YIELD_SIGNED_SENTINEL)
        counts.append(len(decoded_style_preference_method(image, qualified, normalize_address_spelling=True)))
        qualifications.extend(changes)
    assert counts == [67, 52] and len(qualifications) == 1, (counts, qualifications)
    return counts, qualifications


def require_yield_qualification_rejection(line: str, raw: bytes) -> None:
    """Reject every changed captured byte or operand; e.g. require_yield_qualification_rejection(line, raw)."""
    try:
        qualify_captured_native_line(line, raw, YIELD_SIGNED_SENTINEL)
    except AssertionError:
        return
    raise AssertionError((line, raw.hex(), 'expected exact sentinel qualification rejection'))


def verify_yield_qualification_boundaries() -> None:
    """Keep signed32 equivalence and all rejection cases local to one native location; e.g. verify_yield_qualification_boundaries()."""
    original, canonical, encoded, _ = YIELD_SIGNED_SENTINEL[0x1800db110]
    raw = bytes.fromhex(encoded)
    signed = int.from_bytes(raw[-1:], 'little', signed=True)
    assert signed == -2 and signed & 0xffffffff == 0xfffffffe
    assert qualify_captured_native_line(original, raw, YIELD_SIGNED_SENTINEL)[0] == canonical
    for index in range(len(raw)):
        changed = bytearray(raw)
        changed[index] ^= 1
        require_yield_qualification_rejection(original, bytes(changed))
    for changed in (original.replace('r9d', 'r8d'), original.replace('-2', '-3'), original.replace('r9d', 'r9')):
        require_yield_qualification_rejection(changed, raw)
    unknown = original.replace('1800db110', '1800db111')
    assert qualify_captured_native_line(unknown, raw, YIELD_SIGNED_SENTINEL) == (unknown, None)
