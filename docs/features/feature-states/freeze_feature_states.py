"""Freeze the complete native two-word feature-state body and pointer identities (#54)."""
from functools import partial
from pathlib import Path
from typing import cast
import struct
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for folder in ('trim-edge-identifiers','base-flanges','edge-flanges','feature-states'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/folder))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze
from freeze_blank_part import source_module
from edge_fixture_bytes import EdgeGraph,WORDS
from object_fixture_observer import ReferenceObjectCursor,object_cursor

VERSIONS=(0x27cc,0x335a,0x372f,0x42cb,0x42cc,19000,0xffffffff)


def state_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Retain shared and distinct complete state objects; e.g. state_graph(0,19000,5)."""
    graph=EdgeGraph(version,style,first_slot)
    def record(key: str) -> bytes:
        return graph.shared(key,'featureStateData_c',lambda:struct.pack('<2i',
            12345 if edited and key=='first' else WORDS[style],WORDS[(style+3)%6]))
    def array() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=graph.mapping.new('suObArray',array) if style in (2,5) else record('first')
    return raw,graph.mapping.slot+1


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install only the actual scoped native state reader; e.g. configure(cursor,source)."""
    cursor.readers['featureStateData_c']=source_module('flatpatterns',reference).feature_state


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require exact EOF and repeated-root identity; e.g. verify(source,raw,19000,0,5)."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cast(dict[str,object],cursor.object())
    if cursor.p!=len(raw):raise ValueError(f'feature-state end {cursor.p}; expected {len(raw)}')
    if style in (2,5) and (root['items'][0] is root['items'][1] or root['items'][0] is not root['items'][2]):
        raise ValueError('feature-state identities; expected distinct second and repeated first roots')


def fixture_family(reference: Path) -> ObjectFixtureFamily:
    """Keep every signed-word style across the ordinary version window; e.g. fixture_family(source)."""
    return ObjectFixtureFamily(build=state_graph,verify=verify,configure=partial(configure,reference=reference),
        magic=b'FST1',kinds=1,versions=VERSIONS,edited_kinds=(0,))
