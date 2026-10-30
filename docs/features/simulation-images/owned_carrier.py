"""Independent complete CWR templates, reusing only the reviewed CFB fixture primitives."""
import math
import struct
from owned_compound import directory_entry, free_entries, header, table_counts, END, FREE, FAT

SECTOR = 4096
OTHER = b'other-stream-keep\0'


def append_normal(chunks: list[bytes], chains: list[list[int]], payload: bytes) -> int:
    """Assign complete sector chains without borrowing any Bessemer allocator state."""
    if not payload:
        return END
    start = len(chunks)
    chunks.extend(payload[n:n + SECTOR].ljust(SECTOR, b'\0') for n in range(0, len(payload), SECTOR))
    chains.append(list(range(start, len(chunks))))
    return start


def append_mini(payload: bytes, mini: bytearray, pointers: list[int]) -> int:
    """Pack one logical small stream into independently linked 64-byte units."""
    if not payload:
        return END
    first = len(pointers)
    count = math.ceil(len(payload) / 64)
    mini.extend(payload.ljust(count * 64, b'\0'))
    pointers.extend(first + n + 1 if n + 1 < count else END for n in range(count))
    return first


def compound_study(master: bytes, gen: bytes) -> bytes:
    """Construct complete root MAS/GEN/Other streams and a fixed valid sibling tree."""
    chunks, chains = [bytes(SECTOR)], [[0]]
    mini, pointers, starts = bytearray(), [], []
    payloads = [gen, master, OTHER]
    for payload in payloads:
        starts.append(append_mini(payload, mini, pointers) if len(payload) < 4096 else append_normal(chunks, chains, payload))
    root_start = append_normal(chunks, chains, bytes(mini))
    mini_words = pointers + [FREE] * (-len(pointers) % (SECTOR // 4))
    mini_start = append_normal(chunks, chains, struct.pack('<' + 'I' * len(mini_words), *mini_words))
    chunks[0] = directory(starts, payloads, root_start, len(mini))
    return finish(chunks, chains, mini_start)


def directory(starts: list[int], payloads: list[bytes], root_start: int, root_size: int) -> bytes:
    """Pin names and red/black metadata separately from physical allocation."""
    rows = [directory_entry('Root Entry', 5, child=2, start=root_start, size=root_size, color=0),
            directory_entry('GEN', 2, start=starts[0], size=len(payloads[0]), color=0),
            directory_entry('MAS', 2, left=1, right=3, start=starts[1], size=len(payloads[1])),
            directory_entry('Other', 2, start=starts[2], size=len(payloads[2]), color=0)]
    return b''.join(rows) + free_entries(SECTOR // 128 - len(rows))


def finish(chunks: list[bytes], chains: list[list[int]], mini_start: int) -> bytes:
    """Complete the FAT and standard header with exact physical EOF."""
    count, difat = table_counts(len(chunks), SECTOR // 4)
    assert difat == 0, 'owned templates require only inline DIFAT'
    fat_ids = list(range(len(chunks), len(chunks) + count))
    words = [FREE] * (count * (SECTOR // 4))
    for chain in chains:
        for index, sector in enumerate(chain):
            words[sector] = chain[index + 1] if index + 1 < len(chain) else END
    for sector in fat_ids:
        words[sector] = FAT
    encoded = struct.pack('<' + 'I' * len(words), *words)
    chunks.extend(encoded[n:n + SECTOR] for n in range(0, len(encoded), SECTOR))
    return header(4, fat_ids, 0, mini_start, []) + b''.join(chunks)
