"""Freeze the body-reference extension to the exact wrapper inventory (#54)."""
from pathlib import Path
from functools import partial
import json
import sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','sketch-wrapper-roles','projected-line-subtypes',
             'end-spec-wrapper-roles','intersection-wrapper-roles','stock-features'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_intersection_wrappers import configure
from object_fixture_observer import object_cursor
from body_wrapper_bytes import body_wrapper_object


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require complete existing caller and target bodies; e.g. verify(source,raw,19000,3,1)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference))
    cursor.object()
    if cursor.p!=len(raw):raise ValueError(f'body wrapper end {cursor.p}; expected {len(raw)}')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Retain all five generic-wrapper caller kinds; e.g. fixture_family(source)."""
    versions=(0x27cc,0x2b72,0x2b73,0x2faf,0x2fb0,0x3b01,0x3b02,19000,0xffffffff)
    return ObjectFixtureFamily(build=body_wrapper_object,verify=verify,configure=partial(configure,reference=reference),
        magic=b'BWR1',kinds=5,versions=versions,edited_kinds=())


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/body-reference-wrappers',fixture_family(REFERENCE))
    Path(__file__).with_name('wrapper-reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
