"""Complete literal sketch owners for stored index and selector roles (#185)."""
from collections.abc import Iterator
from dataclasses import dataclass
from itertools import product
from pathlib import Path
import hashlib
import struct
from owned_intersection_match import ReferenceMatchWords, reference_match_literal
from sketch_tail_fixture_bytes import WireMap, intersection

ROLE_SIGNED_VALUES: tuple[int, ...] = (-2**31, -2**31 + 1, -2, -1, 0, 1, 2**31 - 2, 2**31 - 1)
ROLE_SELECTORS: tuple[int, ...] = (0, 1, 2, 3, 4, 5, 0x55aa, 0x7fff, 0x8000, 0xfffe, 0xffff)
ROLE_ORDINALS: tuple[int, ...] = (-2**31, -2, 2**31 - 1)
ROLE_GROUP_STARTS: tuple[int, ...] = (440, 460, 480, 500, 1187, 1207, 1227, 1247)
ROLE_INDEX_STARTS: tuple[int, ...] = (436, 1183)


@dataclass(frozen=True)
class OwnedIntersectionRoles:
    raw: bytes
    indices: tuple[tuple[int, int], ...]
    references: tuple[tuple[int, ReferenceMatchWords], ...]
    owner_edge_counts: tuple[int, int]


def role_original_control() -> bytes:
    """Require the independently accepted complete control; e.g. role_original_control()."""
    path = Path(__file__).resolve().parents[3] / 'tests/fixtures/intersection-reference-match/controls.bin'
    raw = path.read_bytes()
    expected = 'e75d60beec167841c83bb9054c026720f564a02782ddc21a9a50112560920100'
    assert hashlib.sha256(raw).hexdigest() == expected and len(raw) == 1518
    return raw


def role_matrix_inputs() -> Iterator[tuple[int, int, int]]:
    """Keep all signed/selector/opaque combinations; e.g. list(role_matrix_inputs())."""
    return iter(product(ROLE_SIGNED_VALUES, ROLE_SELECTORS, ROLE_ORDINALS))


def role_reference_words(selector: int, ordinal: int, group: int) -> ReferenceMatchWords:
    """Distinguish physical groups without validity assumptions; e.g. role_reference_words(3, -2, 0)."""
    masks = (0, 1, 3, 65535, 32768, 32769, 32767, 65534)
    if not 0 <= group < 8:
        raise ValueError(f'group {group!r}; expected physical group index in0..8')
    return (selector ^ masks[group], ordinal if group < 4 else ~ordinal,
            ROLE_SIGNED_VALUES[group], ROLE_SIGNED_VALUES[7 - group])


def role_matrix_root(index: int, selector: int, ordinal: int) -> OwnedIntersectionRoles:
    """Patch a complete root with literal role values; e.g. role_matrix_root(-2, 3, -2)."""
    reference_match_literal((selector, ordinal, index, ~index))
    original = role_original_control()
    raw = bytearray(original)
    indices = tuple(zip(ROLE_INDEX_STARTS, (index, ~index), strict=True))
    references = tuple((start, role_reference_words(selector, ordinal, group))
                       for group, start in enumerate(ROLE_GROUP_STARTS))
    for start, value in indices:
        struct.pack_into('<i', raw, start, value)
    for start, words in references:
        raw[start:start + 14] = reference_match_literal(words)
    return OwnedIntersectionRoles(bytes(raw), indices, references, (1, 1))


def role_domain_edge(edge: int, template: bytes) -> tuple[bytes, tuple[ReferenceMatchWords, ...]]:
    """Assign four consecutive selectors in a complete edge; e.g. role_domain_edge(0, template)."""
    words = tuple((4 * edge + group, ROLE_ORDINALS[(4 * edge + group) % 3],
                   ROLE_SIGNED_VALUES[(4 * edge + group + 3) % 8],
                   ROLE_SIGNED_VALUES[(4 * edge + group + 5) % 8]) for group in range(4))
    prefix = template[32:48] + struct.pack('<i', ROLE_SIGNED_VALUES[edge % 8])
    return prefix + b''.join(reference_match_literal(value) + bytes(6) for value in words), words


def role_domain_body(template: bytes) -> tuple[bytes, tuple[tuple[int, int], ...], tuple[tuple[int, ReferenceMatchWords], ...]]:
    """Frame every u16 through the ordinary full/edge/group chain; e.g. role_domain_body(template)."""
    count = 16384
    prefix = template[:6] + struct.pack('<i', count) + template[10:26] + bytes(2 * count) + template[28:32]
    edges, indices, references = bytearray(), [], []
    for edge in range(count):
        start = len(prefix) + len(edges)
        raw, words = role_domain_edge(edge, template)
        assert len(raw) == 100
        indices.append((start + 16, ROLE_SIGNED_VALUES[edge % 8]))
        references.extend((start + 20 + 20 * group, values) for group, values in enumerate(words))
        edges.extend(raw)
    return prefix + edges + template[132:], tuple(indices), tuple(references)


def role_full_selector_root() -> OwnedIntersectionRoles:
    """Retain two definitions and their alias around the full domain; e.g. role_full_selector_root()."""
    original = role_original_control()
    template = intersection(WireMap(), 19000, False)
    start = ROLE_GROUP_STARTS[0] - 52
    body, indices, references = role_domain_body(template)
    raw = original[:start] + body + original[start + len(template):]
    shift = len(body) - len(template)
    indices = tuple((start + offset, value) for offset, value in indices)
    indices += ((ROLE_INDEX_STARTS[1] + shift, struct.unpack_from('<i', original, ROLE_INDEX_STARTS[1])[0]),)
    references = tuple((start + offset, words) for offset, words in references)
    references += tuple((offset + shift, struct.unpack_from('<H3i', original, offset)) for offset in ROLE_GROUP_STARTS[4:])
    return OwnedIntersectionRoles(raw, indices, references, (16384, 1))
