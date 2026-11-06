"""Stable little-endian fixture framing shared by field and width oracles."""
import struct


def packed(values: tuple | list, shape: str) -> bytes:
    return struct.pack("<" + shape * len(values), *values)


def blob(raw: bytes) -> bytes:
    return struct.pack("<I", len(raw)) + raw


def optional(value: int | None, shape: str) -> bytes:
    return b"\0" if value is None else b"\1" + struct.pack("<" + shape, value)


def counted(values: tuple | list, shape: str) -> bytes:
    return struct.pack("<I", len(values)) + packed(values, shape)
