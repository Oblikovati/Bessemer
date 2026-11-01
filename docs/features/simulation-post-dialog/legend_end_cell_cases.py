"""Independent C853 short-circuit and raw MOVSS-copy expectations (#166)."""
from collections.abc import Iterator
from itertools import product
import struct

EndCellCase = tuple[int, ...]
END_CELL_FLAGS = (0x80000000, 0xffffffff, 0, 1, 2, 42, 0x7fffffff)
END_CELL_COLORS = ((0, 0, 0, 0), (0x3f800000, 0, 0, 0x3f800000),
    (0x3f000000,) * 4, (0x80000000, 0, 0x80000000, 0xffffffff),
    (0x7fc12345, 0x7f800001, 0xff800000, 0x7fa12345), (0x7f800000, 0xff800000, 1, 0),
    (0xffffffff, 0x80000001, 0x7fffffff, 0x80000000), (0x12345678, 0x87654321, 0xfedcba98, 0xabcdef01))


def end_cell_operation_cases() -> Iterator[EndCellCase]:
    """Cross every presence gate and signed boundary with raw color patterns; e.g. list(end_cell_operation_cases())."""
    for mask, plot, flag, color in product(range(8), END_CELL_FLAGS, END_CELL_FLAGS, END_CELL_COLORS):
        yield (mask, plot, flag, *color)
    seed = 0x853ec
    for number in range(4096):
        words: list[int] = []
        for _ in range(6):
            seed = (1664525 * seed + 1013904223) % (1 << 32); words.append(seed)
        if number % 4 == 0: words[0] = 0
        if number % 4 == 1: words[1] = 0
        yield ((seed >> 24) & 7, *words)


def expected_end_cell_words(case: EndCellCase) -> tuple[int, ...]:
    """Use the native two-predicate truth table and exact first-three-word identity; e.g. expected_end_cell_words(case)."""
    assert len(case) == 7 and all(type(word) is int and 0 <= word <= 0xffffffff for word in case), case
    mask, plot, flag, red, green, blue, _ = case
    assert 0 <= mask < 8, case
    if not mask & 1: return (10, 0, 0, 0, 0, 0)
    if plot == 0: return (1, 1, 0, 0, 0, 1)
    if not mask & 2: return (11, 0, 0, 0, 0, 0)
    if flag == 0: return (1, 3, 0, 0, 0, 1)
    if not mask & 4: return (12, 0, 0, 0, 0, 0)
    return (2, 7, red, green, blue, 2)


def end_cell_case_packet(case: EndCellCase) -> bytes:
    """Keep all supplied parents and six independently expected result words; e.g. end_cell_case_packet(case)."""
    return struct.pack('<13I', *case, *expected_end_cell_words(case))
