"""Verify parsed thickening targets independently from byte partitions (#54)."""
from pathlib import Path
from typing import cast
from weld_corner_identity import verified_pair
from object_fixture_observer import object_cursor


def feature_pair(reference: Path, raw: bytes, version: int) -> tuple[dict[str, object], dict[str, object]]:
    """Require the complete prelude followed by distinct roots and a repeated alias; e.g. feature_pair(source, wire, 19000)."""
    root = cast(dict[str, object], object_cursor(reference, raw, version, None).object())
    seed,first,second,repeated = cast(list[dict[str, object]], root['items'])
    if seed['_class'] != 'moRib_c' or first is second or first is not repeated:
        raise ValueError('thickening aliases; expected rib, distinct equal roots and repeated first')
    return first,second


def attribute_pair(first: dict[str, object], second: dict[str, object]) -> None:
    """Check complete direct attribute lists independently from root aliases; e.g. attribute_pair(first, second)."""
    left,right = (cast(list[object], cast(dict[str, object], row['body_attributes'])['items']) for row in (first,second))
    if left[0] is not right[0] or left[0] is not left[-1]:
        raise ValueError('constant-surface attributes; expected shared complete first/last target')


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Keep distinct equal roots, repeated aliases and complete shared targets; e.g. verify_identity(source, wire, 19000, 0, 2)."""
    if style not in (2,5):
        return
    first,second = feature_pair(reference, raw, version) if kind == 0 else verified_pair(reference, raw, version)
    if kind == 1:
        labels = ('component_reference','body_attributes')
    elif kind == 2:
        labels = ('reference_geometry',)
    else:
        labels = ('list_808','dimension_800')+ (('direction_830',) if version >= 0x2f74 else ())
    for name in labels:
        if name == 'body_attributes':
            attribute_pair(first,second);continue
        if first[name] is not second[name]:
            raise ValueError(f'thickening {name}; expected shared complete target')
