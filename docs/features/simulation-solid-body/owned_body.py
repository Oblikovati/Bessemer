"""Independent complete solid-body literals from C-731/C-734 (#150)."""
import struct
from owned_components import signed_words
from owned_thermal import persisted_literal
from owned_embedded import literal_wrapper, literal_transform, patterned_doubles
from owned_chains import tagged_component
from owned_references import wrapper_bodies

GATES = (800066, 800068, 810025, 810029, 830007, 830011, 830013, 830025,
         830029, 840002, 840006, 840013, 840025, 850002, 850013, 850026,
         870005, 870008, 870019, 880004)


def body_versions() -> list[int]:
    """Cover every literal historical threshold and neighbor; e.g. body_versions()."""
    return sorted({0, 2147483647, *(gate + delta for gate in GATES for delta in (-1, 0, 1))})


def body_text(profile: int) -> bytes:
    """Keep null, empty, unpaired and odd raw frames; e.g. body_text(3)."""
    return persisted_literal((None, b'\0\0', b'\x00\xd8\0\0', b'\x33\0\0')[profile])


def body_archives(profile: int) -> bytes:
    """Use actual archive framing and existing independent wrapper literals; e.g. body_archives(1)."""
    if profile == 0: return (struct.pack('<IH', 14000, 0)) * 2
    transform = literal_wrapper(14000) + literal_transform(255, 0x2f) + literal_transform(1, 0x4a)
    first = struct.pack('<I', 14000) + tagged_component('moCompXFormWrapper_c', transform)
    second = struct.pack('<I', 4281) + tagged_component('moLineRef_w', wrapper_bodies(4281)['moLineRef_w'])
    return first + second


def early_body_words(version: int) -> bytes:
    """Keep physical mixed-block order and raw doubles separate; e.g. early_body_words(830029)."""
    raw = b''
    for gate, count in ((800066, 1), (800068, 1), (810025, 1), (830007, 2),
                        (830011, 3), (830013, 1), (830025, 1)):
        if version >= gate: raw += signed_words(*range(-count, 0))
    if version >= 830029: raw += signed_words(-1, 2, -3, 4) + patterned_doubles(5)
    if version >= 810029: raw += signed_words(-2147483648, 2147483647)
    return raw


def late_body_words(version: int) -> bytes:
    """Encode documented suffix gates without version inference; e.g. late_body_words(880004)."""
    raw = b''
    for gate, count in ((840006, 3), (840013, 1), (840025, 1), (850002, 1),
                        (850013, 1), (850026, 1), (870005, 1)):
        if version >= gate: raw += signed_words(*range(-count, 0))
    return raw


def solid_body_literal(version: int, profile: int) -> bytes:
    """Encode one full direct body independently of parser spans; e.g. solid_body_literal(880004, 3)."""
    blob = b'' if profile == 0 else b'\xaa\0\xff'
    raw = signed_words(-2147483648, 2147483647, len(blob)) + blob
    if version >= 840002: raw += body_archives(profile)
    raw += body_text(profile) * 2 + signed_words(-1, 2, -3, 4) + body_text(profile)
    raw += early_body_words(version) + late_body_words(version)
    if version >= 870008:
        texts = () if profile == 0 else (body_text(profile), body_text(profile), body_text(0))
        raw += signed_words(len(texts)) + b''.join(texts)
    if version >= 870019: raw += signed_words(-1)
    if version >= 880004: raw += signed_words(-1, 2) + body_text(profile) * 3
    return raw


def owned_solid_bodies() -> list[tuple[int, int, bytes]]:
    """Keep every gate/profile pair in deterministic order; e.g. owned_solid_bodies()."""
    return [(profile, version, solid_body_literal(version, profile)) for version in body_versions() for profile in range(4)]


def solid_identifier_archives() -> list[bytes]:
    """Keep all four concrete suffixes and their strict inner-version boundaries; e.g. solid_identifier_archives()."""
    layouts = (('moSimpleEdgeIdRep_c', False, 1), ('moCopySurfIdRep_c', True, 1),
               ('moMacroFeatRefSurfIdRep_c', True, 3), ('moEndCapChamferSurfIdRep_c', False, 2))
    cases: list[bytes] = []
    for name, source, count in layouts:
        prefix = bytes(4) + (bytes(2) if source else b'')
        words = signed_words(*(-2147483648 if index % 2 == 0 else 2147483647 for index in range(count)))
        for version in (4281, 4306, 4307, 14000, 19000, 19001):
            cases.append(struct.pack('<I', version) + tagged_component(name, prefix + words))
    return cases
