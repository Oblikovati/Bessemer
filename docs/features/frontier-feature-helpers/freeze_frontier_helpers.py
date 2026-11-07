"""Freeze complete native-bound feature/helper families against stable source (#54)."""
from pathlib import Path
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(WORKSPACE/'docs/features/trim-edge-identifiers'))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor, object_cursor
from frontier_helper_bytes import VERSIONS, frontier_helper_object


def configure(cursor: ReferenceObjectCursor) -> None:
    """Keep exact source registrations local to this cursor; e.g. configure(cursor)."""
    source_module('frontierfeatures',REFERENCE).install(cursor)


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check actual source target identities, including an unqualified cycle; e.g. verify_identity(source, raw, 19000, 4, 5)."""
    if style not in (2,4,5):return
    root=cast(dict[str,object],object_cursor(reference,raw,version,configure).object())
    items=cast(list[dict[str,object]],root['items']);first,second,repeated=items[1:] if kind>=3 else items
    if first is second or repeated is not first:
        raise ValueError('frontier helper aliases; expected distinct equal roots and repeated first')
    if kind<3:return
    if first['dimension_08'] is not second['dimension_08']:
        raise ValueError('frontier helper dimension; expected shared complete target')
    if kind==3:
        verify_edge_targets(first,second,style)
    elif first['object_10'] is not (root if style==5 else second['object_10']):
        raise ValueError('radius CObject; expected shared arbitrary target or containing array')


def verify_edge_targets(first: dict[str,object], second: dict[str,object], style: int) -> None:
    """Keep distinct equal dimensions separate from repeated targets; e.g. verify_edge_targets(first, second, 4)."""
    for key in ('edge_10','dimension_28','dimension_30'):
        if first[key] is not second[key]:
            raise ValueError(f'edge helper {key}; expected shared target')
    if (first['dimension_08'] is first['dimension_28']) == (style==4):
        raise ValueError(f'edge helper style {style}; expected separate equal dimension only for style 4')
    if first['dimension_08'] is not first['dimension_30']:
        raise ValueError('edge helper dimension_30; expected repeated first dimension')


FAMILY=ObjectFixtureFamily(frontier_helper_object,verify_identity,configure,b'FFH1',5,VERSIONS,(0,1,2,3))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/frontier-feature-helpers',FAMILY)
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8')
