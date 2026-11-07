"""Observe complete corner and hem graphs with actual independent readers (#54)."""
from functools import partial
from pathlib import Path
from typing import cast
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for folder in ('trim-edge-identifiers','base-flanges','edge-flanges','corner-hems'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/folder))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_edge_flanges import configure as configure_edges
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from hem_fixture_bytes import CLASSES,VERSIONS,EDITED_KINDS,hem_graph


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Bind both native geometry aliases and every complete hem serializer; e.g. configure(cursor,source)."""
    configure_edges(cursor,reference)
    corners,hems=(source_module(name,reference) for name in ('cornerfeatures','hemfeatures'))
    readers=(corners.read,corners.face_geometry,corners.face_geometry,hems.read,hems.data,
             hems.edge_chain_data,hems.edge_chain,hems.chain_element,hems.relief)
    cursor.readers.update(zip(CLASSES,readers))


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require actual reader EOF and preserved repeated roots; e.g. verify(source,raw,19000,3,5)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'hem end {cursor.p}; expected {len(raw)}')
    if style not in (2,5):return
    items=root['items']
    if items[0] is items[1] or items[0] is not items[2]:raise ValueError('hem identities; expected distinct second and repeated first roots')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Cross native classes with inherited boundaries and pointer styles; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=hem_graph,verify=verify,configure=partial(configure,reference=reference),
        magic=b'CHM1',kinds=9,versions=VERSIONS,edited_kinds=EDITED_KINDS)
