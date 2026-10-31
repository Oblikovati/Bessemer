"""Independent complete thermal images with populated archives, optionals and table pairs (#134)."""
import struct
from owned_components import signed_words
from owned_embedded import literal_wrapper, literal_transform, patterned_doubles
from owned_chains import tagged_component
from owned_references import wrapper_bodies

THERMAL_KINDS = ('Temperature', 'Convection', 'HeatFlux', 'HeatPower', 'Radiation')


def persisted_literal(payload: bytes | None) -> bytes:
    """Keep raw null, empty, odd and surrogate payloads distinct; e.g. persisted_literal(b'\0\0')."""
    payload = b'' if payload is None else payload
    return struct.pack('<I', len(payload)) + payload


def mixed_texts(count: int) -> bytes:
    """Cycle reviewed raw frames without Unicode repair; e.g. mixed_texts(4)."""
    payloads = (None, b'\0\0', b'\x00\xd8\0\0', b'\x33\0\0')
    return b''.join(persisted_literal(payloads[index % 4]) for index in range(count))


def thermal_reference(version: int) -> bytes:
    """Use two nonnull inner archives under their own distinct versions; e.g. thermal_reference(890501)."""
    blob = signed_words(3) + b'\xaa\0\xff'
    if version < 840002:
        raw = signed_words(-1, 2, -3, 4, -5, 6, -7) + mixed_texts(1) + blob
        return raw + (signed_words(-1, -2) if version >= 800075 else b'')
    transform = literal_wrapper(14000) + literal_transform(255, 0x2f) + literal_transform(1, 0x4a)
    first = struct.pack('<I', 14000) + tagged_component('moCompXFormWrapper_c', transform)
    second = struct.pack('<I', 4281) + tagged_component('moLineRef_w', wrapper_bodies(4281)['moLineRef_w'])
    raw = signed_words(-1, 2) + blob + first + second + signed_words(1, -2, 3, -4, 5)
    raw += mixed_texts(1) + signed_words(-1, -2)
    return raw + (signed_words(3, -4, 5) if version >= 890021 else b'')


def optional_literal(version: int) -> bytes:
    """Populate the selector-four matrix and interleaved double pairs; e.g. optional_literal(820011)."""
    raw = mixed_texts(1) + signed_words(-1, 4, -3, 4, 2) + patterned_doubles(9)
    raw += signed_words(-7) if version >= 820011 else b''
    return raw + patterned_doubles(4)


def base_late_literal(version: int) -> bytes:
    """Encode common gates directly from the frozen schema; e.g. base_late_literal(890501)."""
    raw = b''
    for enabled in (version > 800002, version > 800037, version >= 800066):
        if enabled: raw += signed_words(-1)
    if version <= 810001: return raw
    for threshold in (870006, 870010):
        if version >= threshold: raw += signed_words(-1)
    if version >= 880008: raw += patterned_doubles(1)
    if version < 890009: return raw
    raw += signed_words(-1, 2, -3) + mixed_texts(2)
    if version >= 890011:
        table = mixed_texts(1) + b''.join(patterned_doubles(1) + signed_words(-index) for index in range(15))
        raw += signed_words(-100, 2) + table * 2
    return raw


def thermal_base(version: int) -> bytes:
    """Build two identical populated records without native insertion/deduplication; e.g. thermal_base(840002)."""
    raw = signed_words(1) + thermal_reference(version) + signed_words(-1, 2)
    raw += signed_words(-198) + mixed_texts(4) + signed_words(-104, 88)
    raw += patterned_doubles(1) + signed_words(-236) + patterned_doubles(1)
    raw += struct.pack('<I', 0xffffffff) + signed_words(-272, 168, -480)
    if version > 800027: raw += signed_words(-256)
    raw += patterned_doubles(6) + mixed_texts(6)
    if version > 800001:
        raw += signed_words(-536, -1) + optional_literal(version)
        raw += signed_words(552, 2) + optional_literal(version)
    return raw + base_late_literal(version)


def pair_literal(selector: int) -> bytes:
    """Only exact selector one has a table; e.g. pair_literal(-1)."""
    return signed_words(selector) + (signed_words(3, -1, 2, -1, 2, -2147483648, 2147483647) if selector == 1 else b'')


def thermal_tail(kind: str, version: int, selector: int) -> bytes:
    """Construct all five complete derived grammars independently; e.g. thermal_tail('HeatFlux', 890501, 1)."""
    if kind in ('HeatFlux', 'HeatPower'): return heat_tail(kind, version, selector)
    if kind == 'Temperature':
        raw = signed_words(-1, 2) + patterned_doubles(1) + mixed_texts(1) + signed_words(-24)
        return raw + (pair_literal(selector) if version >= 890501 else b'')
    if kind == 'Convection':
        raw = signed_words(-1, 2) + patterned_doubles(2) + mixed_texts(2)
        if version > 800001: raw += signed_words(-800, -1) + optional_literal(version)
    else:
        raw = signed_words(-1, 2, -3, 4) + patterned_doubles(4) + mixed_texts(3)
    return raw + (signed_words(-1) if version >= 890501 else b'')


def heat_tail(kind: str, version: int, selector: int) -> bytes:
    """Keep HeatFlux and HeatPower suffix differences explicit; e.g. heat_tail('HeatPower', 830027, 1)."""
    raw = signed_words(-1, 2) + patterned_doubles(1) + signed_words(-3)
    raw += patterned_doubles(2) + signed_words(-4) + mixed_texts(3)
    if kind == 'HeatPower': raw += signed_words(-24)
    if version >= 800037: raw += signed_words(-1)
    if kind == 'HeatPower' and version >= 830027: raw += signed_words(-836)
    if version >= 890501:
        raw += pair_literal(selector) if kind == 'HeatPower' else signed_words(-1, 2)
    return raw


def owned_thermal_cases() -> list[tuple[int, int, bytes]]:
    """Cover each common and derived threshold and its neighbors; e.g. owned_thermal_cases()."""
    gates = (800001, 800002, 800027, 800037, 800066, 800075, 810001, 820011, 830027,
             840002, 870006, 870010, 880008, 890009, 890011, 890021, 890501)
    versions = sorted({0, 891107, *(gate + delta for gate in gates for delta in (-1, 0, 1))})
    cases: list[tuple[int, int, bytes]] = []
    for kind_index, kind in enumerate(THERMAL_KINDS):
        for version in versions:
            for selector in ((0, 1, 2, -1) if version >= 890501 else (0,)):
                record = thermal_base(version) + thermal_tail(kind, version, selector)
                cases.append((kind_index, version, signed_words(2) + record * 2))
    return cases
