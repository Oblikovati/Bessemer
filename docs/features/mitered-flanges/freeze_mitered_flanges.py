"""Observe complete mitered flanges with the actual scoped source reader (#54)."""
from pathlib import Path
from functools import partial
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','base-flanges','edge-flanges','reference-curves','plane-point-roles','structural-members','mitered-flanges'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_edge_flanges import configure as configure_edges
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from mitered_flange_bytes import mitered_object,VERSIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Retain the actual dimension, bend and mitered readers on a local cursor."""
    configure_edges(cursor,reference)
    cursor.readers['moSMMiteredFlange_c']=source_module('miteredflanges',reference).read


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require complete boundaries and one sketch occurrence per populated handle array."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw)
    for offset in ('9f0','a10','a30','a50'):
        group=root['payload']['handles_'+offset]
        assert len(group['handles'])==group['count']
        assert ('sketch' in group)==(group['count']>0)


def fixture_family() -> ObjectFixtureFamily:
    """Bind complete inherited records and independent trailing-word edits."""
    return ObjectFixtureFamily(build=mitered_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'MTF1',kinds=1,versions=VERSIONS,edited_kinds=(0,))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/mitered-flanges',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
