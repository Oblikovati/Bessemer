"""Independent raw cell-count and modular32 final-instruction expectations (#166)."""
from collections import Counter
from collections.abc import Iterator
from itertools import product
from typing import cast
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from style_preferences_projection import raw_style_native_contexts
from legend_height_projection import legend_height_observation

HEIGHT_COUNTS = (-2147483648, -2147483647, -65536, -1, 0, 1, 2, 10, 12, 65536, 2147483646, 2147483647)
CELL_RESULTS = (-2147483648, -2147483647, -65536, -2, -1, 0, 1, 2, 32768, 65536, 1073741823, 1073741824, 2147483646, 2147483647)
HeightCase = tuple[int, int, int, int]


def signed_height_word(word: int) -> int:
    """Interpret exactly one generated uint32 pattern as signed32; e.g. signed_height_word(0xffffffff)."""
    assert 0 <= word <= 0xffffffff
    raw = struct.pack('<I', word)
    signed = struct.unpack('<i', raw)[0]
    return signed


def height_operation_cases() -> Iterator[HeightCase]:
    """Cross missing-input order, signed extrema, overflow and deterministic products; e.g. list(height_operation_cases())."""
    yield from product(range(2), HEIGHT_COUNTS, range(2), CELL_RESULTS)
    seed = 0x849c011
    for _ in range(8192):
        seed = (1664525 * seed + 1013904223) & 0xffffffff
        count = signed_height_word(seed)
        seed = (1664525 * seed + 1013904223) & 0xffffffff
        yield (1, count, 1, signed_height_word(seed))


def expected_height_words(case: HeightCase) -> tuple[int, int, int, int]:
    """Use independent unbounded integer multiplication followed by the native low32 mask; e.g. expected_height_words(case)."""
    count_present, count, cell_present, cell = case
    if not cell_present: return (10, 0, 0, 0)
    if not count_present: return (11, 0, 0, 0)
    bits = (count * cell) & 0xffffffff
    return (1 if bits == 0 else 2, bits, 1, 1)


def height_case_packet(case: HeightCase) -> bytes:
    """Retain original presence/raw words and complete independent result bits; e.g. height_case_packet(case)."""
    arguments = struct.pack('<4I', *(value & 0xffffffff for value in case))
    expected = struct.pack('<4I', *expected_height_words(case))
    assert len(arguments) == len(expected) == 16
    return arguments + expected


def verify_height_operation_packets() -> None:
    """Freeze all8864 exact products and missing-dependency outcomes; e.g. verify_height_operation_packets()."""
    cases = list(height_operation_cases())
    assert len(cases) == 8864
    packet = struct.pack('<II', 1, len(cases)) + b''.join(map(height_case_packet, cases))
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-height-operations.bin': packet})
    dialog_baseline('legend-height-operation-packets-reference.json', dict(cases=len(cases),
        status_counts=dict(Counter(str(expected_height_words(case)[0]) for case in cases)), packets=summaries,
        oracle='Independent unbounded integer product masked to32 bits; byte-bound cell-helper-call-before-IMUL dependency order',
        native_font_helper_executed=False, pixel_validity_established=False))


def height_pattern_contexts(owned: list[tuple[int, int, bytes]]) -> list[tuple[int, int, bytes]]:
    """Patch only the actual count word in complete styles without resizing any other part; e.g. height_pattern_contexts(owned)."""
    family, version, original = next(context for context in owned if context[:2] == (2, 700006))
    fields = legend_height_observation(family, version, original)
    assert len(fields) == 1
    counts = list(HEIGHT_COUNTS)
    seed = 0x849c011
    for _ in range(256):
        seed = (1664525 * seed + 1013904223) & 0xffffffff
        counts.append(signed_height_word(seed))
    offset = cast(int, fields[0]['offset'])
    return [(family, version, original[:offset] + struct.pack('<i', count) + original[offset + 4:]) for count in counts]


def height_projection_context(context: tuple[int, int, bytes]) -> bytes:
    """Retain each complete parent and five explicit hypothetical helper results; e.g. height_projection_context(context)."""
    family, version, raw = context
    fields = legend_height_observation(family, version, raw)
    assert len(fields) <= 1
    present = int(bool(fields))
    count = cast(int, fields[0]['signed_value']) if fields else 0
    cells = (None, 0, 2, -2147483648, 2147483647)
    cases = [(present, count, int(cell is not None), cell or 0) for cell in cells]
    return struct.pack('<III', family, version, len(raw)) + raw + struct.pack('<I', len(cases)) + b''.join(map(height_case_packet, cases))


def verify_height_projection_packets() -> None:
    """Retain425 owned,276 native and268 complete count-mutation contexts; e.g. verify_height_projection_packets()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert (len(owned), len(native)) == (425, 276)
    mutants = height_pattern_contexts(owned)
    assert len(mutants) == 268
    packets = [struct.pack('<II', 1, len(contexts)) + b''.join(map(height_projection_context, contexts)) for contexts in (owned + mutants, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'legend-height-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-legend-height-projection-native.bin': packets[1]}))
    dialog_baseline('legend-height-projection-packets-reference.json', dict(owned_contexts=425, native_contexts=276,
        complete_count_mutants=268, helper_contexts_per_parent=5, packets=summaries,
        runtime_font_metrics_inferred=False, universal_tick_count_inferred=False, native_helper_executed=False))
