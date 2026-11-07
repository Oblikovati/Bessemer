"""Freeze complete cosmetic weld tree bodies using actual independent readers (#54)."""
from pathlib import Path
from functools import partial
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','component-detail-references','weld-symbols','display-welds','cosmetic-weld-tree'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_weld_symbols import configure as configure_symbols
from freeze_display_welds import configure as configure_display
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from tree_bytes import CLASSES,VERSIONS,tree_graph


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Use only actual schema readers for every dependency; e.g. configure(cursor,source)."""
    configure_symbols(cursor,reference);configure_display(cursor,reference)
    module=source_module('weldcutlists',reference)
    readers=(module.subfolder,module.bead,module.bead_data,module.from_to,module.intermittent,module.wire_map)
    cursor.readers.update(zip(CLASSES,readers))


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact EOF and actual root identity; e.g. verify(source,raw,19000,2,5)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'weld tree end {cursor.p}; expected {len(raw)}')
    if style not in (2,5):return
    items=cast(list[dict[str,object]],root['items']);first,second,repeated=items if style==2 else items[1:]
    if first is second or first is not repeated:raise ValueError('weld tree roots; expected distinct first/second and cached first')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Cross six native classes with all display-item gates; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=tree_graph,verify=verify,configure=partial(configure,reference=reference),
        magic=b'CWT1',kinds=6,versions=VERSIONS,edited_kinds=tuple(range(6)))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/cosmetic-weld-tree',fixture_family(REFERENCE))
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
