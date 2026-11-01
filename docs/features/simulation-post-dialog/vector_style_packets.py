"""Independent local word stores and actual Source packed construction for C-886 (#166)."""
from typing import cast
import importlib
import struct
from colorref_import_packets import owned_colorref_cases
from color_comparison_packets import PATTERNS
from render_policy_packets import signed_gate_values
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets


def vector_request_arguments() -> list[tuple[tuple[int, ...], tuple[int, ...]]]:
    """Cover every RGB byte/high byte, raw alpha patterns and signed integer boundaries; e.g. vector_request_arguments()."""
    cases: list[tuple[tuple[int, ...], tuple[int, ...]]] = []
    for index, (packed, color) in enumerate(owned_colorref_cases()):
        words = tuple((index * factor + offset) & 0xffffffff for factor, offset in
            ((0x9e3779b9, 0), (0x85ebca6b, 1), (0xc2b2ae35, 2), (0x27d4eb2f, 3)))
        original = words + struct.unpack('<4I', color)
        requests = tuple(word ^ 0xaaaaaaaa for word in words) + (packed,)
        cases.append((original, requests))
    for index, signed in enumerate(signed_gate_values()):
        word = signed & 0xffffffff
        original = tuple(PATTERNS[(index + position) % len(PATTERNS)] for position in range(8))
        cases.append((original, (word, word, word ^ 0xffffffff, word, word ^ 0x9e3779b9)))
    return cases


def vector_request_case(original: tuple[int, ...], requests: tuple[int, ...], active: bool) -> bytes:
    """Patch independent raw word positions and copy the Source constructor's complete color; e.g. vector_request_case(original, requests, True)."""
    assert len(original) == 8 and len(requests) == 5
    helper = importlib.import_module('cosworks_colorref_import_view')
    constructed = helper.unpack_colorref(requests[4], rounding_mode='nearest_even')
    color = struct.unpack('<4I', bytes.fromhex(cast(str, constructed['rgba_raw_hex'])))
    assert color[3] == 0x3f800000
    outputs = [list(original) for _ in range(3)]
    if active:
        outputs[0][0] = requests[0]
        outputs[1][1:3] = requests[1:3]
        outputs[2][3:] = (requests[3], *color)
    return struct.pack('<14I', *original, int(active), *requests) + b''.join(struct.pack('<8I', *output) for output in outputs)


def verify_vector_style_packets() -> None:
    """Freeze active/null requests with all controls and original/result RGBA patterns intact; e.g. verify_vector_style_packets()."""
    arguments = vector_request_arguments()
    rows = [vector_request_case(original, requests, active) for original, requests in arguments for active in (False, True)]
    assert len(arguments) == 1461 and len(rows) == 2922
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog',
        {'vector-style-requests.bin': struct.pack('<II', 1, len(rows)) + b''.join(rows)})
    dialog_baseline('vector-style-requests-reference.json', dict(cases=2922, active_cases=1461, bypassed_cases=1461,
        expected='Exact verified field stores and Source nearest-even construction followed by whole-color copy',
        excluded='Native callbacks, downstream density processing and complete native return/failure behavior',
        additional_stored_byte_credit=0, packets=summaries))
