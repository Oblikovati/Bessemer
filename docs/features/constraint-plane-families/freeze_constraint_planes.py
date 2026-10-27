"""Freeze complete plane families against real scoped source readers (#54, #46)."""
from pathlib import Path
from typing import cast
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for folder in ('trim-edge-identifiers','reference-axes','reference-axis-owners','plane-point-roles','projected-line-subtypes','constraint-plane-families'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/folder))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor, object_cursor
from constraint_plane_bytes import CLASSES, VERSIONS, plane_object


def source_surface(cursor: ReferenceObjectCursor, schema: int=1) -> object:
    """Adapt the actual one-argument surface base reader; e.g. source_surface(cursor)."""
    return source_module('modelreferences',REFERENCE).surface(cursor)


def configure(cursor: ReferenceObjectCursor) -> None:
    """Register only native-bound complete readers on this cursor; e.g. configure(cursor)."""
    planes=source_module('referenceplanes',REFERENCE);planes.install_scoped(cursor)
    constraints=source_module('constraintplanes',REFERENCE)
    for kind,name in enumerate(CLASSES[:17]):
        if kind==0:reader=planes.point_surface_data
        elif kind==3:reader=constraints.coincident_line_at_angle
        elif kind in (7,8):reader=constraints.two_lines
        elif kind<9:reader=constraints.data
        elif kind==9:reader=constraints.interface
        elif kind in (10,11,12):reader=constraints.geometry
        elif kind==16:reader=constraints.angle_geometry
        else:reader=constraints.midplane_geometry
        cursor.readers[name]=reader
    cursor.readers['moSurfaceRef_w']=source_surface
    cursor.readers['sgExtEnt_c']=source_module('sketchprefix',REFERENCE).external_entity


def verify_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require distinct roots, actual shared slots and native flag interpretation; e.g. verify_identity(source,raw,19000,13,2)."""
    root=cast(dict[str,object],object_cursor(reference,raw,version,configure).object())
    if style in (2,5):
        first,second,repeated=cast(list[dict[str,object]],root['items'])
        if first is second or repeated is not first:raise ValueError('plane roots; expected distinct equal roots and repeated first')
        root=first
    if kind<9 and style in (4,5):root=cast(dict[str,object],root['plane_data'])
    if kind==9 and style and root['geometry_08'] is not root['geometry_18']:
        raise ValueError('plane interface; expected actual shared first/third geometry')
    if kind in (13,14,15,16):
        flag='40' if kind==16 else '38'
        if root['loaded_flag_'+flag]!=(root['raw_flag_'+flag]==1):raise ValueError('geometry flag; expected native equality with one')


FAMILY=ObjectFixtureFamily(build=plane_object,verify=verify_identity,configure=configure,magic=b'CPL1',
                          kinds=18,versions=VERSIONS,edited_kinds=tuple(range(18)))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/constraint-plane-families',FAMILY)
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
