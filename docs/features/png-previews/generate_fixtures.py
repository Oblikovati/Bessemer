"""Owned PNG vectors: closed-form samples, independently filtered/interlaced wire encoding."""
from pathlib import Path
import struct
import zlib

ROOT = Path(__file__).resolve().parents[3] / "tests/fixtures/png"
COMBINATIONS = [(0,d) for d in (1,2,4,8,16)] + [(2,d) for d in (8,16)] + [(3,d) for d in (1,2,4,8)] + [(c,d) for c in (4,6) for d in (8,16)]
CHANNELS = {0:1,2:3,3:1,4:2,6:4}
PASSES = [(0,0,8,8),(4,0,8,8),(0,4,4,8),(2,0,4,4),(0,2,2,4),(1,0,2,2),(0,1,1,2)]


def chunk(kind: bytes, payload: bytes) -> bytes:
    framed = kind + payload
    return struct.pack(">I", len(payload)) + framed + struct.pack(">I", zlib.crc32(framed))


def values(width: int, height: int, color: int, depth: int) -> list[int]:
    maximum = (1 << depth) - 1
    return [((y * width + x) * 313 + channel * 191) & maximum
            for y in range(height) for x in range(width) for channel in range(CHANNELS[color])]


def packed(samples: list[int], depth: int) -> bytes:
    if depth == 16:
        return b"".join(struct.pack(">H", value) for value in samples)
    if depth == 8:
        return bytes(samples)
    bits = "".join(f"{value:0{depth}b}" for value in samples)
    bits += "0" * (-len(bits) % 8)
    return bytes(int(bits[i:i+8], 2) for i in range(0,len(bits),8))


def prediction(kind: int, a: int, b: int, c: int) -> int:
    if kind != 4:
        return (0, a, b, (a+b)//2)[kind]
    p = a+b-c
    return min(((abs(p-a),0,a),(abs(p-b),1,b),(abs(p-c),2,c)))[2]


def filtered(row: bytes, previous: bytes, stride: int, kind: int) -> bytes:
    encoded = [kind]
    for index, value in enumerate(row):
        a = row[index-stride] if index >= stride else 0
        b = previous[index]
        c = previous[index-stride] if index >= stride else 0
        encoded.append((value-prediction(kind,a,b,c)) & 255)
    return bytes(encoded)


def pass_bytes(width: int, height: int, color: int, depth: int, pattern: tuple[int,int,int,int], ordinal: int) -> bytes:
    x0,y0,dx,dy = pattern
    positions = list(range(x0,width,dx))
    if not positions:
        return b""
    result = b""
    previous = bytes((len(positions)*CHANNELS[color]*depth+7)//8)
    all_values = values(width,height,color,depth)
    for row,y in enumerate(range(y0,height,dy)):
        samples = [all_values[(y*width+x)*CHANNELS[color]+c] for x in positions for c in range(CHANNELS[color])]
        encoded = packed(samples,depth)
        result += filtered(encoded,previous,max(1,(CHANNELS[color]*depth+7)//8),(row+ordinal)%5)
        previous = encoded
    return result


def palette(depth: int) -> bytes:
    return bytes(value for i in range(1 << depth) for value in ((i*17)%256,(i*31)%256,(i*47)%256))


def transparency(color: int, depth: int) -> bytes:
    if color in (0,2):
        return bytes(2*CHANNELS[color])
    if color == 3:
        return bytes((i*37)%256 for i in range(1 << depth))
    return b""


def png(width: int, height: int, color: int, depth: int, interlace: int) -> bytes:
    patterns = PASSES if interlace else [(0,0,1,1)]
    raw = b"".join(pass_bytes(width,height,color,depth,p,i) for i,p in enumerate(patterns))
    parts = [chunk(b"IHDR",struct.pack(">IIBBBBB",width,height,depth,color,0,0,interlace)),chunk(b"vpAg",b"safe-private")]
    if color == 3:
        parts.append(chunk(b"PLTE",palette(depth)))
    if transparency(color,depth):
        parts.append(chunk(b"tRNS",transparency(color,depth)))
    compressed = zlib.compress(raw)
    middle = len(compressed)//2
    parts += [chunk(b"IDAT",compressed[:middle]),chunk(b"IDAT",b""),chunk(b"IDAT",compressed[middle:]),chunk(b"vpAG",b"unsafe-private"),chunk(b"IEND",b"")]
    return b"\x89PNG\r\n\x1a\n" + b"".join(parts)


def rgba(width: int, height: int, color: int, depth: int) -> bytes:
    source = values(width,height,color,depth)
    result = []
    maximum = (1 << depth)-1
    for offset in range(0,len(source),CHANNELS[color]):
        pixel = source[offset:offset+CHANNELS[color]]
        scaled = [value*65535//maximum for value in pixel]
        if color == 0:
            result.extend([scaled[0]]*3+[0 if pixel[0]==0 else 65535])
        elif color == 2:
            result.extend(scaled+[0 if pixel==[0,0,0] else 65535])
        elif color == 3:
            i = pixel[0]
            result.extend([((i*k)%256)*257 for k in (17,31,47,37)])
        elif color == 4:
            result.extend([scaled[0]]*3+[scaled[1]])
        else:
            result.extend(scaled)
    return b"".join(struct.pack(">H",value) for value in result)


def generate() -> None:
    ROOT.mkdir(parents=True,exist_ok=True)
    rows = []
    for width,height in ((9,9),(1,1),(2,3),(4,1)):
        for color,depth in COMBINATIONS:
            for interlace in (0,1):
                name = f"c{color}-d{depth}-i{interlace}-{width}x{height}"
                (ROOT / f"{name}.png").write_bytes(png(width,height,color,depth,interlace))
                (ROOT / f"{name}.samples").write_bytes(b"".join(struct.pack(">H",value) for value in values(width,height,color,depth)))
                (ROOT / f"{name}.rgba").write_bytes(rgba(width,height,color,depth))
                rows.append(name+"\n")
    (ROOT / "cases.txt").write_text("".join(rows),encoding="utf-8",newline="\n")


if __name__ == "__main__":
    generate()
