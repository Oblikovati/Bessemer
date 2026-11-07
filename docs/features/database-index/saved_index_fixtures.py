"""Independent saved-index edits preserve noncanonical words and duplicate pairs (#230)."""
from dataclasses import dataclass
from pathlib import Path
import struct
from file_fixtures import complete_file
from index_reference import read_index


@dataclass(frozen=True)
class SavedIndexMutation:
    kind: int
    ordinal: int
    words: tuple[int, ...]
    span: tuple[int, int]
    expected: bytes


def saved_variants() -> tuple[bytes, ...]:
    """Construct canonical, duplicate, raw-word and empty-map variants; e.g. saved_variants()."""
    raw, _ = complete_file()
    duplicate = bytearray(raw)
    struct.pack_into('>QQ', duplicate, 3 * 4096 + 12, 0, 2)
    extreme = bytearray(duplicate)
    struct.pack_into('>QQQ', extreme, 4096 + 8, (1 << 64) - 1, (1 << 64) - 1, 0)
    struct.pack_into('>QQ', extreme, 4096 + 44, (1 << 64) - 1, 0)
    empty = bytearray(raw)
    struct.pack_into('>I', empty, 4096 + 40, 0)
    struct.pack_into('>I', empty, 3 * 4096 + 8, 0)
    return raw, bytes(duplicate), bytes(extreme), bytes(empty)


def mapping_locations(raw: bytes) -> tuple[int, ...]:
    """Locate persisted pairs independently in chain order; e.g. mapping_locations(raw)."""
    selected = read_index(raw, 4096)
    locations: list[int] = []
    for identity in selected.map_pages:
        start = 44 if identity == 1 else 12
        count = struct.unpack_from('>I', raw, identity * 4096 + start - 4)[0]
        locations.extend(identity * 4096 + start + index * 16 for index in range(count))
    assert len(locations) == len(selected.entries)
    return tuple(locations)


def mutation(raw: bytes, kind: int, ordinal: int, words: tuple[int, ...], start: int) -> SavedIndexMutation:
    """Encode only literal declared word spans; e.g. mutation(raw,0,0,(0,0,1),4104)."""
    encoded = b''.join(struct.pack('>Q', word) for word in words)
    result = bytearray(raw)
    result[start:start + len(encoded)] = encoded
    read_index(bytes(result), 4096)
    return SavedIndexMutation(kind, ordinal, words, (start, start + len(encoded)), bytes(result))


def saved_mutations(raw: bytes) -> tuple[SavedIndexMutation, ...]:
    """Supply changed/no-op headers and every changed/no-op pair; e.g. saved_mutations(raw)."""
    selected = read_index(raw, 4096)
    original = (selected.free_head, selected.split, selected.maximum)
    changes = [mutation(raw, 0, 0, ((1 << 64) - 1, (1 << 64) - 1, 0), 4096 + 8),
               mutation(raw, 0, 0, original, 4096 + 8)]
    for ordinal, start in enumerate(mapping_locations(raw)):
        changes.append(mutation(raw, 1, ordinal, ((1 << 64) - 1, 0), start))
        changes.append(mutation(raw, 1, ordinal, selected.entries[ordinal], start))
    return tuple(changes)


def saved_cases(fixtures: Path) -> tuple[tuple[bytes, tuple[SavedIndexMutation, ...]], ...]:
    """Include all owned variants and three retained native images; e.g. saved_cases(root)."""
    native = tuple((fixtures / f'native-{index}.db').read_bytes() for index in range(3))
    return tuple((raw, saved_mutations(raw)) for raw in saved_variants() + native)
