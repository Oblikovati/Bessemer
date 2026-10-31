"""Independent complete Material literals with all table-presence and historical boundaries (#140)."""
from owned_components import signed_words
from owned_embedded import patterned_doubles
from owned_thermal import persisted_literal


def material_texts(count: int, start: int = 0) -> bytes:
    """Keep null, empty, odd, embedded NUL and surrogate strings distinct; e.g. material_texts(4)."""
    payloads = (None, b'\0\0', b'\0\xd8\0\0', b'3\0\0', b'A\0\0\0B\0\0\0')
    return b''.join(persisted_literal(payloads[(index + start) % len(payloads)]) for index in range(count))


def property_literal(version: int, present: int, count: int) -> bytes:
    """Retain absent positive declarations without adding point bytes; e.g. property_literal(891107, 0, 10000)."""
    raw = material_texts(1, 4) + signed_words(-2147483648) + patterned_doubles(1)
    raw += material_texts(1, 3) + signed_words(present) + material_texts(1)
    raw += signed_words(count, -1, 2, -3, 2147483647)
    if present and count:
        raw += patterned_doubles(count * 2)
    return raw + (signed_words(-7) if version >= 800010 else b'')


def curve_literal(version: int, count: int) -> bytes:
    """Keep the direct helper's mixed scalars and interleaved points; e.g. curve_literal(830016, 2)."""
    raw = material_texts(1) + signed_words(-1, 2, -3, 4, -5, 6, -7)
    if version > 800032: raw += signed_words(-256)
    raw += patterned_doubles(9) + signed_words(-144) + patterned_doubles(1)
    raw += signed_words(148) + patterned_doubles(2) + signed_words(count)
    if version > 820010: raw += signed_words(-248)
    raw += patterned_doubles(count * 2)
    return raw + (material_texts(1) if version > 830015 else b'')


def material_groups(version: int, empty: bool) -> bytes:
    """Encode both separate direct table groups without tagged headers; e.g. material_groups(891107, False)."""
    raw = b''
    if version > 800016: raw += signed_words(-14592, 14596)
    if version > 800028: raw += signed_words(-108)
    if version >= 800039:
        raw += signed_words(-112, 116, 0 if empty else 2)
        if not empty: raw += curve_literal(version, 2) + curve_literal(version, 0)
    if version >= 800049:
        raw += signed_words(-14680, 14728, -14732, 14736, 0 if empty else 1)
        if not empty: raw += curve_literal(version, 3)
    return raw


def material_late(version: int) -> bytes:
    """Encode each tail gate literally and in wire order; e.g. material_late(890022)."""
    raw = material_texts(1) if version >= 810004 else b''
    for threshold, word in ((820002, -14740), (830001, 168)):
        if version >= threshold: raw += signed_words(word)
    if 830003 <= version < 830028: raw += signed_words(-2147483648)
    if version >= 830014: raw += signed_words(-184)
    if version >= 830016: raw += material_texts(3)
    if version >= 850023: raw += material_texts(1)
    if version >= 880011: raw += signed_words(-224)
    if version >= 890003: raw += patterned_doubles(1) + signed_words(-128, 132, -136)
    if version >= 890013: raw += signed_words(-140, 144)
    if version >= 890022: raw += signed_words(-148)
    if version >= 890015: raw += patterned_doubles(1)
    return raw


def material_literal(version: int, present: int, empty: bool = False) -> bytes:
    """Build a complete body independently of either parser; e.g. material_literal(891107, 0)."""
    raw = material_texts(2) + signed_words(0 if empty else 2, -1, -2147483648, 2147483647)
    raw += material_texts(3) + signed_words(-104) + material_texts(2)
    if not empty:
        raw += property_literal(version, present, 3 if present else 10000)
        raw += property_literal(version, 0, 7)
    raw += material_texts(1) + signed_words(0 if empty else 2, -1, 2, -3)
    if not empty: raw += patterned_doubles(4)
    return raw + material_groups(version, empty) + material_late(version)


def owned_material_cases() -> list[tuple[int, bytes]]:
    """Cover every recovered threshold and neighbor plus count-only and empty forms; e.g. owned_material_cases()."""
    gates = (800010, 800017, 800029, 800033, 800039, 800049, 810004, 820002, 820011,
             830001, 830003, 830014, 830016, 830028, 850023, 880011, 890003, 890013,
             890015, 890022)
    versions = sorted({0, 891107, 2147483647, *(gate + delta for gate in gates for delta in (-1, 0, 1))})
    return [(version, material_literal(version, present)) for version in versions for present in (-1, 0, 1, 2)] + [
        (version, material_literal(version, 0, True)) for version in versions]
