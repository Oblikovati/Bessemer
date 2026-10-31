"""Actual frozen ParameterSets reads and independent full-pattern edit/history packets (#133)."""
from dataclasses import replace
import unittest
from owned_persistence import word, span
from parameter_cases import ParameterCase, read
from parameter_fields import fields, structural


def context(case: ParameterCase) -> bytes:
    """Retain explicit normalized version and all caller budgets."""
    return b''.join(word(value) for value in (case.version, case.maximum_records, case.maximum_table_items, case.maximum_text_bytes))


def coverage(case: ParameterCase, completion: int, edited: bool) -> bytes:
    """Exact framing scope and five native/semantic exclusions; no sampled or geometry pass."""
    record = read(case)
    values = (completion, int(edited), len(case.raw), len(fields(record)), record.table_count, record.record_count, len(record.texts), 31, 0)
    return context(case) + b''.join(word(value) for value in values)


def image(case: ParameterCase) -> bytes:
    """Validate before emitting every typed role, physical byte position and full pattern."""
    record = read(case)
    entries = []
    for field in fields(record):
        entries.append(b''.join(word(value) for value in (*field.role, field.offset, field.kind)) + span(field.raw))
    header = context(case) + span(case.raw) + b''.join(word(value) for value in record.header)
    header += word(record.table_count) + word(record.record_count) + word(len(entries))
    return header + b''.join(entries) + coverage(case, int(record.record_count != 0), False)


def edited(case: ParameterCase) -> bytes:
    """Compare a literal splice, complete reparsed fields and every ordered history parent."""
    originals = fields(read(case))
    original = originals[case.ordinal]
    result = replace(case, raw=case.raw[:original.offset] + case.replacement + case.raw[original.offset + len(original.raw):])
    children = fields(read(result))
    changed = case.raw != result.raw
    entries = []
    for ordinal, (parent, child) in enumerate(zip(originals, children, strict=True)):
        assert parent.role == child.role and parent.kind == child.kind
        derived = changed and ordinal == case.ordinal
        assert child.raw == (case.replacement if derived else parent.raw)
        entries.append(b''.join(word(value) for value in (ordinal, parent.offset, child.offset, *parent.role, parent.kind)) + span(parent.raw) + span(child.raw) + word(int(derived)))
    header = span(case.raw) + span(image(result)) + word(case.ordinal) + word(case.replacement_kind) + span(case.replacement) + word(int(changed)) + word(len(entries))
    return header + b''.join(entries) + coverage(result, 3 if changed else 2, True)


def rejected(case: ParameterCase) -> bytes:
    """Count-edit policy is independent of byte parsing; malformed reads must actually fail."""
    if case.ordinal < 0:
        with unittest.TestCase().assertRaises(ValueError):
            read(case)
        return word(0)
    original = fields(read(case))[case.ordinal]
    assert structural(original, case.version) and original.kind == case.replacement_kind
    assert original.raw != case.replacement
    return word(0)


def packet(cases: list[ParameterCase]) -> bytes:
    """Retain every caller input and complete independently checked read/edit or rejection."""
    entries = []
    for case in cases:
        expected = rejected(case) if not case.accepted else word(1) + (image(case) if case.ordinal < 0 else edited(case))
        header = context(case) + word(case.ordinal) + word(case.replacement_kind) + word(int(case.accepted))
        entries.append(header + span(case.raw) + span(case.replacement) + span(expected))
    return b'PARM' + word(len(cases)) + b''.join(entries)
