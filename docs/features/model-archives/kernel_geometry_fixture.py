"""Independent structural transmit/frame vectors, not a geometric validity oracle."""
import struct
import zlib

GUID = bytes.fromhex('231dd571da8148a2a85898b21b89ef99')


def string32(value: bytes) -> bytes:
    return struct.pack('>I', len(value))+value


def string8(value: bytes) -> bytes:
    return bytes([len(value)])+value


def transmit(identifier: int = -7, bodies: int = 1, bare: bool = False, title: bytes = b': TRANSMIT FILE (partition) structural fixture') -> bytes:
    out = b'PS'+string32(title)+string32(b'SCH_3701250_37102_13006')
    out += struct.pack('>HI', 65535, 0)
    for index in range(bodies):
        out += struct.pack('>H', 60000)
        if index == 0:
            out += bytes([1])+string8(b'BODY')+string8(b'Structural fixture; no topology claim')
            out += string8(b'identifier')+struct.pack('>HH', 0, 1)+string8(b'd')+bytes([0])
        out += struct.pack('>Hi', index+2, identifier)
    out += struct.pack('>HH', 1, 1)
    if bare:
        raise ValueError('bare True; expected neutral-only fixture builder')
    return out


def compressed(payload: bytes) -> bytes:
    encoded = zlib.compress(payload, 6)
    chunk = GUID+struct.pack('<II', len(payload), len(encoded))+encoded+bytes(8)
    return struct.pack('<I', len(chunk))+chunk


def frame(identifier: int = -7, bodies: int = 1) -> bytes:
    payload = compressed(transmit(identifier, bodies))
    return b'\0'+struct.pack('<iB', len(payload)+1, 0)+payload
