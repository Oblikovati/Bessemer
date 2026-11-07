"""Observe mid-surface bytes through unchanged independent source serializers (#54)."""
from pathlib import Path
from functools import partial
import json,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','reference-axes','reference-axis-owners','base-flanges','edge-flanges','revolved-surfaces','mid-surfaces'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_edge_flanges import configure as configure_edges
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from mid_surface_bytes import mid_surface_object,VERSIONS,CLASSES


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Bind actual native-layout readers locally, including complete dimensions."""
    configure_edges(cursor,reference)
    mid=source_module('midsurfaces',reference);boundary=source_module('modelboundary',reference)
    cursor.readers.update(zip(CLASSES,(mid.read,mid.base_reference,mid.reference,mid.data,mid.face_pair,mid.face_edges,mid.edge_pair,boundary.derived_surface)))


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact boundaries and the base-only external sheet branch."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw)
    if kind==1:
        assert ('external_sheet_count' in root['surface'])==(style not in (0,3))
    if kind==4:assert root['face_08'] is root['face_10']
    if kind==6:assert root['edge_08'] is root['edge_10']


def fixture_family() -> ObjectFixtureFamily:
    """Cross the inherited gates and retain independent signed-word edit goldens."""
    return ObjectFixtureFamily(build=mid_surface_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'MSF1',kinds=8,versions=VERSIONS,edited_kinds=(2,3,7))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/mid-surfaces',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
