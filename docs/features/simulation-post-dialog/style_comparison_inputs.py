"""C863 single-storage input identities and exact complete-source projections (#166)."""
from typing import cast
import importlib
import struct

StyleFrame = dict[str, bytes]
STYLE_WORD_OFFSETS = (0x1b8, 0x21c, 0x230, 0x28c, 0x290, 0x294, 0x298, 0x1ec, 0x1f0, 0x204, 0x208)
STYLE_COLOR_OFFSETS = (0x1bc, 0x1cc, 0x220, 0x29c, 0x1f4, 0x20c, 0x2ac)
STYLE_INPUT_PATHS = tuple(f'style.word_{offset:x}' for offset in STYLE_WORD_OFFSETS) + tuple(
    f'style.color_{offset:x}.component_{channel}' for offset in STYLE_COLOR_OFFSETS for channel in range(4))
STYLE_MEMORY_PATHS = dict(zip(STYLE_WORD_OFFSETS, STYLE_INPUT_PATHS[:11], strict=True)) | {
    offset + 4 * channel: f'style.color_{offset:x}.component_{channel}'
    for offset in STYLE_COLOR_OFFSETS for channel in range(4)}


def loaded_style_frame(spans: list[dict[str, object]]) -> StyleFrame:
    """Keep every relevant actual wire input with aliases represented by one channel key; e.g. loaded_style_frame(spans)."""
    fields = {cast(str, span['label']): bytes.fromhex(cast(str, span['raw_hex']))
        for span in spans if span['label'] in STYLE_INPUT_PATHS}
    legend = importlib.import_module('cosworks_legend_comparison').loaded_fields(spans)
    return fields | cast(StyleFrame, legend)


def style_state_packet(frame: StyleFrame) -> bytes:
    """Retain all39 optional style patterns plus the existing64-word legend input; e.g. style_state_packet(frame)."""
    from legend_comparison_packets import legend_state_packet
    values = [frame.get(path) for path in STYLE_INPUT_PATHS]
    assert all(value is None or len(value) == 4 for value in values), frame
    mask = sum(1 << index for index, value in enumerate(values) if value is not None)
    return struct.pack('<Q', mask) + b''.join(bytes(4) if value is None else value for value in values) + legend_state_packet(frame)
