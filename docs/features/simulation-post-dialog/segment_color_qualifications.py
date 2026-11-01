"""Six C892 recorded COMISD qualifiers, exact text/bytes only (#169)."""
from types import MappingProxyType
import pefile
from dialog_evidence import dialog_baseline
from native_decoder_qualifications import CapturedNativeText, qualify_captured_native_line, qualify_captured_native_method
from style_preferences_disassembly import decoded_style_preference_method, StylePreferenceInstruction

CAPTURED_SEGMENT_TEXT: CapturedNativeText = MappingProxyType({
    0x1800beb3f: ('1800beb3f comisd xmm6, xmmword ptr [rip + 0x218f71]', '1800beb3f comisd xmm6, qword ptr [rip + 0x218f71]', '660f2f35718f2100', "recorded_comisd_qualifier"),
    0x1800bebaa: ('1800bebaa comisd xmm1, xmmword ptr [rip + 0x218f0e]', '1800bebaa comisd xmm1, qword ptr [rip + 0x218f0e]', '660f2f0d0e8f2100', "recorded_comisd_qualifier"),
    0x1800bebb7: ('1800bebb7 comisd xmm0, xmmword ptr [rip + 0x218f11]', '1800bebb7 comisd xmm0, qword ptr [rip + 0x218f11]', '660f2f05118f2100', "recorded_comisd_qualifier"),
    0x1800bedb7: ('1800bedb7 comisd xmm7, xmmword ptr [rip + 0x218cf9]', '1800bedb7 comisd xmm7, qword ptr [rip + 0x218cf9]', '660f2f3df98c2100', "recorded_comisd_qualifier"),
    0x1800bee1a: ('1800bee1a comisd xmm1, xmmword ptr [rip + 0x218c9e]', '1800bee1a comisd xmm1, qword ptr [rip + 0x218c9e]', '660f2f0d9e8c2100', "recorded_comisd_qualifier"),
    0x1800bee27: ('1800bee27 comisd xmm0, xmmword ptr [rip + 0x218ca1]', '1800bee27 comisd xmm0, qword ptr [rip + 0x218ca1]', '660f2f05a18c2100', "recorded_comisd_qualifier"),
})


def segment_color_decoded_methods(image: pefile.PE, methods: list[dict[str, object]]) -> list[list[StylePreferenceInstruction]]:
    """Retain all556 instructions and exactly six captured discrepancies; e.g. segment_color_decoded_methods(image, methods)."""
    decoded: list[list[StylePreferenceInstruction]] = []
    qualifications: list[dict[str, str]] = []
    for method in methods:
        qualified, records = qualify_captured_native_method(image, method, CAPTURED_SEGMENT_TEXT)
        decoded.append(decoded_style_preference_method(image, qualified, normalize_address_spelling=True))
        qualifications.extend(records)
    assert {int(record['address'], 16) for record in qualifications} == set(CAPTURED_SEGMENT_TEXT)
    dialog_baseline('segment-color-decoder-qualifications.json', dict(issue=169,
        qualification='Six recorded COMISD qualifiers; exact bytes and all other instruction text retained', records=qualifications))
    return decoded


def require_segment_qualification_rejection(line: str, raw: bytes) -> None:
    """Keep captured-text or byte drift fatal; e.g. require_segment_qualification_rejection('1800beb3f nop', bytes(8))."""
    try:
        qualify_captured_native_line(line, raw, CAPTURED_SEGMENT_TEXT)
    except AssertionError:
        return
    raise AssertionError((line, raw.hex(), 'expected exact captured text/bytes rejection'))


def verify_segment_qualification_boundaries() -> None:
    """Reject each changed capture and preserve unknown addresses; e.g. verify_segment_qualification_boundaries()."""
    for address, (original, _, raw_hex, _) in CAPTURED_SEGMENT_TEXT.items():
        raw = bytes.fromhex(raw_hex)
        require_segment_qualification_rejection(original, bytes(len(raw)))
        require_segment_qualification_rejection(original.replace('comisd', 'ucomisd'), raw)
        unknown = original.replace(f'{address:x}', f'{address + 1:x}', 1)
        assert qualify_captured_native_line(unknown, raw, CAPTURED_SEGMENT_TEXT) == (unknown, None)
    assert qualify_captured_native_line('180000000 ret', None, CAPTURED_SEGMENT_TEXT) == ('180000000 ret', None)
