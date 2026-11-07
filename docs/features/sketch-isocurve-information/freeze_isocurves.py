"""Freeze complete isocurve fields through actual reference readers (#54)."""
from functools import partial
from pathlib import Path
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','sketch-isocurve-information'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor, object_cursor
from isocurve_bytes import RATIOS, VERSIONS, isocurve_object


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install real readers only in the local cursor; e.g. configure(cursor,source)."""
    source_module('sketchprefix',reference).install_scoped_readers(cursor)
    cursor.readers['moSkIsoCurveInfo_c']=source_module('sketchisocurve',reference).read


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check complete null, shared and distinct parent/reference graphs; e.g. verify_identity(source,raw,19000,2,5)."""
    root=cast(dict[str,object],object_cursor(reference,raw,version,partial(configure,reference=reference)).object())
    if style==4:
        if root['isocurve'] is not None:raise ValueError('external isocurve; expected preserved null')
    if style==2:
        first,second,repeated=root['items']
        if first is second or first is not repeated:raise ValueError('isocurve roots; expected distinct objects and repeated first')
    if style==3 and root['items'][1]['isocurve'] is not root['items'][0]:raise ValueError('external isocurve; expected actual seed')
    if style!=5:return
    first,parent,second,other,repeated=root['items']
    if first is second or parent is other or parent is not repeated:raise ValueError('isocurve parents; expected distinct roots and repeated first parent')
    if parent['isocurve'] is not first or other['isocurve'] is not second:raise ValueError('isocurve references; expected actual seeded objects')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Keep raw numeric extremes separate from caller identity styles; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=isocurve_object,verify=verify_identity,configure=partial(configure,reference=reference),
        magic=b'ISO1',kinds=len(RATIOS),versions=VERSIONS,edited_kinds=tuple(range(len(RATIOS))))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/sketch-isocurve-information',fixture_family(REFERENCE))
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
