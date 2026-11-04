"""C860 exact zero-test oracle with independent bounded native-mask setter decisions (#166)."""
from collections.abc import Iterator
import importlib
import struct
from segment_rgb_oracle import segment_signed_word

InverseCase = tuple[int, ...]
INVERSE_WORD_EDGES = (-2147483648, -2147483647, -3, -2, -1, 0, 1, 2, 3, 22, 2147483646, 2147483647)
INVERSE_PLOT_EDGES = (-2147483648, -42, -7, -1, 0, 4, 9, 21, 22, 23, 56, 57, 58, 69, 70, 71, 72, 73, 74, 85, 86, 120, 121, 136, 137, 138, 2147483647)


def inverse_operation_cases() -> Iterator[InverseCase]:
    """Cross missing/known source words with unknown/null/present plot contexts and signed boundaries; e.g. list(inverse_operation_cases())."""
    sources = [(0, 0), *((1, word) for word in INVERSE_WORD_EDGES)]
    for present, word in sources: yield (0, 0, 0, present, word)
    for kind, plot in [(0, 0), (1, 0), *((2, word) for word in INVERSE_PLOT_EDGES)]:
        for present, word in sources: yield (1, kind, plot, present, word)
    yield from generated_inverse_cases()


def generated_inverse_cases() -> Iterator[InverseCase]:
    """Exercise arbitrary signed words and wrap-sensitive plot codes with repeated zero/missing sources; e.g. list(generated_inverse_cases())."""
    seed = 0x860a8
    for number in range(12288):
        seed = (1664525 * seed + 1013904223) % (1 << 32)
        plot = INVERSE_PLOT_EDGES[(number // 3) % len(INVERSE_PLOT_EDGES)] if number % 3 == 0 else segment_signed_word(seed)
        seed = (1664525 * seed + 1013904223) % (1 << 32)
        present = int(number % 13 != 0); word = 0 if number % 8 == 0 or not present else segment_signed_word(seed)
        if number < 4096:
            yield (0, 0, 0, present, word)
            continue
        kind = 0 if number % 17 == 0 else 1 if number % 19 == 0 else 2
        yield (1, kind, plot if kind == 2 else 0, present, word)


def expected_inverse_words(case: InverseCase) -> tuple[int, ...]:
    """Keep complete early returns distinct from unknown required facts using the original scalar model; e.g. expected_inverse_words((1,2,22,0,0))."""
    mode, kind, plot, present, word = case
    assert mode in (0, 1) and kind in (0, 1, 2) and present in (0, 1), case
    if mode == 0:
        if not present: return (10,) + (0,) * 9
        value = importlib.import_module('cosworks_legend_inverse_preference').zero_test(word)
        return (1, 0, 0, value, 1, 0, 0, 1, 4, 1)
    if kind == 0: return (11,) + (0,) * 9
    delta = (plot - 22) % (1 << 32)
    if kind == 2 and delta <= 0x33 and (0x8000800000001 >> (delta & 63)) & 1:
        return (2, 3, 2, 0, 0, 1, 1, 0, 0, 2)
    if not present: return (12,) + (0,) * 9
    value = importlib.import_module('cosworks_legend_inverse_preference').zero_test(word)
    route = 2 if kind == 2 and plot == 70 else 1
    return (1, route, 1, value, 1, 1, int(kind == 2), 1, 4, 1)


def inverse_case_packet(case: InverseCase) -> bytes:
    """Retain five explicit inputs and ten independent expected output words; e.g. inverse_case_packet((0,0,0,1,2))."""
    return struct.pack('<15I', *(word & 0xffffffff for word in case), *expected_inverse_words(case))
