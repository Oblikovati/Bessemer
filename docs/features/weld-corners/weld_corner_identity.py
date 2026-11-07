"""Check source-parsed shared targets independently of primitive spans (#54)."""
from pathlib import Path
from typing import cast
from object_fixture_observer import object_cursor


def verified_pair(reference: Path, raw: bytes, version: int) -> tuple[dict[str, object], dict[str, object]]:
    """Require distinct records and a repeated first pointer; e.g. verified_pair(source, wire, 19000)."""
    root = cast(dict[str, object], object_cursor(reference, raw, version, None).object())
    first, second, repeated = cast(list[dict[str, object]], root['items'])
    if first is second or first is not repeated:
        raise ValueError('weld-corner aliases; expected distinct first/second and repeated first')
    return first, second


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check complete target identities for alias graphs; e.g. verify_identity(source, wire, 19000, 0, 2)."""
    if style not in (2, 5):
        return
    first, second = verified_pair(reference, raw, version)
    if kind != 0:
        left, right = (cast(dict[str, object], row['array_108'])['items'] for row in (first, second))
        if cast(list[object], left)[0] is not cast(list[object], right)[0]:
            raise ValueError('weld data target; expected shared complete weldment')
        return
    left, right = (cast(dict[str, object], row['payload']) for row in (first, second))
    for name in ('reference_8c0', 'point_reference_860'):
        if left[name] is not right[name]:
            raise ValueError(f'weld corner {name}; expected shared complete reference')
