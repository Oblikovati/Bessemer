"""Independent complete wrapper-family regressions at three end-specification callers (#46)."""
from pathlib import Path
from functools import partial
from typing import cast
import json
import struct
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','solid-references','sketch-wrapper-roles','projected-line-subtypes','end-spec-wrapper-roles'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from freeze_blank_part import source_module, reference_snapshot
from object_fixture_observer import ReferenceObjectCursor, object_cursor, observe_object_body
from end_wrapper_bytes import CLASSES, VERSIONS, end_wrapper_object


def source_surface(cursor: ReferenceObjectCursor, schema: int=1, *, reference: Path) -> object:
    """Adapt only the actual complete base-surface reader; e.g. source_surface(cursor,reference=source)."""
    return source_module('modelreferences',reference).surface(cursor)


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Retain real complete dependency bodies in one copied cursor map; e.g. configure(cursor,source)."""
    source_module('sketchprefix',reference).install_scoped_readers(cursor)
    cursor.readers['moSurfaceRef_w']=partial(source_surface,reference=reference)
    bodies=source_module('bodyfeatures',reference)
    cursor.readers.update(moEndSpec_c=bodies.end_spec,moFromEndSpec_c=bodies.from_end_spec,moRevEndSpec_c=bodies.revolution_end_spec)


def selected_specification(root: dict[str,object], caller: int, cached: bool) -> dict[str,object]:
    """Select the actual containing end specification; e.g. selected_specification(root,0,True)."""
    if not cached or caller==1:return root
    if caller==0:return cast(dict[str,object],cast(dict[str,object],root['extrusion_suffix'])['end_spec'])
    return cast(dict[str,object],cast(dict[str,object],root['revolution_suffix'])['end_spec'])


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require actual fresh/cached target classes and repeated owners; e.g. verify_identity(source,raw,19000,13,1)."""
    root=cast(dict[str,object],object_cursor(reference,raw,version,partial(configure,reference=reference)).object())
    seed=None;caller=style//2
    if style%2:
        seed,root,second,other,repeated=cast(list[dict[str,object]],root['items'])
        if root is other or root is not repeated or seed is second:raise ValueError('end-wrapper roots; expected distinct owners/targets and repeated first owner')
    selected=selected_specification(root,caller,bool(style%2))
    key=('reference_78','reference','reference_118')[caller]
    target=cast(dict[str,object],selected[key])
    if target['_class']!=CLASSES[kind] or (seed is not None and target is not seed):raise ValueError('end wrapper; expected exact class and actual cached target')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Resolve complete source functions once for the selected root; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=end_wrapper_object,verify=verify_identity,configure=partial(configure,reference=reference),
        magic=b'EWR1',kinds=len(CLASSES),versions=VERSIONS,edited_kinds=tuple(range(len(CLASSES))))


def additional_witnesses(reference: Path, destination: Path) -> list[dict[str,object]]:
    """Separate null preservation and complete unrelated objects from role acceptance; e.g. additional_witnesses(source,fixtures)."""
    rows=[]
    for style in range(6):
        for null in ((False,True) if style%2==0 else (False,)):
            label=('null' if null else 'unrelated')+'-'+str(style)
            raw,slots=end_wrapper_object(0,19000,style,null=null,foreign=not null)
            frame,row=observe_object_body(reference,raw,slots,style,19000,label,partial(configure,reference=reference))
            (destination/(label+'.bin')).write_bytes(raw)
            (destination/(label+'.tsv')).write_bytes(frame[struct.calcsize('<BIII')+len(raw):])
            rows.append(row)
    return rows


if __name__=='__main__':
    destination=WORKSPACE/'tests/fixtures/end-spec-wrapper-roles'
    facts=freeze(REFERENCE,destination,fixture_family(REFERENCE))
    extra=additional_witnesses(REFERENCE,destination)
    if reference_snapshot(REFERENCE)!=facts['source_hashes']:raise ValueError('end wrapper source; expected stable generation and witnesses')
    Path(__file__).with_name('additional-reference.json').write_text(json.dumps(extra,indent=2)+'\n',encoding='utf-8',newline='\n')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
