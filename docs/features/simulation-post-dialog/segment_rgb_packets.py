"""Independent guard/bit-pattern matrix for the local C892 read (#166)."""
from collections import Counter
from itertools import product
import hashlib
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets
from segment_rgb_oracle import segment_rgb_oracle

SEGMENT_MODES = ((0, 0),) + tuple((1, value) for value in (0, 1, 2, 3, 4, 5, 0xffffffff, 0x80000000, 0x7fffffff))
SEGMENT_INDICES = ((0, 0),) + tuple((1, value) for value in (0x80000000, 0xffffffff, 0, 1, 2, 0x7fffffff))
SEGMENT_FLAGS = ((0, 0),) + tuple((1, value) for value in (0, 1, 3, 4, 5, 8, 0x80000000, 0xffffffff))
SEGMENT_COLORS = ((0, 0, 0, 0, 0), (1, 0, 0, 0, 0), (1, 0x7fa00042, 0xffc00042, 0xffffffff, 0x80000000),
    (1, 0x7f800000, 0xff800000, 1, 0x7fffff), (1, 0x3f800001, 0xbf800000, 0x80000001, 0x3f800000))


def segment_rgb_cases() -> list[list[int]]:
    """Cover all lazy dependencies and exact float classes independently of alpha; e.g. segment_rgb_cases()."""
    cases = [[*mode, *first, *last, *flags, *color] for mode, first, last, flags, color in
        product(SEGMENT_MODES, SEGMENT_INDICES, SEGMENT_INDICES, SEGMENT_FLAGS, SEGMENT_COLORS)]
    patterns = (0, 0x80000000, 1, 0x80000001, 0x7fffff, 0x800000, 0x3f800000, 0x3f800001,
        0x7f800000, 0xff800000, 0x7fa00042, 0xffc00042, 0xffffffff)
    patterns += tuple(int.from_bytes(hashlib.sha256(struct.pack('<I', index)).digest()[:4], 'little') for index in range(128))
    for mode, pattern, channel, alpha in product((3, 4), patterns, range(3), (0, 0x80000000, 0x7fa00042, 0xffffffff)):
        rgb = [0x80000000, 1, 0x7f800000]
        rgb[channel] = pattern
        cases.append([1, mode, 1, 1, 1, 1, 1, 0, 1, *rgb, alpha])
    return cases


def verify_segment_rgb_packets() -> None:
    """Freeze ordered local reads without granting endpoint or draw validity; e.g. verify_segment_rgb_packets()."""
    cases = segment_rgb_cases()
    packet = struct.pack('<2I', 1, len(cases)) + b''.join(struct.pack('<13I', *words) + segment_rgb_oracle(words) for words in cases)
    packets = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'segment-rgb-plans.bin': packet})
    dialog_baseline('segment-rgb-plan-reference.json', dict(cases=len(cases),
        statuses=dict(sorted(Counter(str(struct.unpack_from('<I', segment_rgb_oracle(words))[0]) for words in cases).items())),
        packets=packets, target_executed=False, geometry_validated=False, alpha_purpose_bound=False, additional_stored_byte_credit=0))
