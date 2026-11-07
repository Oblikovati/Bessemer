"""Observe complete end caps with actual independent schema readers (#54)."""
from pathlib import Path
from functools import partial
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','end-caps'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from end_cap_bytes import VERSIONS,end_cap_graph


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Bind the actual end-cap and dimension readers locally; e.g. configure(cursor,source)."""
    source_module('displaydimensions',reference).install_scoped(cursor)
    cursor.readers['moEndCap_c']=source_module('endcaps',reference).read


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact EOF and actual shared dimension identity; e.g. verify(source,raw,19000,1,2)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'end-cap end {cursor.p}; expected {len(raw)}')
    first=root if style in (0,1,4) else root['items'][0 if style==2 else 1]
    values=first['payload']
    if values['dimension_858'] is not None:raise ValueError(f"arc-length pointer class {values['dimension_858']['_class']}; expected null")
    if style not in (0,4) and values['dimension_840'] is None:raise ValueError('distance pointer None; expected complete distance dimension')
    if style not in (0,4) and any(values['dimension_'+offset] is not values['dimension_840'] for offset in ('838','848','850')):
        raise ValueError(f"distance pointer identities {len({id(values['dimension_'+offset]) for offset in ('840','838','848','850')})}; expected 1")
    if style not in (2,5):return
    items=root['items'] if style==2 else root['items'][1:]
    if items[0] is items[1] or items[0] is not items[2]:raise ValueError(f'end-cap root identities {(items[0] is items[1],items[0] is items[2])}; expected (False,True)')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Cross raw scalar sets with inherited and distance-dimension gates; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=end_cap_graph,verify=verify,configure=partial(configure,reference=reference),
        magic=b'ECF1',kinds=3,versions=VERSIONS,edited_kinds=(0,1,2))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/end-caps',fixture_family(REFERENCE))
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
