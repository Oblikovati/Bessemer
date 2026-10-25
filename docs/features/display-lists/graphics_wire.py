"""Independent graphics-reference wire graphs; pointer cycles establish archive identity only."""
import struct
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_transform_vectors import doubles
from generate_shaded_vectors import ObjectBuilder


def ref(identity: int) -> bytes:
    return struct.pack("<H",identity)


def next_identity(builder: ObjectBuilder, name: str) -> int:
    identity = builder.next_index
    return identity+int(name not in builder.classes)


def wrapper(owner: int, has_plane: bool, edited: bool) -> bytes:
    raw = cstring("wrapper 鋼😀\0",True)+ref(owner)+integers(-3 if has_plane else 0)
    if not has_plane:
        return raw
    point = struct.pack("<d",23.0) if edited else doubles(1)
    return raw+point+doubles(5,1)


def magnetic(version: int, wrap: int, owner: int) -> bytes:
    raw = integers(-16)+ref(wrap)+ref(owner)+cstring("magnetic 鋼😀\0",True)
    return raw+(struct.pack("<Q",0x8000000000000000) if version>0x2bc3 else b"")


def manager(builder: ObjectBuilder, version: int, identity: int, empty: bool, edited: bool) -> bytes:
    if empty:
        return integers(-8,0,0)
    wrap = next_identity(builder,"moGraphicsRefWrapData_c")
    raw = integers(-8,-17)+builder.object("moGraphicsRefWrapData_c",lambda _:wrapper(identity,True,edited))
    magnet = next_identity(builder,"moGraphicsMagneticConnectRefData_c")
    raw += integers(4)+builder.object("moGraphicsMagneticConnectRefData_c",lambda _:magnetic(version,wrap,identity))
    raw += ref(magnet)+ref(0)+builder.object("moGraphicsRefWrapData_c",lambda _:wrapper(0,False,False))
    return raw
