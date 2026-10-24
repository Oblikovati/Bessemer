"""Owned clause-12 vectors assembled from wire primitives, independently of Rust."""
from pathlib import Path
import struct

ROOT = Path(__file__).resolve().parents[3] / "tests/fixtures/tessellation"


def words(*values: int) -> bytes:
    return b"".join(struct.pack("<I", value & 0xffffffff) for value in values)


def doubles(count: int) -> bytes:
    return b"".join(struct.pack("<Q", 0x7ff8000000000042 + i) for i in range(count))


def array(width: int = 4, count: int = 1, implied: bool = False) -> bytes:
    prefix = b"" if implied else words(width)
    return prefix + words(17, 23, count) + bytes(range(width)) * count


def string(text: str) -> bytes:
    encoded = text.encode("utf-16le")
    return b"\xff\xfe\xff" + bytes([len(encoded) // 2]) + encoded


def texture(version: int) -> bytes:
    result = words(-11, 2) + array() * 2 + words(3) + array() * 3
    return result + (words(-7) + doubles(9) if version > 0x971 else b"")


def edges(version: int) -> bytes:
    first = words(31, 32, -1, 4) + doubles(4) + words(33, 2, 34, 35, -1)
    second = (b"" if version == 0x3f0f else words(36))
    second += words(37, -1, 0, 38, 0, 0)
    return words(2) + first + second


def header_fields(version: int, mode: int) -> bytes:
    result = words(1, 1)
    if version <= 0x3ee7:
        result += array()
    elif mode == 4:
        result += words(mode) + array(implied=True)
    elif mode == 0:
        result += words(mode)
    else:
        result += words(mode, 0x11223344, 0x55667788, -1, -2, -3)
        result += words(2, -1, -2, 2, 0x7fc00042, 0x80000000) + array()
    return result


def early_fields(version: int, enabled: bool) -> bytes:
    result = array(12, 3) + words(0)
    if version >= 0x3de:
        result += array() + words(0) + (array() if version > 0x533 else b"")
    if 0x4e5 < version < 0xa71:
        result += array() * 2
    if version <= 0x330:
        result += array(24 if version >= 0x2cd else 4)
    else:
        result += words(-7, 2, 0x7fc00042, 0x80000000) if enabled else words(0)
    result += words(-2, 9) + doubles(10)
    result += doubles(1) if version < 0x116a else b""
    result += words(-23) if 0x49d < version < 0x7dc else b""
    result += words(-2) if version > 0x4e5 else b""
    result += array() if 0x4fb < version < 0xc6e else b""
    return result


def texture_fields(version: int, enabled: bool) -> bytes:
    result = words(-3) if version > 0x689 else b""
    result += words(0xdeadbeef) if version >= 0x82a else b""
    result += words(-1, 2, 0x7fc00042, 1, 2, 3) if version >= 0x83b else b""
    if version > 0x900:
        result += words(7) + array() + words(-9) + array() * 2 if enabled else words(0, 0)
    if version > 0x1098:
        result += words(-5) + array() * 2 if enabled else words(0)
    if version > 0x966:
        result += words(3) + texture(version) if enabled else words(0)
    return result


def surface_fields(version: int, enabled: bool) -> bytes:
    result = b""
    if version > 0x1c3a:
        result += words(-1 if enabled else 0)
    if version > 0x1c23 and (version < 0x1c3b or enabled):
        result += array()
    if version > 0x2791:
        result += words(-8) + (words(2, -1, -2) if enabled else b"")
    result += words(0xf1234567) if version > 0x279d else b""
    result += doubles(3) if version > 0x279f else b""
    if version > 0x27ad:
        result += words(-41) + doubles(8) + words(2, -1, -2, -3, -4) + doubles(1)
    result += words(-3) if version > 0x27b5 else b""
    result += words(0xfedcba98) if version > 0x27e0 else b""
    return result


def final_fields(version: int, lightweight: bool, enabled: bool) -> bytes:
    result = b""
    if version > 0x2bcf:
        result += words(0) + string("λ") if enabled else words(-3)
    if version > 0x3751:
        result += b"\xfe" + array() if enabled else b"\0"
    result += b"\xff" if version > 0x3757 else b""
    if version > 0x2bd0 and not lightweight:
        result += struct.pack("<q", 1) + words(-51) + doubles(3)
    result += edges(version) if version > 0x3b28 else b""
    if version > 0x3b0d:
        result += (b"" if version == 0x3f0f else words(0xaabbccdd)) + words(0xabcdef01, -52)
    return result


def facet(version: int, mode: int, lightweight: bool, enabled: bool) -> bytes:
    return (header_fields(version, mode) + early_fields(version, enabled)
            + texture_fields(version, enabled) + surface_fields(version, enabled)
            + final_fields(version, lightweight, enabled))


def directory(version: int) -> bytes:
    result = words(2, 7, 1, -1) + string("λ/bolt") + words(0x87654321)
    result += words(0x01234567, 0x89abcdef, 0xfedcba98, 0x76543210, 0x12345678, 0x9abcdef0)
    result += string("000-000-007") + words(42)
    if version >= 0x3b23:
        result += words(2, -1, 1, 2, -1, 3, 4)
    elif version >= 0x3b1c:
        result += words(-1, 2)
    return result


def generate() -> None:
    ROOT.mkdir(parents=True, exist_ok=True)
    thresholds = [0x2cd, 0x331, 0x3de, 0x49e, 0x4e6, 0x4fc, 0x534, 0x68a,
                  0x7dc, 0x82a, 0x83b, 0x901, 0x967, 0x972, 0xc7c, 0x1099,
                  0x116a, 0x1c24, 0x1c3b, 0x2792, 0x279e, 0x27a0, 0x27ae,
                  0x27b6, 0x27e1, 0x2bd0, 0x2bd1, 0x3752, 0x3758, 0x3b0e,
                  0x3b29, 0x3ee8, 0x3f0f, 0x3f10]
    versions = sorted({0, 19000, *(v + delta for v in thresholds for delta in (-1, 0, 1))})
    write_facets(versions)
    write_modes_and_directory()


def write_facets(versions: list[int]) -> None:
    lines = []
    for version in versions:
        if 0xa25 <= version <= 0xc7b:
            continue
        for enabled in (False, True):
            name = f"facet-{version:04x}-{int(enabled)}.bin"
            (ROOT / name).write_bytes(facet(version, -17, False, enabled))
            lines.append(f"{name}\t{version}\tfull\n")
    (ROOT / "cases.tsv").write_text("".join(lines), encoding="utf-8", newline="\n")


def write_modes_and_directory() -> None:
    lines = []
    for mode in (0, 4):
        name = f"mode-{mode}.bin"
        (ROOT / name).write_bytes(facet(19000, mode, False, True))
        lines.append(f"{name}\t19000\tfull\n")
    (ROOT / "lightweight.bin").write_bytes(facet(19000, -17, True, True))
    lines.append("lightweight.bin\t19000\tlight\n")
    manifest = ROOT / "cases.tsv"
    manifest.write_text(manifest.read_text(encoding="utf-8") + "".join(lines), encoding="utf-8", newline="\n")
    for version in (0x3b1b, 0x3b1c, 0x3b22, 0x3b23):
        (ROOT / f"directory-{version:04x}.bin").write_bytes(directory(version))
    modern = facet(19000, -17, False, True)
    (ROOT / "block.bin").write_bytes(words(42, 0x12345678, 0x9abcdef0, 2) + modern * 2)


if __name__ == "__main__":
    generate()
