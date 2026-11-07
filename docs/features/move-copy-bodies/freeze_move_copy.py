"""Observe all supported move/copy branches through the actual independent reader (#54)."""
from pathlib import Path
from functools import partial
import json
import sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','mirror-solids','move-copy-bodies'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from freeze_blank_part import source_module
from move_copy_bytes import move_copy_object,VERSIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Use the complete actual dimension scope; e.g. configure(cursor,source)."""
    source_module('displaydimensions',reference).install_scoped(cursor)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact boundaries and shared array identity; e.g. verify(source,raw,19000,1,2)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw)
    if kind==2 or style in (0,4):return
    payload=root['data_9c8'] if kind==0 else root
    assert payload['reference_3d8'] is payload['reference_3e0'] is payload['reference_478']
    assert payload['array_370']['items'][0] is payload['array_370']['items'][1]
    assert payload['array_370']['items'][2] is payload


def fixture_family() -> ObjectFixtureFamily:
    """Bind all three complete tagged bodies and owned edit goldens; e.g. fixture_family()."""
    return ObjectFixtureFamily(build=move_copy_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'MCB1',kinds=3,versions=VERSIONS,edited_kinds=(0,2))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/move-copy-bodies',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
