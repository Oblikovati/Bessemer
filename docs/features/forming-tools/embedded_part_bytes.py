"""Compose an inline part from the owned blank part with observed MFC relocation (#54)."""
from collections.abc import Callable
from functools import partial
from pathlib import Path
from typing import TypeVar
import struct
import sys

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for name in ('model-archives', 'manager-scalars', 'resolved-features'):
    sys.path.insert(0, str(FEATURE.parent / name))
from manager_tag_observation import ManagerTagObservation, relocated_tail
from freeze_owned_features import ReferenceDocument

T = TypeVar('T')


class EditedPartDocument:
    """Keep the original header and names while selecting the independent edited configuration."""
    def __init__(self, original: ReferenceDocument) -> None:
        self.original = original
        self.configuration = (ROOT / 'tests/fixtures/model-archives/native-part-name-edited.bin').read_bytes()

    def names(self) -> list[str]:
        return self.original.names()

    def read(self, name: str) -> bytes:
        return self.configuration if name == 'Contents/Config-0' else self.original.read(name)


def observed_read(raw: bytes, read: Callable[[], T]) -> tuple[T, list[tuple[int, int]]]:
    """Profile actual object calls without replacing readers; e.g. observed_read(raw,read)."""
    from mfcobjects import ObjectArchive
    observer = ManagerTagObservation(raw, ObjectArchive.object.__code__)
    previous = sys.getprofile()
    try:
        sys.setprofile(observer.capture)
        result = read()
    finally:
        sys.setprofile(previous)
    return result, observer.events


def part_indices(raw: bytes, events: list[tuple[int, int]], added: int, self_reference: int) -> bytes:
    """Keep the part owner and imported self-reference distinct; e.g. part_indices(raw,events,20,12)."""
    changed = bytearray(relocated_tail(raw, events, 17, 4, added))
    replacements = {1: 3, 2: 4, 3: self_reference}
    for offset, _ in events:
        if offset <= 17:
            continue
        tag = struct.unpack_from('<H', raw, offset)[0]
        if tag in (0, 65535):
            continue
        extended = tag == 32767
        start, shape = (offset + 2, '<I') if extended else (offset, '<H')
        value = struct.unpack_from(shape, raw, start)[0]
        mask = 0x80000000 if extended else 0x8000
        if value & (mask - 1) in replacements:
            struct.pack_into(shape, changed, start, (value & mask) | replacements[value & (mask - 1)])
    return bytes(changed)


def blank_tool(edited: bool = False) -> bytes:
    """Insert a real inline header instead of synthetic imported map entries; e.g. blank_tool()."""
    import modelprefix
    from mfcobjects import ObjectArchive
    from swcontainer import Document
    document = Document((ROOT / 'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    if edited:
        document = EditedPartDocument(document)
    raw = document.read('Contents/Config-0')
    (part, cursor), events = observed_read(raw, partial(modelprefix.read, document, through_part_resolved=True))
    assert cursor.p == len(raw) and cursor.map[3]['_class'] == 'moExtObject_c'
    header_raw = document.read('Contents/Config-0-ModelHeader')
    header_cursor = ObjectArchive(header_raw, 19000)
    header, header_events = observed_read(header_raw, partial(header_cursor.object, 'moHeader_c'))
    external = header['external_references']
    assert header_cursor.p == len(header_raw) and (external is None or not external['items'])
    self_slot = next(i for i, value in enumerate(header_cursor.map) if value is header['self_reference']) + 4
    inline = relocated_tail(header_raw, header_events, -1, 1, 4)
    changed = part_indices(raw, events, len(header_cursor.map), self_slot)
    lightweight = struct.pack('<i', part['lightweight_size'] + len(inline)) + inline + changed[18:]
    name = b'moFormTool_c'
    return struct.pack('<HHH', 65535, 1, len(name)) + name + changed[:14] + lightweight + struct.pack('<ii', -7, 8)


def paired_tools(reference: Path) -> bytes:
    """Reuse the real part identity in a second tool; e.g. paired_tools(reference)."""
    from object_fixture_observer import object_cursor
    raw = blank_tool()
    cursor = object_cursor(reference, raw, 19000, None)
    tool, events = observed_read(raw, cursor.object)
    assert cursor.p == len(raw) and tool['part_08']['stage'] == 'complete'
    relocated = relocated_tail(raw, events, -1, 1, 2)
    name = b'suObArray'
    array = struct.pack('<HHH', 65535, 1, len(name)) + name + struct.pack('<H', 2)
    return array + relocated + struct.pack('<HHii', 0x8003, 6, -15, 16)


def tool_cases(reference: Path) -> list[tuple[str, bytes]]:
    """Cover fresh, null and shared embedded pointers without fake map slots; e.g. tool_cases(pin)."""
    name = b'moFormTool_c'
    empty = struct.pack('<HHH', 65535, 1, len(name)) + name + struct.pack('<Hii', 0, -7, 8)
    return [('blank-tool', blank_tool()), ('null-part-tool', empty), ('paired-tools', paired_tools(reference)),
            ('edited-tool', blank_tool(edited=True))]
