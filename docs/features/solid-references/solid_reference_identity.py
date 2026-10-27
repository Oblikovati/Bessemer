"""Check actual shared component and attribute targets in reference-parsed graphs (#54)."""
from pathlib import Path
from typing import cast
from object_fixture_observer import object_cursor


def verify_identity(reference: Path, raw: bytes, version: int, style: int, presence: int) -> None:
    """Require distinct solid records and repeated/shared object identity; e.g. verify_identity(source, wire, 19000, 2, -1)."""
    if style != 2:
        return
    root = cast(dict[str, object], object_cursor(reference, raw, version, None).object())
    first, second, repeated = cast(list[dict[str, object]], root['items'])
    if first is second or first is not repeated:
        raise ValueError('solid reference identities; expected distinct first/second and repeated first')
    if first['component_reference'] is not second['component_reference']:
        raise ValueError('solid component; expected shared complete component body')
    if not presence:
        return
    left, right = (cast(dict[str, object], row['body_attributes'])['items'] for row in (first, second))
    if cast(list[object], left)[0] is not cast(list[object], right)[0]:
        raise ValueError('solid body attribute; expected a shared complete attribute')
