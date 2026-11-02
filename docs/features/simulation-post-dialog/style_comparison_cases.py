"""C863 owned runtime facts, physical alpha aliases and conditional/missing/IEEE boundaries (#166)."""
from collections.abc import Iterator
from itertools import product
import struct
from color_comparison_packets import PATTERNS
from legend_comparison_packets import baseline_legend, legend_boundary_pairs, legend_float_pairs
from style_comparison_inputs import StyleFrame, STYLE_INPUT_PATHS


def baseline_style(controls: tuple[int, ...]) -> StyleFrame:
    """Supply explicit complete runtime facts rather than loading constructor defaults; e.g. baseline_style((2,1,0,1,1))."""
    state = baseline_legend(0, 1, (0, 0, 0)) | {path: bytes(4) for path in STYLE_INPUT_PATHS}
    for path, word in zip(('style.word_1b8', 'style.word_21c', 'style.word_230', 'style.word_1ec', 'style.word_204'), controls, strict=True):
        state[path] = struct.pack('<I', word)
    return state


def owned_style_pairs() -> Iterator[tuple[StyleFrame, StyleFrame]]:
    """Cross every outer gate, isolated differences and both/one-sided unknown physical inputs; e.g. list(owned_style_pairs())."""
    for controls in product((0, 1, 2, 3), (0, 0xffffffff), (0, 0xffffffff), (0, 0xffffffff), (0, 0xffffffff)):
        state = baseline_style(controls)
        yield state, state
        for path in STYLE_INPUT_PATHS:
            raw = struct.unpack('<I', state[path])[0]
            yield state, state | {path: struct.pack('<I', raw ^ 0x3f800000)}
            missing = {name: raw for name, raw in state.items() if name != path}
            yield state, missing
            yield missing, state
            yield missing, missing
    yield from style_float_pairs()
    yield from style_partial_color_pairs()
    for left, right in (*legend_boundary_pairs(), *legend_float_pairs()):
        base = baseline_style((0, 0, 0, 0, 0))
        yield {key: raw for key, raw in base.items() if key.startswith('style.')} | left, {key: raw for key, raw in base.items() if key.startswith('style.')} | right
    yield from generated_style_pairs()


def style_float_pairs() -> Iterator[tuple[StyleFrame, StyleFrame]]:
    """Exercise scalar aliases and all reached color channels under original C864 IEEE rules; e.g. list(style_float_pairs())."""
    state = baseline_style((2, 1, 0, 1, 1))
    for path, left, right in product(('style.color_220.component_3', 'style.color_2ac.component_3'), PATTERNS, PATTERNS):
        yield state | {path: struct.pack('<I', left)}, state | {path: struct.pack('<I', right)}
    for path, word in product(STYLE_INPUT_PATHS[11:], PATTERNS):
        yield state, state | {path: struct.pack('<I', word)}


def style_partial_color_pairs() -> Iterator[tuple[StyleFrame, StyleFrame]]:
    """Prove early channel differences can skip unknown later channels and alpha-only inputs can be sufficient; e.g. list(style_partial_color_pairs())."""
    base = baseline_style((2, 1, 0, 1, 1))
    for prefix in ('1bc', '1cc', '220', '29c', '1f4', '20c'):
        left = {path: raw for path, raw in base.items() if path != f'style.color_{prefix}.component_1'}
        right = left | {f'style.color_{prefix}.component_0': struct.pack('<I', 0x3f800000)}
        yield left, right
    state = baseline_style((0, 1, 1, 0, 0))
    alpha_only = {path: raw for path, raw in state.items() if not any(path == f'style.color_220.component_{i}' for i in range(3))}
    yield alpha_only, alpha_only
    yield {}, {}


def generated_style_pairs() -> Iterator[tuple[StyleFrame, StyleFrame]]:
    """Mix arbitrary raw patterns, optional channels and outer control configurations deterministically; e.g. list(generated_style_pairs())."""
    seed = 0x863
    for number in range(4096):
        seed = (1664525 * seed + 1013904223) % (1 << 32)
        controls = (seed % 4, (seed >> 4) & 1, (seed >> 5) & 1, (seed >> 6) & 1, (seed >> 7) & 1)
        left = baseline_style(controls); right = left.copy()
        for change in range(1 + number % 3):
            seed = (1664525 * seed + 1013904223) % (1 << 32)
            path = STYLE_INPUT_PATHS[(seed >> 8) % len(STYLE_INPUT_PATHS)]
            if (number + change) % 5 == 0: right.pop(path, None)
            else: right[path] = struct.pack('<I', seed)
        yield left, right
