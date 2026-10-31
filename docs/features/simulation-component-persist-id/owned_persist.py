"""Independent standalone Component PersistID literals from C-738 (#152)."""
from owned_components import signed_words
from owned_body import body_archives


def owned_component_persist_ids() -> list[tuple[int, int, bytes]]:
    """Include empty/populated blobs and distinct inner versions around the exact outer gate; e.g. owned_component_persist_ids()."""
    cases: list[tuple[int, int, bytes]] = []
    for version in (0, 840001, 840002, 840003, 2147483647):
        for profile in range(4):
            blob = (b'', b'\xaa\0\xff', b'\0\0\0\0', b'\xff\0\xff\0\xff')[profile]
            raw = signed_words(-2147483648, 2147483647, len(blob)) + blob
            if version >= 840002: raw += body_archives(profile)
            cases.append((profile, version, raw))
    return cases
