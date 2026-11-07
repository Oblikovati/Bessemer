"""Assemble RGB555 archives and raw words without importing Bessemer or an image codec."""
from pathlib import Path
from itertools import product
import hashlib
import json
import struct

ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / 'tests/fixtures/preview-rgb555'


def word_bytes(words: list[int]) -> bytes:
    return b''.join(struct.pack('<H', word) for word in words)


def packed_words(width: int, height: int, palette_count: int, omitted: bool,
                 words: list[int]) -> bytes:
    rows = [word_bytes(words[start:start + width]) for start in range(0, len(words), width)]
    stored_rows = rows if height < 0 else list(reversed(rows))
    padding = (-width * 2) % 4
    pixels = b''.join(row + bytes((0xA0 + number * 7 + pos) % 256 for pos in range(padding))
                      for number, row in enumerate(stored_rows))
    palette = bytes((3, 4, 5, 0xE1, 0xFA, 0xFB, 0xFC, 0xE2))[:4 * palette_count]
    header = struct.pack('<IiiHHIIiiII', 40, width, height, 1, 16, 0,
                         0 if omitted else len(pixels), -1234, 5678, palette_count, 0xFFFFFFFF)
    body = header + palette + pixels
    return struct.pack('<I', len(body)) + body


LAYOUTS = list(product((1, 2, 3, 4, 5, 31, 32, 33), (-2, -1, 1, 2), (0, 2), (False, True)))
LAYOUTS += list(product((256,), (-256, 256), (0, 2), (False, True)))
assert len(LAYOUTS) == 136
PACKET = bytearray(b'B165' + struct.pack('<I', len(LAYOUTS)))
PIXELS = 0
for width, height, palette_count, omitted in LAYOUTS:
    words = list(range(65536)) if width == 256 else [
        ((column * 7 + row * 3) & 31) | (((column * 11 + row * 5) & 31) << 5)
        | (((column * 13 + row * 17) & 31) << 10) | (((column + row) % 2) << 15)
        for row in range(abs(height)) for column in range(width)]
    changed = [word ^ 0x8421 for word in words]
    original = packed_words(width, height, palette_count, omitted, words)
    edited = packed_words(width, height, palette_count, omitted, changed)
    assert sum((a ^ b).bit_count() for a, b in zip(original, edited)) == len(words) * 4
    for blob in (original, edited, word_bytes(words), word_bytes(changed)):
        PACKET.extend(struct.pack('<I', len(blob)) + blob)
    PIXELS += len(words) * 2
FIXTURES.mkdir(parents=True, exist_ok=True)
(FIXTURES / 'cases.bin').write_bytes(PACKET)
REPORT = {'cases': 136, 'images': 272, 'pixels': PIXELS, 'full_raw_word_domain_cases': 8,
          'raw_word_values': 65536, 'edit_xor': '0x8421', 'packet_bytes': len(PACKET),
          'packet_sha256': hashlib.sha256(PACKET).hexdigest()}
(FIXTURES / 'manifest.json').write_text(json.dumps(REPORT, indent=2) + '\n', encoding='utf-8', newline='\n')
print(json.dumps(REPORT))
