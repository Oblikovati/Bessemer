"""Observe real detail serializers in complete independent graph contexts (#54)."""
from pathlib import Path
from functools import partial
from typing import cast
import json
import sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','component-detail-references'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from detail_reference_bytes import detail_graph,VERSIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Keep the actual guarded component and complete detail-item readers; e.g. configure(cursor,source)."""
    source_module('displaydimensions',reference).install_scoped(cursor)
    cursor.readers['moAttachPoint_c']=source_module('camerafeatures',reference).attach_point
    cursor.readers.update(moDetailItem_c=source_module('displaynotes',reference).detail_item,
        moCompDetailItem_c=source_module('weldinternal',reference).component)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check exact graph endpoints and actual shared detail identities; e.g. verify(source,raw,19000,1,5)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'detail graph end {cursor.p}; expected {len(raw)}')
    if style not in (2,3,5):return
    items=cast(list[dict[str,object]],root['items'])
    if style==3:
        if kind==1 and items[1]['detail_e8'] is not items[0]:raise ValueError('detail seed shares=False; expected True')
        return
    first,second,repeated=items
    if first is second or first is not repeated:raise ValueError(f'detail roots {(first is second,first is repeated)}; expected (False,True)')
    if kind==1 and first['detail_e8'] is not second['detail_e8']:raise ValueError('detail targets share=False; expected True')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Cross both exact layouts with inherited component version gates; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=detail_graph,verify=verify,configure=partial(configure,reference=reference),
        magic=b'DRF1',kinds=2,versions=VERSIONS,edited_kinds=(0,1))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/component-detail-references',fixture_family(REFERENCE))
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
