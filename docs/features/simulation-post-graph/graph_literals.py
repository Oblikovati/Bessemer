"""Independent complete C-765 wire literals without native normalization (#165)."""
import struct

GraphBlocks = list[tuple[str, bytes]]
GRAPH_WORDS = (-2147483648, -1, 0, 1, 2147483647)
GRAPH_RAW64 = (0, 0x8000000000000000, 1, 0x7ff0000000000000, 0x7ff8000000000042)
GRAPH_TEXTS = (b'', b'A\0', b'\0\xd8', b'\0\0A\0\xff\xff', b'\x3d\xd8\0\xde')


def graph_words(count: int, rotation: int) -> bytes:
    """Preserve signed extremes without destination meanings; e.g. graph_words(12, 1)."""
    return b''.join(struct.pack('<i', GRAPH_WORDS[(index + rotation) % 5]) for index in range(count))


def graph_raw64(count: int, rotation: int) -> bytes:
    """Preserve unassigned eight-byte patterns without floating-point conversion; e.g. graph_raw64(16, 2)."""
    return b''.join(struct.pack('<Q', GRAPH_RAW64[(index + rotation) % 5]) for index in range(count))


def graph_text_blocks(label: str, rotation: int, empty_only: bool) -> GraphBlocks:
    """Keep character counts and raw code units without a terminator rule; e.g. graph_text_blocks('textb0', 2, False)."""
    raw = b'' if empty_only else GRAPH_TEXTS[rotation % 5]
    return [(label + '.character_count', struct.pack('<i', len(raw) // 2)), (label + '.utf16', raw)]


def graph_initial_blocks(version: int, rotation: int, empty_only: bool) -> GraphBlocks:
    """Place the two 870002 words around the original three-word block; e.g. graph_initial_blocks(870002, 1, False)."""
    blocks = [('twelve_words08_34', graph_words(12, rotation))]
    if version >= 870002: blocks.append(('word38', graph_words(1, rotation + 1)))
    blocks.append(('three_words3c_44', graph_words(3, rotation + 2)))
    if version >= 870002: blocks.append(('word48', graph_words(1, rotation + 3)))
    for index, label in enumerate(('textb0', 'textb8', 'textc0')):
        blocks.extend(graph_text_blocks(label, rotation + index, empty_only))
    return blocks + [('six_words4c_60', graph_words(6, rotation + 1)),
                     ('eighteen_words64_a8', graph_words(18, rotation + 2))]


def graph_blocks(version: int, rotation: int, empty_only: bool = False) -> GraphBlocks:
    """Encode every continuation gate in documented physical order; e.g. graph_blocks(890110, 3)."""
    blocks = graph_initial_blocks(version, rotation, empty_only)
    if version >= 800026:
        blocks.append(('wordc8', graph_words(1, rotation)))
        blocks.extend(graph_text_blocks('textd0', rotation + 3, empty_only))
        blocks.append(('sixteen_raw64_d8_150', graph_raw64(16, rotation)))
    if version >= 800062: blocks.append(('words158_160', graph_words(3, rotation + 1)))
    if version >= 820022:
        blocks.append(('word16c_two_raw64_words180_184', graph_words(1, rotation) + graph_raw64(2, rotation + 1) + graph_words(2, rotation + 2)))
    if version >= 840028: blocks.append(('word190', graph_words(1, rotation + 2)))
    if version >= 890005: blocks.append(('six_words198_1ac_raw64_1b0', graph_words(6, rotation + 3) + graph_raw64(1, rotation + 4)))
    if version >= 890110: blocks.append(('word1f8', graph_words(1, rotation + 4)))
    return blocks
