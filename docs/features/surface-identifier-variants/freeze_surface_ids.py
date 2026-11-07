"""Observe complete surface identifier variants with actual approved readers (#54)."""
from pathlib import Path
from functools import partial
from typing import cast
import json
import sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','surface-identifier-variants'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from surface_id_bytes import surface_identifier,VERSIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Keep real one-word and two-word serializers in one local reader map; e.g. configure(cursor,source)."""
    boundary=source_module('modelboundary',reference)
    cursor.readers.update(moFromSktTopSurfIdRep_c=boundary.from_sketch_entity,moEndCapChamferSurfIdRep_c=boundary.end_face)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check actual aliases and shared inherited references; e.g. verify(source,raw,19000,1,5)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'surface identifier end {cursor.p}; expected {len(raw)}')
    if style not in (2,3,5):return
    items=cast(list[dict[str,object]],root['items'])
    if style==3:
        if items[1]['representation'] is not items[0]:raise ValueError('surface identifier seed shares=False; expected shares=True')
        return
    first,second,repeated=items
    if first is second or first is not repeated:raise ValueError(f'surface identifier roots {(first is second,first is repeated)}; expected (False, True)')
    if style==5:first,second=(row['representation'] for row in (first,second))
    for key in ('feature','external_feature_reference'):
        if first[key] is not second[key]:raise ValueError(f'surface identifier {key} shares=False; expected shares=True')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Keep both exact variants across inherited version gates and pointer styles; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=surface_identifier,verify=verify,configure=partial(configure,reference=reference),
        magic=b'SIV1',kinds=2,versions=VERSIONS,edited_kinds=(0,1))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/surface-identifier-variants',fixture_family(REFERENCE))
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
