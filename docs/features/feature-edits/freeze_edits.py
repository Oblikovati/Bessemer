"""Observe complete records with the actual independent feature-edit readers (#54)."""
from pathlib import Path
from functools import partial
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','base-flanges','edge-flanges','mirror-solids','stock-features','feature-edits'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_blank_part import source_module
from freeze_edge_flanges import configure as configure_edges
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from edit_bytes import edit_object,VERSIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install actual readers locally while preserving the inherited registrations."""
    configure_edges(cursor,reference)
    source_module('bodyextensions',reference).install(cursor)
    source_module('featureedits',reference).install(cursor)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require EOF and actual repeated roots and shared bending data."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw)
    if style not in (2,5):return
    first,second,repeated=root['items']
    assert first is repeated
    if kind==2:
        assert first is second
    else:
        assert first is not second
        if kind==1:assert first['data_810'] is second['data_810']


def fixture_family() -> ObjectFixtureFamily:
    """Cover complete bodies and each independent raw-word edit golden."""
    return ObjectFixtureFamily(build=edit_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'FED1',kinds=3,versions=VERSIONS,edited_kinds=(0,1,2))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/feature-edits',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
