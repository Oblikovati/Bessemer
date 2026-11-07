"""Independent complete single-body mass Property literals (#138)."""
from owned_components import signed_words
from owned_embedded import patterned_doubles
from owned_thermal import thermal_base, mixed_texts, pair_literal


def mass_suffix(version: int, selector: int) -> bytes:
    """Keep the direct body and exact selector-one gate; e.g. mass_suffix(890204, 1)."""
    raw = signed_words(-2147483648, 2147483647, -1) + mixed_texts(2)
    raw += signed_words(-1, 2, -3) + patterned_doubles(6)
    raw += signed_words(-1, 2, -3, 4, -5, 6) + patterned_doubles(16)
    if version >= 890204:
        raw += pair_literal(selector)
    return raw + mixed_texts(6) + signed_words(-1, 2, -3, 4, -5, 6, -7)


def owned_mass_cases() -> list[tuple[int, int, bytes]]:
    """Cover all shared gates and derived selector branches without a count prefix; e.g. owned_mass_cases()."""
    gates = (800001, 800002, 800027, 800037, 800066, 800075, 810001, 820011,
             840002, 870006, 870010, 880008, 890009, 890011, 890021, 890204, 891107)
    versions = sorted({0, *(gate + delta for gate in gates for delta in (-1, 0, 1))})
    return [(selector, version, thermal_base(version) + mass_suffix(version, selector))
            for version in versions for selector in (-1, 0, 1, 2)]
