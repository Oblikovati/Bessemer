"""Literal complete edge records, independent of Bessemer (#204)."""
import struct
from pathlib import Path

REFERENCE: Path | None = None
FLAGS = (0, 8, 14, -2147483640)
COUNTS = (-2147483648, -1, 0, 1, 2)
CLASSES = ('moCompEdge_c', 'moCompSilhEdge_c')


def edge_record(kind: int, flags: int, count: int, identity: int) -> bytes:
    """Use the documented147-byte empty-base body and explicit self aliases; e.g. edge_record(0,8,-1,2)."""
    name = CLASSES[kind].encode('ascii')
    body = bytearray(147)
    struct.pack_into('<i', body, 87, flags)
    struct.pack_into('<6dI', body, 91, -0.0, 1, 2, 3, 4, 5, 0xfedcba98)
    struct.pack_into('<i', body, 143, count)
    references = (struct.pack('<HH', 0, identity))[:4 if count > 1 else 2 if count > 0 else 0]
    pair = struct.pack('<HH', identity, 0) if flags & 6 != 6 and flags & 8 else b''
    suffix = bytes(4) if kind else b''
    return struct.pack('<HHH', 65535, 7, len(name)) + name + body + references + pair + suffix


def owned_edge_graph(kind: int, flags: int, count: int) -> bytes:
    """Retain the blank configuration's97 imported slots; e.g. owned_edge_graph(1,8,-1)."""
    return struct.pack('<iH', -123, 1) + edge_record(kind, flags, count, 98)
