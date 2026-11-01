"""Independent C846 branch matrix and binary32-to-binary64 magnitude oracle (#166)."""
from collections.abc import Iterator
from itertools import product
import importlib
import struct

LegendFormatCase = tuple[int, int, int, int, int, int, int, int, int]
LegendFormatExpected = tuple[int, int, int, int, str]
FORMAT_PRECISIONS = (-2147483648, -2147483642, -8, -7, -1, 0, 1, 3, 2147483640, 2147483647)


def legend_primary_cases() -> Iterator[LegendFormatCase]:
    """Cross raw modes, wrapping precisions and every presence mask; e.g. list(legend_primary_cases())."""
    for mask, mode, precision in product(range(16), (-2147483648, -1, 0, 1, 2, 3, 2147483647), FORMAT_PRECISIONS):
        yield (0, mask, mode, precision, -2147483648, 2147483647, 0, 0, 0x7fa00042)


def legend_alternate_contexts() -> Iterator[LegendFormatCase]:
    """Distinguish every unavailable word and reached runtime fact; e.g. list(legend_alternate_contexts())."""
    controls = product(range(16), (-1, 0, 1), (-1, 0, 1), (-1, 0, 1),
        (-2147483648, -8, 3, 2147483647), range(3), range(2))
    for mask, mode, enabled, selector, precision, pointer, present in controls:
        yield (1, mask, mode, precision, enabled, selector, pointer, present, 0x3f800000)


def legend_magnitude_patterns() -> list[int]:
    """Keep adjacent native bounds, both NaN classes, subnormals and deterministic raw patterns; e.g. legend_magnitude_patterns()."""
    patterns = {0, 1, 0x7fffff, 0x800000, 0x7f7fffff, 0x7f800000, 0x7fa00042, 0x7fc00042}
    for bound in (0x3a83126f, 0x447a0000):
        patterns.update(range(bound - 8, bound + 9))
    seed = 0x846c907
    for _ in range(128):
        seed = (1664525 * seed + 1013904223) & 0xffffffff
        patterns.add(seed & 0x7fffffff)
    return sorted(patterns | {word | 0x80000000 for word in patterns})


def legend_numeric_cases() -> Iterator[LegendFormatCase]:
    """Cross all magnitude classes with raw enable/selector values and signed precision extrema; e.g. list(legend_numeric_cases())."""
    for bits, enabled, selector, precision in product(legend_magnitude_patterns(),
            (-2147483648, -1, 1, 2147483647), (-2147483648, -1, 0, 1, 2147483647), FORMAT_PRECISIONS):
        yield (1, 15, 0, precision, enabled, selector, 2, 1, bits)


def expected_legend_format(case: LegendFormatCase) -> LegendFormatExpected:
    """Use the original copied Source constructor, without formatting a floating value; e.g. expected_legend_format(case)."""
    operation, mask, mode, precision, _, _, _, _, _ = case
    helper = importlib.import_module('cosworks_legend_number_format')
    if operation == 1: return expected_alternate_format(case)
    if mask & 1 == 0: return (11, 0, 0, 0, '')
    if mask & 2 == 0: return (15, 0, 0, 0, '')
    output = helper.format_string(mode, precision)
    return (1, 3, 0, 0, output)


def expected_alternate_format(case: LegendFormatCase) -> LegendFormatExpected:
    """Follow complete native guard order, then widen binary32 for the double lower bound; e.g. expected_alternate_format(case)."""
    _, mask, mode, precision, enabled, selector, pointer, present, bits = case
    if pointer == 0: return (10, 0, 0, 0, '')
    if pointer == 1: return (3, 0, 1, 0, '')
    if mask & 1 == 0: return (11, 0, 0, 0, '')
    if mode != 0: return (4, 1, 1, 0, '')
    if mask & 4 == 0: return (12, 0, 0, 0, '')
    if enabled == 0: return (5, 5, 1, 0, '')
    if not present: return (13, 0, 0, 0, '')
    magnitude = abs(struct.unpack('<f', struct.pack('<I', bits))[0])
    lower = struct.unpack('<d', bytes.fromhex('fca9f1d24d62503f'))[0]
    if not lower <= magnitude <= 1000.0: return (6, 5, 1, 1, '')
    if mask & 8 == 0: return (14, 0, 0, 0, '')
    if mask & 2 == 0: return (15, 0, 0, 0, '')
    helper = importlib.import_module('cosworks_legend_number_format')
    return (2, 15, 1, 1, helper.format_string(1 if selector == 0 else 2, precision))


def legend_case_packet(case: LegendFormatCase) -> bytes:
    """Retain complete inputs and bounded independent expected ASCII output; e.g. legend_case_packet(case)."""
    status, mask, pointer_checked, value_checked, output = expected_legend_format(case)
    encoded = output.encode('ascii')
    assert len(encoded) <= 32
    arguments = struct.pack('<9I', *(word & 0xffffffff for word in case))
    expected = struct.pack('<5I', status, mask, pointer_checked, value_checked, len(encoded))
    return arguments + expected + encoded.ljust(32, b'\0')
