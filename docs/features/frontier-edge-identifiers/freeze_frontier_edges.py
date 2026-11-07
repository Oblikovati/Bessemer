"""Freeze independently assembled C-392 edge identifiers against scoped source readers (#54)."""
from pathlib import Path
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(WORKSPACE/'docs/features/trim-edge-identifiers'))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor, object_cursor
from frontier_edge_bytes import frontier_edge_object


def configure(cursor: ReferenceObjectCursor) -> None:
    """Install the exact native-bound source readers locally; e.g. configure(cursor)."""
    source_module('frontierfeatures',REFERENCE).install(cursor)


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check actual source identities through direct and tagged members; e.g. verify_identity(source, raw, 19000, 1, 5)."""
    if style not in (2,5):
        return
    root=cast(dict[str,object],object_cursor(reference,raw,version,configure).object())
    first,second,repeated=cast(list[dict[str,object]],root['items'])
    if style==5:
        first,second=(cast(dict[str,object],row['representation']) for row in (first,second))
    if first is second or repeated is not first:
        raise ValueError('frontier-edge identities; expected distinct equal roots and repeated first')
    for name in ('feature','external_feature_reference'):
        if first[name] is not second[name]:
            raise ValueError(f'frontier-edge {name}; expected shared complete target')
    if kind and cast(dict[str,object],first['source_entity'])['representation'] is not cast(dict[str,object],second['source_entity'])['representation']:
        raise ValueError('frontier-edge source_entity; expected shared representation inside distinct direct IDs')


FAMILY=ObjectFixtureFamily(frontier_edge_object,verify_identity,configure,b'FEI1')


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/frontier-edge-identifiers',FAMILY)
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8')
