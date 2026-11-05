"""Independent packed BI_RGB indices and exact edited archives; standard library only."""
from pathlib import Path
from itertools import product
import hashlib
import json
import struct

ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / 'tests/fixtures/preview-indexed-rgb'
WIDTHS = (1, 2, 3, 4, 5, 6, 7, 8, 9, 15, 16, 17, 31, 32, 33, 255, 256, 257)


def stored_pixels(indices: bytes, width: int, height: int, bits: int) -> bytes:
    stride = ((width * bits + 31) // 32) * 4
    output = bytearray((offset * 37 + 0xA5) % 256 for offset in range(stride * abs(height)))
    for position, value in enumerate(indices):
        y, x = divmod(position, width)
        stored_y = abs(height) - 1 - y if height > 0 else y
        offset = stored_y * stride + (x * bits) // 8
        shift = 8 - bits - (x * bits) % 8
        mask = ((1 << bits) - 1) << shift
        output[offset] = (output[offset] & (255 ^ mask)) | (value << shift)
    return bytes(output)


def packed_image(width: int, height: int, bits: int, explicit: bool, omitted: bool,
                 indices: bytes) -> bytes:
    palette_count = 2 if explicit else 1 << bits
    palette = bytes(value for index in range(palette_count)
                    for value in (index, 255 - index, index ^ 0x5A, (0x80 + index) % 256))
    pixels = stored_pixels(indices, width, height, bits)
    header = struct.pack('<IiiHHIIiiII', 40, width, height, 1, bits, 0,
                         0 if omitted else len(pixels), -1234, 5678,
                         palette_count if explicit else 0, 0xFFFFFFFF)
    body = header + palette + pixels
    return struct.pack('<I', len(body)) + body


PACKET = bytearray(b'BIX1' + struct.pack('<I', 864))
PIXELS = 0
for bits, width, height, explicit, omitted in product((1, 4, 8), WIDTHS, (-2, -1, 1, 2), (False, True), (False, True)):
    palette_count = 2 if explicit else 1 << bits
    indices = bytes((column * 5 + row * 3) % palette_count
                    for row in range(abs(height)) for column in range(width))
    edited_indices = bytes(value ^ 1 for value in indices)
    original = packed_image(width, height, bits, explicit, omitted, indices)
    edited = packed_image(width, height, bits, explicit, omitted, edited_indices)
    assert len(original) == len(edited)
    assert sum((a ^ b).bit_count() for a, b in zip(original, edited)) == len(indices)
    for blob in (original, edited, indices, edited_indices):
        PACKET.extend(struct.pack('<I', len(blob)) + blob)
    PIXELS += len(indices) * 2
FIXTURES.mkdir(parents=True, exist_ok=True)
(FIXTURES / 'cases.bin').write_bytes(PACKET)
MINIMAL = {}
for bits in (1, 4, 8):
    header = struct.pack('<IiiHHIIiiII', 40, 1, -1, 1, bits, 0, 4, -1234, 5678, 1, 0xFFFFFFFF)
    body = header + bytes((0, 255, 0x5A, 0x80, (1 << (8 - bits)) - 1, 0xA1, 0xB2, 0xC3))
    packed = struct.pack('<I', len(body)) + body
    name = f'one-entry-{bits}.bin'
    (FIXTURES / name).write_bytes(packed)
    MINIMAL[name] = hashlib.sha256(packed).hexdigest()

assert all({(-((width * bits + 7) // 8)) % 4 for width in WIDTHS} == {0, 1, 2, 3} for bits in (1, 4, 8))
REPORT = {'cases': 864, 'images': 1728, 'pixels': PIXELS, 'widths': WIDTHS,
          'bit_counts': [1, 4, 8], 'heights': [-2, -1, 1, 2],
          'explicit_palette_entries': 2, 'default_palette_entries': [2, 16, 256],
          'image_size_forms': ['exact', 'zero'], 'packet_bytes': len(PACKET),
          'packet_sha256': hashlib.sha256(PACKET).hexdigest(), 'one_entry_sha256': MINIMAL}
(FIXTURES / 'manifest.json').write_text(json.dumps(REPORT, indent=2) + '\n', encoding='utf-8', newline='\n')
print(json.dumps(REPORT))
