"""Owned raw logical components; no physical record extraction or chunk codec."""
from __future__ import annotations
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'flow-control-records'))
import control_requests as controls
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "flow-field-values"))
from wire_packet import packed
ROOT, SOURCE = controls.ROOT, controls.SOURCE
sys.path.insert(0, str(SOURCE))
import flowcheckpointcomponents as reference
KINDS = {'graph1': 1, 'graph2': 2, 'auxiliary': 3, 'trailing': 4, 'chunks': 5}
READERS = {'graph1': reference.read_graph1_object, 'graph2': reference.read_graph2_object,
           'auxiliary': reference.read_auxiliary_grid, 'trailing': reference.read_trailing_containers,
           'chunks': reference.read_chunk_buffer}
COUNTS = (0, 1, 2, 3, 16, 64, 257, 4096, 65536)
WIDE_PATTERNS = tuple(dict.fromkeys(controls.wides.RAW_PATTERNS))
WORD_PATTERNS = tuple(dict.fromkeys(controls.definitions.PATTERNS))
GroupWire = tuple[bytes, list[bytes]]


def integers(values: tuple[int, ...] | list[int]) -> bytes:
    return packed(values, 'i')


def words(values: tuple[int, ...] | list[int], width: int) -> bytes:
    assert width in (4, 8), (width, 'expected four- or eight-byte words')
    return packed(values, 'I' if width == 4 else 'Q')


def signed_pattern(value: int) -> int:
    return value if value < 0x80000000 else value - 0x100000000


def container(first: list[int], indices: list[int], second: list[int]) -> bytes:
    assert len(first) == len(indices) and len(second) == 2 * len(first)
    return integers([len(first)]) + words(first, 8) + words(indices, 4) + words(second, 8)


def patterned_container(count: int) -> bytes:
    wide = WIDE_PATTERNS
    return container([wide[n % len(wide)] for n in range(count)],
                     [(n * 0x80000001) & 0xffffffff for n in range(count)],
                     [wide[(n + 7) % len(wide)] for n in range(2 * count)])


def row(indices: list[int], slot_wire: bytes = b'') -> bytes:
    return integers([len(indices)]) + words(indices, 4) + slot_wire


def grid(dimensions: list[int], members: list[int], groups: list[GroupWire]) -> bytes:
    assert len(dimensions) == len(members) == len(groups) == 3
    for axis, (presence, rows) in enumerate(groups):
        assert len(presence) == dimensions[(axis + 1) % 3]
        assert len(rows) == sum(flag != 0 for flag in presence)
    return integers(dimensions + members) + b''.join(presence + b''.join(rows) for presence, rows in groups)


def empty_groups(dimensions: list[int]) -> list[GroupWire]:
    return [(bytes(dimensions[(axis + 1) % 3]), []) for axis in range(3)]


def one_row(axis: int, body: bytes, presence: int = 1, members: list[int] | None = None) -> bytes:
    dimensions = [0, 0, 0]
    dimensions[(axis + 1) % 3] = 1
    groups = empty_groups(dimensions)
    groups[axis] = (bytes([presence]), [body] if presence else [])
    return grid(dimensions, members or [0, 0, 0], groups)


def chunks(encoded: list[bytes], declared: int = 0) -> bytes:
    assert len(encoded) > 0
    return integers([0, len(encoded)]) + words([declared], 8) + b''.join(integers([len(part)]) + part for part in encoded)
