"""Independent composite scalar/selector/optional-ID literals (#157)."""
from itertools import product
from typing import TypedDict
import struct
from owned_persist import owned_component_persist_ids

VERSIONS = (0, 840001, 840002, 840003, 2147483647)
DOUBLE_BITS = (0x8000000000000000, 0x7ff8000000000042, 0x7ff0000000000000,
               0xfff0000000000000, 1, 0x3ff0000000000000)


class OwnedComposite(TypedDict):
    kind: str
    wire_profile: str
    version: int
    raw: bytes
    values: dict[str, int]
    persist_present: bool


def flip_turn_case(version: int, profile: str, count: int, pattern: int) -> OwnedComposite:
    """Construct all four signed words per record independently; e.g. flip_turn_case(0, 'reader', 3, 1)."""
    patterns = (-2147483648, 2147483647, -1, 0)
    records = [patterns[(ordinal * 4 + index + pattern) % 4] for ordinal in range(count) for index in range(4)]
    raw = struct.pack('<i', count) + b''.join(struct.pack('<i', word) for word in records)
    return OwnedComposite(kind='CompositeFlipTurn', wire_profile=profile, version=version,
                          raw=raw, values=dict(count=count), persist_present=False)


def composite_extra_case(version: int, profile: str, selector: int, mode: int, persist: bytes | None) -> OwnedComposite:
    """Keep reader consumption and the unconditional writer prefix separate; e.g. composite_extra_case(0, 'writer', 0, 4, None)."""
    values = dict(member37c=-2147483648, member3f0=2147483647, member3f4=selector)
    raw = struct.pack('<3i', *values.values())
    extended = selector == 1 or profile == 'writer'
    present = extended and mode == 4 and persist is not None
    if extended:
        values['member3f8'] = mode
        raw += struct.pack('<i6Q', mode, *DOUBLE_BITS)
    if present:
        assert persist is not None
        raw += persist
        values['persistid.byte_count'] = struct.unpack_from('<i', persist, 8)[0]
    assert not (extended and mode == 4 and profile == 'reader' and not present)
    return OwnedComposite(kind='CompositeExtra', wire_profile=profile, version=version,
                          raw=raw, values=values, persist_present=present)


def extra_mode_cases(version: int, profile: str, selector: int, persists: list[bytes]) -> list[OwnedComposite]:
    """Treat tail presence as explicit synthetic framing, not a native pointer assertion; e.g. extra_mode_cases(0, 'writer', 1, persists)."""
    modes = (-1, 0, 3, 4) if selector == 1 or profile == 'writer' else (0,)
    cases: list[OwnedComposite] = []
    for mode in modes:
        tails: list[bytes | None] = [None]
        if mode == 4: tails = list(persists) + ([None] if profile == 'writer' else [])
        cases.extend(composite_extra_case(version, profile, selector, mode, tail) for tail in tails)
    return cases


def owned_composite_cases() -> list[OwnedComposite]:
    """Cross exact archive-gate neighbors with every selector and mode branch; e.g. owned_composite_cases()."""
    cases = [flip_turn_case(version, profile, count, pattern) for version in VERSIONS
             for profile, count, pattern in product(('reader', 'writer'), (0, 1, 3), (0, 1))]
    persist_cases = owned_component_persist_ids()
    for version in VERSIONS:
        persists = [raw for _, candidate, raw in persist_cases if candidate == version]
        assert len(persists) == 4
        for profile, selector in product(('reader', 'writer'), (-1, 0, 1, 2)):
            cases.extend(extra_mode_cases(version, profile, selector, persists))
    return cases
