"""Independent dialog wire primitives and explicit populated/empty fixture contexts (#166)."""
from dataclasses import dataclass
import struct
from graph_literals import graph_words, graph_raw64, graph_text_blocks

DialogBlocks = list[tuple[str, bytes]]
FLOAT32_PATTERNS = (0, 0x80000000, 1, 0x7f800000, 0x7fc00042)


@dataclass(frozen=True)
class DialogLiteralProfile:
    rotation: int = 0
    populated: bool = False
    duplicate_records: bool = False
    empty_texts: bool = True
    equation: bool = False


def dialog_words(prefix: str, offsets: tuple[int, ...], rotation: int) -> DialogBlocks:
    """Keep native member labels distinct from physical offsets; e.g. dialog_words('style', (0x238, 0x234), 1)."""
    return [(f'{prefix}.word_{offset:x}', graph_words(1, rotation + index)) for index, offset in enumerate(offsets)]


def dialog_color(prefix: str, offset: int, rotation: int) -> DialogBlocks:
    """Keep exact RGBA float32 bits, including nonfinite patterns; e.g. dialog_color('style', 0x1f4, 2)."""
    return [(f'{prefix}.color_{offset:x}.component_{index}', struct.pack('<I', FLOAT32_PATTERNS[(rotation + index) % 5])) for index in range(4)]


def dialog_text(label: str, profile: DialogLiteralProfile, increment: int = 0) -> DialogBlocks:
    """Encode signed code-unit counts without a terminator rule; e.g. dialog_text('text578', profile)."""
    return graph_text_blocks(label, profile.rotation + increment, profile.empty_texts)


def dialog_count(label: str, count: int) -> tuple[str, bytes]:
    """Encode only an independently selected stored count; e.g. dialog_count('sensors.count', 2)."""
    return label, struct.pack('<i', count)


def dialog_raw64(label: str, count: int, profile: DialogLiteralProfile) -> tuple[str, bytes]:
    """Retain unassigned eight-byte patterns without numeric conversion; e.g. dialog_raw64('four_raw64_548_560', 4, profile)."""
    return label, graph_raw64(count, profile.rotation)
