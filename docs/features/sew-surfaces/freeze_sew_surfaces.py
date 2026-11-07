"""Freeze complete native-bound sew surfaces and zero-byte keeper callers (#54)."""
from pathlib import Path
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(WORKSPACE/'docs/features/trim-edge-identifiers'))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from object_fixture_observer import object_cursor
from sew_surface_bytes import VERSIONS, sew_surface_object


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Keep actual target/keeper aliases without inventing direct-array identities; e.g. verify_identity(source, raw, 19000, 1, 5)."""
    if style not in (2,4,5) and not (kind==1 and style>=1):return
    root=cast(dict[str,object],object_cursor(reference,raw,version,None).object())
    first,second,repeated=cast(list[dict[str,object]],root['items'])
    if kind==1 and style>=3:
        body=cast(dict[str,object],second['body'])
        if first is not repeated or body['id_keeper'] is not first:
            raise ValueError('sew keeper; expected seeded empty keeper retained by actual body header')
        return
    if first is second or repeated is not first:
        raise ValueError(f'sew kind/style {kind}/{style}; expected distinct equal roots and repeated first')
    if kind==0:verify_arrays(first,second,style)


def verify_arrays(first: dict[str,object], second: dict[str,object], style: int) -> None:
    """Share complete targets while keeping each direct array independent; e.g. verify_arrays(first, second, 4)."""
    a,b=(cast(dict[str,object],row['payload']) for row in (first,second))
    for label in ('array_30','array_50','array_70','array_90','array_b0'):
        left,right=(cast(dict[str,object],row[label]) for row in (a,b))
        if left is right:raise ValueError(f'{label}; expected distinct direct array bodies')
        x,y=(cast(list[object],row['items']) for row in (left,right))
        if any(l is not r for l,r in zip(x,y)):
            raise ValueError(f'{label}; expected shared complete targets at matching positions')
    if style>=3:
        left,right=(cast(dict[str,object],row['body']) for row in (first,second))
        if left['id_keeper'] is not right['id_keeper']:
            raise ValueError('sew header keeper; expected shared concrete empty keeper')


FAMILY=ObjectFixtureFamily(sew_surface_object,verify_identity,None,b'SEW1',2,VERSIONS,(0,))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/sew-surfaces',FAMILY)
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8')
