"""Observe complete punch revisions through the actual independent source reader (#54)."""
from pathlib import Path
from functools import partial
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','base-flanges','edge-flanges','drafts','punches'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_edge_flanges import configure as configure_edges
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from punch_bytes import punch_object,VERSIONS,REVISIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Bind the actual punch reader and inherited dimension dependencies locally."""
    configure_edges(cursor,reference)
    cursor.readers['moPunch_c']=source_module('punches',reference).read


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact boundaries and revision-controlled tagged-array presence."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw)
    assert root['payload']['revision_858']==REVISIONS[kind]
    assert ('tagged_array_920' in root['payload'])==(REVISIONS[kind]>=7)


def fixture_family() -> ObjectFixtureFamily:
    """Bind ten complete revisions with independent terminal-tag edit goldens."""
    return ObjectFixtureFamily(build=punch_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'PCH1',kinds=10,versions=VERSIONS,edited_kinds=(0,3,9))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/punches',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
