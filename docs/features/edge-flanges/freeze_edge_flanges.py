"""Observe complete edge-flange families through actual independent native readers (#54)."""
from pathlib import Path
from functools import partial
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','base-flanges','edge-flanges'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_base_flanges import install_flange_dimensions
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from edge_fixture_bytes import CLASSES,VERSIONS,edge_graph


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Use actual serializers and inherited dependencies on each local cursor; e.g. configure(cursor,source)."""
    install_flange_dimensions(cursor,reference)
    module=source_module('edgeflanges',reference)
    readers=(module.read,module.data,module.one_bend,module.per_edge,module.geometry,module.geometry)
    cursor.readers.update(zip(CLASSES,readers))


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require actual reader EOF and distinct/repeated roots; e.g. verify(source,raw,19000,2,5)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'edge-flange end {cursor.p}; expected {len(raw)}')
    if style not in (2,5):return
    items=root['items'];actual=(items[0] is items[1],items[0] is items[2])
    if actual!=(False,True):raise ValueError(f'edge-flange identities {actual}; expected (False,True)')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Cross six native classes with all documented late fields; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=edge_graph,verify=verify,configure=partial(configure,reference=reference),
        magic=b'EFL1',kinds=6,versions=VERSIONS,edited_kinds=tuple(range(6)))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/edge-flanges',fixture_family(REFERENCE))
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
