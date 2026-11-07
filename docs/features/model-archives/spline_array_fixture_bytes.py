"""Independent spline interpolation and curve array wire assembly."""
import struct


def doubles(count: int) -> bytes:
    return b''.join(struct.pack('<Q', 0x8000000000000000 if index == 0 else 0x3ff0000000000000+index) for index in range(count))


def vectors(count: int) -> bytes:
    return doubles(3*count)


def interpolation(mode: int) -> bytes:
    if mode == 0:
        return struct.pack('<I', 0xffffffff)
    count, width = [(0, 0), (2, 0), (2, 2), (1, 3)][mode-1]
    out = struct.pack('<Ii2d', count, width, -0.0, 1.25)+doubles(count)
    out += struct.pack('<'+'H'*count, *[65535, 32768][:count])
    out += struct.pack('<'+'i'*count, *[-1, -2**31][:count])
    return out+doubles(count*width)


def secondary(count: int, width: int = -2**31) -> bytes:
    return struct.pack('<Ii2d', count, width, -0.0, 1.25)+doubles(count)


def curve(dimension: int, knots: int, points: int, periodic: int, tolerance: int) -> bytes:
    count = (dimension+1)*points+(1 if periodic else knots) if dimension else 0
    out = struct.pack('<4H', dimension, knots, points, periodic)+doubles(count)+struct.pack('<H', tolerance)
    return out+(struct.pack('<d', -0.0) if tolerance else b'')
