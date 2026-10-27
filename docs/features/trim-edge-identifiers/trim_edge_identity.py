"""Independent alias checks for complete left/right trim-edge identifiers (#54)."""
from pathlib import Path
from typing import cast
from object_fixture_observer import object_cursor


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int,
                    source_member: bool = True) -> None:
    """Require distinct representations and repeated aliases through direct and tagged entity IDs; e.g. verify_identity(source, wire, 19000, 0, 5)."""
    if style not in (2,5):
        return
    root=cast(dict[str,object],object_cursor(reference,raw,version,None).object())
    first,second,repeated=cast(list[dict[str,object]],root['items'])
    if style==5:
        first,second=(cast(dict[str,object],row['representation']) for row in (first,second))
    if first is second or repeated is not first:
        raise ValueError('trim-edge aliases; expected distinct equal representations and repeated first')
    for name in ('feature','external_feature_reference'):
        if first[name] is not second[name]:
            raise ValueError(f'trim-edge {name}; expected shared complete target')
    if not source_member:
        return
    left,right=(cast(dict[str,object],row['source_entity'])['representation'] for row in (first,second))
    if left is not right:
        raise ValueError('direct source entity; expected one shared source representation')
