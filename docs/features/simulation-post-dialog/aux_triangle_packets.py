"""Original Source request oracle over all binary32 classes and threshold neighbors (#166)."""
from itertools import product
import importlib
import struct
from color_comparison_packets import PATTERNS
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets


def aux_alpha_patterns() -> list[int]:
    """Keep all exponent/sign classes plus exact0.99 and1.0 adjacent binary32 values; e.g. aux_alpha_patterns()."""
    values = set(PATTERNS)
    values.update(sign | exponent << 23 | fraction
        for sign, exponent, fraction in product((0, 0x80000000), range(256), (0, 1, 0x7fffff)))
    values.update(range(0x3f7d70a1, 0x3f7d70a7))
    values.update(range(0x3f7ffffe, 0x3f800003))
    return sorted(values)


def aux_request_case(enable: int, selector: int, alpha: int) -> bytes:
    """Use the copied Source gate and cap only its admitted alpha domain; e.g. aux_request_case(2, 1, 0x3f7d70a4)."""
    helper = importlib.import_module('cosworks_aux_triangle_gate')
    raw = struct.pack('<I', alpha)
    view = helper.request(enable, selector, raw, masked_exceptions=True)
    capped = struct.unpack('<I', struct.pack('<f', min(struct.unpack('<f', raw)[0], 1.0)))[0] if view['request_helper'] else 0
    return struct.pack('<iiIIII', enable, selector, alpha, int(view['request_helper']), int(view['per_group_rgb']), capped)


def verify_aux_triangle_packets() -> None:
    """Freeze enable truth, exact-one selection and masked comparison behavior without DLL execution; e.g. verify_aux_triangle_packets()."""
    enables = (-2147483648, -2, -1, 0, 1, 2, 3, 2147483647)
    selectors = (-2147483648, -1, 0, 1, 2, 3, 2147483647)
    alphas = aux_alpha_patterns()
    rows = [aux_request_case(*arguments) for arguments in product(enables, selectors, alphas)]
    assert struct.unpack_from('<I', aux_request_case(1, 0, 0x3f7d70a3), 12)[0] == 0
    assert struct.unpack_from('<I', aux_request_case(1, 0, 0x3f7d70a4), 12)[0] == 1
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog',
        {'aux-triangle-request.bin': struct.pack('<II', 1, len(rows)) + b''.join(rows)})
    dialog_baseline('aux-triangle-request-reference.json', dict(cases=len(rows), alpha_patterns=len(alphas),
        enable_values=len(enables), selector_values=len(selectors), cap_domain='Only admitted caller alpha; no generalized native MINSS claim',
        additional_stored_byte_credit=0, packets=summaries))
