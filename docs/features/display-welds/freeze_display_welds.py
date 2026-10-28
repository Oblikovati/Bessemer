"""Observe complete display welds through real inherited reference readers (#54)."""
from pathlib import Path
from functools import partial
from typing import cast
import json
import sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','display-welds'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from display_weld_bytes import display_weld_object,VERSIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install actual display-item dependencies and the exact derived serializer; e.g. configure(cursor,source)."""
    source_module('displaydimensions',reference).install_scoped(cursor)
    cursor.readers['moDisplayWeld_c']=source_module('displayweld',reference).read


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Keep self, seeded, null and repeated-root identities intact; e.g. verify(source,raw,19000,1,5)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'display weld end {cursor.p}; expected {len(raw)}')
    if style in (0,4) and root['payload']['object_5c0'] is not None:raise ValueError('display weld payload is_null=False; expected True')
    if style==1 and root['payload']['object_5c0'] is not root:raise ValueError('display weld self identity matches=False; expected True')
    if style in (0,1,4):return
    items=cast(list[dict[str,object]],root['items']);first=items[0] if style==2 else items[1]
    if style==3:
        if first['payload']['object_5c0'] is not items[0]:raise ValueError('display weld seed identity matches=False; expected True')
        return
    second,repeated=items[1:] if style==2 else items[2:]
    if first is second or first is not repeated:raise ValueError(f'display weld roots {(first is second,first is repeated)}; expected (False,True)')
    if first['payload']['object_5c0'] is not second['payload']['object_5c0']:raise ValueError('display weld shared payload identity matches=False; expected True')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Cross raw-value and string encodings with inherited item gates; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=display_weld_object,verify=verify,configure=partial(configure,reference=reference),
        magic=b'DWL1',kinds=3,versions=VERSIONS,edited_kinds=(0,1,2))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/display-welds',fixture_family(REFERENCE))
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
