"""Independent annotation-view bodies for every version boundary and collection shape."""
from pathlib import Path
import struct
from generate_transform_vectors import compact,doubles
from generate_named_vectors import cstring
VERSIONS = [0,0x1c39,0x1c3a,0x1c40,0x1c41,0x1fa8,0x1fa9,19000,0xffffffff]


def record(version: int, scenario: int) -> bytes:
    raw = cstring("鋼\0λ",True)+struct.pack("<iii",-99,2,-1)
    if version>0x1c40:
        raw += struct.pack("<i",-17)
    raw += cstring("second\0é")+compact(2,0x2f)+compact(0,0x4a)+cstring("retained é")
    count = 2 if scenario==2 else 0
    plane = doubles(3)+doubles(3,3)+doubles(1,6)+struct.pack("<i",-23)
    return raw+struct.pack("<i",count)+plane*count


def collection(version: int, scenario: int) -> bytes:
    if version<=0x1c39:
        return b""
    count = [0,1,2][scenario]
    raw = struct.pack("<i",count)+record(version,scenario)*count
    if version>0x1fa8:
        raw += struct.pack("<ii",-2147483648,2147483647)
    return raw


if __name__ == "__main__":
    records = [(version,scenario,collection(version,scenario)) for version in VERSIONS for scenario in range(3)]
    output = bytearray(struct.pack("<I",len(records)))
    for version,scenario,payload in records:
        output.extend(struct.pack("<IBI",version,scenario,len(payload))+payload)
    Path("tests/fixtures/display-lists/annotation-views.bin").write_bytes(output)
    print(f"{len(records)} annotation vectors, {len(output)} bytes")
