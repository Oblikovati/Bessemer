"""Assemble BI_RGB32 zero-size DIBs without importing the implementation."""
from pathlib import Path
import hashlib
import json
import struct

ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / "tests/fixtures/preview-zero-size"
FIXTURES.mkdir(parents=True, exist_ok=True)
MANIFEST = []
for width in (1, 3):
    for height in (-2, 2):
        for palette_count in (0, 2):
            name = f"w{width}-{'top' if height < 0 else 'bottom'}-p{palette_count}"
            triples = bytes((0x12, 0x34, 0x56, 0x7F, 0xFF, 0, 0x80, 0xA5))
            raster = (triples * width)[:width * 8]
            rows = [raster[:width * 4], raster[width * 4:]]
            pixels = b"".join(rows if height < 0 else reversed(rows))
            palette = bytes((3, 4, 5, 0xE1, 0xFA, 0xFB, 0xFC, 0xE2))[:palette_count * 4]
            header = struct.pack("<IiiHHIIiiII", 40, width, height, 1, 32, 0, 0,
                                 -1234, 5678, palette_count, 0xFFFFFFFF)
            body = header + palette + pixels
            packed = struct.pack("<I", len(body)) + body
            for suffix, payload in (("bin", packed), ("bgrx", raster)):
                (FIXTURES / f"{name}.{suffix}").write_bytes(payload)
            MANIFEST.append({"name": name, "width": width, "height": height,
                             "palette_count": palette_count,
                             "packed_sha256": hashlib.sha256(packed).hexdigest(),
                             "raster_sha256": hashlib.sha256(raster).hexdigest()})
(FIXTURES / "manifest.json").write_text(json.dumps(MANIFEST, indent=2) + "\n", encoding="utf-8")
assert len(MANIFEST) == 8
assert all((FIXTURES / f"{case['name']}.bin").read_bytes()[24:28] == bytes(4)
           for case in MANIFEST)
