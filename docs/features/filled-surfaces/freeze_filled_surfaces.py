"""Freeze complete filled-surface and contact families against stable source (#54)."""
from pathlib import Path
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for folder in ('trim-edge-identifiers','sketch-wrapper-roles','solid-references'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/folder))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from object_fixture_observer import object_cursor
from filled_surface_bytes import VERSIONS, filled_surface_object


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check actual target sharing and the unqualified containing-array cycle; e.g. verify_identity(source, raw, 19000, 3, 5)."""
    if style not in (2,4,5):return
    root=cast(dict[str,object],object_cursor(reference,raw,version,None).object())
    first,second,repeated=cast(list[dict[str,object]],root['items'])
    if first is second or repeated is not first:
        raise ValueError(f'filled kind/style {kind}/{style}; expected distinct equal roots and cached first')
    if kind==3:
        if first['wrapper_18'] is second['wrapper_18']:
            raise ValueError('curve-contact data wrappers; expected distinct equal complete targets')
        if style==5 and (first['reference_08'] is not root or second['reference_08'] is not root):
            raise ValueError('contact data reference_08; expected cached containing array')
        return
    if kind==2:
        for key in ('angle_a0','synch_e0'):
            if first[key] is not second[key]:raise ValueError(f'curve contact {key}; expected shared complete target')
        return
    verify_arrays(first,second,kind)


def verify_arrays(first: dict[str,object], second: dict[str,object], kind: int) -> None:
    """Keep direct containers distinct without losing shared child identities; e.g. verify_arrays(first, second, 0)."""
    if kind==0:
        a,b=(cast(dict[str,object],row['payload']) for row in (first,second))
        for key in ('array_10','array_30','array_50'):
            verify_array(cast(dict[str,object],a[key]),cast(dict[str,object],b[key]))
        a,b=(cast(dict[str,object],row['keeper_118']) for row in (a,b))
    else:a,b=first,second
    verify_array(cast(dict[str,object],a['array_48']),cast(dict[str,object],b['array_48']))


def verify_array(first: dict[str,object], second: dict[str,object]) -> None:
    """Require independent direct lists and retained tagged child aliases; e.g. verify_array(first, second)."""
    if first is second or any(a is not b for a,b in zip(cast(list[object],first['items']),cast(list[object],second['items']))):
        raise ValueError('filled direct array; expected distinct containers with shared child targets')


FAMILY=ObjectFixtureFamily(filled_surface_object,verify_identity,None,b'FLS1',4,VERSIONS,(0,2,3))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/filled-surfaces',FAMILY)
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8')
