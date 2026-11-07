"""Independent C848 raw-control diagnostics and bounded dispatcher truth table (#166)."""
from collections.abc import Iterator
from itertools import product
import importlib
import struct

PaletteCase = tuple[int, int, int, int, int, int, int, int, int, int]
PALETTE_COUNTS = (-2147483648, -10, -1, 0, 1, 5, 9, 10, 2147483647)
PALETTE_REVERSE = (-2147483648, -2, -1, 0, 1, 2, 2147483647)
PALETTE_MODES = (-2147483648, -1, 0, 3, 4, 5, 2147483647)
PALETTE_GRAY = (-2147483648, -1, 0, 1, 2147483647)


def palette_context_cases() -> Iterator[PaletteCase]:
    """Cross every optional control, required runtime fact and nonboolean branch value; e.g. list(palette_context_cases())."""
    returns = (None, -2147483648, -1, 0, 1, 2147483647)
    plots = (None, -2147483648, -1, 0, 3, 4, 2147483647)
    for mask, mode, gray, returned, pointer, plot in product(range(16), PALETTE_MODES, PALETTE_GRAY, returns, range(3), plots):
        yield (mask, mode, -2, 10, gray, int(returned is not None), returned or 0, pointer, int(plot is not None), plot or 0)


def palette_diagnostic_cases() -> Iterator[PaletteCase]:
    """Retain raw count/reversal values even when dispatcher dependencies bypass them; e.g. list(palette_diagnostic_cases())."""
    scenarios = ((0, -1, 1, 0, 0, 0), (4, 0, 0, 2, 1, 3), (4, 1, 0, 2, 1, -1), (42, -2147483648, -1, 1, 0, 0))
    for count, reverse, mask, scenario in product(PALETTE_COUNTS, PALETTE_REVERSE, range(16), scenarios):
        mode, gray, returned, pointer, present, plot = scenario
        yield (mask, mode, reverse, count, gray, 1, returned, pointer, present, plot)


def palette_raw_diagnostics(case: PaletteCase) -> list[int]:
    """Use the unchanged copied Source model for flag and fixed-slot comparison views; e.g. palette_raw_diagnostics(case)."""
    mask, mode, reverse, count, gray, *_ = case
    controls = [('legend.word_8', mode), ('legend.word_c', reverse), ('legend.word_10', count), ('legend.word_d4', gray)]
    spans = [dict(label=path, offset=index * 4, end=index * 4 + 4, raw_hex=struct.pack('<i', value).hex())
        for index, (path, value) in enumerate(controls) if mask & 1 << index]
    helper = importlib.import_module('cosworks_legend_color_policy_fields')
    observed = {field['name']: field for field in helper.decode_fields(spans)}
    output: list[int] = []
    for role, key in (('reverse_base_palette_raw', 'consumer_nonzero'),
            ('show_gray_above_yield_enabled_raw', 'consumer_nonzero'), ('base_palette_color_count', 'within_nine_serialized_slots')):
        field = observed.get(role)
        output.extend((int(field is not None), int(field[key]) if field is not None else 0))
    return output


def expected_palette_dispatch(case: PaletteCase) -> list[int]:
    """Follow the byte-bound branch order without running a destination algorithm; e.g. expected_palette_dispatch(case)."""
    mask, mode, _, _, gray, returned_present, returned, pointer, plot_present, plot = case
    if mask & 8 == 0: return [10, 0, 0, 0, 0]
    if gray != 0 and not returned_present: return [11, 0, 0, 0, 0]
    if gray != 0 and returned != 0: return [1, 8, 1, 0, 0]
    if mask & 1 == 0: return [12, 0, 0, 0, 0]
    if mode != 4: return [2, 9, int(gray != 0), 0, 0]
    if pointer == 0: return [13, 0, 0, 0, 0]
    if pointer == 1: return [14, 0, 0, 0, 0]
    if not plot_present: return [15, 0, 0, 0, 0]
    return [3 if plot == 3 else 4, 9, int(gray != 0), 1, 1]


def palette_case_packet(case: PaletteCase) -> bytes:
    """Keep all supplied words/facts and independent diagnostic/selection results; e.g. palette_case_packet(case)."""
    expected = expected_palette_dispatch(case) + palette_raw_diagnostics(case)
    assert len(case) == 10 and len(expected) == 11
    return struct.pack('<10I', *(word & 0xffffffff for word in case)) + struct.pack('<11I', *expected)
