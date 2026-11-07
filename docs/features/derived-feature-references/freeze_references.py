"""Observe complete reference objects and unchanged generic callers (#54)."""
from pathlib import Path
from functools import partial
import json
import sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','stock-features','sketch-wrapper-roles','projected-line-subtypes',
             'end-spec-wrapper-roles','intersection-wrapper-roles','derived-feature-references'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_intersection_wrappers import configure as configure_callers
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from reference_bytes import reference_object,wrapper_object,VERSIONS


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install the actual plane-data scope before reference children; e.g. configure(cursor,source)."""
    source_module('referenceplanes',reference).install_scoped(cursor)


def configure_wrapper(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Retain the established caller registry as well as plane data."""
    configure_callers(cursor,reference)
    configure(cursor,reference)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require complete real objects, including cached child identities; e.g. verify(source,raw,19000,0,3)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw)
    if kind==0 and style not in (0,4):
        assert root['array_e0']['items'][0] is root['array_e0']['items'][1]
        assert root['array_e0']['items'][2] is root


def verify_caller(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Keep inherited caller scope explicit; e.g. verify_caller(source,raw,19000,3,1)."""
    cursor=object_cursor(reference,raw,version,partial(configure_wrapper,reference=reference));cursor.object()
    if cursor.p!=len(raw):raise ValueError(f'wrapper end {cursor.p}; expected {len(raw)}')


def fixture_family() -> ObjectFixtureFamily:
    """Retain both exact reference grammars; e.g. fixture_family()."""
    return ObjectFixtureFamily(build=reference_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'DRF1',kinds=2,
        versions=VERSIONS,edited_kinds=(0,1))


def wrapper_family(reference: Path) -> ObjectFixtureFamily:
    """Verify both new wrappers across all five callers; e.g. wrapper_family(source)."""
    return ObjectFixtureFamily(build=wrapper_object,verify=verify_caller,configure=partial(configure_wrapper,reference=reference),
        magic=b'DRW1',kinds=10,versions=(0x27cc,19000,0xffffffff),edited_kinds=())


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/derived-feature-references',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    wrappers=freeze(REFERENCE,WORKSPACE/'tests/fixtures/derived-reference-wrappers',wrapper_family(REFERENCE))
    Path(__file__).with_name('wrapper-reference.json').write_text(json.dumps(wrappers,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']),len(wrappers['cases']),wrappers['reads'],len(wrappers['owned']))
