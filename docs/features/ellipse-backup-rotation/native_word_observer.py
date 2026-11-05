"""Observe the original ellipse scalar call without replacing a class reader (#192)."""
from contextlib import contextmanager
from dataclasses import dataclass
from types import FrameType
from typing import Iterator, cast
import sys

from primitive_inventory import PrimitiveTake, ReferenceCursor


@dataclass(frozen=True)
class NativeEllipseWord:
    object_identity: int
    offset: int
    value: int
    schema: int


def ellipse_object_frame(frame: FrameType) -> tuple[int, int] | None:
    """Require the exact unknown/ellipse/object call chain; e.g. ellipse_object_frame(frame)."""
    if frame.f_code.co_name != 'unknown' or frame.f_locals.get('name') != 'word_20':
        return None
    ellipse = frame.f_back
    if ellipse is None or ellipse.f_code.co_name != 'ellipse_backup':
        return None
    owner = ellipse.f_back
    assert owner is not None and owner.f_code.co_name == 'object'
    instance, schema = owner.f_locals.get('obj'), owner.f_locals.get('schema')
    assert isinstance(instance, dict) and instance.get('_class') == 'moEllipseBackedUpData_c'
    assert isinstance(schema, int)
    return id(instance), schema


def current_ellipse_owner() -> tuple[int, int] | None:
    """Retain a current physical Source object identity during its scalar read; e.g. current_ellipse_owner()."""
    frame = sys._getframe(1)
    try:
        while frame is not None:
            owner = ellipse_object_frame(frame)
            if owner is not None:
                return owner
            frame = frame.f_back
        return None
    finally:
        del frame


def ellipse_primitive_observer(take: PrimitiveTake, expected: bytes,
                               reads: list[NativeEllipseWord]) -> PrimitiveTake:
    """Delegate every primitive and capture only the qualified ellipse word; e.g. ellipse_primitive_observer(take, raw, rows)."""
    def observed(cursor: ReferenceCursor, kind: str, size: int) -> object:
        start = cursor.p
        value = take(cursor, kind, size)
        if cursor.b is not expected or kind != 'i' or size != 4:
            return value
        owner = current_ellipse_owner()
        if owner is not None:
            assert isinstance(value, int)
            reads.append(NativeEllipseWord(owner[0], start, value, owner[1]))
        return value
    return observed


@contextmanager
def observe_native_ellipse_words(archive_type: type,
                                 expected: bytes) -> Iterator[list[NativeEllipseWord]]:
    """Restore the original primitive method even on failure; e.g. with observe_native_ellipse_words(Archive, raw)."""
    original = cast(PrimitiveTake, getattr(archive_type, '_take'))
    reads: list[NativeEllipseWord] = []
    archive_type._take = ellipse_primitive_observer(original, expected, reads)
    try:
        yield reads
    finally:
        archive_type._take = original
