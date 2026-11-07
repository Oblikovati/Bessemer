"""Observe the actual fixed archive operator in complete valued-dimension graphs (#86)."""
from pathlib import Path
from functools import partial
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','fixed-valued-transforms'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from object_fixture_observer import object_cursor
from freeze_blank_part import source_module
from valued_transform_bytes import CLASSES, VERSIONS, transform_object


def configure(cursor: object, reference: Path) -> None:
    """Install the actual complete base bodies and handle readers locally; e.g. configure(cursor,source)."""
    source_module('sketchprefix',reference).install_scoped_readers(cursor)
    source_module('sketchdimensions',reference).install(cursor)


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require actual repeated roots while retaining full derived suffixes; e.g. verify_identity(source,raw,19000,1,2)."""
    root=cast(dict[str,object],object_cursor(reference,raw,version,partial(configure,reference=reference)).object())
    if style in (2,5):
        first,second,repeated=root['items']
        if first is second or first is not repeated:raise ValueError('valued roots; expected distinct objects and repeated first')
        root=first
    if root['_class']!=CLASSES[kind]:raise ValueError(f'valued class {root["_class"]}; expected {CLASSES[kind]}')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Bind complete real readers to the chosen immutable source root; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=transform_object,verify=verify_identity,configure=partial(configure,reference=reference),magic=b'VTF1',kinds=4,versions=VERSIONS,edited_kinds=(0,1,2,3))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/fixed-valued-transforms',fixture_family(REFERENCE))
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
