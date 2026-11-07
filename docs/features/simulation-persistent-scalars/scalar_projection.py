"""Complete frozen-reader projections and independent scalar edit splices (#129)."""
import unittest
from cosworkspersist import read_integer_array, read_unique_ids
import cosworks_study_versions as newer
from owned_persistence import word, span
from scalar_cases import ScalarCase, field_patterns


def validate(case: ScalarCase) -> None:
    """Require every accepted byte and both array readers' explicit limit agreement."""
    if case.profile == 0:
        record = read_integer_array(case.raw, max_items=case.maximum)
        other = newer.parse(case.raw, max_items=case.maximum)
        assert other['success'] and other['complete'] and tuple(other['versions']) == record.values
        values = record.values
        assert record.count_header + record.payload == case.raw and record.end == len(case.raw)
    else:
        record = read_unique_ids(case.raw, max_entries=case.maximum)
        values = record.keys
        assert record.used_count == len(values)
        assert record.allocation_count == int.from_bytes(case.raw[4:8], 'little', signed=True)
        assert record.header + record.payload == case.raw and record.end == len(case.raw)
    expected = tuple(int.from_bytes(raw, 'little', signed=True) for _, role, _, raw in field_patterns(case) if role == 2)
    assert values == expected


def coverage(case: ScalarCase, edit: bool) -> bytes:
    """Complete byte/field scope, empty completion and explicit native/semantic exclusions."""
    used = int.from_bytes(case.raw[:4], 'little', signed=True)
    header = 4 if case.profile == 0 else 8
    return b''.join(word(value) for value in (case.profile, len(case.raw), header, used, case.maximum, int(used != 0), int(edit), 7, 0))


def image(case: ScalarCase) -> bytes:
    """Serialize every header/entry role, physical position and complete raw pattern."""
    validate(case)
    used = int.from_bytes(case.raw[:4], 'little', signed=True)
    header = 4 if case.profile == 0 else 8
    fields = field_patterns(case)
    capacity = int.from_bytes(case.raw[4:8], 'little', signed=True) if case.profile else 0
    packet = word(1) + word(case.profile) + span(case.raw) + span(case.raw[:header]) + span(case.raw[header:])
    packet += word(used) + word(case.profile) + word(capacity) + word(len(fields))
    entries = [word(offset) + word(role) + word(index) + span(raw) for offset, role, index, raw in fields]
    return packet + b''.join(entries) + coverage(case, False)


def edit(case: ScalarCase) -> bytes:
    """Use a literal splice and retain complete ordered count/capacity/entry history."""
    fields = field_patterns(case)
    offset, _, _, previous = fields[case.ordinal]
    raw = case.raw[:offset] + case.replacement + case.raw[offset + len(previous):]
    result = ScalarCase(case.profile, case.maximum, raw)
    changed = raw != case.raw
    packet = span(case.raw) + span(image(result)) + word(case.ordinal) + span(case.replacement) + word(int(changed)) + word(len(fields))
    for ordinal, ((before_offset, role, index, before), (after_offset, after_role, after_index, after)) in enumerate(zip(fields, field_patterns(result), strict=True)):
        assert (role, index) == (after_role, after_index)
        derived = changed and ordinal == case.ordinal
        assert after == (case.replacement if derived else before)
        packet += word(ordinal) + word(before_offset) + word(after_offset) + word(role) + word(index) + span(before) + span(after) + word(int(derived))
    return packet + coverage(result, True)


def rejected(case: ScalarCase) -> bytes:
    """Handle only independently declared invalid shapes or invalid literal splices."""
    raw = case.raw
    if case.ordinal >= 0:
        offset, _, _, previous = field_patterns(case)[case.ordinal]
        raw = raw[:offset] + case.replacement + raw[offset + len(previous):]
    with unittest.TestCase().assertRaises(ValueError):
        validate(ScalarCase(case.profile, case.maximum, raw))
    if case.profile == 0:
        result = newer.parse(raw, max_items=case.maximum)
        assert not result['success'] and not result['complete']
    return word(0) + word(1)


def packet(cases: list[ScalarCase]) -> bytes:
    """Include every caller input and the complete independently checked result."""
    entries = []
    for case in cases:
        expected = rejected(case) if not case.accepted else (image(case) if case.ordinal < 0 else edit(case))
        header = b''.join(word(value) for value in (case.profile, case.maximum, case.ordinal, int(case.accepted)))
        entries.append(header + span(case.raw) + span(case.replacement) + span(expected))
    return b'SCAL' + word(len(cases)) + b''.join(entries)
