"""Observe complete weldment finishing bodies through actual reference readers (#54)."""
from pathlib import Path
from functools import partial
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','display-welds','weldment-finishing'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_display_welds import configure as configure_display
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from finishing_bytes import CLASSES,VERSIONS,finishing_graph


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Bind actual ordinary helper-clear finishing readers; e.g. configure(cursor,source)."""
    configure_display(cursor,reference)
    cursor.readers['moPMarkRecord_c']=source_module('bodyfeatures',reference).permanent_mark_record
    gussets=source_module('gussets',reference);beads=source_module('weldbeadfeatures',reference)
    cursor.readers.update(zip(CLASSES,(gussets.read,beads.read,beads.side)))


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact EOF and real repeated-root identity; e.g. verify(source,raw,19000,1,2)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'finishing end {cursor.p}; expected {len(raw)}')
    if style not in (2,5):return
    items=root['items'] if style==2 else root['items'][1:]
    actual=(items[0] is items[1],items[0] is items[2])
    if actual!=(False,True):raise ValueError(f'finishing roots {actual}; expected (False,True)')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Cross three native classes with inherited/cabinet/display version gates; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=finishing_graph,verify=verify,configure=partial(configure,reference=reference),
        magic=b'WFF1',kinds=3,versions=VERSIONS,edited_kinds=(0,1,2))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/weldment-finishing',fixture_family(REFERENCE))
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
