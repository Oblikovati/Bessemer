"""Freeze complete native-bound ruled surfaces against actual scoped readers (#54)."""
from pathlib import Path
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','reference-axes','reference-axis-owners','revolved-surfaces','ruled-surfaces'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from object_fixture_observer import object_cursor
from ruled_surface_bytes import VERSIONS, ruled_object


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check actual self owners, shared dimensions and repeated roots; e.g. verify_identity(source,raw,19000,0,5)."""
    root=cast(dict[str,object],object_cursor(reference,raw,version,None).object())
    roots=cast(list[dict[str,object]],root['items']) if style in (2,5) else [root]
    first=roots[0];payload=first['direct_data_7b0'] if kind==0 else first
    owner=payload['owner']
    if kind==0 and owner is not first:raise ValueError('ruled direct owner; expected the containing feature')
    if kind==1 and owner['direct_data_7b0']['owner'] is not owner:raise ValueError('ruled tagged data owner; expected complete self-owned feature')
    if len(roots)==1:return
    if roots[0] is roots[1] or roots[0] is not roots[2]:raise ValueError('ruled roots; expected distinct roots and repeated first')
    other=roots[1]['direct_data_7b0'] if kind==0 else roots[1]
    if style>=2 and payload['distance_18'] is not other['distance_18']:raise ValueError('ruled dimensions; expected actual shared target')


def fixture_family() -> ObjectFixtureFamily:
    """Cover both native dispatches and every inherited modern version gate; e.g. fixture_family()."""
    return ObjectFixtureFamily(build=ruled_object,verify=verify_identity,magic=b'RUL1',kinds=2,versions=VERSIONS,edited_kinds=(0,1))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/ruled-surfaces',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
