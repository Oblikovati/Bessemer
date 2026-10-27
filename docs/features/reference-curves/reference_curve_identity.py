"""Independent cached-array, source and attribute identity checks (#54)."""
from pathlib import Path
from typing import cast
from object_fixture_observer import object_cursor,ReferenceObjectCursor
from freeze_blank_part import source_module


def install_reader(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Select the same guarded native reader used by read_supported in this cursor only; e.g. install_reader(cursor, source)."""
    cursor.readers=dict(cursor.readers)
    cursor.readers['moRefCurve_c']=source_module('referencecurves',reference).read


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require the preloaded array to drive both equal features without cloning targets; e.g. verify_identity(source, wire, 19000, 0, 2)."""
    if style not in (2,4,5):return
    root=cast(dict[str,object],object_cursor(reference,raw,version,lambda cursor:install_reader(cursor,reference)).object())
    array,first,second,repeated=cast(list[dict[str,object]],root['items'])
    if first is second or repeated is not first:
        raise ValueError('reference-curve aliases; expected distinct equal features and repeated first')
    left,right=(cast(dict[str,object],row['payload']) for row in (first,second))
    if left['references_860'] is not array or right['references_860'] is not array:
        raise ValueError('reference arrays; expected the same previously decoded array')
    if bool(array['items'])!=(style==2):
        raise ValueError('reference array shape; expected nonempty style 2 and empty style 5')
    if ('objects_8c8' in left)!=(style==2) or ('dwords_8a8' in left)!=(style==5):
        raise ValueError('reference branch; expected populated-array or signed flag-bit selection')
