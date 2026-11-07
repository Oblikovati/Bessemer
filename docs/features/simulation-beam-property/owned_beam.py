"""Independent beam Property literals with complete historical references (#142)."""
import struct
from owned_components import signed_words
from owned_embedded import patterned_doubles
from owned_thermal import thermal_reference, mixed_texts


def beam_suffix(version: int, points: int) -> bytes:
    """Preserve short widths and high-bit unsigned words; e.g. beam_suffix(850036, 2)."""
    raw = struct.pack('<4i2h', -2147483648, 2147483647, -1, 0, -32768, 32767)
    raw += mixed_texts(2)
    if version >= 840014:
        raw += patterned_doubles(4)
        if version >= 850035: raw += patterned_doubles(3)
        if version >= 840024: raw += signed_words(-2147483648)
    if version >= 820007: raw += signed_words(points) + patterned_doubles(points * 3)
    raw += signed_words(-1) + struct.pack('<iiIiI', -1, 2, 0xffffffff, -3, 0x80000000)
    if version >= 810029: raw += signed_words(-2147483648, 2147483647)
    if version >= 850036: raw += signed_words(-1)
    return raw


def owned_beam_cases() -> list[tuple[int, int, bytes]]:
    """Cover every beam/reference threshold neighbor and empty/populated bodies; e.g. owned_beam_cases()."""
    gates = (800075, 810029, 820007, 840002, 840014, 840024, 850035, 850036, 890021)
    versions = sorted({0, *(gate + delta for gate in gates for delta in (-1, 0, 1))})
    cases: list[tuple[int, int, bytes]] = []
    for version in versions:
        for profile in range(4):
            reference = thermal_reference(version) if profile & 1 else b''
            prefix = signed_words(profile & 1) + reference + signed_words(-1, 2)
            cases.append((profile, version, prefix + beam_suffix(version, 2 if profile & 2 else 0)))
    return cases
