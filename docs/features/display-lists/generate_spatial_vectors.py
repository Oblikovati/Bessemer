"""Independent 3D-view wire bodies at every conditional boundary, with raw BITMAP framing."""
from pathlib import Path
import struct
from generate_transform_vectors import compact,doubles
from generate_named_vectors import cstring
VERSIONS = [0,0x1c18,0x1c19,0x1c1e,0x1c1f,0x1c42,0x1c43,0x1fb5,0x1fb6,0x2012,0x2013,0x3331,0x3332,0x46c9,0x46ca,19000,0xffffffff]


def plane(version: int) -> bytes:
    raw = doubles(3)+doubles(3,3)+doubles(1,6)+struct.pack("<i",-11)
    if version>0x2012:
        raw += struct.pack("<iiI",-123,456,0xfedcba98)
    return raw


def bitmap(scenario: int) -> bytes:
    if scenario==1:
        return struct.pack("<iiiHH",42,0,17,0,64)
    return struct.pack("<iiiHH",-99,-2,-3,0,64)+bytes([0,255,13,10,128,17])


def tail(version: int, planes: int) -> bytes:
    raw = struct.pack("<i",-27) if planes and version>0x2012 else b""
    if version>0x1fb5:
        raw += struct.pack("<iii",-468,-2147483648,2147483647)
    if version>0x3331:
        raw += cstring("unknown text 鋼",True)
    return raw


def record(version: int, scenario: int) -> bytes:
    raw = cstring("鋼\0λ",True)
    if version>0x46c9:
        raw += struct.pack("<i",-17)
    raw += compact(2,0x2f)+compact(0,0x4a)+cstring("é\0kept")
    if version>0x1c1e:
        raw += bitmap(scenario)
    if version>0x1c42:
        raw += cstring("raw ids -1,  02")
    count = 2 if scenario==2 else 0
    return raw+struct.pack("<i",count)+plane(version)*count+tail(version,count)


def collection(version: int, scenario: int) -> bytes:
    if version<=0x1c18:
        return b""
    return struct.pack("<i",scenario)+record(version,scenario)*scenario


if __name__ == "__main__":
    records = [(version,scenario,collection(version,scenario)) for version in VERSIONS for scenario in range(3)]
    output = bytearray(struct.pack("<I",len(records)))
    for version,scenario,payload in records:
        output.extend(struct.pack("<IBI",version,scenario,len(payload))+payload)
    Path("tests/fixtures/display-lists/spatial-views.bin").write_bytes(output)
    print(f"{len(records)} spatial vectors, {len(output)} bytes")
