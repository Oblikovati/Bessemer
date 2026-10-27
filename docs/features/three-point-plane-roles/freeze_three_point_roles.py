"""Independent complete three-point plane regressions, including rejected role witnesses (#46)."""
from pathlib import Path
from functools import partial
from typing import cast
import json
import struct
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','reference-axes','reference-axis-owners','plane-point-roles','three-point-plane-roles'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from freeze_plane_point_roles import install_planes
from freeze_blank_part import reference_snapshot
from object_fixture_observer import object_cursor, observe_object_body
from three_point_bytes import VERSIONS, three_point_object


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Verify actual aliases independently of expected-class enforcement; e.g. verify_identity(source,raw,19000,0,2)."""
    cursor=object_cursor(reference,raw,version,partial(install_planes,reference=reference))
    root=cast(dict[str,object],cursor.object())
    if style in (2,5):
        seed,selected,repeated=cast(list[dict[str,object]],root['items'])
        if selected is not repeated:raise ValueError('three-point roots; expected repeated exact target')
        root=cast(dict[str,object],selected['plane_data']) if style==5 else selected
        if root['point_reference_88'] is not seed:raise ValueError('three-point seed; expected actual earlier cached target')
    if style==4:
        root=cast(dict[str,object],root['plane_data'])
        points=[cast(dict[str,object],root['point_reference_'+offset]) for offset in ('88','90','98')]
        if [p['_class'] for p in points]!=['moPointRef_w','moEndPointRef_w','moLoftSynchRef_c'] or len({id(p) for p in points})!=3:raise ValueError('mixed points; expected three distinct complete point-family objects')
        return
    if style and root['point_reference_88'] is not root['point_reference_98']:raise ValueError('third point; expected actual repeated target')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Resolve scoped readers against one explicit source root; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=three_point_object,verify=verify_identity,
        configure=partial(install_planes,reference=reference),magic=b'TPP1',kinds=1,versions=VERSIONS,edited_kinds=(0,))


def negative_roles(reference: Path, destination: Path) -> list[dict[str,object]]:
    """Require byte-valid complete unrelated wrappers before Rust rejects their point roles; e.g. negative_roles(source,fixtures)."""
    facts=[]
    for style,label in ((1,'unrelated-fresh'),(2,'unrelated-cached')):
        raw,slots=three_point_object(0,19000,style,point_override=3)
        frame,observed=observe_object_body(reference,raw,slots,style,19000,label,partial(install_planes,reference=reference))
        (destination/(label+'.bin')).write_bytes(raw)
        (destination/(label+'.tsv')).write_bytes(frame[struct.calcsize('<BIII')+len(raw):])
        facts.append(observed)
    return facts


if __name__=='__main__':
    destination=WORKSPACE/'tests/fixtures/three-point-plane-roles'
    facts=freeze(REFERENCE,destination,fixture_family(REFERENCE))
    negatives=negative_roles(REFERENCE,destination)
    if reference_snapshot(REFERENCE)!=facts['source_hashes']:raise ValueError('three-point source; expected stable positive and negative readers')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    Path(__file__).with_name('negative-reference.json').write_text(json.dumps(negatives,indent=2)+'\n',encoding='utf-8',newline='\n')
