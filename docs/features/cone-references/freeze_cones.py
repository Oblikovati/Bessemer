"""Observe complete cones through independent approved readers (#54)."""
from pathlib import Path
from functools import partial
import json
import sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','cone-references'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from cone_bytes import cone_object,VERSIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install the actual cone serializer locally; e.g. configure(cursor,source)."""
    cursor.readers['moConeRef_w']=source_module('geometricfrontier',reference).cone


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require actual scoped readers to reach graph EOF; e.g. verify(source,raw,19000,0,0)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference))
    cursor.object()
    if cursor.p!=len(raw):raise ValueError(f'cone end {cursor.p}; expected {len(raw)}')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Keep independent value sets across every wrapper version gate; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=cone_object,verify=verify,configure=partial(configure,reference=reference),
        magic=b'CON1',kinds=3,versions=VERSIONS,edited_kinds=(0,1,2))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/cone-references',fixture_family(REFERENCE))
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
