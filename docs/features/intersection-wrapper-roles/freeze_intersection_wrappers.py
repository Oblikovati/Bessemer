"""Observe complete sketches and independently reject wrong wrapper roles in Rust (#46)."""
from functools import partial
from pathlib import Path
from typing import cast
import json
import struct
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','sketch-wrapper-roles','projected-line-subtypes','end-spec-wrapper-roles','intersection-wrapper-roles'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from freeze_blank_part import source_module, reference_snapshot
from object_fixture_observer import ReferenceObjectCursor, object_cursor, observe_object_body
from freeze_end_wrappers import configure as configure_wrappers
from intersection_wrapper_bytes import CLASSES, VERSIONS, intersection_wrapper_object


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Use actual complete sketch and dependency readers in the local cursor; e.g. configure(cursor,source)."""
    configure_wrappers(cursor,reference)
    cursor.readers['sgSketch']=source_module('assemblyprofiles',reference).sketch


def first_sketch(root: dict[str,object], style: int) -> dict[str,object]:
    """Select the real containing sketch after any seed objects; e.g. first_sketch(root,5)."""
    if style not in (1,2,5):return root
    items=cast(list[dict[str,object]],root['items'])
    return items[4] if style in (1,5) else items[0]


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Verify concrete classes and actual cached/null identities; e.g. verify_identity(source,raw,19000,3,5)."""
    root=cast(dict[str,object],object_cursor(reference,raw,version,partial(configure,reference=reference)).object())
    sketch=first_sketch(root,style)
    collections=cast(dict[str,object],sketch['styled_intersections'])
    selected=cast(list[dict[str,object]],collections['intersections'])[1]
    for position,offset in enumerate(('10','18','20','28')):
        target=selected['reference_'+offset]
        nullable=(style==3 and position%2==1) or (style==4 and position%2==0)
        if nullable:
            if target is not None:raise ValueError('intersection reference; expected preserved null')
            continue
        if target['_class']!=CLASSES[kind]:raise ValueError(f'intersection target {target["_class"]}; expected {CLASSES[kind]}')
        if style in (1,5) and target is not root['items'][position]:raise ValueError('intersection target; expected actual seeded object')
    verify_roots(root,style)


def verify_roots(root: dict[str,object], style: int) -> None:
    """Keep distinct sketches and the repeated first owner separate; e.g. verify_roots(root,2)."""
    if style not in (2,5):return
    items=cast(list[dict[str,object]],root['items']);first,second,repeated=items[4:] if style==5 else items
    if first is second or first is not repeated:raise ValueError('intersection sketches; expected distinct roots and repeated first')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Cross all four fields with the reviewed finite wrapper family; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=intersection_wrapper_object,verify=verify_identity,configure=partial(configure,reference=reference),
        magic=b'IWR1',kinds=len(CLASSES),versions=VERSIONS,edited_kinds=tuple(range(len(CLASSES))))


def additional_witnesses(reference: Path, destination: Path) -> list[dict[str,object]]:
    """Retain physically complete unrelated targets at every typed field; e.g. additional_witnesses(source,fixtures)."""
    rows=[]
    for position in range(4):
        raw,slots=intersection_wrapper_object(0,19000,0,foreign_field=position)
        frame,row=observe_object_body(reference,raw,slots,position,19000,f'unrelated-{position}',partial(configure,reference=reference))
        (destination/f'unrelated-{position}.bin').write_bytes(raw)
        (destination/f'unrelated-{position}.tsv').write_bytes(frame[struct.calcsize('<BIII')+len(raw):])
        rows.append(row)
    return rows


def freeze_family(reference: Path, destination: Path) -> dict[str,object]:
    """Bind regular and negative complete observations to one stable source tree; e.g. freeze_family(source,fixtures)."""
    facts=freeze(reference,destination,fixture_family(reference));extra=additional_witnesses(reference,destination)
    if reference_snapshot(reference)!=facts['source_hashes']:raise ValueError('intersection source changed; expected original stable hashes')
    return facts|dict(unrelated=extra)


if __name__=='__main__':
    facts=freeze_family(REFERENCE,WORKSPACE/'tests/fixtures/intersection-wrapper-roles')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
