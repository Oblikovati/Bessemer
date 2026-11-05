"""Literal BI_RGB24 oracles; no Rust, Bessemer or image-codec imports."""
from pathlib import Path
from itertools import product
import hashlib
import json
import struct

ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / 'tests/fixtures/preview-bgr24'
FIXTURES.mkdir(parents=True, exist_ok=True)
MANIFEST = []
RUST = ['pub const BGR24_CASES: &[(&[u8], &[u8], &[u8])] = &[']
for width, height, palette_count, omitted in product(range(1, 5), (-2, -1, 1, 2), (0, 2), (0, 1)):
    name = f'w{width}-{"top" if height < 0 else "bottom"}-p{palette_count}-z{omitted}'
    name += '-h1' if abs(height) == 1 else ''
    padding = (-3 * width) % 4
    stride = 3 * width + padding
    raster = bytes(((0, 255, 0x80, 0x12, 0x34, 0x56)[index % 6] ^ (index // 6))
                   for index in range(width * 3 * abs(height)))
    palette = bytes((3, 4, 5, 0xE1, 0xFA, 0xFB, 0xFC, 0xE2))[:4 * palette_count]
    header = struct.pack('<IiiHHIIiiII', 40, width, height, 1, 24, 0,
                         0 if omitted else abs(height) * stride, -1234, 5678, palette_count, 0xFFFFFFFF)
    for edited in (False, True):
        suffix = '-edited' if edited else ''
        expected = bytes(value ^ (0x80 if edited else 0) for value in raster)
        rows = [expected[start:start + width * 3] for start in range(0, len(expected), width * 3)]
        stored_rows = rows if height < 0 else list(reversed(rows))
        pixels = b''.join(row + bytes(0xA0 + index * 7 + pad for pad in range(padding))
                          for index, row in enumerate(stored_rows))
        body = header + palette + pixels
        packed = struct.pack('<I', len(body)) + body
        (FIXTURES / f'{name}{suffix}.bin').write_bytes(packed)
        (FIXTURES / f'{name}{suffix}.bgr').write_bytes(expected)
        MANIFEST.append({'name': name + suffix, 'width': width, 'height': height,
                         'palette_count': palette_count, 'padding': padding,
                         'omitted_size': bool(omitted), 'edited': edited,
                         'packed_sha256': hashlib.sha256(packed).hexdigest(),
                         'raster_sha256': hashlib.sha256(expected).hexdigest()})
    RUST.extend(['    (', f'        include_bytes!("../fixtures/preview-bgr24/{name}.bin"),',
                 f'        include_bytes!("../fixtures/preview-bgr24/{name}-edited.bin"),',
                 f'        include_bytes!("../fixtures/preview-bgr24/{name}.bgr"),', '    ),'])
RUST.append('];')
(ROOT / 'tests/support/bgr24_cases.rs').write_text('\n'.join(RUST) + '\n', encoding='utf-8', newline='\n')
(FIXTURES / 'manifest.json').write_text('[\n' + ',\n'.join(json.dumps(case, sort_keys=True) for case in MANIFEST) + '\n]\n', encoding='utf-8', newline='\n')
assert len(MANIFEST) == 128
assert {case['padding'] for case in MANIFEST} == {0, 1, 2, 3}
