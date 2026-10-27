"""Independent parsed identity and direct-member checks for surface operations (#54)."""
from pathlib import Path
from typing import cast
from weld_corner_identity import verified_pair
from surface_operation_bytes import COUNTS


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require shared nested targets while keeping chooser owners distinct; e.g. verify_identity(source, wire, 19000, 1, 2)."""
    if style not in (2,5):
        return
    first,second = verified_pair(reference, raw, version)
    chooser = 'chooser_838' if kind == 0 else 'chooser_808'
    for root in (first,second):
        if cast(dict[str, object], root[chooser])['owner'] is not root:
            raise ValueError(f'{chooser}; expected the complete containing feature as owner')
    if kind == 0:
        cut_pair(first,second,style);return
    left,right = (cast(dict[str, object], row['trim_data_800']) for row in (first,second))
    if left is right:
        raise ValueError('trim data; expected independent direct member dictionaries')
    trim_pair(left,right,style)


def trim_pair(left: dict[str, object], right: dict[str, object], style: int) -> None:
    """Keep shared complete trim targets and independent direct-member extents; e.g. trim_pair(left, right, 2)."""
    for label in ('profile_18','direction_40','solid_68'):
        if left[label] is not right[label]:
            raise ValueError(f'trim {label}; expected shared complete target')
    counts(left, ('array_20','array_48','entity_keeper_78','array_a0'), (COUNTS[style][0],COUNTS[style][1],COUNTS[style][3],COUNTS[style][4]))


def cut_pair(first: dict[str, object], second: dict[str, object], style: int) -> None:
    """Keep atom self-pointers independent of shared face/surface/plane targets; e.g. cut_pair(first, second, 2)."""
    for root in (first,second):
        if root['atom_feature_808'] is not root:
            raise ValueError('surface-cut atom; expected the complete containing feature')
    for label in ('face_820','surface_810','plane_900'):
        if first[label] is not second[label]:
            raise ValueError(f'cut {label}; expected shared complete target')
    counts(first, ('array_8c0','array_8e0'), COUNTS[style][:2])


def counts(root: dict[str, object], labels: tuple[str, ...], expected: tuple[int, ...]) -> None:
    """Check independent direct array extents without a constructed +80 collection; e.g. counts(root, ('array_20',), (1,))."""
    actual = tuple(len(cast(list[object], cast(dict[str, object], root[label])['items'])) for label in labels)
    if actual != expected:
        raise ValueError(f'direct array counts {actual}; expected {expected}')
