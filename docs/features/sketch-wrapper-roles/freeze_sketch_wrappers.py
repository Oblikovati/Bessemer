"""Freeze complete known wrapper bodies at the sgExtEnt reference boundary (#46)."""
from pathlib import Path
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for folder in ('trim-edge-identifiers','solid-references'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/folder))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze as freeze_family
from freeze_blank_part import source_module, reference_snapshot
from object_fixture_observer import ReferenceObjectCursor, object_cursor, observe_object_body
from sketch_wrapper_bytes import CLASSES, VERSIONS, sketch_wrapper_object, unrelated_fresh_object


def configure(cursor: ReferenceObjectCursor) -> None:
    """Use real scoped source readers, with no synthetic body substitutions; e.g. configure(cursor)."""
    source_module('sketchprefix',REFERENCE).install_scoped_readers(cursor)


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require the selected runtime class and preserve fresh/cached identities; e.g. verify_identity(source, raw, 19000, 3, 2)."""
    root=cast(dict[str,object],object_cursor(reference,raw,version,configure).object())
    if style in (0,4):
        target=root['reference']
    else:
        items=cast(list[dict[str,object]],root['items'])
        target=items[0]['reference'] if style==2 else items[0]
        verify_aliases(items,style)
    if style==4:
        if target is not None:raise ValueError(f'null reference {target}; expected None')
        return
    if cast(dict[str,object],target)['_class']!=CLASSES[kind]:
        raise ValueError(f'wrapper kind {kind}; expected actual class {CLASSES[kind]}')


def verify_aliases(items: list[dict[str,object]], style: int) -> None:
    """Keep equal reference objects distinct while repeating actual owners; e.g. verify_aliases(items, 2)."""
    if style==2:
        if items[0] is items[1] or items[2] is not items[0] or items[0]['reference'] is items[1]['reference']:
            raise ValueError('fresh sketch references; expected distinct equal owners/targets and cached first owner')
        return
    if items[1]['reference'] is not items[0]:
        raise ValueError('cached sketch reference; expected seeded complete target')
    if style==3 and items[3]['reference'] is not items[2]:
        raise ValueError('second cached sketch reference; expected separate second complete target')
    if style==5 and (items[3] is not items[1] or items[2]['reference'] is items[0]):
        raise ValueError('mixed sketch references; expected repeated first owner and distinct second target')


FAMILY=ObjectFixtureFamily(sketch_wrapper_object,verify_identity,configure,b'SWR1',len(CLASSES),VERSIONS,tuple(range(len(CLASSES))))


def freeze(reference: Path, destination: Path) -> dict[str,object]:
    """Observe valid bodies independently while leaving role rejection to Rust; e.g. freeze(source, fixtures)."""
    facts=freeze_family(reference,destination,FAMILY)
    raw,slots=unrelated_fresh_object()
    _,observed=observe_object_body(reference,raw,slots,120,19000,'unrelated-fresh',configure)
    (destination/'unrelated-fresh.bin').write_bytes(raw)
    if facts['source_hashes']!=reference_snapshot(reference):
        raise ValueError('source changed during foreign-body observation; expected original stable source hashes')
    return facts|dict(unrelated_fresh=observed,source_role_oracle=False)


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/sketch-wrapper-roles')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8')
