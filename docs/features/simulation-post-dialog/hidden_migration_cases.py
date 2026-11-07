"""C856 scoped operation expectations from the frozen model and independent byte stores (#166)."""
from collections.abc import Iterator
from itertools import product
import importlib
import struct
from hidden_migration_evidence import hidden_instruction_pair

HiddenCase = tuple[int, int, int]
HIDDEN_VERSIONS = (0, 700005, 700006, 700007, 800000, 840006, 840007, 840008, 840009, 2147483647)
HIDDEN_WORDS = (-2147483648, -2147483647, -2, -1, 0, 1, 2, 3, 4, 5, 2147483646, 2147483647)


def hidden_operation_cases() -> Iterator[HiddenCase]:
    """Cross native scope boundaries, missing words and every store branch; e.g. list(hidden_operation_cases())."""
    yield from product(HIDDEN_VERSIONS, range(2), HIDDEN_WORDS)
    seed = 0x8561f0
    for number in range(4096):
        seed = (1664525 * seed + 1013904223) % (1 << 32)
        word = seed - (1 << 32) if seed >= 1 << 31 else seed
        yield (HIDDEN_VERSIONS[number % len(HIDDEN_VERSIONS)], (seed >> 30) & 1, word)


def expected_hidden_words(case: HiddenCase) -> tuple[int, ...]:
    """Require native version scope before word reads and compare model results to byte stores; e.g. expected_hidden_words((700006, 1, 2))."""
    version, present, word = case
    assert 0 <= version <= 2147483647 and present in (0, 1) and -2147483648 <= word <= 2147483647, case
    if not 700006 <= version < 840008: return (10, 0, 0, 0, 0, 0, 0, 0)
    if not present: return (11, 0, 0, 0, 0, 0, 0, 0)
    original = importlib.import_module('cosworks_post_style_legacy_hidden_policy').migrate(word, version)
    option, current = hidden_instruction_pair(word)
    assert original == dict(stored_word_1f0=word, derived_hidden_parts_option=option, runtime_word_1f0=current)
    store = 0 if word == 0 else 1 if ((word - 1) & 0xffffffff) <= 2 else 2
    completion = 1 if option == 0 else 2
    return (completion, option, current, store, 1 if store == 0 else 2,
        4 if store == 0 else 8, 2 if store == 2 else 1, completion)


def hidden_case_packet(case: HiddenCase) -> bytes:
    """Keep explicit typed-version inputs and all eight independent result words; e.g. hidden_case_packet((700006, 1, -1))."""
    return struct.pack('<11I', *(word & 0xffffffff for word in case), *expected_hidden_words(case))
