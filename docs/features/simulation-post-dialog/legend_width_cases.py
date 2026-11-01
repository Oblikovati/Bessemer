"""C854 signed boundaries, conversion transitions and explicit presence cases (#166)."""
from collections.abc import Iterator
from itertools import product
import struct
from legend_width_oracle import rational_width_patterns

WidthCase = tuple[int, int, int]
WIDTH_SELECTORS = (-2147483648, -1, 0, 1, 2, 42, 2147483647)


def width_font_boundaries() -> list[int]:
    """Cross both signs around every signed32 power-of-two conversion transition; e.g. width_font_boundaries()."""
    values = {-2147483648, -2147483647, -1, 0, 1, 2147483647}
    for power, delta, sign in product(range(31), range(-3, 4), (-1, 1)):
        value = sign * ((1 << power) + delta)
        if -2147483648 <= value <= 2147483647: values.add(value)
    for power, delta, sign in product(range(24, 32), (-1, 0, 1), (-1, 1)):
        step = 1 << (power - 23)
        for offset in (-step // 4, step // 2, 3 * step // 2):
            value = sign * ((1 << power) + offset + delta)
            if -2147483648 <= value <= 2147483647: values.add(value)
    return sorted(values)


def width_operation_cases() -> Iterator[WidthCase]:
    """Cross all missing-fact orderings and generate additional signed raw patterns; e.g. list(width_operation_cases())."""
    yield from product(range(4), width_font_boundaries(), WIDTH_SELECTORS)
    seed = 0x854b4
    for number in range(4096):
        seed = (1664525 * seed + 1013904223) % (1 << 32); font = seed - (1 << 32) if seed >= 1 << 31 else seed
        seed = (1664525 * seed + 1013904223) % (1 << 32); selector = seed - (1 << 32) if seed >= 1 << 31 else seed
        if number % 3 < 2: selector = number % 3
        yield ((seed >> 24) & 3, font, selector)


def expected_width_words(case: WidthCase) -> tuple[int, ...]:
    """Retain exact rational results or the first unknown native entry read; e.g. expected_width_words((3, 1, 0))."""
    mask, font, selector = case
    assert 0 <= mask < 4 and -2147483648 <= font <= 2147483647 and -2147483648 <= selector <= 2147483647, case
    if not mask & 1: return (10, 0, 0, 0, 0, 0, 0, 0)
    if not mask & 2: return (11, 0, 0, 0, 0, 0, 0, 0)
    converted, scaled, initial, selected, operation = rational_width_patterns(font, selector)
    completion = 1 if selected & 0x7fffffff == 0 else 2
    return (completion, converted, scaled, initial, selected, operation, completion, 3 if selector == 0 else 2)


def width_case_packet(case: WidthCase) -> bytes:
    """Keep all three explicit inputs and eight independent expected words; e.g. width_case_packet((3, 1, 0))."""
    return struct.pack('<11I', *(word & 0xffffffff for word in case), *expected_width_words(case))
