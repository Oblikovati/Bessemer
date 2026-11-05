"""Independent complete stored visual word patterns and scalar expectations (#181)."""
from typing import NamedTuple, cast
import struct


class VisualStoredWords(NamedTuple):
    layer_number: int = 23
    by_layer: int = 0xa5a5
    cache: int = 0xfd
    color: int = 0x80563412
    line_style: int = 0x8000
    standard_width: int = -32768
    custom_bits: int = 0x80000000


def visual_word_bytes(words: VisualStoredWords) -> bytes:
    """Encode the physical record without Source or Rust writers; e.g. visual_word_bytes(VisualStoredWords())."""
    raw = struct.pack('<HHBIHhI', *words)
    assert len(raw) == 17, f'{len(raw)}; expected the complete modern17-byte visual record'
    return raw


def contrasting_visual_words(words: VisualStoredWords) -> VisualStoredWords:
    """Give the loaded-class definition distinct stored values and float bits; e.g. contrasting_visual_words(words)."""
    width = struct.unpack('<h', struct.pack('<H', (words.standard_width & 0xffff) ^ 0x8000))[0]
    return VisualStoredWords(words.layer_number ^ 0xffff, words.by_layer ^ 0xffff, words.cache ^ 0xff,
        words.color ^ 0x80abcdef, words.line_style ^ 0xffff, width, words.custom_bits ^ 0x80000000)


def visual_rgb_cases() -> list[tuple[str, VisualStoredWords]]:
    """Cross every high byte with six RGB patterns and both layer states; e.g. visual_rgb_cases()."""
    rows: list[tuple[str, VisualStoredWords]] = []
    colors = ((0, 0, 0), (255, 0, 0), (0, 255, 0), (0, 0, 255), (0x12, 0x34, 0x56), (255, 255, 255))
    for high in range(256):
        for rgb, layer in ((rgb, layer) for rgb in colors for layer in (23, 0xffff)):
            packed = high * 0x1000000 + rgb[0] + rgb[1] * 256 + rgb[2] * 65536
            rows.append(('rgb', VisualStoredWords(layer, 0xa5a0 | (high & 31), 0xa4 | (high & 3), packed)))
    assert len(rows) == 3072, f'{len(rows)}; expected256high bytes times6RGB tuples times2layer states'
    return rows


def visual_flag_cases() -> list[tuple[str, VisualStoredWords]]:
    """Cross stored cache bytes and ByLayer combinations with unresolved patterns; e.g. visual_flag_cases()."""
    rows = [('cache', VisualStoredWords(layer, cache=cache)) for cache in range(256) for layer in (23, 0xffff)]
    for low in range(32):
        for extra, layer in ((extra, layer) for extra in (0, 0xffe0, 0xa5a0) for layer in (23, 0xffff)):
            rows.append(('by_layer', VisualStoredWords(layer, low | extra)))
    assert len(rows) == 704, f'{len(rows)}; expected512cache and192ByLayer cases'
    return rows


def visual_width_layer_cases() -> list[tuple[str, VisualStoredWords]]:
    """Retain signed width limits and exact NaN/infinity/zero patterns; e.g. visual_width_layer_cases()."""
    bits = (0, 0x80000000, 0x7fc01234, 0x7f800042, 0x7f800000, 0xff800000)
    rows = [('width', VisualStoredWords(standard_width=width, custom_bits=raw))
            for width in (-32768, -1, 0, 1, 32767) for raw in bits]
    values = (0, 1, 23, 65534, 65535)
    rows.extend(('layer_style', VisualStoredWords(layer_number=layer, line_style=style)) for layer in values for style in values)
    assert len(rows) == 55, f'{len(rows)}; expected30width and25layer/style cases'
    return rows


def verify_visual_scalar_projection(actual: dict[str, object], words: VisualStoredWords) -> None:
    """Require named and raw Source aliases to agree with independent words; e.g. verify_visual_scalar_projection(actual, words)."""
    for key, expected in (('layer_number', words.layer_number), ('unknown_0', words.layer_number),
        ('by_layer_flags', words.by_layer), ('unknown_2', words.by_layer),
        ('stored_layer_attribute_cache', words.cache), ('unknown_4', words.cache),
        ('color_word', words.color), ('unknown_8', words.color), ('line_style', words.line_style), ('layer', words.line_style)):
        assert actual[key] == expected, f'{key}/{actual[key]}; expected literal stored value {expected}'
    rgb = (words.color & 255, (words.color // 256) & 255, (words.color // 65536) & 255)
    assert actual['color_rgb'] == rgb and actual['color_unresolved_high_byte'] == words.color // 0x1000000, actual
    assert actual['color_is_missing'] == (words.color == 0xffffffff), actual
    width = cast(dict[str, object], actual['line_width'])
    assert width['standard_width'] == words.standard_width, width
    assert actual['stored_visible'] == bool(words.cache & 1) and actual['stored_printable'] == bool(words.cache & 2), actual
    for name, mask in (('color', 1), ('font', 2), ('width', 4), ('font_sp', 8), ('width_sp', 16)):
        assert actual['by_layer_' + name] == bool(words.by_layer & mask), actual
    assert actual['by_layer_unresolved_bits'] == words.by_layer & 0xffe0, actual
