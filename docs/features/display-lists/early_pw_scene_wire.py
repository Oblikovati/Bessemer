"""Independent early PW bodies in the generic CObject role of bucket thread keys (#41)."""
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_transform_vectors import doubles
from generate_shaded_vectors import ObjectBuilder
import struct


def pw_wall_record(version: int, index: int, dense: bool) -> bytes:
    key = integers(-7 if index%2==0 else 9) if version>=3576 else b""
    names = cstring("")+cstring("") if dense else cstring(f"wall {index} 鋼",True)+cstring("info é\0")
    suffix = integers(44) if version>=3601 else b""
    return key+names+integers(-18,28,-32)+suffix


def pw_background(version: int) -> bytes:
    raw = doubles(3)+doubles(3,3)+cstring("background 鋼",True)+doubles(1,6)+integers(9,-10)
    raw += doubles(3,1)+doubles(3,4)+cstring("environment",True)+doubles(2,7)+integers(-11,12)
    raw += doubles(3,2)+doubles(2,5)+integers(-13,14)
    if version>3595:raw += integers(-15,16,-17)
    if version>3897:raw += integers(18)
    return raw


def early_pw_body(version: int, scenario: int) -> bytes:
    variant = scenario-224
    if variant not in range(5):raise ValueError(f"PW scenario {scenario}; expected 224..228")
    count = -3 if variant==2 else 256 if variant>=3 else 3
    raw = b"".join(cstring(value,True) for value in ["C:\\example\\scene","display 鋼","lights"])
    raw += integers(-2 if variant in (1,4) else -1,2,-3,4,count)
    raw += b"".join(pw_wall_record(version,index,variant>=3) for index in range(max(count,0)))
    raw += doubles(4,1)+integers(-5,6,-7,8)+pw_background(version)
    return raw


def pw_bucket_body(builder: ObjectBuilder, version: int, scenario: int) -> bytes:
    if not 2771<=version<=4300:raise ValueError(f"PW bucket version {version}; expected 2771..4300")
    raw = doubles(3)+integers(-1,2)+doubles(1)+integers(-3,0)
    raw += integers(*([0]*11))+bytes(4)+integers(0)+bytes(2)
    identity = builder.next_index+1
    raw += integers(1)+builder.object("pwSceneData_c",lambda _:early_pw_body(version,scenario))+integers(0)
    raw += integers(1)+struct.pack("<H",identity)+integers(0)+integers(-2,0)
    raw += integers(0,0,-1,2,-3,4,-5,6,-7,0)
    return raw+(bytes(2) if version>3216 else b"")
