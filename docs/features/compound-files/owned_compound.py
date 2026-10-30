"""Independent complete MS-CFB fixtures; no Bessemer allocator is imported (#106)."""
import math
import struct
from collections.abc import Callable
from dataclasses import dataclass

FREE = 0xffffffff
END = 0xfffffffe
FAT = 0xfffffffd
DIF = 0xfffffffc


@dataclass(frozen=True)
class OwnedCapacity:
    normal_extra: int = 0
    mini_count: int = 1
    root_extra: int = 0


def put_word(raw: bytearray, offset: int, value: int) -> None:
    """Keep every persisted little-endian field independently inspectable."""
    struct.pack_into('<I', raw, offset, value)


def directory_entry(name: str, kind: int, *, left: int = FREE, right: int = FREE,
                    child: int = FREE, start: int = END, size: int = 0, color: int = 1) -> bytes:
    """Construct reviewed fixed directory records with explicit tree and allocation links."""
    raw = bytearray(128)
    encoded = name.encode('utf-16-le') + b'\0\0'
    raw[:len(encoded)] = encoded
    struct.pack_into('<HBBIII', raw, 64, len(encoded), kind, color, left, right, child)
    struct.pack_into('<IQ', raw, 116, start, size)
    if kind == 1:
        raw[80:96] = bytes(range(16))
        struct.pack_into('<IQQ', raw, 96, 0x12345678, 0x1122334455667788, 0x8877665544332211)
    return bytes(raw)


def free_entries(count: int) -> bytes:
    """Use the normative unused-entry pointer values rather than relying on zero padding."""
    raw = bytearray(128)
    struct.pack_into('<III', raw, 68, FREE, FREE, FREE)
    return bytes(raw) * count


def header(major: int, fat: list[int], directory: int, mini_fat: int, difat: list[int], directory_count: int = 1) -> bytes:
    """Build the standard header, including the version-4 header-sector zero padding."""
    raw = bytearray(512 if major == 3 else 4096)
    raw[:8] = bytes.fromhex('d0cf11e0a1b11ae1')
    struct.pack_into('<5H', raw, 24, 62, major, 0xfffe, 9 if major == 3 else 12, 6)
    struct.pack_into('<9I', raw, 40, 0 if major == 3 else directory_count, len(fat), directory, 0,
                     4096, mini_fat, int(mini_fat != END), difat[0] if difat else END, len(difat))
    inline = fat[:109] + [FREE] * max(0, 109 - len(fat))
    struct.pack_into('<109I', raw, 76, *inline)
    return bytes(raw)


def table_counts(content_sectors: int, words: int) -> tuple[int, int]:
    """Solve allocation-table self-coverage independently of stream byte layout."""
    fat, difat = 1, 0
    while True:
        new_fat = math.ceil((content_sectors + fat + difat) / words)
        new_difat = math.ceil(max(new_fat - 109, 0) / (words - 1))
        if (new_fat, new_difat) == (fat, difat):
            return fat, difat
        fat, difat = new_fat, new_difat


