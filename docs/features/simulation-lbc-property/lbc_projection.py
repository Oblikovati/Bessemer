"""Actual frozen reader projections and independent fixed-writer/splice expectations."""
import struct
import unittest
from cosworkspersist import read_lbc_property
from lbc_cases import LbcCase
from owned_lbc_property import LAYOUTS
from owned_persistence import word, span


def fields(case: LbcCase) -> tuple[tuple[int, ...], tuple[int, ...]]:
    """Use the actual supplied reader context or the independent fixed writer order."""
    if case.fixed_writer:
        if len(case.raw) != 216:
            raise ValueError(f'{len(case.raw)} bytes; expected fixed writer size 216')
        return LAYOUTS[216], tuple(value[0] for value in struct.iter_unpack('<i', case.raw))
    record = read_lbc_property(case.raw, case.version)
    assert record.raw == case.raw
    return record.destinations, record.values


def coverage(size: int, count: int, edit: bool) -> bytes:
    """Retain complete scope, completion, zero pattern error and all exclusions."""
    return b''.join(word(value) for value in (size, count, 1, 0, 7, int(edit)))


def image(case: LbcCase) -> bytes:
    """Serialize every stored word and both physical and runtime offsets."""
    destinations, values = fields(case)
    assert b''.join(word(value) for value in values) == case.raw
    packet = word(1) + word(int(case.fixed_writer)) + word(case.version) + span(case.raw) + word(len(values))
    for ordinal, (destination, value) in enumerate(zip(destinations, values, strict=True)):
        packet += word(ordinal * 4) + word(destination) + word(value)
    return packet + coverage(len(case.raw), len(values), False)


def edit(case: LbcCase) -> bytes:
    """Check a literal four-byte splice and complete ordered source/replacement history."""
    destinations, values = fields(case)
    offset = case.ordinal * 4
    result = case.raw[:offset] + word(case.replacement) + case.raw[offset + 4:]
    changed = result != case.raw
    result_case = LbcCase(0, case.fixed_writer, case.version, result, 0, 0, True)
    packet = span(case.raw) + span(image(result_case)) + word(case.ordinal) + word(case.replacement) + word(int(changed)) + word(len(values))
    for ordinal, (destination, value) in enumerate(zip(destinations, values, strict=True)):
        derived = changed and ordinal == case.ordinal
        replacement = case.replacement if derived else value
        packet += b''.join(word(item) for item in (ordinal * 4, ordinal * 4, destination, value, replacement, int(derived)))
    return packet + coverage(len(result), len(values), True)


def project(case: LbcCase) -> bytes:
    """Preserve externally expected failures before any reader exception is caught."""
    if not case.accepted:
        assert case.kind == 0
        with unittest.TestCase().assertRaises(ValueError):
            fields(case)
        return word(0) + word(1)
    if case.kind == 0:
        return image(case)
    assert case.kind == 1
    return edit(case)


def packet(cases: list[LbcCase]) -> bytes:
    """Retain every supplied context and complete independently checked output."""
    records = []
    for case in cases:
        header = b''.join(word(value) for value in (case.kind, int(case.fixed_writer), case.version, case.ordinal, case.replacement, int(case.accepted)))
        records.append(header + span(case.raw) + span(project(case)))
    return b'LBCP' + word(len(cases)) + b''.join(records)
