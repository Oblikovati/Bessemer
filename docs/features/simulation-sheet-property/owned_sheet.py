"""Independent inherited-sheet literals from the bound C-737 grammar (#151)."""
from owned_components import signed_words
from owned_body import body_archives, body_text


def sheet_versions() -> list[int]:
    """Include every inherited outer gate and neighbor; e.g. sheet_versions()."""
    return sorted({0, 2147483647, *(gate + delta for gate in (830004, 840002) for delta in (-1, 0, 1))})


def sheet_literal(version: int, profile: int) -> bytes:
    """Encode one complete inherited Property without a SurfaceBody suffix; e.g. sheet_literal(840002, 3)."""
    blob = b'' if profile == 0 else b'\xaa\0\xff'
    raw = signed_words(-2147483648, 2147483647, len(blob)) + blob
    if version >= 840002: raw += body_archives(profile)
    raw += body_text(profile) * 2 + signed_words(-2147483648, 2147483647, -1)
    return raw + (signed_words(-2) if version >= 830004 else b'')


def owned_sheets() -> list[tuple[int, int, bytes]]:
    """Retain all literal version/profile pairs in total order; e.g. owned_sheets()."""
    return [(profile, version, sheet_literal(version, profile)) for version in sheet_versions() for profile in range(4)]
