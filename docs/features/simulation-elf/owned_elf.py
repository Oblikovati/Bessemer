"""Independent ELF constructor, index and scalar-record fixtures (#115)."""
import struct
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'simulation-out'))
from owned_reports import transfer as wrap
from dataclasses import dataclass

RECORD = 244
INDEX = 1220
PATTERNS = (0, 0x80000000, 1, 0x007fffff, 0x00800000, 0x3f800000,
            0xbf800000, 0x7f800000, 0xff800000, 0x7fc12345, 0xffc54321, 0x7fa00001)


def initial(parameter_a: int, parameter_b: int) -> bytes:
    """Use the native closed forms, retaining independent parameter patterns."""
    record_words = RECORD // 4
    marker = 199 // record_words + 3
    body = bytearray((marker - 1) * RECORD)
    struct.pack_into('<7I', body, 0, marker, RECORD, 100, parameter_a, 0, parameter_b, 1)
    return wrap(bytes(body))


def scalar_record(identifier: int, groups: list[list[int]]) -> bytes:
    """Build all store-stage bytes independently, with positive-zero omitted cells."""
    assert -0x80000000 <= identifier <= 0x7fffffff
    assert len(groups) <= 10 and all(len(group) <= 6 for group in groups)
    raw = bytearray(RECORD)
    struct.pack_into('<i', raw, 0, identifier)
    for group_index, group in enumerate(groups):
        for component_index, pattern in enumerate(group):
            struct.pack_into('<I', raw, 4 + group_index * 24 + component_index * 4, pattern)
    return bytes(raw)


def patterned_record(identifier: int) -> bytes:
    """Exercise IEEE boundary patterns in every physical scalar group."""
    groups = [[PATTERNS[(group * 6 + component) % len(PATTERNS)] for component in range(6)] for group in range(10)]
    return scalar_record(identifier, groups)


def indexed(identifiers: tuple[int, ...], selector: int = 1, watermark: int = -2) -> bytes:
    """Construct complete synthetic records after the five-record header/index area."""
    assert 1 <= selector <= 100
    body = bytearray(initial(0xdeadbeef, len(identifiers))[4:])
    struct.pack_into('<I', body, 0, 6 + len(identifiers))
    struct.pack_into('<i', body, 16, watermark)
    struct.pack_into('<2i', body, RECORD + 8 * (selector - 1), 6, len(identifiers))
    body.extend(b''.join(patterned_record(identifier) for identifier in identifiers))
    return wrap(bytes(body))


def header_projection(raw: bytes, selector: int) -> bytes:
    """Compute only the documented cleared header/index write, never advertised records."""
    words = list(struct.unpack_from('<7I', raw, 4))
    watermark = struct.unpack_from('<i', raw, 20)[0]
    marker, count = words[0], words[5]
    assert 6 <= marker and marker + count <= 0x7fffffff and 1 <= selector <= 100
    words[0], words[4] = marker + count, max(watermark, selector) & 0xffffffff
    body = bytearray(INDEX)
    struct.pack_into('<7I', body, 0, *words)
    struct.pack_into('<2i', body, RECORD + 8 * (selector - 1), marker, count)
    return bytes(body)


@dataclass(frozen=True)
class MappingCase:
    label: str
    raw: bytes
    selector: int
    source_map: tuple[int, ...]
    destination_count: int
    expected_slots: tuple[bytes | None, ...]
    expected_writes: tuple[tuple[int, int], ...]


def mapping_cases() -> list[MappingCase]:
    """Specify mapping outcomes literally rather than reimplementing the reference loop."""
    seven = patterned_record(7)
    overwritten = bytearray(seven)
    struct.pack_into('<I', overwritten, 240, 0x12345678)
    overwrite_source = bytearray(indexed((7, 1, 7, -1)))
    offset = 4 + INDEX + 2 * RECORD
    overwrite_source[offset:offset + RECORD] = overwritten
    return [
        MappingCase('stop-skip-map', indexed((7, 0, -1, 9)), 1, (0, 0, 0, 0, 0, 0, 2), 3, (None, seven, None), ((7, 1),)),
        MappingCase('overwrite', bytes(overwrite_source), 1, (2, 0, 0, 0, 0, 0, 2), 3, (None, bytes(overwritten), None), ((7, 1), (1, 1), (7, 1))),
        MappingCase('zero-map', indexed((1, 0, -1)), 1, (0,), 2, (None, None), ()),
        MappingCase('last-selector', indexed((7, -1), 100), 100, (0, 0, 0, 0, 0, 0, 1), 1, (seven,), ((7, 0),)),
        MappingCase('declared-count-exhausted', indexed((1, 0, 7)), 1, (2, 0, 0, 0, 0, 0, 1), 2, (seven, patterned_record(1)), ((1, 1), (7, 0))),
        MappingCase('empty', initial(1, 2), 1, (), 0, (), ()),
    ]
