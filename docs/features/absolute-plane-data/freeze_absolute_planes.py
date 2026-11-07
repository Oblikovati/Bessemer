"""Observe absolute planes using the actual shared reference reader (#54)."""
from pathlib import Path
from functools import partial
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','reference-axes','reference-axis-owners','absolute-plane-data'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from absolute_plane_bytes import VERSIONS,absolute_graph


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install the native-bound shared data reader with its real typed parent; e.g. configure(cursor,source)."""
    planes=source_module('referenceplanes',reference);planes.install_scoped(cursor)
    cursor.readers['moAbsoluteRefPlnData_c']=planes.data


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require actual EOF and distinct/repeated root identity; e.g. verify(source,raw,19000,1,2)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'absolute-plane end {cursor.p}; expected {len(raw)}')
    if style not in (2,5):return
    items=root['items'];actual=(items[0] is items[1],items[0] is items[2])
    if actual!=(False,True):raise ValueError(f'absolute-plane root identities {actual}; expected (False,True)')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Cross the non-default body and real containing planes with all base gates; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=absolute_graph,verify=verify,configure=partial(configure,reference=reference),
        magic=b'APL1',kinds=2,versions=VERSIONS,edited_kinds=(0,1))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/absolute-plane-data',fixture_family(REFERENCE))
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
