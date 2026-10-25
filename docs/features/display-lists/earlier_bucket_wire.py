"""Earlier bucket records preserve pointers discarded by the reference object graph."""
import struct
from generate_component_vectors import integers
from generate_transform_vectors import doubles
from transition_bucket_wire import arc_payload,edge
import bucket_vector_wire as wire


def coordinates(count: int, version: int) -> bytes:
    if version>=0x2f9a:
        return doubles(count)
    bits = [0,0x80000000,0x7f800000,0xff800000,0x7fc00123,0x7f800042,1,0x7f7fffff]
    return b"".join(struct.pack("<I",bits[index%len(bits)]) for index in range(count))


def polygon_encoder(builder: object, version: int) -> object:
    discarded: list[int] = []
    def encode(count: int, dimensions: int, kind: int = 1, arc: bool = True, line: bool = True) -> bytes:
        raw = integers(-1,2,-17,count)
        if count<=0:
            return raw
        raw += coordinates(count*dimensions,version)+bytes([255 if line else 0])+(wire.line_property() if line else b"")
        raw += integers(kind)
        if kind in (1,2,3):
            raw += integers(-2 if kind==2 else 2)+(b"" if kind==2 else doubles(2))
        raw += integers(-3,-6,0,7)+(integers(6) if version>0x12dd else b"")
        raw += (integers(-7) if version>=0x23b4 else b"")+(integers(-8,9,-10,0,-11) if version>0x2789 else b"")
        if version<0x2f63:
            return raw
        if version<=0x2f95:
            return raw+discarded_pointer(builder,version,discarded)
        return raw+integers(-21,-3 if arc else 0)+(arc_payload(version) if arc else b"")+integers(-2,-17,42)
    return encode


def discarded_pointer(builder: object, version: int, identities: list[int]) -> bytes:
    if identities:
        return struct.pack("<H",identities[0])
    identity = builder.next_index+("gcBucketEdgeData_c" not in builder.classes)
    identities.append(identity)
    return builder.object("gcBucketEdgeData_c",lambda _:edge(version))


def bucket_encoder(original: object, version: int) -> object:
    def encode(builder: object, identity: int, two_dimensional: bool, empty: bool) -> bytes:
        raw = original(builder,identity,two_dimensional,empty)
        removed = 4+(2 if empty else 42)*(version<=0x2faa)+4*(version<=0x2f8b)
        removed += (4 if empty else 60) if version<=0x2789 else 0
        removed += 4*(version<=0x1c3d)
        raw = raw[:-removed]
        if version<=0x184c:
            halo = wire.poly_list([] if empty else [wire.poly(1,3,9,False,False)])
            assert raw.endswith(halo)
            raw = raw[:-len(halo)]
        if version<=0x147d:raw = raw[:-4]
        return raw
    return encode
