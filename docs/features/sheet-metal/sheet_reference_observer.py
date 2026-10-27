"""Observe complete sheet records and the reference's actual dynamic-array spans (#54)."""
from collections.abc import Callable
from functools import partial
from pathlib import Path
from typing import cast
import hashlib
import struct
from freeze_blank_part import source_module
from object_fixture_observer import ReferenceObjectCursor, object_cursor
from primitive_inventory import PrimitiveRead, observe_primitives
from sheet_primitive_projection import dynamic_payload_ranges, project_payload_words


def install_sheet_context(cursor: ReferenceObjectCursor, reference: Path, state: int, verified: bool) -> None:
    """Select explicit state and copied scoped readers; e.g. install_sheet_context(cursor, source, 1, True)."""
    components = source_module('modelcomponents', reference)
    setattr(cursor, 'sheet_metal_helper_850', state)
    setattr(cursor, 'sheet_metal_helper_verified', verified)
    cursor.readers['moCompSheetmetalFeat_c'] = components.component_sheet_metal


def observe_sheet_object(reference: Path, raw: bytes, slots: int, kind: int, version: int,
                         state: int, verify: Callable[[dict[str, object]], None]) -> tuple[bytes, dict[str, object]]:
    """Require exact EOF and slots while retaining original inventory hashes; e.g. observe_sheet_object(source, raw, 3, 0, 19000, 0, check)."""
    configure = partial(install_sheet_context, reference=reference, state=state, verified=True)
    cursor = object_cursor(reference, raw, version, configure)
    archive = source_module('swarchive', reference)
    with observe_primitives(archive.Archive, raw) as reads, dynamic_payload_ranges(reference, raw) as ranges:
        verify(cast(dict[str, object], cursor.object()))
    if cursor.p != len(raw) or len(cursor.map) != slots:
        raise ValueError(f'sheet end/map {(cursor.p, len(cursor.map))}; expected {(len(raw), slots)}')
    projected = project_payload_words(raw, reads, ranges)
    oracle = inventory_bytes(projected)
    frame = struct.pack('<BIII', kind, version, len(raw), len(oracle))+raw+oracle
    return frame, dict(kind=kind, version=version, bytes=len(raw), slots=slots, reads=len(projected),
        source_reads=sum(read.end > read.start for read in reads), raw_payload_ranges=[list(span) for span in ranges],
        source_inventory_sha256=hashlib.sha256(inventory_bytes(reads)).hexdigest(),
        sha256=hashlib.sha256(raw).hexdigest(), oracle_sha256=hashlib.sha256(oracle).hexdigest())


def inventory_bytes(reads: list[PrimitiveRead]) -> bytes:
    """Keep source primitive order without synthesized values; e.g. inventory_bytes(reads)."""
    return ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n'
        for read in reads if read.end > read.start).encode()
