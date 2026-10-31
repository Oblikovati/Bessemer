"""Actual frozen readers and complete independent final-LBC result packets (#130)."""
from dataclasses import replace
import struct
import unittest
from cosworkspersist import read_lbc_order, read_legacy_midsurf_map
from final_cases import FinalCase, fields


def words(*values: int) -> bytes:
    """Encode fixed packet words with explicit two's-complement signed ordinals."""
    return b''.join(struct.pack('<I', value & 0xffffffff) for value in values)


def span(raw: bytes) -> bytes:
    """Retain complete bytes, including empty buffers."""
    return words(len(raw)) + raw


def validate(case: FinalCase) -> None:
    """Validate before deriving fields so hostile counts never drive unchecked iteration."""
    if case.profile == 0:
        record = read_lbc_order(case.raw, max_items=case.maximum, max_string_bytes=case.text_maximum)
        assert record.raw == case.raw
        assert [item.header + item.payload for item in record.items] == [raw for _, raw in fields(case)]
        return
    groups = read_legacy_midsurf_map(case.raw, max_records=case.maximum)
    actual = [struct.pack('<i', value) for group in groups for record in group for value in record]
    assert actual == [raw for _, raw in fields(case)]


def image(case: FinalCase) -> bytes:
    """Include every complete source byte, normalized field offset and stored pattern."""
    validate(case)
    entries = fields(case)
    return span(case.raw) + words(len(entries)) + b''.join(words(offset) + span(raw) for offset, raw in entries)


def projection(case: FinalCase) -> bytes:
    """Require independent rejection or exact full-image splice and reparsing."""
    if not case.accepted:
        with unittest.TestCase().assertRaises(ValueError):
            validate(case)
        return words(0)
    validate(case)
    if case.ordinal < 0:
        return words(1) + image(case)
    offset, previous = fields(case)[case.ordinal]
    result = replace(case, raw=case.raw[:offset] + case.replacement + case.raw[offset + len(previous):])
    changed = case.raw != result.raw
    history = [words(before, after, int(changed and index == case.ordinal)) for index, ((before, _), (after, _)) in enumerate(zip(fields(case), fields(result), strict=True))]
    return words(2) + span(case.raw) + image(result) + span(case.replacement) + words(int(changed), len(history)) + b''.join(history)


def packet(cases: list[FinalCase]) -> bytes:
    """Keep every caller context and independently checked full result."""
    entries = []
    for case in cases:
        header = words(case.profile, case.maximum, case.text_maximum, case.ordinal, int(case.accepted))
        entries.append(header + span(case.raw) + span(case.replacement) + span(projection(case)))
    return b'LBCF' + words(len(cases)) + b''.join(entries)
