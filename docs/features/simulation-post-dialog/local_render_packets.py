"""Independent signed-count and complete binary32-class cases for C-884 (#166)."""
from itertools import product
import importlib
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets
from color_comparison_packets import PATTERNS
from render_policy_packets import signed_gate_values


def local_render_case(color_count: int, cell_count: int, horizontal: int, vertical: int) -> bytes:
    """Preserve raw inputs and the actual conditional Source result; e.g. local_render_case(1, 1, 0x7fa00042, 0)."""
    helper = importlib.import_module('cosworks_render_safety')
    result = helper.is_safe_for_rendering(color_count, cell_count, struct.pack('<I', horizontal),
        struct.pack('<I', vertical), masked_exceptions_daz_off=True)
    return struct.pack('<iiIII', color_count, cell_count, horizontal, vertical, int(result))


def local_render_arguments() -> list[tuple[int, int, int, int]]:
    """Cover all count signs, both float branches, every exponent and representative significands; e.g. local_render_arguments()."""
    counts = (-2147483648, -1, 0, 1, 2, 9, 2147483647)
    cases = set(product(counts, counts, PATTERNS, PATTERNS))
    for exponent, sign, fraction in product(range(256), (0, 0x80000000), (0, 1, 0x7fffff)):
        bits = sign | exponent << 23 | fraction
        cases.update(((1, 1, bits, 0), (1, 1, 0, bits)))
    for count, position in product(signed_gate_values(), ((0, 0), (0xffa00042, 0), (0, 0x80000001))):
        cases.update(((count, 1, *position), (1, count, *position)))
    return sorted(cases)


def local_render_packet(rows: list[bytes]) -> bytes:
    """Frame each complete independent result in deterministic input order; e.g. local_render_packet(rows)."""
    return struct.pack('<II', 1, len(rows)) + b''.join(rows)


def verify_local_render_packets() -> None:
    """Freeze positive/negative counts, unordered float behavior and both independent raw position tests; e.g. verify_local_render_packets()."""
    arguments = local_render_arguments()
    cases = [local_render_case(*args) for args in arguments]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog',
        {'local-render-predicate.bin': local_render_packet(cases)})
    accepted = sum(struct.unpack_from('<I', row, 16)[0] for row in cases)
    assert 0 < accepted < len(cases), (accepted, len(cases))
    dialog_baseline('local-render-packets-reference.json', dict(cases=len(cases), accepted=accepted,
        rejected=len(cases) - accepted, additional_stored_byte_credit=0, packets=summaries))
