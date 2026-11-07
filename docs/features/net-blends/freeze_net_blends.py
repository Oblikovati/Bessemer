"""Freeze complete net-blend graphs against stable independent native-bound readers (#54)."""
from pathlib import Path
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(WORKSPACE/'docs/features/trim-edge-identifiers'))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from object_fixture_observer import object_cursor
from net_blend_bytes import VERSIONS, net_blend_object


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check shared dependencies and actual cached cycles without placeholder targets; e.g. verify_identity(source, raw, 19000, 3, 4)."""
    if style not in (2,4,5):return
    root=cast(dict[str,object],object_cursor(reference,raw,version,None).object())
    first,second,repeated=cast(list[dict[str,object]],root['items'])
    if kind>=3 and style==4:
        first,second=(cast(dict[str,object],row['component_reference']) for row in (first,second))
    if first is second or repeated is not first:
        raise ValueError(f'net-blend kind/style {kind}/{style}; expected distinct equal roots and repeated first')
    if kind<2:
        verify_feature(first,second,style)
    else:
        verify_dependencies(first,second,kind)


def verify_feature(first: dict[str,object], second: dict[str,object], style: int) -> None:
    """Keep shared contact/dimension identity through all three direct sets; e.g. verify_feature(first, second, 4)."""
    left,right=(cast(dict[str,object],row['data']) for row in (first,second))
    contact=None
    for label in ('curves_50','curves_2a8','centerline_500'):
        a,b=(cast(dict[str,object],row[label]) for row in (left,right))
        x,y=(cast(list[object],row['contacts_08'])[0] for row in (a,b))
        if x is not y or (contact is not None and x is not contact):
            raise ValueError(f'net-blend {label}; expected one shared complete contact target')
        contact=x
    if first['dimension_10f8'] is not second['dimension_10f8']:
        raise ValueError('net-blend dimension_10f8; expected shared complete dimension')
    if style==4 and first['dimension_1100'] is not first['dimension_10f8']:
        raise ValueError('net-blend dimension_1100; expected alias of primary dimension')


def verify_dependencies(first: dict[str,object], second: dict[str,object], kind: int) -> None:
    """Check cached groups, direct lists and multi-edge targets; e.g. verify_dependencies(first, second, 4)."""
    if kind==2:
        if first['reference_08'] is not second['reference_08']:
            raise ValueError('contact reference_08; expected shared complete wrapper')
        return
    if kind==3:
        for label in ('list_f0','list_138'):
            a,b=(cast(dict[str,object],row[label]) for row in (first,second))
            if a is b or cast(list[object],a['items'])[0] is not cast(list[object],b['items'])[0]:
                raise ValueError(f'component group {label}; expected distinct direct lists with shared targets')
        return
    a,b=(cast(list[object],row['edges_08']) for row in (first,second))
    if a[0] is not b[0] or a[0] is not a[-1]:
        raise ValueError('multi-edge edges_08; expected shared complete edge targets')


FAMILY=ObjectFixtureFamily(net_blend_object,verify_identity,None,b'NBL1',5,VERSIONS,(0,1,3,4))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/net-blends',FAMILY)
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8')
