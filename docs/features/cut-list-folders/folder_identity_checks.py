"""Validate actual reference targets in addition to complete byte partitions (#54)."""
from pathlib import Path
from typing import cast
from object_fixture_observer import object_cursor


def shared_body(kind: int, root: dict[str, object]) -> dict[str, object]:
    """Select the documented inherited layer without flattening folder ancestry; e.g. shared_body(5, root)."""
    if kind == 5:
        root = cast(dict[str, object], root['sub_weld'])
    return cast(dict[str, object], root['body_folder']) if kind >= 3 else root


def verify_aliases(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require source-parsed target identity, not just matching map counts; e.g. verify_aliases(source, wire, 19000, 8, 2)."""
    if style not in (2, 5):
        return
    root = cast(dict[str, object], object_cursor(reference, raw, version, None).object())
    first, second, repeated = cast(list[dict[str, object]], root['items'])
    if first is not repeated or first is second:
        raise ValueError('folder aliases; expected distinct first/second and repeated first')
    if kind >= 6:
        first_target, second_target = first['unknown_380'], second['unknown_380']
    else:
        first_target, second_target = (shared_body(kind, row)['cut_list_sort_options'] for row in (first, second))
    expected = 'moCompFeature_c' if kind >= 6 else 'moCutListSortOptions_c'
    if first_target is not second_target or cast(dict[str, object], first_target)['_class'] != expected:
        raise ValueError(f'folder target identity/class; expected shared {expected}')
