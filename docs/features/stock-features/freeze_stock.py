"""Observe composed stock bodies with the unchanged independent readers (#54)."""
from pathlib import Path
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','stock-features'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from object_fixture_observer import object_cursor
from stock_fixture_bytes import stock_graph, VERSIONS


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact object boundaries and real duplicate identities; e.g. verify(source,raw,19000,0,3)."""
    cursor=object_cursor(reference,raw,version,None)
    try:
        parsed=cursor.object()
    except Exception as error:
        raise ValueError(f'stock graph {kind}/{version:#x}/{style} cursor {cursor.p}; expected complete documented composition') from error
    assert cursor.p==len(raw)
    if kind in (0,1) and style%2:
        assert parsed['array_a10']['items'][0] is parsed['array_a88']['items'][1]
        assert parsed['late']['pairs_e20'][0]['edge'] is parsed['late']['sets_e60'][0]['members'][0]


def fixture_family() -> ObjectFixtureFamily:
    """Cross conditional bodies, direct arrays and historical hole ownership; e.g. fixture_family()."""
    return ObjectFixtureFamily(build=stock_graph,verify=verify,magic=b'STK1',kinds=4,
                               versions=VERSIONS,edited_kinds=tuple(range(4)))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/stock-features',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
