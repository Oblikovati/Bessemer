"""Independent wire vectors for both native transform overloads (specification 11.6)."""
from pathlib import Path
import struct
BITS = [0,0x8000000000000000,0x7ff0000000000000,0xfff0000000000000,0x7ff8000000000123,0x7ff0000000000042,1,0x7fefffffffffffff]


def doubles(count: int, start: int = 0) -> bytes:
    return b"".join(struct.pack("<Q",BITS[(start+index)%len(BITS)]) for index in range(count))


def compact(presence: int, flags: int) -> bytes:
    result = bytes([presence])+(doubles(9) if presence else b"")
    result += doubles(3,9)+doubles(1,12)+bytes([flags])
    selector = flags & 0xf0
    if selector in (0x10,0x20):
        result += doubles(4,13)
        if selector == 0x20:
            result += doubles(2,17)
    if selector == 0x40:
        result += struct.pack("<i",-1234567)+doubles(25,19)
    return result


def fixed_transform() -> bytes:
    return doubles(9)+struct.pack("<ii",-1,2147483647)+doubles(3,9)+doubles(1,12)+struct.pack("<ii",-2147483648,123456789)


if __name__ == "__main__":
    records = [(0,compact(presence,flags)) for presence in (0,1,2,255) for flags in range(256)]
    fixed = fixed_transform()
    assert len(fixed) == 120
    records.append((1,fixed))
    output = bytearray(struct.pack("<I",len(records)))
    for kind,payload in records:
        output.extend(struct.pack("<BI",kind,len(payload))+payload)
    root = Path("tests/fixtures/display-lists")
    root.mkdir(parents=True,exist_ok=True)
    root.joinpath("transforms.bin").write_bytes(output)
    print(f"{len(records)} vectors, {len(output)} bytes")
