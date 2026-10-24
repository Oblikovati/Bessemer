"""Owned wire vectors with manually specified pixels, independent of the Rust codec."""
from pathlib import Path
import struct

ROOT = Path(__file__).resolve().parents[3] / "tests/fixtures/preview"


def packed(width: int, height: int, bits: int, compression: int, palette: bytes, pixels: bytes, used: int) -> bytes:
    header = struct.pack("<IiiHHIIiiII", 40, width, height, 1, bits, compression,
                         len(pixels), -1234, 5678, used, 0xffffffff)
    body = header + palette + pixels
    return struct.pack("<I", len(body)) + body


def generate() -> None:
    ROOT.mkdir(parents=True, exist_ok=True)
    palette = b"".join(bytes((i, i + 1, i + 2, 0x80 + i)) for i in range(5))
    rle = bytes([2, 1, 0, 3, 2, 3, 4, 0, 0, 0, 0, 2, 1, 1, 3, 2, 0, 0,
                 0, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 1, 0xa5, 0x5a])
    (ROOT / "rle8.bin").write_bytes(packed(5, 4, 8, 1, palette, rle, 5))
    (ROOT / "default-palette.bin").write_bytes(packed(1, 1, 8, 1, bytes(range(256)) * 4, b"\0\1", 0))
    top = bytes(range(16))
    (ROOT / "bgrx-top.bin").write_bytes(packed(2, -2, 32, 0, palette[:4], top, 1))
    (ROOT / "bgrx-bottom.bin").write_bytes(packed(2, 2, 32, 0, palette[:4], top[8:] + top[:8], 1))


if __name__ == "__main__":
    generate()
