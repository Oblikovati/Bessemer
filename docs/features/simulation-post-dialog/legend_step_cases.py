"""Original C851 policy outcomes and independently bound reached-read masks (#166)."""
from collections.abc import Iterator
from itertools import product
import importlib
import struct

StepCase = tuple[int, ...]
STEP_RUNTIME_CONTEXTS = ((1, 21, 8, 0, 2, -3), (2, 21, 8, 1, 2, -3), (2, 0, 0, 1, 0, 0),
    (2, 0, 0, 0, 0, 0), (2, 0, 8, 0, 1, -3), (2, 0, 8, 0, 2, -3),
    (2147483647, 0, 8, 0, 2, -2147483648), (2, 0, 8, 0, 2, 0))


def step_operation_cases() -> Iterator[StepCase]:
    """Cross all policy equalities/signed boundaries and generated raw BEC branches; e.g. list(step_operation_cases())."""
    counts = (-2147483648, -2, -1, 0, 1, 2, 3, 2147483647)
    plots = (-2147483648, -1, 0, 20, 21, 22, 2147483647)
    analyses = (-2147483648, -1, 0, 1, 2, 8, 2147483647)
    bd8 = (-2147483648, -1, 0, 1, 2147483647)
    be8 = (-2147483648, -1, 0, 1, 2, 2147483647)
    bec = (-2147483648, -3, -1, 0, 1, 2, 3, 2147483647)
    yield from product(counts, plots, analyses, bd8, be8, bec)
    seed = 0x851fc
    for number in range(8192):
        words: list[int] = []
        for _ in range(6):
            seed = (1664525 * seed + 1013904223) % (1 << 32)
            words.append(struct.unpack('<i', struct.pack('<I', seed))[0])
        if number % 4 == 0: words[0:5] = [2, 0, 8, 0, 2]
        if number % 4 == 1: words[0:2] = [2, 21]
        yield tuple(words)


def expected_step_words(case: StepCase) -> tuple[int, int, int]:
    """Use the frozen Source policy and exact native reached-read predicates; e.g. expected_step_words(case)."""
    assert len(case) == 6 and all(type(word) is int and -2147483648 <= word <= 2147483647 for word in case), case
    count, plot, analysis, bd8, be8, _ = case
    result = importlib.import_module('cosworks_legend_step_policy').available(*case)
    assert type(result) is int and -2147483648 <= result <= 2147483647, result
    ordinary_analysis = count > 1 and bd8 == 0 and plot != 21
    special = ordinary_analysis and analysis == 8
    mask = 3 | (8 if count > 1 else 0) | (4 if ordinary_analysis else 0)
    mask |= (16 if special else 0) | (32 if special and be8 == 2 else 0)
    return (result & 0xffffffff, mask, 1 if result == 0 else 2)


def step_case_packet(case: StepCase) -> bytes:
    """Retain all six original signed words and three expected result words; e.g. step_case_packet(case)."""
    arguments = struct.pack('<6I', *(word & 0xffffffff for word in case))
    return arguments + struct.pack('<3I', *expected_step_words(case))
