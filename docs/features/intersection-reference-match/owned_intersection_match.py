"""Literal signed reference controls inside complete ordinary sketch owners (#184)."""
import struct
from typing import TypeAlias
from sketch_tail_fixture_bytes import WireMap, intersection, ordinary

ReferenceMatchWords: TypeAlias = tuple[int, int, int, int]


def reference_match_literal(words: ReferenceMatchWords) -> bytes:
    """Keep the state and all three raw signed words; e.g. reference_match_literal((3, -2, 1, 0))."""
    if len(words) != 4 or not 0 <= words[0] <= 65535 or any(not -2**31 <= value < 2**31 for value in words[1:]):
        raise ValueError(f'reference words {words!r}; expected u16 state and three signed32 words')
    return struct.pack('<H3i', *words)


def intersection_match_body(groups: tuple[ReferenceMatchWords, ...]) -> bytes:
    """Replace only literal reference prefixes; e.g. intersection_match_body(((3, -2, 1, 0),) * 4)."""
    if len(groups) != 4:
        raise ValueError(f'reference groups {groups!r}; expected exactly four physical groups')
    original = intersection(WireMap(), 19000, False)
    result = bytearray(original)
    for index, words in enumerate(groups):
        start = 52 + 20 * index
        assert original[start:start + 20] == struct.pack('<H3i3H', 65535, -2**31, 2**31 - 1, -1, 0, 0, 0)
        result[start:start + 14] = reference_match_literal(words)
    return bytes(result)


def intersection_match_sketch(groups: tuple[ReferenceMatchWords, ...]) -> tuple[bytes, list[int]]:
    """Use the ordinary complete sketch constructor; e.g. intersection_match_sketch(((3, -2, 1, 0),) * 4)."""
    fragment = intersection_match_body(groups)
    owner = ordinary(WireMap(), 19000, False, intersection_body=lambda: fragment)
    start = owner.find(fragment)
    assert start >= 0 and start == owner.rfind(fragment), 'Expected one complete intersection fragment in its sketch owner'
    return owner, [start + 52 + 20 * index for index in range(4)]


def intersection_match_stream(first: tuple[ReferenceMatchWords, ...], second: tuple[ReferenceMatchWords, ...]) -> tuple[bytes, list[int]]:
    """Retain new/loaded definitions and a first-instance alias; e.g. intersection_match_stream(words, words)."""
    name = b'sgSketch'
    tag = struct.pack('<3H', 65535, 1, len(name)) + name
    header = struct.pack('<iH', -71, 3)
    left, first_starts = intersection_match_sketch(first)
    right, second_starts = intersection_match_sketch(second)
    first_base = len(header) + len(tag)
    second_base = first_base + len(left) + 2
    starts = [first_base + start for start in first_starts] + [second_base + start for start in second_starts]
    return header + tag + left + struct.pack('<H', 0x8061) + right + struct.pack('<HI', 98, 0), starts
