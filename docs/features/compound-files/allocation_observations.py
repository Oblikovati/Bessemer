"""Measure strict-allocation prerequisites separately from the permissive Source reader."""
import math
import struct
from collections import Counter

FREE, END, FAT, DIF = 0xffffffff, 0xfffffffe, 0xfffffffd, 0xfffffffc


def allocation_tables(document: object) -> tuple[list[int], list[int]]:
    """Recover table-sector ownership without replacing actual Source parsing."""
    fat = list(struct.unpack_from('<109I', document.data, 76))
    difat, sector = [], document.difat_start
    for _ in range(document.n_difat):
        difat.append(sector)
        values = struct.unpack_from('<' + 'I' * (document.ssize // 4), document.data, document._off(sector))
        fat.extend(values[:-1])
        sector = values[-1]
    return fat[:document.n_fat], difat


def stream_length(document: object, entry: object) -> int:
    """Apply MS-CFB's required legacy high-DWORD compatibility rule explicitly."""
    return entry.size & 0xffffffff if document.major == 3 else entry.size


def claimed_chains(document: object) -> tuple[list[int], list[int], int, list[int]]:
    """Count claims and mismatched logical extents; do not infer validity from readable bytes."""
    normal, mini, mismatch, empty = [], [], 0, []
    for entry in document.entries:
        if entry.type not in (2, 5):
            continue
        size = stream_length(document, entry)
        if not size:
            empty.append(entry.start)
            continue
        small = entry.type == 2 and size < document.cutoff
        chain = document._chain(entry.start, document.minifat if small else document.fat)
        mismatch += len(chain) != math.ceil(size / (document.mssize if small else document.ssize))
        (mini if small else normal).extend(chain)
    return normal, mini, mismatch, empty


def observations(document: object) -> dict[str, object]:
    """Keep measured prerequisites distinct from a complete CFB validator."""
    normal, mini, mismatch, empty = claimed_chains(document)
    fat, difat = allocation_tables(document)
    directory = document._chain(document.dir_start, document.fat)
    mini_fat = document._chain(document.minifat_start, document.fat)
    claims = normal + fat + difat + directory + mini_fat
    sectors = len(document.data) // document.ssize - 1
    claimed = set(claims)
    unowned = [n for n, value in enumerate(document.fat[:sectors]) if value != FREE and n not in claimed]
    return dict(normal_duplicate_claims=len(claims) - len(set(claims)), mini_duplicate_claims=len(mini) - len(set(mini)),
                stream_extent_mismatches=mismatch, unowned_allocated_sectors=len(unowned),
                empty_start_values={str(key): count for key, count in Counter(empty).items()},
                directory_chain_sectors=len(directory), mini_fat_chain_sectors=len(mini_fat),
                legacy_high_size_words=sum(entry.type in (2, 5) and entry.size >> 32 != 0 for entry in document.entries) if document.major == 3 else 0)
