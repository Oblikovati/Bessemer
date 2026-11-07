"""Bind complete sweep-surface fixtures to stable approved source (#54)."""
from pathlib import Path
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for folder in ('trim-edge-identifiers','reference-axes','reference-axis-owners','revolved-surfaces','sweep-reference-surfaces'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/folder))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from object_fixture_observer import object_cursor
from sweep_surface_bytes import VERSIONS, sweep_surface_object


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require distinct equal roots and a repeated first object; e.g. verify_identity(source,raw,19000,0,5)."""
    if style not in (2,5):return
    root=cast(dict[str,object],object_cursor(reference,raw,version,None).object())
    first,second,repeated=cast(list[dict[str,object]],root['items'])
    if first is second or repeated is not first:
        raise ValueError('sweep surface roots; expected distinct equal objects and repeated first')
    if kind==1 and first['endpoint_18'] is not first['endpoint_28']:
        raise ValueError('edge endpoints; expected shared complete endpoint identity')
    if kind==0:
        a=cast(dict[str,object],first['payload']);b=cast(dict[str,object],second['payload'])
        if a['edge_data'][0] is not b['edge_data'][0]:
            raise ValueError('surface edge data; expected shared complete target')


FAMILY=ObjectFixtureFamily(build=sweep_surface_object,verify=verify_identity,magic=b'SRS1',
                          kinds=3,versions=VERSIONS,edited_kinds=(0,))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/sweep-reference-surfaces',FAMILY)
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
