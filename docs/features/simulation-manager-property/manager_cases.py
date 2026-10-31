"""Independent complete direct-manager contexts and literal edit goldens (#131)."""
from dataclasses import dataclass, replace
import itertools
import struct
import unittest
from cosworkspersist import read_manager_property
from freeze_manager_property import carriers, destinations
from owned_persistence import word, span


@dataclass(frozen=True)
class ManagerCase:
    kind: str
    version: int | None
    raw: bytes
    accepted: bool = True
    ordinal: int = -1
    replacement: int = 0


def edits(case: ManagerCase) -> list[ManagerCase]:
    """Keep every original word no-op and all five complete signed replacement patterns."""
    cases = []
    for ordinal, (value,) in enumerate(struct.iter_unpack('<i', case.raw)):
        for replacement in (value, -0x80000000, -1, 0, 1, 0x7fffffff):
            cases.append(replace(case, ordinal=ordinal, replacement=replacement))
    return cases


def owned_cases() -> list[ManagerCase]:
    """Cross signed-word extrema with each independent explicit boundary context."""
    contexts = [('BeamManager', version) for version in (0, 810023, 810024, 810025, 820005, 820006, 820007, 0x7fffffff)]
    contexts += [('MassElementManager', version) for version in (None, 0, 810024, 820006, 0x7fffffff)]
    cases = []
    for kind, version in contexts:
        count = len(destinations(kind, version))
        for values in itertools.product((-0x80000000, -1, 0, 1, 0x7fffffff), repeat=count):
            case = ManagerCase(kind, version, b''.join(struct.pack('<i', value) for value in values))
            cases += [case, *edits(case)]
        cases.extend(ManagerCase(kind, version, bytes(size), False) for size in range(17) if size != count * 4)
    return cases


def native_cases() -> list[ManagerCase]:
    """Keep every original candidate context, including all independently invalid sizes."""
    cases = []
    for row in carriers():
        for attempt in row['attempts']:
            case = ManagerCase(row['kind'], attempt['version'], bytes.fromhex(row['raw_hex']), attempt['success'])
            cases.append(case)
            cases.extend(edits(case) if case.accepted else ())
    return cases


def validate(case: ManagerCase) -> None:
    """Require the actual reader and independently literal fields to agree completely."""
    record = read_manager_property(case.raw, case.kind, case.version)
    assert record.raw == case.raw and record.kind == case.kind and record.version == case.version
    assert record.destinations == destinations(case.kind, case.version)
    assert record.values == tuple(value for value, in struct.iter_unpack('<i', case.raw))


def context(case: ManagerCase) -> bytes:
    """Retain kind and optional version without conflating absent and zero context."""
    return word(int(case.kind == 'MassElementManager')) + word(int(case.version is not None)) + word(case.version or 0)


def coverage(case: ManagerCase, completion: int, edited: bool) -> bytes:
    """Complete byte/field scope, explicit check mode and four native/semantic exclusions."""
    return context(case) + b''.join(word(value) for value in (completion, int(edited), len(case.raw), len(case.raw) // 4, 15, 0))


def image(case: ManagerCase) -> bytes:
    """Include complete bytes and every physical offset/runtime destination/signed pattern."""
    validate(case)
    fields = [word(ordinal * 4) + word(destination) + span(case.raw[ordinal * 4:ordinal * 4 + 4]) for ordinal, destination in enumerate(destinations(case.kind, case.version))]
    return context(case) + span(case.raw) + word(len(fields)) + b''.join(fields) + coverage(case, 0, False)


def edited(case: ManagerCase) -> bytes:
    """Use an independent literal four-byte splice and every ordered parent/result word."""
    offset = case.ordinal * 4
    replacement = struct.pack('<i', case.replacement)
    result = replace(case, raw=case.raw[:offset] + replacement + case.raw[offset + 4:])
    changed = case.raw != result.raw
    packet = span(case.raw) + span(image(result)) + word(case.ordinal) + span(replacement) + word(int(changed)) + word(len(case.raw) // 4)
    history = []
    for ordinal, destination in enumerate(destinations(case.kind, case.version)):
        begin = ordinal * 4
        derived = changed and ordinal == case.ordinal
        history.append(word(ordinal) + word(begin) + word(begin) + word(destination) + span(case.raw[begin:begin + 4]) + span(result.raw[begin:begin + 4]) + word(int(derived)))
    return packet + b''.join(history) + coverage(result, 2 if changed else 1, True)


def projection(case: ManagerCase) -> bytes:
    """Expected invalidity is fixed before either parser or field iteration."""
    if not case.accepted:
        assert len(case.raw) != 4 * len(destinations(case.kind, case.version))
        with unittest.TestCase().assertRaises(ValueError):
            validate(case)
        return word(0)
    validate(case)
    return word(1) + (image(case) if case.ordinal < 0 else edited(case))


def packet(cases: list[ManagerCase]) -> bytes:
    """Retain every caller input and complete independently checked read/edit result."""
    entries = []
    for case in cases:
        header = context(case) + word(int(case.accepted)) + word(case.ordinal) + word(case.replacement)
        entries.append(header + span(case.raw) + span(projection(case)))
    return b'MGRP' + word(len(cases)) + b''.join(entries)
