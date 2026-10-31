"""Independent complete shell Property literals with explicit newest layer context (#159)."""
from typing import TypedDict
import struct
from owned_components import signed_words
from owned_embedded import patterned_doubles
from owned_thermal import thermal_base, mixed_texts
from owned_material import material_literal


class ShellRawContext(TypedDict):
    kind: int
    version: int
    raw: bytes
    layer_presence: list[bool] | None


class OwnedShellProperty(ShellRawContext):
    base_end: int
    subtype: int
    inline_materials: list[bytes]


def empty_shell_base(version: int) -> bytes:
    """Use independent scalar widths and null text/reference frames; e.g. empty_shell_base(840005)."""
    size = 148
    for enabled, width in ((version > 800027, 4), (version > 800001, 16),
                           (version > 800002, 4), (version > 800037, 4), (version >= 800066, 4),
                           (version >= 870006, 4), (version >= 870010, 4), (version >= 880008, 8),
                           (version >= 890009, 20), (version >= 890011, 8)):
        if enabled: size += width
    return bytes(size)


def shell_derived_prefix(version: int, subtype: int, word368: int) -> bytes:
    """Keep the loaded double bits even when native word368 changes its runtime value; e.g. shell_derived_prefix(840005, 2, 1)."""
    raw = patterned_doubles(1) + signed_words(-2147483648, subtype, 2147483647)
    raw += mixed_texts(2) + struct.pack('<2I', 0xffffffff, 0x80000000)
    if version >= 820017: raw += struct.pack('<2I', 0x80000001, 0xfffffffe)
    if version > 830008:
        raw += signed_words(-1)
        if version >= 840005: raw += signed_words(word368) + struct.pack('<Q', 0x7ff8000000000042)
    return raw


def shell_descriptor(flags: int) -> bytes:
    """Order bit-1 then bit-2 bodies and retain unknown flag bits; e.g. shell_descriptor(0x83)."""
    raw = signed_words(flags)
    for bit in (1, 2):
        if flags & bit: raw += struct.pack('<Q', 0x8000000000000000 if bit == 1 else 0x7ff8000000000042) + mixed_texts(1)
    return raw


def shell_layers(version: int, profile: int) -> tuple[bytes, list[bool] | None, list[bytes]]:
    """Separate absent/inline Material from externally supplied native layer presence; e.g. shell_layers(891106, 6)."""
    count = 0 if profile == 2 else 2
    external = 1 if profile == 3 else -1 if profile == 7 else 0
    raw = signed_words(count, -2147483648, 2147483647, external)
    presence = [bool((profile + index) % 2) for index in range(count)] if version >= 891106 else None
    materials: list[bytes] = []
    for index in range(count):
        raw += patterned_doubles(2)
        if external == 0:
            present = 0 if profile == 4 else -1 if profile == 6 else 1
            raw += signed_words(present)
            if present:
                body = material_literal(version, index % 2, empty=profile == 5)
                raw += body
                materials.append(body)
        if presence is not None and presence[index]: raw += shell_descriptor((0, 1, 2, 3, 0x80, 0x83)[profile % 6])
    return raw, presence, materials


def shell_property_literal(kind: int, version: int, profile: int) -> OwnedShellProperty:
    """Construct a complete body without asking any parser for offsets; e.g. shell_property_literal(0, 891106, 6)."""
    base = empty_shell_base(version) if profile == 0 else thermal_base(version)
    subtype = shell_subtype(profile)
    raw = base + shell_derived_prefix(version, subtype, profile % 4 - 1)
    presence, materials = None, []
    if subtype == 2 and version >= 830010:
        layers, presence, materials = shell_layers(version, profile)
        raw += layers
    if version >= 870022: raw += signed_words(-2147483648)
    if version >= 890006: raw += mixed_texts(1)
    return OwnedShellProperty(kind=kind, version=version, raw=raw, base_end=len(base), subtype=subtype,
                              layer_presence=presence, inline_materials=materials)


def owned_shell_properties() -> list[OwnedShellProperty]:
    """Cross every shared/derived/material threshold neighbor with explicit contexts; e.g. owned_shell_properties()."""
    gates = (800001, 800002, 800010, 800016, 800027, 800028, 800032, 800037, 800039, 800049,
             800066, 800075, 810001, 810004, 820002, 820010, 820011, 820017, 830001, 830003,
             830008, 830010, 830014, 830015, 830016, 830028, 840002, 840005, 850023,
             870006, 870010, 870022, 880008, 880011, 890003, 890006, 890009, 890011,
             890013, 890015, 890021, 890022, 891106)
    versions = sorted({0, 2147483647, *(gate + delta for gate in gates for delta in (-1, 0, 1))})
    cases = [shell_property_literal(kind, version, profile) for kind in range(2) for version in versions for profile in range(10)]
    cases.extend(empty_curve_mode_case(kind, version, mode) for kind in range(2)
                 for version in (890011, 891106, 2147483647) for mode in (-2147483648, -101, 101, 2147483647))
    return cases


def shell_subtype(profile: int) -> int:
    """Keep noncomposite positive/negative subtypes distinct from exact two; e.g. shell_subtype(9)."""
    if profile == 0: return 0
    if profile == 1: return -1
    return 3 if profile == 9 else 2


def empty_curve_mode_case(kind: int, version: int, mode: int) -> OwnedShellProperty:
    """A zero-record shell table does not restrict its unused mode; e.g. empty_curve_mode_case(0, 890011, 101)."""
    case = shell_property_literal(kind, version, 0)
    offset = case['base_end'] - 8
    case['raw'] = case['raw'][:offset] + signed_words(mode) + case['raw'][offset + 4:]
    return case
