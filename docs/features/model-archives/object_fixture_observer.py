"""Observe complete reference objects while allowing explicit local reader registration (#54)."""
from collections.abc import Callable
from pathlib import Path
from typing import Protocol, cast
import hashlib
import struct
from freeze_blank_part import source_module
from primitive_inventory import observe_primitives, verify_primitive_coverage


class ReferenceObjectCursor(Protocol):
    p: int
    map: list[object]
    readers: dict[str, Callable[..., object]]
    feature_context: str
    model_root_class: str
    def object(self) -> object: ...


def object_cursor(reference: Path, raw: bytes, version: int,
                  configure: Callable[[ReferenceObjectCursor], None] | None) -> ReferenceObjectCursor:
    """Keep registrations local to one reference cursor; e.g. object_cursor(source, raw, 19000, None)."""
    objects, bodies = (source_module(name, reference) for name in ('mfcobjects', 'bodyfeatures'))
    cursor = cast(ReferenceObjectCursor, objects.ObjectArchive(raw, version))
    cursor.feature_context = 'ordinary_external_xml'
    cursor.model_root_class = 'moPart_c'
    bodies.install_scoped(cursor, overwrite=True)
    if configure is not None:
        configure(cursor)
    return cursor


def observe_object_body(reference: Path, raw: bytes, slots: int, kind: int, version: int,
                        label: str, configure: Callable[[ReferenceObjectCursor], None] | None = None
                        ) -> tuple[bytes, dict[str, object]]:
    """Require exact EOF, slot count and complete primitive coverage; e.g. observe_object_body(source, raw, 5, 0, 19000, 'shell')."""
    cursor = object_cursor(reference, raw, version, configure)
    archive = source_module('swarchive', reference)
    with observe_primitives(archive.Archive, raw) as reads:
        cursor.object()
    if cursor.p != len(raw) or len(cursor.map) != slots:
        raise ValueError(f'{label} end/map {(cursor.p, len(cursor.map))}; expected {(len(raw), slots)}')
    reads = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, reads)
    oracle = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in reads).encode()
    frame = struct.pack('<BIII', kind, version, len(raw), len(oracle))+raw+oracle
    return frame, dict(label=label, kind=kind, version=version, bytes=len(raw), slots=slots, reads=len(reads),
        sha256=hashlib.sha256(raw).hexdigest(), oracle_sha256=hashlib.sha256(oracle).hexdigest())
