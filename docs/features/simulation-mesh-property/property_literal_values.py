"""Independent raw scalar patterns and physical literal assembly for Mesh Property (#163)."""
from typing import TypedDict
import struct
from owned_components import signed_words
from owned_embedded import patterned_doubles
from owned_thermal import persisted_literal
from native_carriers import digest

PropertyBlocks = list[tuple[str, bytes]]
EDGE_WORDS = (-2147483648, -1, 0, 1, 2147483647)
EDGE_TEXTS: tuple[bytes | None, ...] = (None, b'\0\0', b'\0\xd8\0\0', b'\x33\0\0')


class PropertyLiteralSpan(TypedDict):
    label: str
    offset: int
    end: int
    sha256: str


def property_words(count: int, rotation: int) -> bytes:
    """Keep signed extremes without assigning field meanings; e.g. property_words(3, 1)."""
    return signed_words(*(EDGE_WORDS[(index + rotation) % len(EDGE_WORDS)] for index in range(count)))


def property_raw64(count: int, rotation: int) -> bytes:
    """Retain signed zero, subnormal, infinity and NaN bit patterns; e.g. property_raw64(33, 2)."""
    start = rotation % 6
    return patterned_doubles(count + start)[start * 8:]


def property_text(rotation: int, null_only: bool = False) -> bytes:
    """Keep null, allocated empty, surrogate and odd raw text frames distinct; e.g. property_text(3)."""
    return persisted_literal(None if null_only else EDGE_TEXTS[rotation % len(EDGE_TEXTS)])


def property_count(label: str, count: int) -> tuple[str, bytes]:
    """Encode an independently chosen count, never a parser-derived repair; e.g. property_count('mesh.text_list.count', 2)."""
    return label, struct.pack('<i', count)


def assemble_property_blocks(blocks: PropertyBlocks) -> tuple[bytes, list[PropertyLiteralSpan]]:
    """Derive literal boundaries before any reference parser runs; e.g. assemble_property_blocks(blocks)."""
    raw = bytearray()
    spans: list[PropertyLiteralSpan] = []
    for label, fragment in blocks:
        start = len(raw)
        raw.extend(fragment)
        spans.append(PropertyLiteralSpan(label=label, offset=start, end=len(raw), sha256=digest(fragment)))
    return bytes(raw), spans
