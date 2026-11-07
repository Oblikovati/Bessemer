"""Independent alias checks for distinct component spellings and direct surface data (#54)."""
from pathlib import Path
from typing import cast
from object_fixture_observer import object_cursor,ReferenceObjectCursor
from freeze_blank_part import source_module


def install_reader(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Select exactly the public guarded readers in a copied cursor map; e.g. install_reader(cursor, source)."""
    components=source_module('modelcomponents',reference);surfaces=source_module('referencesurfaces',reference)
    cursor.readers=dict(cursor.readers)
    cursor.readers.update(moCompRefCurve_c=components.component_feature,moBlendRefSurface_c=surfaces.blend_surface,moBlendRefSurfIdKeeper_c=surfaces.keeper)


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require distinct equal roots and shared actual native objects; e.g. verify_identity(source, wire, 19000, 1, 2)."""
    if style not in (2,5):return
    root=cast(dict[str,object],object_cursor(reference,raw,version,lambda cursor:install_reader(cursor,reference)).object())
    first,second=alias_pair(root,kind,style)
    if kind==2:return
    if kind==0:
        left,right=(cast(dict[str,object],row['component_reference'])['component'] for row in (first,second))
        if left is not right or first['_class']!='moCompRefCurve_c':
            raise ValueError('component curve; expected shared unit and exact moCompRefCurve_c spelling')
        return
    left,right=(cast(dict[str,object],cast(dict[str,object],row['payload'])['data']) for row in (first,second))
    for label in ('direction_68','direction_70'):
        if left[label] is not right[label]:
            raise ValueError(f'blend {label}; expected shared complete direction target')


def alias_pair(root: dict[str,object], kind: int, style: int) -> tuple[dict[str,object],dict[str,object]]:
    """Check the cached-wrapper graph separately from its complete targets; e.g. alias_pair(root, 0, 5)."""
    items=cast(list[dict[str,object]],root['items'])
    if kind==0 and style==5:
        first,owner,second,repeated=items
        if owner['component_reference'] is not first:
            raise ValueError('wrapper component; expected the cached actual moCompRefCurve_c')
    else:first,second,repeated=items
    if first is second or repeated is not first:
        raise ValueError('blend-reference aliases; expected distinct equal roots and repeated first')
    return first,second
