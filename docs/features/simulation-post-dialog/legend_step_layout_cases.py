"""Independent unbounded expectations for C851 local layout reads and low32 arithmetic (#166)."""
from collections.abc import Iterator
from itertools import product
import struct

LayoutCase = tuple[int, ...]
LAYOUT_COORDINATES = ((0, 0, 0, 0, 0, 0), (42, 100, 20, 119, 100, 7), (-42, 100, 20, 121, 100, 7),
    (2147483647, -2147483648, 2147483647, -2147483648, 1, 2147483647),
    (-2147483648, 2147483647, -2147483648, 2147483647, -1, -2147483648),
    (1, -20, 0, -19, 0, 1), (-1, 20, 0, -21, 0, -1), (0, -2147483648, 0, -2147483648, 0, -2147483648),
    (0, 2147483647, 0, 2147483647, 0, 2147483647), (2147483647, 0, -2147483648, 20, 0, 2),
    (-2147483648, 0, 2147483647, -20, 0, -2), (0, -1, 1, -1, 2147483647, 3))


def signed_layout_pattern(value: int) -> int:
    """Reduce an unbounded integer to its signed32 wire/register pattern; e.g. signed_layout_pattern(2147483648)."""
    unsigned = value % (1 << 32)
    signed = unsigned if unsigned < 1 << 31 else unsigned - (1 << 32)
    assert -2147483648 <= signed <= 2147483647
    return signed


def layout_operation_cases() -> Iterator[LayoutCase]:
    """Cross every optional-read mask and explicit enable/FC/flag paths; e.g. list(layout_operation_cases())."""
    for mask, enable, fc, first in product(range(2048), (0, 1), (0, 1, -1), (0, -1)):
        yield (mask, enable, fc, 17, 100, 20, 121, 100, 7, -1, first, 1)
    flags = (-2147483648, -1, 0, 1, 2147483647)
    for coordinates, fc, after_fc, first, second in product(LAYOUT_COORDINATES, flags, flags, (-2147483648, 0, 1), (-1, 0, 1)):
        yield (2047, 1, fc, *coordinates, after_fc, first, second)
    seed = 0x8512020
    for number in range(8192):
        words: list[int] = []
        for _ in range(10):
            seed = (1664525 * seed + 1013904223) % (1 << 32)
            words.append(signed_layout_pattern(seed))
        if number % 3 == 0: words[0] = 0
        if number % 5 == 0: words[7] = 0
        if number % 2 == 0: words[8] = 0
        enable = (-2147483648, -1, 1, 2147483647)[number % 4]
        yield (2047, enable, *words)


def expected_layout_initial_y(case: LayoutCase) -> tuple[int, int]:
    """Preserve the different zero/nonzero FC read order before text helpers; e.g. expected_layout_initial_y(case)."""
    mask, _, fc, _, point, size, upper, lower, *_ = case
    if not mask & 1: return (10, 0)
    if fc == 0:
        if not mask & 4: return (11, 0)
        if not mask & 8: return (12, 0)
        return (1, signed_layout_pattern(point - size))
    if not mask & 16: return (13, 0)
    if not mask & 32: return (14, 0)
    if not mask & 8: return (12, 0)
    if not mask & 4: return (11, 0)
    difference = signed_layout_pattern(upper - lower)
    quotient = (abs(difference) // 20) * (-1 if difference < 0 else 1)
    return (1, signed_layout_pattern(quotient - size + point))


def expected_layout_post(case: LayoutCase) -> tuple[int, int, int]:
    """Require actual post-helper rereads and retain the short-circuited second flag; e.g. expected_layout_post(case)."""
    mask, _, _, _, _, _, _, _, spacing, fc, first, second = case
    for bit, status in ((6, 16), (7, 17), (8, 18), (9, 19)):
        if not mask & (1 << bit): return (status, 0, 0)
    if first == 0 and not mask & 1024: return (20, 0, 0)
    if fc != 0: spacing = signed_layout_pattern(2 * spacing - 4)
    if first != 0 or second != 0: spacing = signed_layout_pattern(3 * spacing)
    return (1, spacing, 15 if first == 0 else 7)


def expected_layout_words(case: LayoutCase) -> tuple[int, ...]:
    """Compute exact scalar results or the first missing dependency using unbounded math; e.g. expected_layout_words(case)."""
    assert len(case) == 12 and 0 <= case[0] < 2048, (case, 'expected mask plus eleven signed32 arguments')
    if case[1] == 0: return (0,) * 8
    status, initial = expected_layout_initial_y(case)
    if status != 1: return (status,) + (0,) * 7
    if not case[0] & 2: return (15,) + (0,) * 7
    status, spacing, post_mask = expected_layout_post(case)
    if status != 1: return (status,) + (0,) * 7
    values = (1, case[3], initial, spacing, signed_layout_pattern(initial - spacing), 57 if case[2] == 0 else 63, post_mask, 3)
    return tuple(value & 0xffffffff for value in values)


def layout_case_packet(case: LayoutCase) -> bytes:
    """Retain every original optional argument pattern and eight independent expected words; e.g. layout_case_packet(case)."""
    arguments = struct.pack('<12I', *(value & 0xffffffff for value in case))
    expected = struct.pack('<8I', *expected_layout_words(case))
    assert len(arguments) == 48 and len(expected) == 32
    return arguments + expected
