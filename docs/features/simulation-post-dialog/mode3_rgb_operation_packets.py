"""Owned independent C893 group and array operation corpus (#166)."""
from collections import Counter
from itertools import product
import hashlib
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets
from mode3_rgb_operation_oracle import mode3_operation_oracle

MODE3_MODES = (None, 0, 3, 4, -2147483648, 2147483647)
MODE3_FACES = (None, 0, 0x404, 0x405)
MODE3_PLOTS = (None, 53, 54)
MODE3_SELECTORS = (None, -2147483648, -1, 0, 1, 2, 3, 2147483647)
MODE3_STORED = (0x7fa00042, 0x80000000, 0xffffffff, 0xff800000)
MODE3_FIRST = (1, 0x7f800000, 0xbf800000, 0x7fc00042)
MODE3_OTHER = (0xffa00042, 0, 0x80000001, 0x3f800000)


def mode3_tagged_word(value: int | None) -> list[int]:
    """Encode unknown separately from signed zero; e.g. mode3_tagged_word(0)."""
    return [int(value is not None), 0 if value is None else value & 0xffffffff]


def mode3_operation_case(mode: int | None = 3, color: bool = True, plot: int | None = 54, pass_code: int = 1,
                        status: int | None = 0, items: int | None = 1, face: int | None = 0x404,
                        first: bool = True, other: bool = True, selector: int | None = None) -> list[int]:
    """Retain each independently tagged parent and reached runtime fact; e.g. mode3_operation_case()."""
    return [*mode3_tagged_word(mode), int(color), *MODE3_STORED, *mode3_tagged_word(plot), pass_code,
        *mode3_tagged_word(status), *mode3_tagged_word(items), *mode3_tagged_word(face),
        int(first), *MODE3_FIRST, int(other), *MODE3_OTHER, *mode3_tagged_word(selector),
        1, 6, 1, 7, 11, 8, 0, 0, 0, 0, 0, 8]


def mode3_group_operation_cases() -> list[list[int]]:
    """Cover all copy/eligibility/face combinations plus primary/reflected filtering boundaries; e.g. mode3_group_operation_cases()."""
    rows: list[list[int]] = []
    domain = product(MODE3_MODES, (False, True), (None, 0, 1), (None, 0, -1, 1), MODE3_FACES, (False, True), (False, True), (0, 1))
    for index, (mode, color, status, items, face, first, other, pass_code) in enumerate(domain):
        rows.append(mode3_operation_case(mode, color, MODE3_PLOTS[index % 3], pass_code, status, items, face, first, other, MODE3_SELECTORS[index % 8]))
    for mode, face, first, other, plot, selector, pass_code in product(MODE3_MODES, MODE3_FACES, (False, True), (False, True), MODE3_PLOTS, MODE3_SELECTORS, (0, 1)):
        rows.append(mode3_operation_case(mode, True, plot, pass_code, 0, 1, face, first, other, selector))
    return rows


def mode3_array_operation_cases() -> list[list[int]]:
    """Cover every signed capacity, growth result, known prefix and missing destination combination; e.g. mode3_array_operation_cases()."""
    rows: list[list[int]] = []
    for capacity, existing, growth, grown in product((None, -2147483648, -1, 0, 5, 6, 2147483647), (None, 0, 5, 6, 8), (None, -2147483648, -1, 0, 1, 2147483647), (None, 0, 5, 6, 8)):
        words = mode3_operation_case()
        words[28:30] = mode3_tagged_word(capacity)
        words[30:34] = [int(existing is not None), existing or 0, 42, 8]
        words[34:36] = mode3_tagged_word(growth)
        words[36:40] = [int(grown is not None), grown or 0, 99, 8]
        rows.append(words)
    return rows


def mode3_raw_pattern_cases() -> list[list[int]]:
    """Keep raw float classes and seeded mixed words independent of origin and alpha; e.g. mode3_raw_pattern_cases()."""
    patterns = (0, 0x80000000, 1, 0x80000001, 0x7fffff, 0x800000, 0x3f800000, 0x3f800001, 0x7f800000, 0xff800000, 0x7fa00042, 0xffc00042, 0xffffffff)
    patterns += tuple(int.from_bytes(hashlib.sha256(struct.pack('<I', index)).digest()[:4], 'little') for index in range(128))
    rows: list[list[int]] = []
    for pattern, channel, face, mode, pass_code in product(patterns, range(3), (0, 0x404, 0x405), (0, 3), (0, 1)):
        words = mode3_operation_case(mode=mode, face=face, pass_code=pass_code, selector=0)
        words[3 + channel] = pattern
        words[17 + channel] = pattern
        words[22 + channel] = pattern ^ 0xffffffff
        words[6], words[20], words[25] = pattern ^ 0x80000000, pattern, pattern ^ 0xffffffff
        rows.append(words)
    return rows


def verify_mode3_operation_packets() -> None:
    """Freeze complete independently expected group/copy/array outputs; e.g. verify_mode3_operation_packets()."""
    groups, arrays, patterns = mode3_group_operation_cases(), mode3_array_operation_cases(), mode3_raw_pattern_cases()
    cases = groups + arrays + patterns
    expected = [mode3_operation_oracle(words) for words in cases]
    packet = struct.pack('<2I', 1, len(cases)) + b''.join(struct.pack('<40I', *words) + result for words, result in zip(cases, expected, strict=True))
    packets = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'mode3-rgb-operations.bin': packet})
    dialog_baseline('mode3-rgb-operation-reference.json', dict(cases=len(cases), group_cases=len(groups), array_cases=len(arrays), raw_pattern_cases=len(patterns),
        group_statuses=dict(sorted(Counter(str(struct.unpack_from('<I', result)[0]) for result in expected).items())),
        array_statuses=dict(sorted(Counter(str(struct.unpack_from('<I', result, 88)[0]) for result in expected).items())),
        packets=packets, target_executed=False, geometry_validated=False, additional_stored_byte_credit=0))
