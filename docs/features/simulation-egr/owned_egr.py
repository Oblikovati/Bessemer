"""Independent complete conditional EGR images and literal field edits (#124)."""
from dataclasses import dataclass
import struct

SOLVER_NAMES: tuple[tuple[int, str], ...] = (
    (1, 'TRUSS2D'), (8, 'TRUSS3D'), (3, 'BEAM2D'), (4, 'BEAM3D'), (9, 'PIPE'), (29, 'ELBOW'),
    (14, 'BOUND'), (18, 'SPRING'), (21, 'MASS'), (27, 'GENSTIF'), (42, 'PLANE2D'), (11, 'SHELLAX'),
    (43, 'SHELL3T'), (44, 'SHELL4T'), (53, 'SHELL3L'), (54, 'SHELL4L'), (63, 'SHELL3'), (46, 'SOLIDL'),
    (31, 'RLINK'), (34, 'CLINK'), (28, 'RBAR'), (36, 'HLINK'), (37, 'GAP'), (98, 'TETRA4'),
    (45, 'SOLID'), (2, 'TRIANG'), (91, 'SHELL9L'), (93, 'SHELL9'), (90, 'SHELL6T'), (64, 'SHELL4'),
    (92, 'TETRA10'), (99, 'SHELL6L'), (94, 'SHELL6'),
)


@dataclass(frozen=True)
class TableCase:
    name: str
    widths: tuple[int, ...]
    directory: tuple[int, ...]
    free_chain: tuple[int, ...]
    canonical_tags: bool = False


def table_cases() -> list[TableCase]:
    """Exercise allocation partitions, directory alignment and positive-width field variants."""
    standard = (4,) * 32
    return [TableCase('empty-standard', standard, (), (), True),
            TableCase('dense-one', standard, (1,), ()),
            TableCase('dense-permuted', standard, (2, 1), ()),
            TableCase('sparse-deleted', standard, (3, 0, 1, 0, 0), (2,)),
            TableCase('nontrivial-free-order', standard, (4, 0, 0, 0, 0, 0), (3, 1, 2)),
            TableCase('directory-chunk-crossing', standard, (0,) * 33, ()),
            TableCase('only-deleted', standard, (0,) * 4, (2, 1)),
            TableCase('sixteen-fields', (1,) * 16, (1,), ()),
            TableCase('seventeen-fields', (1,) * 17, (1,), ()),
            TableCase('shifted-fields', (3, 8, 1), (1,), ()),
            TableCase('whole-row-field', (128,), (1,), ()),
            TableCase('zero-fields-live-row', (), (1,), ()),
            TableCase('minimal-empty', (), (), ()),
            TableCase('positive-thirty-two', (1,) * 32, (1,), ())]


def padded(size: int) -> int:
    """Use a literal complete 128-byte alignment equation for owned regions."""
    return ((size + 127) // 128) * 128


def descriptor_pairs(case: TableCase) -> list[tuple[int, int]]:
    """Retain noncanonical signed tags independently from the writer's assigned tag one."""
    return [(1 if case.canonical_tags else (100 + index if index % 2 else -100 - index), width)
            for index, width in enumerate(case.widths)]


def physical_row(number: int, logical: int, free_next: int | None, standard: bool) -> bytes:
    """Leave unused/deleted bytes distinctive instead of inventing zero initialization."""
    raw = bytearray((index * 37 + number * 11) % 256 for index in range(128))
    if standard and free_next is None:
        struct.pack_into('<10i', raw, 0, logical, 98 if logical % 2 else 999,
                         -0x80000000, 0x7fffffff, -1, 0, 1, 2, 3, 4)
    if free_next is not None:
        struct.pack_into('<i', raw, 0, free_next)
    return bytes(raw)


def table_image(case: TableCase) -> bytes:
    """Assemble independent header, opaque alignment, logical map and physical partition bytes."""
    active = {number: ordinal for ordinal, number in enumerate(case.directory, 1) if number}
    allocated = len(active) + len(case.free_chain)
    assert sorted((*active, *case.free_chain)) == list(range(1, allocated + 1))
    directory_start = 128 + padded(len(case.widths) * 8)
    rows_start = directory_start + padded(len(case.directory) * 4)
    raw = bytearray(b'\xa5' * (rows_start + allocated * 128))
    write_metadata(raw, case, directory_start, active)
    links = dict(zip(case.free_chain, (*case.free_chain[1:], 0)))
    for number in range(1, allocated + 1):
        start = rows_start + (number - 1) * 128
        raw[start:start + 128] = physical_row(number, active.get(number, 0), links.get(number), case.widths == (4,) * 32)
    return bytes(raw)


def write_metadata(raw: bytearray, case: TableCase, directory_start: int, active: dict[int, int]) -> None:
    """Keep unused header words and descriptor/directory padding raw and nonzero."""
    head = case.free_chain[0] if case.free_chain else 0
    high = max(active.values(), default=0)
    struct.pack_into('<9i', raw, 0, 128, len(case.directory), len(case.widths), -12345,
                     len(active), len(case.free_chain), head, 0x12345678, high)
    for index, pair in enumerate(descriptor_pairs(case)):
        struct.pack_into('<2i', raw, 128 + index * 8, *pair)
    for index, number in enumerate(case.directory):
        struct.pack_into('<i', raw, directory_start + index * 4, number)


def field_edit_golden(raw: bytes, case: TableCase, logical: int, first: int, last: int) -> tuple[bytes, bytes]:
    """Replace a reviewed existing live-field span without deriving offsets from the reader."""
    number = case.directory[logical - 1]
    assert number > 0 and 1 <= first <= last <= len(case.widths)
    rows_start = 128 + padded(len(case.widths) * 8) + padded(len(case.directory) * 4)
    start = rows_start + (number - 1) * 128 + sum(case.widths[:first - 1])
    length = sum(case.widths[first - 1:last])
    replacement = bytes((index * 19 + 0x5a) % 256 for index in range(length))
    return replacement, raw[:start] + replacement + raw[start + length:]


def name_cases() -> list[tuple[int, bytes]]:
    """Exercise every literal name, short destinations, exact byte no-ops and null/unknown states."""
    cases = []
    for code, name in SOLVER_NAMES:
        prefix = name.encode('ascii') + b'\0'
        cases.extend((code, bytes((index * 17 + 3) % 256 for index in range(size)))
                     for size in (len(prefix) - 1, len(prefix), 64, 128))
        cases.append((code, prefix + bytes(range(8))))
    for code in (0, -0x80000000, -1, 7, 999, 0x7fffffff):
        cases.extend((code, bytes(range(size))) for size in (0, 8, 64))
    return cases


def egroup_cases() -> list[tuple[bytes, int, int]]:
    """Cross explicit ordinal/offset contexts with known, null, unknown and mismatched rows."""
    cases = []
    codes = [code for code, _ in SOLVER_NAMES] + [0, -0x80000000, -1, 7, 999, 0x7fffffff]
    for code in codes:
        for logical in (1, 2, 0x7fffffff):
            for offset in (-0x80000000, -1, 0, 0x7fffffff):
                for matched in (False, True):
                    raw = bytearray((index * 37 + 11) % 256 for index in range(128))
                    struct.pack_into('<10i', raw, 0, logical if matched else logical - 1, code,
                                     -0x80000000, 0x7fffffff, -1, 0, 1, 2, 3, 4)
                    cases.append((bytes(raw), logical, offset))
    return cases
