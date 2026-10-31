"""Independent Results hierarchy reader and current-writer wire literals (#164)."""
import struct
from owned_thermal import persisted_literal

ResultsBlocks = list[tuple[str, bytes]]
SIGNED_EDGES = (-2147483648, -1, 0, 1, 2147483647)
UNSIGNED_EDGES = (0, 1, 2147483648, 4294967295, 2147483647)
TEXT_EDGES: tuple[bytes | None, ...] = (None, b'\0\0', b'\0\xd8\0\0', b'\x33\0\0', b'A\0\0\0B\0\0\0')


def results_word(label: str, rotation: int, unsigned: bool = False) -> tuple[str, bytes]:
    """Keep signed and unsigned domains distinct; e.g. results_word('member70', 3, True)."""
    values = UNSIGNED_EDGES if unsigned else SIGNED_EDGES
    return label, struct.pack('<I' if unsigned else '<i', values[rotation % len(values)])


def results_text(label: str, rotation: int, null_only: bool = False) -> tuple[str, bytes]:
    """Preserve null, empty, surrogate, odd and embedded-NUL frames; e.g. results_text('text60', 2)."""
    return label, persisted_literal(None if null_only else TEXT_EDGES[rotation % len(TEXT_EDGES)])


def results_root_blocks(version: int, rotation: int, null_only: bool = False) -> ResultsBlocks:
    """Encode the standalone root reader gates from C-762; e.g. results_root_blocks(840013, 1)."""
    blocks = [results_word('member58', rotation), results_text('text60', rotation, null_only),
              results_word('member70', rotation, True)]
    if version >= 840013: blocks.append(results_word('member74', rotation + 1))
    if version >= 890402: blocks.append(results_word('member08', rotation + 2))
    if version >= 890605: blocks.append(results_text('text68', rotation + 1, null_only))
    return blocks


def results_category_blocks(version: int, rotation: int, null_only: bool = False) -> ResultsBlocks:
    """Keep the retired text list separate from its replacement word; e.g. results_category_blocks(800044, 2)."""
    blocks = [results_word('word10', rotation)]
    if version >= 810010: return blocks + [results_word('unsigneda8', rotation, True)]
    if version < 800044: return blocks
    count = 0 if null_only else rotation % 4
    blocks.append(('text_list.count', struct.pack('<i', count)))
    return blocks + [results_text(f'text_list.{index}', rotation + index) for index in range(count)]


def results_plot_blocks(version: int, rotation: int, null_only: bool = False) -> ResultsBlocks:
    """Retain the original temporary word before runtime normalization; e.g. results_plot_blocks(830035, 0)."""
    words = results_word('temporary', rotation)[1] + results_word('member28', rotation + 1)[1]
    blocks = [('temporary_word_and_member28', words)]
    return blocks + ([results_text('text08', rotation, null_only)] if version >= 830035 else [])


def results_writer_blocks(level: int, rotation: int, null_only: bool = False) -> ResultsBlocks:
    """Encode only documented current root/category writers; e.g. results_writer_blocks(1, 3)."""
    if type(level) is not int or level not in (0, 1):
        raise ValueError(f'{level}; expected current Results writer level 0 or 1')
    if level == 0: return results_root_blocks(890605, rotation, null_only)
    return results_category_blocks(810010, rotation, null_only)
