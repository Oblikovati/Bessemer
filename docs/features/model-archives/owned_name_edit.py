"""Independent expected bytes for one owned model edit, never a runtime encoder."""
import struct

OWNED_ROOT_PREFIX = bytes.fromhex("ffff010008006d6f506172745f63")
OWNED_NAME = b"\xff\xfe\xff\x05" + "Part1".encode("utf-16-le")


def short_wide_name(name: str) -> bytes:
    """Count UTF-16 code units for a short CString; e.g. short_wide_name('Bessemer Ω')."""
    try:
        encoded = name.encode("utf-16-le")
    except UnicodeEncodeError as error:
        raise ValueError(f"model name {name!r}; expected valid UTF-16 text") from error
    units = len(encoded) // 2
    if units >= 255:
        raise ValueError(f"model name length {units}; expected fewer than 255 UTF-16 code units")
    return b"\xff\xfe\xff" + bytes([units]) + encoded


def owned_name_edit(original: bytes, name: str) -> bytes:
    """Patch only the reviewed owned fixture; e.g. owned_name_edit(raw, 'Bessemer Ω')."""
    if len(original) != 24028 or original[:14] != OWNED_ROOT_PREFIX:
        raise ValueError(f"model shape ({len(original)}, {original[:14].hex()}); expected owned 24028-byte moPart_c root")
    if original[44:58] != OWNED_NAME or original[14:18] != struct.pack("<i", 2903):
        raise ValueError(f"owned name/size {original[44:58].hex()}/{original[14:18].hex()}; expected Part1 and 2903")
    replacement = short_wide_name(name)
    delta = len(replacement) - len(OWNED_NAME)
    changed = bytearray(original[:44] + replacement + original[58:])
    struct.pack_into("<i", changed, 14, 2903 + delta)
    return bytes(changed)
