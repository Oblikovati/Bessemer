"""Observe complete bodies with the actual independently maintained readers (#54)."""
from pathlib import Path
from functools import partial
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','base-flanges','edge-flanges','stock-features','body-extensions'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from extension_bytes import extension_object,VERSIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install the actual three class readers in this archive only."""
    source_module('bodyextensions',reference).install(cursor)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require complete bodies, exact EOF and shared roots and reference targets."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw)
    if kind==2 or style not in (2,5):return
    first,second,repeated=root['items']
    assert first is repeated and first is not second
    if kind==0:
        assert first['payload']['face_838'] is second['payload']['face_838']
        assert first['payload']['coordinate_848'] is second['payload']['coordinate_848']


def fixture_family() -> ObjectFixtureFamily:
    """Cross all three complete class bindings and inherited version boundaries."""
    return ObjectFixtureFamily(build=extension_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'BEX1',kinds=3,versions=VERSIONS,edited_kinds=(0,1,2))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/body-extensions',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
