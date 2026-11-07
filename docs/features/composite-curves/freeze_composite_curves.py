"""Freeze complete composite curves using the actual scoped source reader (#54)."""
from pathlib import Path
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','reference-axes','reference-axis-owners','radiated-edge-identifiers','helices','composite-curves'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from object_fixture_observer import object_cursor
from composite_curve_bytes import VERSIONS, composite_object


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check actual aliases independently of the byte partition; e.g. verify_identity(source,raw,19000,1,2)."""
    root=cast(dict[str,object],object_cursor(reference,raw,version,None).object())
    if style not in (2,5):return
    first,second,repeated=cast(list[dict[str,object]],root['items'])
    if first is second or repeated is not first:raise ValueError('composite roots; expected distinct equal roots and repeated first')
    left=cast(dict[str,object],first['payload']);right=cast(dict[str,object],second['payload'])
    if left['tagged_array_778'] is not right['tagged_array_778']:raise ValueError('composite tagged array; expected actual cached target')


def fixture_family() -> ObjectFixtureFamily:
    """Cover present and suppressed wire frames across inherited gates; e.g. fixture_family()."""
    return ObjectFixtureFamily(build=composite_object,verify=verify_identity,magic=b'CCV1',kinds=2,versions=VERSIONS,edited_kinds=(0,1))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/composite-curves',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
