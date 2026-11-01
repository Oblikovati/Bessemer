"""Independent unbounded integer expectations for C850 call requests (#166)."""
from collections.abc import Iterator
from itertools import product
import struct

YieldCase = tuple[int, ...]
YIELD_INDICES = (-2147483648, -3, -2, -1, 0, 1, 715827882, 715827883, 1431655765, 1431655766, 2147483646, 2147483647)
YIELD_COUNTS = (-2147483648, -2, -1, 0, 1, 2, 2147483647)


def yield_operation_cases() -> Iterator[YieldCase]:
    """Cross presence/signed boundaries and generated partial/invalid paths; e.g. list(yield_operation_cases())."""
    colors = (0x7fc00001, 0x80000000, 0x7f800000, 0xff800001)
    for mask, index, count in product(range(16), YIELD_INDICES, YIELD_COUNTS):
        yield (mask, *colors, -2147483648, index, count, 42, -19, -71)
    seed = 0x850178
    for number in range(12288):
        words: list[int] = []
        for _ in range(10):
            seed = (1664525 * seed + 1013904223) % (1 << 32)
            words.append(struct.unpack('<i', struct.pack('<I', seed))[0])
        # Ten LCG advances preserve low-bit parity; high bits vary presence independently.
        mask = (seed >> 16) % 16
        if number >= 8192:
            mask, words[5], words[6] = 15, abs(words[5]) % 2147483647, 2147483647
        yield (mask, *words)


def expected_yield_calls(case: YieldCase) -> tuple[int, list[tuple[int, ...]]]:
    """Compute dependency outcomes with independent unbounded math and modulo32; e.g. expected_yield_calls(case)."""
    mask, _, _, _, _, reverse, index, count, after_reverse, after_count, after_index = case
    if not mask & 1: return (10, [])
    if not mask & 4: return (11, [])
    if index != -2 and (index < 0 or index >= count): return (0, [])
    if not mask & 2: return (12, [])
    if index == -2: return (1, [(0, count, 2, reverse, 0)])
    if not mask & 8: return (13, [])
    triple = (3 * index) % (1 << 32)
    signed = triple if triple < 1 << 31 else triple - (1 << 32)
    return (2, [(4 * signed, count - index, 2, reverse, 0), (0, after_index, after_count, after_reverse, 1)])


def expected_yield_words(case: YieldCase) -> tuple[int, ...]:
    """Encode exact call order and signed64 displacement without Rust arithmetic; e.g. expected_yield_words(case)."""
    status, calls = expected_yield_calls(case)
    words = [status, len(calls), int(bool(calls)), int(bool(calls)), int(len(calls) == 2)]
    for offset, count, base_count, reverse, base in calls:
        low, high = struct.unpack('<II', struct.pack('<q', offset))
        words.extend((low, high, count, base_count, reverse, base))
    words.extend([0] * (17 - len(words)))
    assert len(words) == 17
    return tuple(value & 0xffffffff for value in words)


def yield_case_packet(case: YieldCase) -> bytes:
    """Retain eleven raw argument words and seventeen independent expected words; e.g. yield_case_packet(case)."""
    assert len(case) == 11 and 0 <= case[0] < 16, case
    arguments = struct.pack('<11I', *(value & 0xffffffff for value in case))
    return arguments + struct.pack('<17I', *expected_yield_words(case))
