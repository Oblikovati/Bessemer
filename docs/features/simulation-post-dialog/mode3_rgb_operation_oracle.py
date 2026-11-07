"""Independent C893 whole local group selection and fixed array-copy oracle (#166)."""
import struct
from typing import TypeVar

T = TypeVar('T')

class Mode3OracleDependency(Exception):
    def __init__(self, status: int) -> None:
        super().__init__(f'unknown/invalid local fact; expected admitted dependency for status{status}')
        self.status = status


def mode3_oracle_known(value: T | None, status: int) -> T:
    """Keep unknown facts distinct from zero/empty values; e.g. mode3_oracle_known(0, 10)."""
    if value is None: raise Mode3OracleDependency(status)
    return value


def mode3_oracle_word(words: list[int], offset: int) -> int | None:
    """Decode signed32 through independent packed bytes; e.g. mode3_oracle_word(words, 0)."""
    if words[offset] == 0: return None
    return struct.unpack('<i', struct.pack('<I', words[offset + 1]))[0]


def mode3_oracle_color(words: list[int], offset: int) -> list[int] | None:
    """Retain complete four-word color presence without float conversion; e.g. mode3_oracle_color(words, 2)."""
    return words[offset + 1:offset + 5] if words[offset] else None


def mode3_oracle_selection(words: list[int], mode: int, candidate: list[int]) -> tuple[list[int] | None, list[int] | None, list[int], int]:
    """Preserve both unconditional group copies and mode-dependent face dependency order; e.g. mode3_oracle_selection(words, 3, candidate)."""
    face = mode3_oracle_word(words, 14)
    if mode == 3:
        face = mode3_oracle_known(face, 16)
        if face != 0x405: return None, None, candidate, 1
    first = mode3_oracle_known(mode3_oracle_color(words, 16), 14)
    other = mode3_oracle_known(mode3_oracle_color(words, 21), 15)
    face = mode3_oracle_known(face, 16)
    return first, other, first[:3] if face == 0x404 else other[:3], 2 if face == 0x404 else 3


def mode3_oracle_group(words: list[int]) -> list[int]:
    """Project candidate, copied parents, selection and both pass filters independently; e.g. mode3_oracle_group(words)."""
    mode = mode3_oracle_known(mode3_oracle_word(words, 0), 10)
    stored = mode3_oracle_known(mode3_oracle_color(words, 2), 11)
    candidate = stored[:3] if mode == 3 else [0x3f000000] * 3
    prefix = [0, *candidate, *stored, *([0] * 14)]
    if mode3_oracle_known(mode3_oracle_word(words, 10), 12) != 0: return prefix
    prefix[0] = 1
    if mode3_oracle_known(mode3_oracle_word(words, 12), 13) == 0: return prefix
    first, other, selected, origin = mode3_oracle_selection(words, mode, candidate)
    prefix = [3, *candidate, *stored, int(first is not None), *(first or [0] * 4), *(other or [0] * 4), 1, *selected, origin]
    if words[9] == 1: return prefix
    plot = mode3_oracle_known(mode3_oracle_word(words, 7), 17)
    if plot != 54: return prefix
    selector = mode3_oracle_known(mode3_oracle_word(words, 26), 18)
    if ((selector - 1) & 0xffffffff) in (0, 1): prefix[0] = 2
    return prefix


def mode3_oracle_cells(words: list[int], offset: int, missing_status: int) -> list[list[int]]:
    """Require an explicit observed destination prefix; e.g. mode3_oracle_cells(words, 30, 31)."""
    if words[offset] == 0: raise Mode3OracleDependency(missing_status)
    length, seed, budget = words[offset + 1:offset + 4]
    assert length <= budget <= 8
    return [[(seed ^ ((index + 1) * 0x9e3779b9) ^ (channel * 0x85ebca6b)) & 0xffffffff for channel in range(3)] for index in range(length)]


def mode3_oracle_array(words: list[int], selected: list[int]) -> list[int]:
    """Keep signed capacity, growth acceptance and bounded copy distinct; e.g. mode3_oracle_array(words, [0, 0, 0])."""
    capacity = mode3_oracle_known(mode3_oracle_word(words, 28), 30)
    if capacity > 5:
        parent, cells = 1, mode3_oracle_cells(words, 30, 31)
    else:
        growth = mode3_oracle_known(mode3_oracle_word(words, 34), 32)
        if growth != 0: raise Mode3OracleDependency(33)
        parent, cells = 2, mode3_oracle_cells(words, 36, 34)
    if len(cells) < 6: raise Mode3OracleDependency(35)
    before = cells[5][:]
    cells[5] = selected[:]
    flattened = [word for triple in cells for word in triple]
    return [parent, len(cells), *flattened, *([0] * (24 - len(flattened))), *before, *selected]


def mode3_operation_oracle(words: list[int]) -> bytes:
    """Return exact staged results while preserving typed error distinctions; e.g. mode3_operation_oracle(words)."""
    assert len(words) == 40
    try:
        prefix = mode3_oracle_group(words)
    except Mode3OracleDependency as error:
        return struct.pack('<54I', error.status, *([0] * 53))
    assert len(prefix) == 22
    suffix = [0] * 32
    if prefix[0] == 3:
        try: suffix = mode3_oracle_array(words, prefix[18:21])
        except Mode3OracleDependency as error: suffix = [error.status, *([0] * 31)]
    assert len(suffix) == 32
    return struct.pack('<54I', *prefix, *suffix)
