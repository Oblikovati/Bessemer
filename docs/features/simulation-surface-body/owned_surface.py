"""Independent SurfaceBody literals from the bound C-733 grammar (#153)."""
from owned_components import signed_words
from owned_body import body_text
from owned_embedded import patterned_doubles

SURFACE_GATES = (830015, 830023, 830025, 830029, 830031, 850013, 870008, 870022)


def surface_versions() -> list[int]:
    """Include every exact historical threshold and neighbor; e.g. surface_versions()."""
    return sorted({0, 2147483647, *(gate + delta for gate in SURFACE_GATES for delta in (-1, 0, 1))})


def surface_literal(version: int, profile: int) -> bytes:
    """Preserve the physical suffix order and raw signed/double/text patterns; e.g. surface_literal(870022, 3)."""
    raw = signed_words(-2147483648) + body_text(profile)
    for gate, values in ((830015, (-1,)), (830023, (2147483647,)), (830025, (-1, 2))):
        if version >= gate: raw += signed_words(*values)
    if version >= 830029: raw += signed_words(-1, 2, -3, 4) + patterned_doubles(5)
    if version >= 830031: raw += signed_words(-90)
    if version >= 850013: raw += signed_words(-100, 94)
    if version >= 870008:
        texts = () if profile == 0 else (body_text(profile), body_text(profile), body_text(0))
        raw += signed_words(len(texts)) + b''.join(texts)
    if version >= 870022: raw += signed_words(-144)
    return raw


def owned_surface_bodies() -> list[tuple[int, int, bytes]]:
    """Keep all literal version/profile pairs in deterministic order; e.g. owned_surface_bodies()."""
    return [(profile, version, surface_literal(version, profile)) for version in surface_versions() for profile in range(4)]
