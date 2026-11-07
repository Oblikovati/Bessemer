"""Check independent complete offset-data, surface and keeper identities (#54)."""
from pathlib import Path
from typing import cast
from object_fixture_observer import object_cursor


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Keep distinct equal roots and shared dependencies without direct-array identities; e.g. verify_identity(source, wire, 19000, 0, 2)."""
    if style not in (2,5):return
    root=cast(dict[str,object],object_cursor(reference,raw,version,None).object())
    items=cast(list[dict[str,object]],root['items'])
    first,second,repeated=items[1:] if kind!=2 else items
    if first is second or repeated is not first:
        raise ValueError('offset aliases; expected distinct equal roots and repeated first')
    if kind==2:return
    if kind==0:
        if first['offset_data_7c8'] is not second['offset_data_7c8']:
            raise ValueError('offset data; expected shared complete target')
        return
    if first['dimension_10'] is not second['dimension_10']:
        raise ValueError('offset dimension; expected shared complete target')
    arrays=[cast(dict[str,object],cast(dict[str,object],row['array_08'])['array']) for row in (first,second)]
    left,right=(cast(list[object],row['items']) for row in arrays)
    if left[0] is not right[0] or left[0] is not left[-1]:
        raise ValueError('offset faces; expected shared first/last complete target')
