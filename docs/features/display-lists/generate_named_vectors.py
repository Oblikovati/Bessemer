"""Independent named-view wire vectors at every documented conditional boundary."""
from pathlib import Path
import struct
from generate_transform_vectors import compact,doubles
VERSIONS = [0,0x1be7,0x1be8,0x1bf4,0x1bf5,0x2001,0x2002,0x2003,0x2004,0x2007,0x2008,0x200d,0x200e,19000,0xffffffff]


def cstring(text: str, wide: bool = False) -> bytes:
    raw = text.encode("utf-16le" if wide else "latin-1")
    count = len(raw)//(2 if wide else 1)
    return (b"\xff\xfe\xff" if wide else b"")+b"\xff\xff\xff"+struct.pack("<I",count)+raw


def plane(version: int) -> bytes:
    raw = doubles(3)+doubles(3,3)+doubles(1,6)+struct.pack("<i",-17)
    if version>0x2001:
        raw += struct.pack("<ii",-123,456)
    if version>0x2007:
        raw += struct.pack("<I",0xfedcba98)
    return raw


def selections(flag: int) -> bytes:
    raw = struct.pack("<i",flag)
    if flag:
        raw += struct.pack("<iii",-400,-999,2)+cstring("near\0Cé")+cstring("鋼\0λ",True)
    return raw


def named_view(version: int, scenario: int) -> bytes:
    raw = cstring("鋼\0λ",True)+compact(2,0x1f)+compact(0,0x4a)+cstring("é\0kept")
    plane_count = 2 if scenario>=2 else 0
    if version>0x1bf4:
        raw += struct.pack("<i",plane_count)+plane(version)*plane_count
        if plane_count and version>0x2003:
            raw += selections(-1 if scenario==3 else 0)
    if version>0x200d:
        raw += struct.pack("<ii",-2147483648,2147483647)
    return raw


def collection(version: int, scenario: int) -> bytes:
    if version<=0x1be7:
        return b""
    count = [0,1,1,2][scenario]
    return struct.pack("<i",count)+named_view(version,scenario)*count


if __name__ == "__main__":
    records = [(version,scenario,collection(version,scenario)) for version in VERSIONS for scenario in range(4)]
    output = bytearray(struct.pack("<I",len(records)))
    for version,scenario,payload in records:
        output.extend(struct.pack("<IBI",version,scenario,len(payload))+payload)
    Path("tests/fixtures/display-lists/named-views.bin").write_bytes(output)
    print(f"{len(records)} vectors, {len(output)} bytes")
