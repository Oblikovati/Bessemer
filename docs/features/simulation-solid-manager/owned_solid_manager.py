"""Independent exact direct SolidManager header literals and populated text lists (#148)."""
from owned_components import signed_words
from owned_thermal import persisted_literal


def solid_list_literal(profile: int) -> bytes:
    """Keep null, odd, unpaired and repeated frames without normalization; e.g. solid_list_literal(2)."""
    payloads = ((), (None, b'\0\0', b'\x00\xd8\0\0', b'\x33\0\0'),
                (b'A\0\0\0', b'A\0\0\0'), (b'\x33\0\0',))
    selected = payloads[profile]
    return signed_words(len(selected)) + b''.join(persisted_literal(payload) for payload in selected)


def solid_header_literal(version: int, profile: int) -> bytes:
    """Encode historical wire order independently of the actual reader; e.g. solid_header_literal(870008, 1)."""
    raw = signed_words(-2147483648)
    if version >= 800068: raw += signed_words(2147483647)
    if version >= 850032: raw += signed_words(-1)
    if version >= 840013: raw += signed_words(2)
    if version >= 870004: raw += solid_list_literal(profile)
    if version >= 870008: raw += solid_list_literal(0 if profile == 0 else 4 - profile)
    return raw


def owned_solid_headers() -> list[tuple[int, int, bytes]]:
    """Cover all five gate neighbors plus empty and populated raw lists; e.g. owned_solid_headers()."""
    gates = (800068, 840013, 850032, 870004, 870008)
    versions = sorted({0, 2147483647, *(gate + delta for gate in gates for delta in (-1, 0, 1))})
    return [(profile, version, solid_header_literal(version, profile)) for version in versions for profile in range(4)]
