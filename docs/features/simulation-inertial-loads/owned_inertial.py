"""Independent complete inertial/excitation literals with populated shared bases (#136)."""
import struct
from owned_components import signed_words
from owned_embedded import patterned_doubles
from owned_thermal import thermal_base, mixed_texts

LOAD_KINDS = ('Gravity', 'CentriFugal', 'BaseExcitation')


def gravity_suffix(version: int) -> bytes:
    """Retain all four words and both double arrays; e.g. gravity_suffix(800037)."""
    raw = signed_words(-2147483648, 2147483647, -1, 2)
    raw += patterned_doubles(3) + patterned_doubles(12) + mixed_texts(3)
    return raw + (signed_words(-3, 4, -5) if version >= 800037 else b'')


def centrifugal_suffix(version: int) -> bytes:
    """Keep the exact persisted CentriFugal spelling and complete arrays; e.g. centrifugal_suffix(800037)."""
    raw = signed_words(-2147483648, 2147483647, -1, 2, -3, 4, -5) + mixed_texts(2)
    raw += patterned_doubles(3) + patterned_doubles(12) + patterned_doubles(12)
    return raw + (signed_words(-6, 7) if version >= 800037 else b'')


def excitation_suffix(version: int) -> bytes:
    """Keep the unsigned header and independent three/four-tail gate; e.g. excitation_suffix(820012)."""
    raw = signed_words(2, -1, 3, 1, 0) + struct.pack('<I', 0xffffffff)
    raw += patterned_doubles(3) + signed_words(-1, 2, -3) + patterned_doubles(12)
    raw += mixed_texts(4 if version >= 820012 else 3) + signed_words(-4, 5, -6)
    if version >= 820012:
        raw += signed_words(-7, -2147483648) + patterned_doubles(1) + signed_words(2147483647)
    return raw


def owned_inertial_cases() -> list[tuple[int, int, bytes]]:
    """Cover every shared/derived threshold and neighbor with two full records; e.g. owned_inertial_cases()."""
    gates = (800001, 800002, 800027, 800037, 800066, 800075, 810001, 820011,
             820012, 840002, 870006, 870010, 880008, 890009, 890011, 890021)
    versions = sorted({0, 891107, *(gate + delta for gate in gates for delta in (-1, 0, 1))})
    suffixes = (gravity_suffix, centrifugal_suffix, excitation_suffix)
    return [(kind, version, signed_words(2) + (thermal_base(version) + suffix(version)) * 2)
            for kind, suffix in enumerate(suffixes) for version in versions]
