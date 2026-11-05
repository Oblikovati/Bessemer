"""Independent ZIP32 framing with an explicit expected comment, including Bug232 cases."""
from dataclasses import dataclass
import struct
import zlib


@dataclass(frozen=True)
class ZipEntry:
    name: str
    flags: int
    method: int
    crc: int
    payload: bytes
    extra: bytes
    comment: bytes
    local_offset: int
    local_header: bytes
    stored: bytes
    central_header: bytes


def region(raw: bytes, start: int, count: int) -> bytes:
    """Require complete literal intervals; e.g. region(raw,0,4)."""
    assert 0 <= start <= len(raw) and 0 <= count <= len(raw) - start
    return raw[start:start + count]


def word(raw: bytes, offset: int) -> int:
    """Read a complete little-endian u16; e.g. word(raw,4)."""
    return struct.unpack('<H', region(raw, offset, 2))[0]


def dword(raw: bytes, offset: int) -> int:
    """Read a complete little-endian u32; e.g. dword(raw,16)."""
    return struct.unpack('<I', region(raw, offset, 4))[0]


def inflate(stored: bytes, method: int, expected: int) -> bytes:
    """Check complete raw-deflate termination or stored bytes; e.g. inflate(stored,8,100)."""
    assert expected <= 16_000_000
    if method == 0:
        assert len(stored) == expected
        return stored
    assert method == 8
    decoder = zlib.decompressobj(-15)
    payload = decoder.decompress(stored, expected + 1)
    assert len(payload) == expected and decoder.eof
    assert decoder.unused_data == decoder.unconsumed_tail == b''
    return payload


def central_record(raw: bytes, position: int) -> bytes:
    """Read the complete central record without signature scanning; e.g. central_record(raw,100)."""
    assert region(raw, position, 4) == b'PK\x01\x02'
    size = 46 + sum(word(raw, position + offset) for offset in (28, 30, 32))
    return region(raw, position, size)


def local_payload(raw: bytes, central: bytes) -> tuple[int, bytes, bytes]:
    """Cross-check central/local framing and payload words; e.g. local_payload(raw,central)."""
    position = dword(central, 42)
    assert region(raw, position, 4) == b'PK\x03\x04'
    size = 30 + word(raw, position + 26) + word(raw, position + 28)
    header = region(raw, position, size)
    assert header[6:10] == central[8:12]
    assert header[14:26] == central[16:28]
    assert header[30:30 + word(header, 26)] == central[46:46 + word(central, 28)]
    stored = region(raw, position + size, dword(central, 20))
    return position, header, stored


def entry(raw: bytes, position: int) -> ZipEntry:
    """Decode one complete member with independent CRC checks; e.g. entry(raw,100)."""
    central = central_record(raw, position)
    local_at, local, stored = local_payload(raw, central)
    flags, method = word(central, 8), word(central, 10)
    assert flags & ~0x800 == 0
    name_end = 46 + word(central, 28)
    extra_end = name_end + word(central, 30)
    name = central[46:name_end].decode('utf-8' if flags & 0x800 else 'cp437')
    payload = inflate(stored, method, dword(central, 24))
    crc = dword(central, 16)
    assert zlib.crc32(payload) == crc
    return ZipEntry(name, flags, method, crc, payload, central[name_end:extra_end],
                    central[extra_end:], local_at, local, stored, central)


def end_record(raw: bytes, comment: bytes, count: int) -> tuple[int, int]:
    """Bind the actual end using independently known comment bytes; e.g. end_record(raw,b'',1)."""
    end = len(raw) - 22 - len(comment)
    assert region(raw, end, 4) == b'PK\x05\x06'
    assert region(raw, end + 22, len(comment)) == comment
    assert word(raw, end + 20) == len(comment)
    assert raw[end + 4:end + 8] == bytes(4)
    assert word(raw, end + 8) == word(raw, end + 10) == count
    start = dword(raw, end + 16)
    assert start + dword(raw, end + 12) == end
    return start, end


def inspect_zip(raw: bytes, comment: bytes, count: int) -> tuple[ZipEntry, ...]:
    """Require a complete physical partition and every member integrity check; e.g. inspect_zip(raw,b'',1)."""
    start, end = end_record(raw, comment, count)
    entries: list[ZipEntry] = []
    cursor = start
    for _ in range(count):
        selected = entry(raw, cursor)
        entries.append(selected)
        cursor += len(selected.central_header)
    assert cursor == end
    cursor = 0
    for selected in sorted(entries, key=lambda member: member.local_offset):
        assert selected.local_offset == cursor
        cursor += len(selected.local_header) + len(selected.stored)
    assert cursor == start
    return tuple(entries)


def expected_headers(before: ZipEntry, after: ZipEntry, changed: bool) -> tuple[bytes, bytes]:
    """Allow only required payload words and relocated offsets; e.g. expected_headers(before,after,True)."""
    local = bytearray(before.local_header)
    central = bytearray(before.central_header)
    struct.pack_into('<I', central, 42, after.local_offset)
    if changed:
        words = (after.crc, len(after.stored), len(after.payload))
        struct.pack_into('<III', local, 14, *words)
        struct.pack_into('<III', central, 16, *words)
    return bytes(local), bytes(central)

def check_replacement(before: tuple[ZipEntry, ...], after: tuple[ZipEntry, ...], ordinal: int) -> None:
    """Check all raw headers, all payloads and unchanged compressed bytes; e.g. check_replacement(before,after,0)."""
    assert len(before) == len(after)
    for index, (original, result) in enumerate(zip(before, after)):
        changed = index == ordinal
        payload = original.payload + b'\xa5' if changed else original.payload
        assert result.payload == payload
        assert (result.name, result.flags, result.method, result.extra, result.comment) == (
                original.name, original.flags, original.method, original.extra, original.comment)
        assert (result.local_header, result.central_header) == expected_headers(original, result, changed)
        if not changed:
            assert result.stored == original.stored
