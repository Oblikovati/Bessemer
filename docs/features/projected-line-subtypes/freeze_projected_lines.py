"""Freeze C-377 bodies against explicitly scoped, actual source readers (#54, #46)."""
from pathlib import Path
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for folder in ('trim-edge-identifiers','reference-axes','reference-axis-owners','projected-line-subtypes'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/folder))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor, object_cursor
from projected_line_bytes import VERSIONS, projected_line_object


def source_surface(cursor: ReferenceObjectCursor, schema: int=1) -> object:
    """Adapt only the actual one-argument source surface reader; e.g. source_surface(cursor)."""
    return source_module('modelreferences',REFERENCE).surface(cursor)


def configure(cursor: ReferenceObjectCursor) -> None:
    """Install exact complete readers on one cursor, retaining global defaults; e.g. configure(cursor)."""
    lines=source_module('plines',REFERENCE)
    cursor.readers.update(moSurfaceRef_w=source_surface,moPLineDraft_c=lines.draft,moPLineIntersect_c=lines.intersect)
    setattr(cursor,'projected_line_subtypes',True)


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check self-owned PLine data, standalone shared owners and repeated identities; e.g. verify_identity(source,raw,19000,0,2)."""
    root=cast(dict[str,object],object_cursor(reference,raw,version,configure).object())
    roots=cast(list[dict[str,object]],root['items'])[:2] if style in (2,5) else [root]
    if style in (2,5):
        first,second,repeated=cast(list[dict[str,object]],root['items'])
        if first is second or repeated is not first:raise ValueError('projected roots; expected distinct equal roots and repeated first')
    if kind<2:
        for owner in roots:
            child=cast(dict[str,object],cast(dict[str,object],owner['payload'])['data_808'])
            if child['owner'] is not owner:raise ValueError('PLine data owner; expected actual containing feature')
    if kind in (2,3) and style in (2,5) and roots[0]['owner'] is not roots[1]['owner']:
        raise ValueError('standalone data owners; expected shared complete PLine feature')


FAMILY=ObjectFixtureFamily(build=projected_line_object,verify=verify_identity,configure=configure,magic=b'PLS1',
                          kinds=5,versions=VERSIONS,edited_kinds=(0,1,2,3,4))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/projected-line-subtypes',FAMILY)
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
