"""Independent parsed identity checks for complete mirrored-solid graphs (#54)."""
from pathlib import Path
from typing import cast
from weld_corner_identity import verified_pair


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require distinct equal roots and shared complete nested targets; e.g. verify_identity(source, wire, 19000, 0, 2)."""
    if style not in (2,5):
        return
    first, second = verified_pair(reference, raw, version)
    if kind == 2:
        if first['items'] != second['items']:
            raise ValueError('unsigned arrays; expected equal words in distinct objects')
        return
    if kind == 1:
        verify_matrix(first, second)
        return
    left, right = (cast(dict[str, object], row['payload']) for row in (first, second))
    for name in ('face_800', 'surface_9a0', 'matrix_helper_9c8'):
        if left[name] is not right[name]:
            raise ValueError(f'mirror-solid {name}; expected shared complete target')
    verify_matrix(cast(dict[str, object], left['matrix_helper_9c8']), cast(dict[str, object], right['matrix_helper_9c8']))


def verify_matrix(first: dict[str, object], second: dict[str, object]) -> None:
    """Check populated direct arrays share unsigned objects across member boundaries; e.g. verify_matrix(first, second)."""
    entries = [entry for row in (first,second) for name in ('array_10','array_30')
        for entry in cast(list[object], cast(dict[str, object], row[name])['items']) if entry is not None]
    if not entries or any(entry is not entries[0] for entry in entries):
        raise ValueError('matrix targets; expected one shared complete unsigned array')
