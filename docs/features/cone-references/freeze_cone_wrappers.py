"""Freeze the cone extension to the existing finite wrapper family (#54)."""
from pathlib import Path
from functools import partial
import json
import sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','solid-references','sketch-wrapper-roles','projected-line-subtypes','end-spec-wrapper-roles','intersection-wrapper-roles','cone-references'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_intersection_wrappers import configure as configure_callers
from freeze_cones import configure as configure_cone
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from cone_wrapper_bytes import cone_wrapper_object


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install actual complete callers and the native-bound cone reader; e.g. configure(cursor,source)."""
    configure_callers(cursor,reference)
    configure_cone(cursor,reference)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require every complete caller to consume its exact graph; e.g. verify(source,raw,19000,3,1)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));cursor.object()
    if cursor.p!=len(raw):raise ValueError(f'cone wrapper end {cursor.p}; expected {len(raw)}')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Keep all five containing caller kinds independently observed; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=cone_wrapper_object,verify=verify,configure=partial(configure,reference=reference),
        magic=b'CWR1',kinds=5,versions=(0x27cc,19000,0xffffffff),edited_kinds=())


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/cone-wrappers',fixture_family(REFERENCE))
    Path(__file__).with_name('wrapper-reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
