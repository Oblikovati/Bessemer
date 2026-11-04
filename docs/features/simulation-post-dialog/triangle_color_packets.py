"""Full C889 independent caller/group/alpha fixtures, with native-offset oracle (#166)."""
from itertools import product
from collections import Counter
import struct
from aux_triangle_packets import aux_alpha_patterns
from triangle_color_oracle import triangle_operation_oracle
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets

PROFILES = [(0, 0, 0, 0, 0, 0), (0, 0, 1, 0, 0, 0), (0, 0, 1, 1, 0, 0),
    (0, 0, 1, 1, 1, 0), (0, 0, 1, 0xffffffff, 1, 0), (0, 0, 1, 1, 1, 1), (0, 0, 1, 1, 1, 0xffffffff),
    (1, 1, 0, 0, 1, 5), (1, 1, 0, 0, 1, 4), (1, 1, 1, 0, 1, 4), (1, 1, 1, 0xffffffff, 1, 3),
    (1, 1, 1, 0, 1, 0xffffffff), (1, 1, 1, 1, 1, 5), (1, 1, 1, 1, 1, 0), (1, 0, 1, 0, 1, 0)]


def triangle_group_packets() -> tuple[list[bytes], bytes]:
    """Keep separate packet and native-memory representations of every group field; e.g. triangle_group_packets()."""
    packets, memory = [], bytearray(4 * 0xb8)
    for index, (flag, count, lookup) in enumerate(((0, 2, 2), (1, 3, 2), (-1, 0, 0), (1, 4, 1))):
        first = (0x7fa00042 + index, 0x80000000 + index, 0xffffffff - index, 0xff800000)
        second = (0x3f800000 + index, index, 0x7fc00042 + index, 0x7f800000)
        payloads = (0 if index == 0 else 0x1234567800000000 + index, 0xffffffffffffffff - index)
        key = 0xf0000000 + index
        packets.append(struct.pack('<iiQQ8III', flag, count, *payloads, *first, *second, key, lookup))
        base = index * 0xb8
        struct.pack_into('<i', memory, base + 8, flag)
        struct.pack_into('<4I', memory, base + 0xc, *first)
        struct.pack_into('<4I', memory, base + 0x1c, *second)
        struct.pack_into('<iQQII', memory, base + 0x2c, count, *payloads, key, lookup)
    return packets, bytes(memory)


def triangle_case_packet(profile: tuple[int, ...], selector: int, alpha: int, core: int,
                         count: int, array: int, flag: int, maximum: int = 4, available: int = 4) -> tuple[bytes, int]:
    """Frame explicit caller facts separately from source controls and expected local results; e.g. triangle_case_packet(PROFILES[3], 1, 0, 1, 4, 1, 0)."""
    kind, override, first_tag, first, second_tag, second = profile
    words = [kind, selector, override, 0x7fa00042, 0x80000000, 0xffffffff, alpha,
        first_tag, first, second_tag, second, core, count, array, flag, maximum, available]
    groups, memory = triangle_group_packets()
    expected = triangle_operation_oracle(words, memory)
    return struct.pack('<17I', *words) + b''.join(groups[:available]) + expected, struct.unpack_from('<I', expected)[0]


def triangle_operation_cases() -> list[tuple[bytes, int]]:
    """Cover caller guard order, both group colors, signed counts, resource failures and all alpha classes; e.g. triangle_operation_cases()."""
    cases = [triangle_case_packet(profile, selector, alpha, core, count, array, flag)
        for profile, selector, alpha, core, count, array, flag in product(PROFILES,
            (0, 1, 2, 0xffffffff, 0x80000000, 0x7fffffff), (0, 0x7fa00042), (0, 1),
            (0, 1, 2, 4, 0x80000000, 0xffffffff), (0, 1), (0, 1, 0xffffffff))]
    cases.extend(triangle_case_packet(PROFILES[3 if kind == 0 else 9], selector, alpha, 1, 2, 1, flag, 2, 2)
        for kind, selector, alpha, flag in product((0, 1), (0, 1), aux_alpha_patterns(), (0, 1)))
    cases.extend(triangle_case_packet(PROFILES[3 if kind == 0 else 9], 1, 0x3f800001, 1, 4, 1, flag, maximum, available)
        for kind, flag, maximum, available in product((0, 1), (0, 1), range(5), range(5)))
    return cases


def verify_triangle_color_packets() -> None:
    """Freeze independently expected local outcomes without geometry or native-execution credit; e.g. verify_triangle_color_packets()."""
    cases = triangle_operation_cases()
    packet = struct.pack('<II', 1, len(cases)) + b''.join(row for row, _ in cases)
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'triangle-color-plans.bin': packet})
    dialog_baseline('triangle-color-plan-reference.json', dict(cases=len(cases),
        statuses=dict(sorted(Counter(str(status) for _, status in cases).items())), packets=summaries,
        oracle='Independent pointer-offset traversal and ordered binary32 comparison with explicit masked exceptions and DAZ/FTZ off',
        target_executed=False, additional_stored_byte_credit=0))
