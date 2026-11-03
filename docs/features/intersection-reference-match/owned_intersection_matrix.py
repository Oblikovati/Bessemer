"""Bounded Cartesian signed/state cases with contrasting physical reference groups (#184)."""
from collections.abc import Iterator
from itertools import product
from owned_intersection_match import ReferenceMatchWords

MATCH_SIGNED_CONTROLS: tuple[int, ...] = (-2**31, -2**31 + 1, -2, -1, 0, 1, 2**31 - 2, 2**31 - 1)
MATCH_STATE_CONTROLS: tuple[int, ...] = (0, 1, 2, 3, 4, 5, 65535)
MATCH_OPAQUE_ORDINALS: tuple[int, ...] = (-2**31, -2, 2**31 - 1)


def intersection_match_matrix_inputs() -> Iterator[tuple[int, int, int, int]]:
    """Cover each declared Cartesian input once; e.g. list(intersection_match_matrix_inputs())."""
    assert len(MATCH_SIGNED_CONTROLS) == 8
    assert len(MATCH_STATE_CONTROLS) == 7 and len(MATCH_OPAQUE_ORDINALS) == 3
    return iter(product(MATCH_SIGNED_CONTROLS, MATCH_SIGNED_CONTROLS, MATCH_STATE_CONTROLS, MATCH_OPAQUE_ORDINALS))


def intersection_match_matrix_groups(span: int, backward: int, state: int, ordinal: int) -> tuple[tuple[ReferenceMatchWords, ...], tuple[ReferenceMatchWords, ...]]:
    """Distinguish every group and owner without narrowing signed values; e.g. intersection_match_matrix_groups(1, 0, 3, -2)."""
    pairs = ((span, backward), (backward, span), (~span, ~backward), (~backward, ~span))
    state_masks = (0, 1, 3, 65535)
    first = tuple((state ^ mask, ordinal, left, right) for mask, (left, right) in zip(state_masks, pairs, strict=True))
    second = tuple((group_state ^ 65535, ~opaque, right, left) for group_state, opaque, left, right in first)
    return first, second
