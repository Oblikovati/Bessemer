"""Retain the three byte-checked C-887 decoder qualifier discrepancies (#169)."""
from types import MappingProxyType
from collections.abc import Mapping
from typing import cast
import pefile
from dialog_evidence import dialog_baseline
from style_preferences_disassembly import (StylePreferenceInstruction, decoded_style_preference_method,
    canonical_style_address_operands, canonical_style_operands)

CAPTURED_ANNOTATION_TEXT: Mapping[int, tuple[str, str, str, str]] = MappingProxyType({
    0x18010098b: ('18010098b comisd xmm1, xmmword ptr [rip + 0x1d9c75]', '18010098b comisd xmm1, qword ptr [rip + 0x1d9c75]', '660f2f0d759c1d00', 'recorded_comisd_qualifier'),
    0x1801009b7: ('1801009b7 comisd xmm0, xmmword ptr [rip + 0x1d9c49]', '1801009b7 comisd xmm0, qword ptr [rip + 0x1d9c49]', '660f2f05499c1d00', 'recorded_comisd_qualifier'),
    0x180100c6d: ('180100c6d comisd xmm1, xmmword ptr [rip + 0x1d9993]', '180100c6d comisd xmm1, qword ptr [rip + 0x1d9993]', '660f2f0d93991d00', 'recorded_comisd_qualifier'),
})


def qualify_annotation_line(line: str, raw: bytes | None) -> tuple[str, dict[str, str] | None]:
    """Qualify only an exact captured string and byte sequence, preserving all other text; e.g. qualify_annotation_line('180000000 ret', None)."""
    address = int(line.split(' ', 1)[0], 16)
    captured = CAPTURED_ANNOTATION_TEXT.get(address)
    if captured is None: return line, None
    original, canonical, expected_hex, kind = captured
    assert line == original and raw == bytes.fromhex(expected_hex), (line, raw, captured)
    return canonical, dict(address=hex(address), original=original, canonical=canonical, raw_hex=expected_hex, kind=kind)


def qualified_annotation_method(image: pefile.PE, method: dict[str, object]) -> tuple[dict[str, object], list[dict[str, str]]]:
    """Copy expected text without rewriting the original Source report; e.g. qualified_annotation_method(image, method)."""
    lines: list[str] = []
    qualifications: list[dict[str, str]] = []
    for line in cast(list[str], method['instructions']):
        address = int(line.split(' ', 1)[0], 16)
        captured = CAPTURED_ANNOTATION_TEXT.get(address)
        raw = image.get_data(address - image.OPTIONAL_HEADER.ImageBase, len(captured[2]) // 2) if captured is not None else None
        canonical, qualification = qualify_annotation_line(line, raw)
        lines.append(canonical)
        if qualification is not None: qualifications.append(qualification)
    return method | {'instructions': lines}, qualifications


def annotation_decoded_methods(image: pefile.PE, methods: list[dict[str, object]]) -> dict[int, list[StylePreferenceInstruction]]:
    """Require complete method bytes plus every explicitly retained qualifier discrepancy; e.g. annotation_decoded_methods(image, methods)."""
    decoded: dict[int, list[StylePreferenceInstruction]] = {}
    qualifications: list[dict[str, str]] = []
    for method in methods:
        qualified, records = qualified_annotation_method(image, method)
        decoded[int(cast(str, method['start']), 16)] = decoded_style_preference_method(image, qualified, normalize_address_spelling=True)
        qualifications.extend(records)
    assert {int(record['address'], 16) for record in qualifications} == set(CAPTURED_ANNOTATION_TEXT)
    dialog_baseline('annotation-decoder-qualifications.json', dict(issue=169,
        qualification='Three recorded COMISD qualifiers; exact bytes and all other instruction text retained',
        address_spelling='Explicit zero displacement and unit-scale arithmetic identities only', records=qualifications))
    return decoded


def require_annotation_qualification_rejection(line: str, raw: bytes) -> None:
    """Reject changed captures instead of widening the accepted discrepancy set; e.g. require_annotation_qualification_rejection('18010098b nop', bytes(8))."""
    try:
        qualify_annotation_line(line, raw)
    except AssertionError:
        return
    raise AssertionError((line, raw.hex(), 'expected exact captured text/bytes rejection'))


def verify_annotation_qualification_boundaries() -> None:
    """Pin wrong-byte, wrong-text and unknown-address behavior independently; e.g. verify_annotation_qualification_boundaries()."""
    original, _, raw_hex, _ = CAPTURED_ANNOTATION_TEXT[0x18010098b]
    require_annotation_qualification_rejection(original, bytes(8))
    require_annotation_qualification_rejection(original.replace('xmm1', 'xmm0'), bytes.fromhex(raw_hex))
    unknown = original.replace('18010098b', '18010098c')
    assert qualify_annotation_line(unknown, bytes.fromhex(raw_hex)) == (unknown, None)
    assert qualify_annotation_line('180000000 ret', None) == ('180000000 ret', None)
    assert canonical_style_address_operands('lea', 'rdx,[rbp+0]') == 'rdx,[rbp]'
    assert canonical_style_address_operands('movss', 'dword ptr [r14+rax*1],xmm0') == 'dwordptr[r14+rax],xmm0'
    assert canonical_style_address_operands('lea', 'rdx,[rbp+1]') == 'rdx,[rbp+1]'
    assert canonical_style_address_operands('movss', 'dword ptr [r14+rax*2],xmm0') == 'dwordptr[r14+rax*2],xmm0'
    assert canonical_style_operands('lea', 'rdx,[rbp+0]') == 'rdx,[rbp+0]'