def minimal(major: int, directory_first: bool = False) -> bytes:
    """Pin the complete three-sector empty-file closed form from MS-CFB 2.9."""
    sector_bytes = 512 if major == 3 else 4096
    leading = (END, FAT) if directory_first else (FAT, END)
    table = struct.pack('<' + 'I' * (sector_bytes // 4), *leading, *([FREE] * (sector_bytes // 4 - 2)))
    directory = directory_entry('Root Entry', 5) + free_entries(sector_bytes // 128 - 1)
    if directory_first:
        return header(major, [1], 0, END, []) + directory + table
    return header(major, [0], 1, END, []) + table + directory


def complete(major: int, payload_size: int, fragmented: bool, zero_slack: bool = False, capacity: OwnedCapacity = OwnedCapacity()) -> tuple[bytes, dict[str, bytes]]:
    """Exercise nested storage, mini/normal/empty streams and extended DIFAT."""
    size = 512 if major == 3 else 4096
    payload = (bytes(range(256)) * math.ceil(payload_size / 256))[:payload_size]
    mini = b'mini\0payload\xff' * 3
    chains = content_chains(size, len(payload), capacity)
    directories = len(chains[3])
    blocks = sum(map(len, chains))
    fats, difats = table_counts(blocks, size // 4)
    total = blocks + fats + difats
    physical: Callable[[int], int] = lambda index: total - 1 - index if fragmented else index
    fat_ids = [physical(n) for n in range(blocks, blocks + fats)]
    dif_ids = [physical(n) for n in range(blocks + fats, total)]
    chunks = [b'\0' * size] * total
    layout_content(chunks, physical, chains, payload, mini, size, zero_slack, capacity.mini_count)
    layout_tables(chunks, physical, chains, fat_ids, dif_ids, size)
    prefix = header(major, fat_ids, physical(0), physical(blocks - 1), dif_ids, directories)
    return prefix + b''.join(chunks), {'S/A': mini, 'S/B': payload, 'S/C': b''}


def layout_content(chunks: list[bytes], physical: Callable[[int], int], chains: list[list[int]], payload: bytes, mini: bytes, size: int, zero_slack: bool, mini_count: int) -> None:
    """Bind fixed directory links to an independently permuted physical layout."""
    rows = [directory_entry('Root Entry', 5, child=1, start=physical(chains[1][0]), size=mini_count * 64),
            directory_entry('S', 1, child=3), directory_entry('A', 2, start=0, size=len(mini), color=0),
            directory_entry('B', 2, left=2, right=4, start=physical(chains[0][0]), size=len(payload)),
            directory_entry('C', 2, size=0, color=0)]
    directory = b''.join(rows) + free_entries(len(chains[3]) * size // 128 - len(rows))
    for ordinal, sector in enumerate(chains[3]):
        chunks[physical(sector)] = directory[ordinal * size:(ordinal + 1) * size]
    for position, logical in enumerate(chains[0]):
        chunks[physical(logical)] = payload[position * size:(position + 1) * size].ljust(size, b'\0' if zero_slack else b'\xa7')
    root = mini.ljust(len(chains[1]) * size, b'\0' if zero_slack else b'\xb8')
    for ordinal, sector in enumerate(chains[1]):
        chunks[physical(sector)] = root[ordinal * size:(ordinal + 1) * size]
    chunks[physical(chains[2][0])] = mini_table(size, mini_count)


def layout_tables(chunks: list[bytes], physical: Callable[[int], int], chains: list[list[int]], fat_ids: list[int], dif_ids: list[int], size: int) -> None:
    """Mark every physical allocation and chain link before writing table sectors."""
    words = size // 4
    entries = [FREE] * (len(fat_ids) * words)
    for chain in chains:
        for ordinal, logical in enumerate(chain):
            entries[physical(logical)] = physical(chain[ordinal + 1]) if ordinal + 1 < len(chain) else END
    for sector, marker in [(sector, FAT) for sector in fat_ids] + [(sector, DIF) for sector in dif_ids]:
        entries[sector] = marker
    for ordinal, sector in enumerate(fat_ids):
        chunks[sector] = struct.pack('<' + 'I' * words, *entries[ordinal * words:(ordinal + 1) * words])
    for ordinal, sector in enumerate(dif_ids):
        selected = fat_ids[109 + ordinal * (words - 1):109 + (ordinal + 1) * (words - 1)]
        following = dif_ids[ordinal + 1] if ordinal + 1 < len(dif_ids) else END
        chunks[sector] = struct.pack('<' + 'I' * words, *(selected + [FREE] * (words - 1 - len(selected)) + [following]))


def owned_cases() -> list[tuple[str, bytes, dict[str, bytes]]]:
    """Keep both sector sizes, reversed physical chains and an actual extended DIFAT."""
    cases = [('empty-v3', minimal(3), {}), ('empty-v4', minimal(4), {})]
    for major, count, fragmented in [(3, 7001, False), (4, 8193, True), (3, 7300123, True)]:
        raw, streams = complete(major, count, fragmented)
        cases.append((f'v{major}-{count}-fragmented-{fragmented}', raw, streams))
    raw, streams = complete(3, 7001, False)
    legacy = bytearray(raw)
    put_word(legacy, 512 + 124, 0xfedcba98)
    put_word(legacy, 512 + 3 * 128 + 124, 0x76543210)
    cases.append(('legacy-v3-uninitialized-high-size', bytes(legacy), streams))
    colored = bytearray(raw)
    colored[512 + 67] = 0
    cases.append(('root-color-red-107', bytes(colored), streams))
    cases.extend(capacity_cases())
    return cases


def canonical_case(raw: bytes, streams: dict[str, bytes]) -> bytes:
    """Build independent complete expected output; no input allocation tables are reused."""
    major = struct.unpack_from('<H', raw, 26)[0]
    canonical = bytearray(complete(major, len(streams['S/B']), False, True)[0] if streams else minimal(major, True))
    sector_bytes = 512 if major == 3 else 4096
    directory = struct.unpack_from('<I', raw, 48)[0]
    canonical[sector_bytes + 67] = raw[(directory + 1) * sector_bytes + 67]
    return bytes(canonical)


def content_chains(size: int, payload_size: int, capacity: OwnedCapacity) -> list[list[int]]:
    """Allocate extra normal/root capacity independently of the declared logical sizes."""
    directories = math.ceil(5 * 128 / size)
    normal_end = directories + math.ceil(payload_size / size) + capacity.normal_extra
    root_end = normal_end + math.ceil(capacity.mini_count * 64 / size) + capacity.root_extra
    return [list(range(directories, normal_end)), list(range(normal_end, root_end)),
            [root_end], list(range(directories))]


def mini_table(size: int, count: int) -> bytes:
    """Bind every surplus mini sector to its stream, including capacity beyond the logical size."""
    selected = [n + 1 if n + 1 < count else END for n in range(count)]
    return struct.pack('<' + 'I' * (size // 4), *(selected + [FREE] * (size // 4 - count)))


def capacity_cases() -> list[tuple[str, bytes, dict[str, bytes]]]:
    """Pin legal nonminimal normal, mini and root chains for #109."""
    cases = []
    for label, capacity in [('normal', OwnedCapacity(normal_extra=1)),
                            ('mini', OwnedCapacity(mini_count=2)),
                            ('root', OwnedCapacity(root_extra=1))]:
        raw, streams = complete(3, 7001, False, capacity=capacity)
        cases.append(('preallocated-' + label, raw, streams))
    return cases
