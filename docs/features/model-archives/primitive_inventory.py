"""Observe reference primitives without supplying a production model decoder."""
from contextlib import contextmanager
from dataclasses import dataclass
from pathlib import Path
from typing import Callable, Iterator, Protocol
import traceback


class ReferenceCursor(Protocol):
    b: bytes
    p: int


PrimitiveTake = Callable[[ReferenceCursor, str, int], object]
PrimitiveRaw = Callable[[ReferenceCursor, int], bytes]


@dataclass(frozen=True)
class PrimitiveRead:
    start: int
    end: int
    kind: str
    raw_hex: str
    source_file: str
    source_function: str
    source_line: int


def reference_callsite() -> tuple[str, str, int]:
    """Locate the serializer above the observation hooks; e.g. reference_callsite()."""
    ignored = {Path(__file__).name, "swarchive.py"}
    callers = [frame for frame in traceback.extract_stack()
               if Path(frame.filename).name not in ignored]
    if not callers:
        raise ValueError("reference stack []; expected a serializer caller")
    caller = callers[-1]
    return Path(caller.filename).name, caller.name, caller.lineno


def append_primitive(reads: list[PrimitiveRead], expected: bytes,
                     cursor: ReferenceCursor, start: int, kind: str) -> None:
    """Retain reads from one stream; e.g. append_primitive(rows, raw, cursor, 0, 'i')."""
    if cursor.b != expected:
        return
    end = cursor.p
    if not 0 <= start <= end <= len(expected):
        raise ValueError(f"primitive range {start}..{end}; expected within 0..{len(expected)}")
    source_file, source_function, source_line = reference_callsite()
    reads.append(PrimitiveRead(start, end, kind, expected[start:end].hex(),
                               source_file, source_function, source_line))


@contextmanager
def observe_primitives(archive_type: type, expected: bytes) -> Iterator[list[PrimitiveRead]]:
    """Restore reference methods even on failure; e.g. with observe_primitives(Archive, raw)."""
    reads: list[PrimitiveRead] = []
    take = primitive_take(getattr(archive_type, "_take"), reads, expected)
    raw = primitive_raw(getattr(archive_type, "raw"), reads, expected)
    with replaced_methods(archive_type, take, raw):
        yield reads


def primitive_take(take: PrimitiveTake, reads: list[PrimitiveRead], expected: bytes) -> PrimitiveTake:
    """Observe scalar bit patterns; e.g. primitive_take(Archive._take, rows, raw)."""
    def observed_take(cursor: ReferenceCursor, kind: str, size: int) -> object:
        start = cursor.p
        value = take(cursor, kind, size)
        append_primitive(reads, expected, cursor, start, kind)
        return value
    return observed_take


def primitive_raw(raw: PrimitiveRaw, reads: list[PrimitiveRead], expected: bytes) -> PrimitiveRaw:
    """Observe framed raw fields; e.g. primitive_raw(Archive.raw, rows, raw)."""
    def observed_raw(cursor: ReferenceCursor, size: int) -> bytes:
        start = cursor.p
        value = raw(cursor, size)
        append_primitive(reads, expected, cursor, start, "raw")
        return value
    return observed_raw


@contextmanager
def replaced_methods(archive_type: type, take: PrimitiveTake, raw: PrimitiveRaw) -> Iterator[None]:
    """Scope external hooks; e.g. with replaced_methods(Archive, take, raw)."""
    originals = getattr(archive_type, "_take"), getattr(archive_type, "raw")
    setattr(archive_type, "_take", take)
    setattr(archive_type, "raw", raw)
    try:
        yield
    finally:
        setattr(archive_type, "_take", originals[0])
        setattr(archive_type, "raw", originals[1])


def verify_primitive_coverage(expected: bytes, reads: list[PrimitiveRead]) -> None:
    """Require a byte-exact partition; e.g. verify_primitive_coverage(raw, reads)."""
    next_offset = 0
    for read in reads:
        if read.start != next_offset or not read.start <= read.end <= len(expected):
            raise ValueError(f"primitive range {read.start}..{read.end}; expected start {next_offset} and end <= {len(expected)}")
        if bytes.fromhex(read.raw_hex) != expected[read.start:read.end]:
            raise ValueError(f"primitive bytes at {read.start}: {read.raw_hex}; expected {expected[read.start:read.end].hex()}")
        next_offset = read.end
    if next_offset != len(expected):
        raise ValueError(f"primitive end {next_offset}; expected stream end {len(expected)}")
