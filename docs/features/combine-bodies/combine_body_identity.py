"""Check complete combine/delete aliases independently from primitive observations (#54)."""
from pathlib import Path
from typing import cast
from weld_corner_identity import verified_pair


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require distinct equal roots and shared complete targets; e.g. verify_identity(source, wire, 19000, 0, 2)."""
    if style not in (2,5):
        return
    first, second = verified_pair(reference, raw, version)
    if kind == 1:
        if first['data_800'] is not second['data_800']:
            raise ValueError('delete data; expected shared complete payload')
        return
    if kind == 0 and first['body_800'] is not second['body_800']:
        raise ValueError('combine body; expected shared complete solid component')
    label = 'array_08' if kind == 2 else 'array_808'
    left, right = (cast(list[object], cast(dict[str, object], row[label])['items']) for row in (first,second))
    if left[0] is not right[0] or left[0] is not left[-1]:
        raise ValueError(f'{label}; expected shared first/last complete target')
