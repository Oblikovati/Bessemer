"""Literal conditional database header buffers, independent of Bessemer (#222)."""
from pathlib import Path
import hashlib
import json
import struct

ROOT = Path(__file__).resolve().parents[3]


def header_buffer(index: int) -> bytes:
    """Construct one complete buffer using explicit BE fields; e.g. header_buffer(0)."""
    packed_time = (0, 0x80000001, 0x5d384126, 0xffffffff)[index % 4]
    sector = 32 << (index % 12)
    page = (512, 513, 4096, 65536)[index % 4]
    name = (b'hash', b'mem\0\xff')[index // 8]
    prefix = b'unqlite' + struct.pack('>IIIIH', 0xdb7c2712, packed_time, sector, page, len(name)) + name
    tail = bytes(((offset * 73) ^ 0xa5) & 255 for offset in range(len(prefix), 512))
    return prefix + tail


def header_packet() -> bytes:
    """Freeze sixteen independent complete buffers; e.g. header_packet()."""
    return b'DBH1' + struct.pack('<I', 16) + b''.join(header_buffer(index) for index in range(16))


def verify_header_packet() -> None:
    """Verify the frozen packet and its hash; e.g. python fixture_bytes.py."""
    packet = header_packet()
    reference = json.loads(Path(__file__).with_name('fixture-reference.json').read_text(encoding='utf-8'))
    assert packet == (ROOT / 'tests/fixtures/database-header/cases.bin').read_bytes()
    assert hashlib.sha256(packet).hexdigest() == reference['packet_sha256']
    assert all(len(header_buffer(index)) == 512 for index in range(16))


if __name__ == '__main__':
    verify_header_packet()
