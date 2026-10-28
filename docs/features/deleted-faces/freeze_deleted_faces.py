"""Freeze native-bound deleted-face features against actual scoped source readers (#54)."""
from pathlib import Path
from functools import partial
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','reference-axes','reference-axis-owners','projected-line-subtypes','deleted-faces'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor, object_cursor
from deleted_face_bytes import VERSIONS, deleted_face_object


def source_surface(cursor: ReferenceObjectCursor, schema: int=1, *, reference: Path) -> object:
    """Adapt the actual one-argument base-surface reader; e.g. source_surface(cursor,reference=source)."""
    return source_module('modelreferences',reference).surface(cursor)


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Keep the dependency's complete native-bound reader local to this cursor; e.g. configure(cursor,source)."""
    cursor.readers['moSurfaceRef_w']=partial(source_surface,reference=reference)


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Verify actual repeated roots and shared data independently of byte coverage; e.g. verify_identity(source,raw,19000,0,2)."""
    root=cast(dict[str,object],object_cursor(reference,raw,version,partial(configure,reference=reference)).object())
    if style not in (2,5):return
    first,second,repeated=cast(list[dict[str,object]],root['items'])
    if first is second or repeated is not first:raise ValueError('deleted-face roots; expected distinct equal roots and repeated first')
    if kind==0:
        left=cast(dict[str,object],first['payload']);right=cast(dict[str,object],second['payload'])
        if left['array_800']['items'][0] is not right['array_800']['items'][0]:raise ValueError('nested deleted-face data; expected actual shared target')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Bind all scoped adapters to one explicit source root; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=deleted_face_object,verify=verify_identity,configure=partial(configure,reference=reference),
        magic=b'DEL1',kinds=2,versions=VERSIONS,edited_kinds=(0,1))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/deleted-faces',fixture_family(REFERENCE))
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
