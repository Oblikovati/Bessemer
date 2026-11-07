"""Observe independent complete cosmetic cut lists with the actual reference reader (#54)."""
from pathlib import Path
from functools import partial
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','reference-axes','reference-axis-owners','radiated-edge-identifiers','helices','cosmetic-weld-cut-lists'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from freeze_blank_part import source_module
from cut_list_bytes import VERSIONS,cut_list_graph


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Bind the actual scoped cut-list reader; e.g. configure(cursor,reference)."""
    cursor.readers['moCosmeticWeldCutList_c']=source_module('weldcutlists',reference).read


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require actual source EOF and genuine repeated identity; e.g. verify(source,raw,19000,1,2)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'cut list end {cursor.p}; expected {len(raw)}')
    if style not in (2,5):return
    first,second,repeated=cast(list[dict[str,object]],root['items'])
    if first is second or first is not repeated:raise ValueError('cut-list roots; expected distinct first/second and cached first')


def fixture_family() -> ObjectFixtureFamily:
    """Cross both kernel frame modes and inherited gates; e.g. fixture_family()."""
    return ObjectFixtureFamily(build=cut_list_graph,verify=verify,configure=partial(configure,reference=REFERENCE),
        magic=b'CWL1',kinds=2,versions=VERSIONS,edited_kinds=(0,1))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/cosmetic-weld-cut-lists',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
