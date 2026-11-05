"""Independent complete inline page-buffer literals (#224)."""
from pathlib import Path
import hashlib
import json
import struct

ROOT = Path(__file__).resolve().parents[3]


def inline_buffer(key_hash: int, size: int, next_page: int) -> bytes:
    """Write all fixed BE metadata without the production codec; e.g. inline_buffer(0, 3, 0)."""
    raw = bytearray([0xa5] * 96)
    struct.pack_into('>HHQ', raw, 0, 20, 12, next_page)
    struct.pack_into('>HH', raw, 12, 0, size)
    struct.pack_into('>IIQHQ', raw, 20, key_hash, 3, 4, 0, 0)
    raw[46:53] = b'k\0\xff\0\xff\x80k'
    return bytes(raw)


def buffer_packet() -> bytes:
    """Pin sixty full-width hash/link/free-size combinations; e.g. buffer_packet()."""
    records = [inline_buffer(key_hash, size, next_page)
        for key_hash in (0, 1, 0x80000000, 0xffffffff)
        for size in (0, 1, 3, 4, 65535) for next_page in (0, 1, 0xffffffffffffffff)]
    return b'HSH1' + struct.pack('<I', len(records)) + b''.join(records)


def verify_buffer_packet() -> None:
    """Verify frozen bytes and the retained reference hash; e.g. python fixture_bytes.py."""
    packet = buffer_packet()
    reference = json.loads(Path(__file__).with_name('fixture-reference.json').read_text(encoding='utf-8'))
    assert packet == (ROOT / 'tests/fixtures/database-hash/cases.bin').read_bytes()
    assert hashlib.sha256(packet).hexdigest() == reference['packet_sha256']


if __name__ == '__main__':
    verify_buffer_packet()
