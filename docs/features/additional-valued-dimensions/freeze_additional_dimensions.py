"""Freeze exact native serializer reuse against real complete dimension/sketch readers (#54)."""
from functools import partial
from pathlib import Path
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','additional-valued-dimensions'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor, object_cursor
from additional_dimension_bytes import CLASSES, VERSIONS, GROUPS, additional_dimension_object


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Use only actual complete readers and the local additional-class installer; e.g. configure(cursor,source)."""
    source_module('sketchprefix',reference).install_scoped_readers(cursor)
    source_module('sketchdimensions',reference).install_additional_valued(cursor)
    cursor.readers['sgSketch']=source_module('assemblyprofiles',reference).sketch


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check actual repeated dimensions and collection ownership; e.g. verify_identity(source,raw,19000,1,5)."""
    root=cast(dict[str,object],object_cursor(reference,raw,version,partial(configure,reference=reference)).object())
    if style in (0,1):
        if root['_class']!=CLASSES[kind]:raise ValueError('standalone dimension; expected exact native derived class')
        return
    if style==2:
        first,second,repeated=root['items']
        if first is second or first is not repeated:raise ValueError('dimension roots; expected distinct equal objects and repeated first')
        return
    selected=root if style in (3,4) else root['items'][1]
    group=('valued_g1','valued_g2','valued_g3')[GROUPS[kind]];first,second,repeated=selected[group]
    if first['_class']!=CLASSES[kind] or first is not repeated:raise ValueError('dimension collection; expected exact derived class and repeated first')
    if style==5 and (first is not root['items'][0] or first is not second):raise ValueError('cached collection; expected actual seeded dimension')
    if style in (3,4) and first is second:raise ValueError('dimension collection; expected distinct equal first/second objects')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Bind all three serializers and complete inherited version boundaries; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=additional_dimension_object,verify=verify_identity,configure=partial(configure,reference=reference),
        magic=b'AVD1',kinds=3,versions=VERSIONS,edited_kinds=(0,1,2))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/additional-valued-dimensions',fixture_family(REFERENCE))
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
