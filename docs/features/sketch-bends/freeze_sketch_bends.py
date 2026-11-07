"""Observe inherited sketch bends and retained tagged handles (#54)."""
from pathlib import Path
from functools import partial
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','base-flanges','edge-flanges','sketch-bends'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_edge_flanges import configure as configure_edges
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from sketch_bend_bytes import sketch_bend_object,VERSIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Use the actual inherited and sketch-bend source readers locally."""
    configure_edges(cursor,reference)
    cursor.readers['moSketchBend_c']=source_module('sketchbends',reference).read


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact boundaries and duplicate line-handle identity."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw)
    assert root['one_bend']['raw_ac8']==-1
    if style not in (0,4):
        assert root['payload']['handle_e18'] is root['payload']['handle_e40']
        if kind==2:assert root['one_bend']['reference_860'] is root['payload']['curve_de0']


def fixture_family() -> ObjectFixtureFamily:
    """Bind complete ordinary sketch bends and one independent word-edit golden."""
    return ObjectFixtureFamily(build=sketch_bend_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'SKB1',kinds=3,versions=VERSIONS,edited_kinds=(0,))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/sketch-bends',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
