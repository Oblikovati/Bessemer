"""Independent complete joint/group/list/folder literals with historical references (#144)."""
from owned_components import signed_words
from owned_embedded import patterned_doubles
from owned_thermal import thermal_reference, persisted_literal

KINDS = ('JointGroup', 'Joint', 'JointList', 'BeamFolder')


def reference_body(version: int, profile: int) -> bytes:
    """Keep container framing separate from children and runtime lookup; e.g. reference_body(840002, 1)."""
    count = profile & 1
    return signed_words(count) + (thermal_reference(version) if count else b'') + signed_words(-1, 2)


def profile_text(profile: int) -> bytes:
    """Keep null, allocated empty, unpaired and odd text frames; e.g. profile_text(3)."""
    return persisted_literal((None, b'\0\0', b'\x00\xd8\0\0', b'\x33\0\0')[profile])


def group_body(version: int, profile: int) -> bytes:
    """Encode direct group fields and each independently gated suffix; e.g. group_body(840023, 1)."""
    raw = signed_words(-2147483648) + profile_text(profile) + patterned_doubles(1) + signed_words(1, -2, 3)
    for gate in (810023, 810024, 810027, 830029):
        if version >= gate: raw += signed_words(-gate)
    if version >= 810014: raw += reference_body(version, profile)
    if version >= 840023: raw += patterned_doubles(1)
    return raw


def joint_body(version: int, profile: int) -> bytes:
    """Exercise inclusive old references and retain repeated signed beam IDs; e.g. joint_body(810030, 3)."""
    raw = signed_words(-1, 2)
    if version >= 810021: raw += signed_words(-2147483648, 2147483647)
    if 810000 <= version <= 810030: raw += reference_body(version, profile)
    ids = (-7, 8, -7) if profile & 2 else ()
    raw += signed_words(len(ids), *ids)
    if version >= 810022: raw += patterned_doubles(3)
    if version >= 810030: raw += patterned_doubles(1)
    return raw


def list_body(profile: int) -> tuple[int | None, bytes]:
    """Retain duplicate signed neighbors and optional independent count context; e.g. list_body(3)."""
    count = 2 if profile & 2 else 0
    raw = signed_words(count)
    if count: raw += signed_words(3, -1, 2, 3, -4, -1, 2, 0)
    return (count if profile & 1 else None), raw


def owned_group_cases() -> list[tuple[int, int, int | None, bytes]]:
    """Cover all threshold neighbors, kinds, populations and count contexts; e.g. owned_group_cases()."""
    gates = (800075, 810000, 810014, 810021, 810022, 810023, 810024,
             810027, 810030, 810031, 830029, 840002, 840023, 890021)
    versions = sorted({0, *(gate + delta for gate in gates for delta in (-1, 0, 1))})
    cases: list[tuple[int, int, int | None, bytes]] = []
    for version in versions:
        for profile in range(4):
            cases.append((0, version, None, group_body(version, profile)))
            cases.append((1, version, None, joint_body(version, profile)))
            expected, raw = list_body(profile)
            cases.append((2, version, expected, raw))
            cases.append((3, version, None, signed_words(-2147483648) + profile_text(profile)))
    return cases
