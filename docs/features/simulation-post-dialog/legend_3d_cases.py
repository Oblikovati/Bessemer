"""Independent C852 integer-request and packed-constructor expectations (#166)."""
from collections.abc import Iterator
from itertools import product
import importlib
import struct

Case3D = tuple[int, int, int, int, int]
COUNTS_3D = (-2147483648, -2147483647, -715827883, -357913942, -2, -1, 0, 1, 2, 10, 12, 357913941, 715827882, 2147483647)
ARGUMENTS_3D = (-2147483648, -1, 0, 1, 2, 3, 21, 2147483647)
PRECISIONS_3D = (-2147483648, -2147483647, -11, -10, -9, -1, 0, 1, 3, 10, 2147483637, 2147483638, 2147483646, 2147483647)
PACKED_3D = (0, 1, 0xff, 0xff00, 0xff0000, 0xffffff, 0xff000000, 0x12345678, 0xffffffff)


def signed_3d_word(value: int) -> int:
    """Reduce unbounded arithmetic to signed32 without Rust wrapping operations; e.g. signed_3d_word(2147483648)."""
    unsigned = value % (1 << 32)
    signed = unsigned if unsigned < 1 << 31 else unsigned - (1 << 32)
    assert -2147483648 <= signed <= 2147483647
    return signed


def operation_3d_cases() -> Iterator[Case3D]:
    """Cross all presence/boundary arguments and generated mixed body/background facts; e.g. list(operation_3d_cases())."""
    for mask, entry, later, argument in product(range(8), COUNTS_3D, COUNTS_3D, ARGUMENTS_3D):
        yield (0, mask, entry, later, argument)
    for mask, packed, precision, lighting in product(range(8), PACKED_3D, PRECISIONS_3D, (0, 1, 2, 127, 128, 255)):
        yield (1, mask, packed, precision, lighting)
    seed = 0x8523d
    for number in range(8192):
        words: list[int] = []
        for _ in range(3):
            seed = (1664525 * seed + 1013904223) % (1 << 32)
            words.append(seed)
        if number % 2 == 0:
            argument = 2 if number % 4 == 0 else signed_3d_word(words[2])
            yield (0, 7, signed_3d_word(words[0]), signed_3d_word(words[1]), argument)
        else:
            yield (1, 7, words[0], signed_3d_word(words[1]), words[2] >> 24)


def expected_body_3d_words(entry: int, later: int, argument: int) -> tuple[int, ...]:
    """Use modulo32 and the independently certified signed-extension saturation rule; e.g. expected_body_3d_words(10, 12, 2)."""
    pairs = signed_3d_word(entry + 1)
    vertices = signed_3d_word(pairs * 6)
    requested = signed_3d_word(later + (1 if argument == 2 else 0))
    colors = signed_3d_word((requested + 1) * 3)
    vertex_bytes = (1 << 64) - 1 if vertices < 0 else vertices * 4
    color_bytes = (1 << 64) - 1 if colors < 0 else colors * 4
    return tuple(word & 0xffffffff for word in (1 if requested == 0 else 2, pairs, vertices,
        vertex_bytes, vertex_bytes >> 32, requested, colors, color_bytes, color_bytes >> 32,
        0x1d01 if argument == 2 else 0x1d00, 8 if argument == 2 else 7, 0x1d01))


def expected_background_3d_words(packed: int, precision: int, lighting: int) -> tuple[int, ...]:
    """Reuse the original frozen nearest-even constructor, without emulating the getter; e.g. expected_background_3d_words(0, 3, 0)."""
    helper = importlib.import_module('cosworks_colorref_import_view')
    constructed = helper.unpack_colorref(packed, rounding_mode='nearest_even')
    bits = struct.unpack('<4I', bytes.fromhex(constructed['rgba_raw_hex']))
    return (3, *bits, (precision + 10) & 0xffffffff, 9, 0xb50, int(lighting != 0), 0, 0, 0)


def expected_3d_words(case: Case3D) -> tuple[int, ...]:
    """Preserve the first reached missing fact and exact complete local values; e.g. expected_3d_words(case)."""
    kind, mask, first, second, third = case
    assert kind in (0, 1) and 0 <= mask < 8, (case, 'expected body/background kind and three-bit presence mask')
    offset = 10 if kind == 0 else 20
    for bit in range(3):
        if not mask & (1 << bit): return (offset + bit,) + (0,) * 11
    if kind == 0: return expected_body_3d_words(first, second, third)
    assert 0 <= first <= 0xffffffff and 0 <= third <= 255, (case, 'expected packed u32 and raw GLboolean u8')
    return expected_background_3d_words(first, second, third)


def case_3d_packet(case: Case3D) -> bytes:
    """Retain kind/presence/original facts beside twelve independent expected words; e.g. case_3d_packet(case)."""
    arguments = struct.pack('<5I', *(value & 0xffffffff for value in case))
    expected = struct.pack('<12I', *expected_3d_words(case))
    assert len(arguments) == 20 and len(expected) == 48
    return arguments + expected
