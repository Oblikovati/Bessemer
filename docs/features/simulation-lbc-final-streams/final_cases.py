"""Independent complete final-LBC frames, literal edits and invalid cases (#130)."""
from dataclasses import dataclass, replace
import itertools
import struct
from freeze_lbc_final import carriers, legacy_bytes, literal_names


@dataclass(frozen=True)
class FinalCase:
    profile: int
    maximum: int
    text_maximum: int
    raw: bytes
    ordinal: int = -1
    replacement: bytes = b''
    accepted: bool = True


def fields(case: FinalCase) -> list[tuple[int, bytes]]:
    """Derive every physical text frame or word from the literal complete grammar."""
    if case.profile == 0:
        return text_fields(case.raw)
    entries: list[tuple[int, bytes]] = []
    cursor = 0
    for _ in range(3):
        count = int.from_bytes(case.raw[cursor:cursor + 4], 'little', signed=True)
        cursor += 4
        for _ in range(count):
            entries.extend((cursor + component * 4, case.raw[cursor + component * 4:cursor + component * 4 + 4]) for component in range(4))
            cursor += 16
    assert cursor == len(case.raw)
    return entries


def text_fields(raw: bytes) -> list[tuple[int, bytes]]:
    """Locate complete counted frames without introducing a leading selection string."""
    entries: list[tuple[int, bytes]] = []
    cursor = 4
    for _ in range(int.from_bytes(raw[:4], 'little', signed=True)):
        size = 4 + int.from_bytes(raw[cursor:cursor + 4], 'little')
        entries.append((cursor, raw[cursor:cursor + size]))
        cursor += size
    assert cursor == len(raw)
    return entries


def edits(case: FinalCase) -> list[FinalCase]:
    """Keep every authentic no-op and independent changed replacement at every field."""
    cases = []
    for ordinal, (_, previous) in enumerate(fields(case)):
        replacements = literal_names() if case.profile == 0 else (struct.pack('<i', value) for value in (-0x80000000, -1, 0, 0x7fffffff))
        cases.append(replace(case, ordinal=ordinal, replacement=previous))
        cases.extend(replace(case, ordinal=ordinal, replacement=value) for value in replacements)
    return cases


def owned_cases() -> list[FinalCase]:
    """Cross text forms and populated group positions without native-population claims."""
    cases = []
    selections = [(), *itertools.product(literal_names(), repeat=2), literal_names()]
    for selected in selections:
        raw = struct.pack('<i', len(selected)) + b''.join(selected)
        case = FinalCase(0, 4, 8, raw)
        cases += [case, *edits(case)]
    records = ((-0x80000000, -1, 0, 0x7fffffff), (1, 2, -3, 4))
    for counts in itertools.product(range(3), repeat=3):
        groups = tuple(tuple(records[:count]) for count in counts)
        case = FinalCase(1, 2, 0, legacy_bytes(groups))
        cases += [case, *edits(case)]
    return cases + invalid_cases()


def invalid_cases() -> list[FinalCase]:
    """Declare invalidity before consulting the reference parser."""
    names = struct.pack('<i', 4) + b''.join(literal_names())
    maps = legacy_bytes((((1, 2, 3, 4),), (), ((-1, -2, -3, -4),)))
    cases = []
    for profile, raw in ((0, names), (1, maps)):
        malformed = [raw[:end] for end in range(len(raw))] + [raw + b'\0', struct.pack('<i', -1), struct.pack('<i', 0x7fffffff)]
        cases.extend(FinalCase(profile, 4, 8, value, accepted=False) for value in malformed)
        cases.append(FinalCase(profile, 0, 8, raw, accepted=False))
    cases.append(FinalCase(0, 4, 2, names, accepted=False))
    return cases


def native_cases() -> list[FinalCase]:
    """Replay every original retained list/map and every physical field edit."""
    cases = []
    for row in carriers():
        profile = int(row['kind'] == 'MidsurfMap')
        case = FinalCase(profile, 10000, 0x100000, bytes.fromhex(row['raw_hex']))
        cases += [case, *edits(case)]
    return cases
