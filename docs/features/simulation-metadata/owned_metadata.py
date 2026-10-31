"""Independent GUID-string, scalar and complete-carrier fixtures (#116)."""
import struct
from owned_carrier import append_normal, append_mini, finish, SECTOR
from owned_compound import directory_entry, free_entries, END, FREE

MAX_BYTES = 0x100000
KEEP = bytes(range(251)) * 24
REPLACEMENT = (0x52, 0xd800, 0, 0x5a)


def utf16(units: tuple[int, ...]) -> bytes:
    """Encode raw units without replacing or normalizing surrogate patterns."""
    return b''.join(struct.pack('<H', unit) for unit in units)


def stored_fields(fields: tuple[tuple[int, ...] | None, ...], tail: bytes = b'') -> bytes:
    """Construct bare field frames independently of the actual reader/writer."""
    assert len(fields) == 4
    chunks = []
    for value in fields:
        content = None if value is None else utf16(value) + b'\0\0'
        chunks.append(struct.pack('<I', 0 if content is None else len(content)) + (content or b''))
    return b''.join(chunks) + tail


def guid_cases() -> list[tuple[str, tuple[tuple[int, ...] | None, ...], bytes]]:
    """Retain wire states that the inspected caller's creation gate need not produce."""
    return [
        ('null-empty-ab', (None, (), (65,), (66,)), b''),
        ('all-null', (None, None, None, None), b''),
        ('all-empty', ((), (), (), ()), b''),
        ('unicode-and-surrogates', ((0x4e2d,), (0xd83d, 0xde00), (0xd800,), (0xdc00,)), b''),
        ('stored-nuls', ((65, 0, 66), (0,), None, (67, 0)), b''),
        ('unknown-tail', (None, (), (65,), (66,)), b'\xaa\xbb\xcc'),
        ('inclusive-length-limit', ((88,) * (MAX_BYTES // 2 - 1), None, (65,), (66,)), b''),
    ]


def writer_cases() -> list[tuple[str, tuple[tuple[int, ...] | None, ...]]]:
    """Make both mandatory gates and the post-gate reader-length limit observable."""
    return [
        ('nullable-first', (None, (), (65,), (66,))),
        ('null-first-two', (None, None, (65,), (66,))),
        ('empty-first-two', ((), (), (65,), (66,))),
        ('surrogates', ((0xd800,), (0x4e2d,), (0xdc00,), (0xd83d, 0xde00))),
        ('third-null', ((65,), (66,), None, (67,))),
        ('third-empty', ((65,), (66,), (), (67,))),
        ('fourth-null', ((65,), (66,), (67,), None)),
        ('fourth-empty', ((65,), (66,), (67,), ())),
        ('unused-oversize-first', ((88,) * (MAX_BYTES // 2), None, (), (66,))),
        ('inclusive-created-first', ((88,) * (MAX_BYTES // 2 - 1), None, (65,), (66,))),
    ]


def carrier(guid: bytes | None, stamp: int = 0xdeadbeef, state: int = 0x12345678) -> bytes:
    """Build complete named root streams using the reviewed independent CFB primitives."""
    payloads = [('Keep', KEEP), ('RunState', struct.pack('<I', state)), ('TimeStamp', struct.pack('<I', stamp))]
    if guid is not None:
        payloads.append(('TimeStampGUIDs', guid))
    chunks, chains, mini, pointers = [bytes(SECTOR)], [[0]], bytearray(), []
    starts = [append_mini(raw, mini, pointers) if len(raw) < 4096 else append_normal(chunks, chains, raw) for _, raw in payloads]
    root = append_normal(chunks, chains, bytes(mini))
    mini_words = pointers + [FREE] * (-len(pointers) % (SECTOR // 4))
    mini_fat = append_normal(chunks, chains, struct.pack('<' + 'I' * len(mini_words), *mini_words))
    chunks[0] = directory(payloads, starts, root, len(mini))
    return finish(chunks, chains, mini_fat)


def directory(payloads: list[tuple[str, bytes]], starts: list[int], root: int, root_size: int) -> bytes:
    """Keep a valid length-ordered sibling tree and independent metadata fixed across goldens."""
    rows = [directory_entry('Root Entry', 5, child=2, start=root, size=root_size)]
    links = [(FREE, FREE, 1), (1, 3, 1), (FREE, 4 if len(payloads) == 4 else FREE, 1), (FREE, FREE, 0)]
    for index, (name, raw) in enumerate(payloads):
        left, right, color = links[index]
        rows.append(directory_entry(name, 2, left=left, right=right, start=starts[index], size=len(raw), color=color))
    return b''.join(rows) + free_entries(SECTOR // 128 - len(rows))
