"""Independent complete ParameterSets sections and raw-pattern fixtures (#133)."""
import struct
from collections.abc import Sequence
from owned_persistence import word

DOUBLE_PATTERNS = (0, 0x8000000000000000, 0x7ff0000000000000, 0xfff0000000000000, 0x7ff0000000000001, 0x7ff8123456789abc, 1, 0x7fefffffffffffff)


def parameter_values(count: int) -> bytes:
    """Keep exact infinity, NaN, signed-zero, subnormal and finite eight-byte patterns."""
    return b''.join(struct.pack('<Q', DOUBLE_PATTERNS[index % len(DOUBLE_PATTERNS)]) for index in range(count))


def header_sections(version: int, table_count: int, record_count: int) -> list[tuple[str, bytes]]:
    """Keep legacy ignored header word zero independent of the fixed hundred-entry table."""
    first = table_count if version >= 810026 else -0x80000000
    sections = [('header', struct.pack('<5i', first, -1, 0x7fffffff, 0x12345678, -0x76543211))]
    if version > 700013:
        sections.append(('gate700013', word(0x81abcdef)))
    if version > 700018:
        sections.append(('gate700018', word(0xf0000000)))
    sections.append(('record_count', word(record_count)))
    sections.append(('table', b''.join(word(index ^ 0x80000000) for index in range(table_count))))
    return sections


def record_sections(version: int, table_count: int, index: int, text: bytes) -> list[tuple[str, bytes]]:
    """Build complete record bodies without converting any raw double to floating point."""
    prefix = f'record{index}.'
    sections = [(prefix + 'text', text), (prefix + 'integers', struct.pack('<3I', 0x80000000, 0xffffffff, 0x80000000))]
    if version > 700012:
        sections.append((prefix + 'long', word(0x01ffffff)))
    if version > 700013:
        sections.append((prefix + 'double', struct.pack('<Q', 0x7ff0000000000001)))
    sections.append((prefix + 'values', parameter_values(table_count)))
    return sections


def parameter_sections(version: int, table_count: int, texts: Sequence[bytes]) -> list[tuple[str, bytes]]:
    """Assemble the full version-selected grammar and every named physical section."""
    sections = header_sections(version, table_count, len(texts))
    for index, text in enumerate(texts):
        sections.extend(record_sections(version, table_count, index, text))
    sections.append(('tail', word(0x89abcdef)))
    if version > 820015:
        sections.append(('gate820015', word(0x76543210)))
    if version > 830009:
        sections.append(('gate830009', word(0xffffffff)))
    return sections


def parameter_bytes(version: int, table_count: int, texts: Sequence[bytes]) -> bytes:
    """Retain every literal section byte, including zero-length table/value sections."""
    return b''.join(raw for _, raw in parameter_sections(version, table_count, texts))
