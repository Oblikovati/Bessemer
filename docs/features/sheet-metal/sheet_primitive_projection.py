"""Expose documented raw-array bit words without changing reference reads (#54)."""
from collections.abc import Callable, Iterator
from contextlib import contextmanager
from dataclasses import replace
from pathlib import Path
from typing import cast
from freeze_blank_part import source_module
from primitive_inventory import PrimitiveRead, ReferenceCursor, verify_primitive_coverage


@contextmanager
def dynamic_payload_ranges(reference: Path, expected: bytes) -> Iterator[list[tuple[int, int]]]:
    """Observe actual dynamic-array payload bounds; e.g. with dynamic_payload_ranges(source, raw) as spans."""
    module = source_module('sheetfeatures', reference)
    original = cast(Callable[[ReferenceCursor], dict[str, object] | None], module.dynamic_array)
    ranges: list[tuple[int, int]] = []
    def record(cursor: ReferenceCursor) -> dict[str, object] | None:
        result = original(cursor)
        payload = cast(bytes, result['raw_data']) if result is not None else b''
        if cursor.b == expected and payload:
            ranges.append((cursor.p-len(payload), cursor.p))
        return result
    module.dynamic_array = record
    try:
        yield ranges
    finally:
        module.dynamic_array = original


def project_payload_words(raw: bytes, reads: list[PrimitiveRead], ranges: list[tuple[int, int]]) -> list[PrimitiveRead]:
    """Project only observed strided payloads into lossless 64-bit chunks; e.g. project_payload_words(raw, reads, ranges)."""
    nonempty = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, nonempty)
    targets = set(ranges)
    projected: list[PrimitiveRead] = []
    for read in nonempty:
        if (read.start, read.end) not in targets:
            projected.append(read)
            continue
        targets.remove((read.start, read.end))
        projected.extend(payload_word_spans(read))
    if targets:
        raise ValueError(f'dynamic payload ranges {targets}; expected exact independently observed raw-read spans')
    verify_primitive_coverage(raw, projected)
    return projected


def payload_word_spans(read: PrimitiveRead) -> list[PrimitiveRead]:
    """Retain original caller provenance and every payload bit; e.g. payload_word_spans(read)."""
    width = read.end-read.start
    if read.kind != 'raw' or width <= 0 or width % 8:
        raise ValueError(f'dynamic payload {(read.kind, read.start, read.end)}; expected a nonempty raw span divisible by eight')
    return [replace(read, start=start, end=start+8, kind='Q',
        raw_hex=read.raw_hex[(start-read.start)*2:(start-read.start+8)*2])
        for start in range(read.start, read.end, 8)]
