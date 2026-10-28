"""Single-bend continuation and its explicit unsupported raw-body boundaries (#54)."""
from __future__ import annotations
import struct
from typing import TYPE_CHECKING
from freeze_views import transform
if TYPE_CHECKING:
    from edge_fixture_bytes import EdgeGraph


def direct_handle(style: int) -> bytes:
    """Keep escaped indices and signed handle fields without an outer tag; e.g. direct_handle(3)."""
    return struct.pack('<Hiii',0x777f,-1,-2147483648,2147483647) if style%2 else struct.pack('<H2i',7,-19,2147483647)


def raw_tail(version: int, style: int, edited: bool=False) -> bytes:
    """Retain nonpositive counts and every late presence gate; e.g. raw_tail(19000,4)."""
    count=(-2147483648,-1,0,0,-32769,0)[style]
    raw=struct.pack('<i',count)
    if version>0x335a:raw+=struct.pack('<i',count)
    if version>=0x334c:raw+=bytes(4)
    if version>=0x334f:raw+=bytes(4)
    if version>0x3734:raw+=struct.pack('<i',-1)
    return raw+struct.pack('<3i',12345 if edited else -2147483648,2147483647,-1)


def prefix(graph: EdgeGraph) -> bytes:
    """Keep the initial handle pointer separate from its later direct body; e.g. prefix(graph)."""
    raw=graph.marker() if graph.style in (1,3,5) else bytes(2)
    raw+=graph.marker()+graph.display(True)+graph.display()
    raw+=struct.pack('<HiQ',65535,-2147483648,graph.bits())
    raw+=struct.pack('<5i',-2147483648,2147483647,-1,0,-32769)
    return raw+graph.display()+graph.display()+struct.pack('<iB',-1,255)


def ordinary_handle(graph: EdgeGraph) -> bytes:
    """The helper-zero path contains a pointer, optional direct handle and tagged list; e.g. ordinary_handle(graph)."""
    if graph.extended:return b''
    present=graph.style in (3,5)
    return (graph.marker() if present else bytes(2))+(direct_handle(graph.style) if present else b'')+graph.listing()


def continuation(graph: EdgeGraph) -> bytes:
    """Preserve both compact transforms and the conic null pointer; e.g. continuation(graph)."""
    raw=graph.marker()+graph.component()+graph.component()+struct.pack('<i',-1)+bytes(2)
    raw+=transform(graph.style%2==1,(0,7,0x1a,0x2b,0x40,0x4c)[graph.style])+transform(False,7)
    raw+=struct.pack('<5i',-2147483648,2147483647,-1,0,-32769)
    raw+=graph.marker()+graph.component()+graph.component()
    return raw+(direct_handle(graph.style) if graph.style in (1,3,5) else b'')
