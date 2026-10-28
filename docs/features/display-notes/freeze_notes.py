"""Observe complete note/text graphs through unchanged scoped readers (#54)."""
from pathlib import Path
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','component-detail-references','display-notes'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from object_fixture_observer import object_cursor
from note_fixture_bytes import note_graph, VERSIONS


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check real graph identities and full boundaries; e.g. verify(source, raw, 19000, 0, 3)."""
    cursor=object_cursor(reference,raw,version,None)
    root=cursor.object()
    assert cursor.p==len(raw)
    if kind==0 and style%2:
        component=root['component_reference']
        assert component['detail_e8'] is component['detail_f8']
        assert component['detail_e8']['text_90']['display_item_170'] is root
    if kind==6:
        assert root['raw_40']==0xd800 and root['raw_10']==32768


def fixture_family() -> ObjectFixtureFamily:
    """Cross each exact historical tail with typed graphs; e.g. fixture_family()."""
    return ObjectFixtureFamily(build=note_graph,verify=verify,magic=b'NTE1',kinds=8,
                               versions=VERSIONS,edited_kinds=tuple(range(7)))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/display-notes',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
