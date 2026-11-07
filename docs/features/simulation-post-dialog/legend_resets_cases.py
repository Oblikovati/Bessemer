"""C857 ordered reset events from the frozen exact version model (#166)."""
from collections.abc import Iterator
import importlib
import struct
from legend_resets_evidence import LEGEND_RESET_LABELS

RESET_VERSION_EDGES = (0, 700005, 700006, 700007, 840007, 840008, 840009, 840010, 840011, 890102, 890103, 2147483647)


def reset_operation_versions() -> Iterator[int]:
    """Cross complete threshold neighborhoods and generated versions in every policy interval; e.g. list(reset_operation_versions())."""
    yield from RESET_VERSION_EDGES
    yield from range(700000, 700013)
    yield from range(840000, 840016)
    seed = 0x857c0
    for number in range(4096):
        seed = (1664525 * seed + 1013904223) % (1 << 32)
        if number % 4 == 0: yield seed % 700006
        elif number % 4 == 1: yield 700006 + seed % (840008 - 700006)
        elif number % 4 == 2: yield 840008 + ((seed >> 24) & 1)
        else: yield 840010 + seed % (2147483647 - 840010 + 1)


def expected_reset_words(version: int) -> tuple[int, ...]:
    """Retain unsupported scope separately from ordered complete empty/nonempty event results; e.g. expected_reset_words(840009)."""
    assert type(version) is int and 0 <= version <= 2147483647, (version, 'expected normalized nonnegative signed32 context')
    if version < 700006: return (10,) + (0,) * 9
    assignments = importlib.import_module('cosworks_legend_read_resets').forced_zero_fields(version)
    offsets = {'legend.word_c0': 0xc0, 'legend.word_c4': 0xc4, 'legend.word_c8': 0xc8}
    assert set(assignments) <= set(offsets), (version, assignments)
    words: list[int] = []
    for label in LEGEND_RESET_LABELS:
        if label not in assignments: continue
        assert type(assignments[label]) is int and assignments[label] == 0, (version, assignments)
        words.extend((offsets[label], assignments[label]))
    while len(words) < 6: words.extend((0xffffffff, 0))
    count = len(assignments); completion = 1 if count == 0 else 2
    return (completion, count, *words, completion, count * 4)


def reset_case_packet(version: int) -> bytes:
    """Keep one explicit version and ten independent ordered result words; e.g. reset_case_packet(700006)."""
    return struct.pack('<11I', version, *expected_reset_words(version))
