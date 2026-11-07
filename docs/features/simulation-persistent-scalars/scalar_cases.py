"""Independent scalar-frame inputs and complete exact splice contexts (#129)."""
from dataclasses import dataclass
import itertools
import struct


@dataclass(frozen=True)
class ScalarCase:
    profile: int
    maximum: int
    raw: bytes
    ordinal: int = -1
    replacement: bytes = b''
    accepted: bool = True


def field_patterns(case: ScalarCase) -> list[tuple[int, int, int, bytes]]:
    """Derive physical positions independently from the literal profile header."""
    used = int.from_bytes(case.raw[:4], 'little', signed=True)
    fields = [(0, 0, 0, case.raw[:4])]
    header, width = (4, 4) if case.profile == 0 else (8, 8)
    if case.profile == 1:
        fields.append((4, 1, 0, case.raw[4:8]))
    for index in range(used):
        offset = header + index * width
        fields.append((offset, 2, index, case.raw[offset:offset + width]))
    return fields


def edits(case: ScalarCase) -> list[ScalarCase]:
    """Keep authentic no-ops, changed entries/capacity and invalid count/capacity edits."""
    cases = []
    used = int.from_bytes(case.raw[:4], 'little', signed=True)
    for ordinal, (_, role, _, raw) in enumerate(field_patterns(case)):
        cases.append(ScalarCase(case.profile, case.maximum, case.raw, ordinal, raw))
        if role == 0:
            cases.append(ScalarCase(case.profile, case.maximum, case.raw, ordinal, struct.pack('<i', used + 1), False))
            continue
        if role == 1:
            capacity = int.from_bytes(raw, 'little', signed=True)
            replacement = struct.pack('<i', used if capacity == 0x7fffffff else 0x7fffffff)
            cases.append(ScalarCase(case.profile, case.maximum, case.raw, ordinal, struct.pack('<i', -1), False))
        else:
            replacement = (int.from_bytes(raw, 'little') ^ (1 << (len(raw) * 8 - 1))).to_bytes(len(raw), 'little')
        cases.append(ScalarCase(case.profile, case.maximum, case.raw, ordinal, replacement))
    return cases


def sequences(width: int) -> list[tuple[int, ...]]:
    """Independent signed limits, high words, duplicate values and nonuniform pairs."""
    minimum, maximum = -(1 << (width - 1)), (1 << (width - 1)) - 1
    samples = (minimum, maximum, -1, 0, 1, 0x12345678, -0x76543211, (1 << 40) if width == 64 else 0x10203040)
    return [(), (minimum,), (maximum,), (-1, 0, -1), (0, 1, minimum, maximum), *itertools.product(samples, repeat=2)]


def owned_cases() -> list[ScalarCase]:
    """Prepare both complete explicit profiles without inferring runtime policies."""
    cases = []
    for values in sequences(32):
        raw = struct.pack('<i', len(values)) + b''.join(struct.pack('<i', value) for value in values)
        case = ScalarCase(0, max(4, len(values)), raw)
        cases += [case, *edits(case)]
    for keys in sequences(64):
        for capacity in (len(keys), 1000, 0x7fffffff):
            raw = struct.pack('<ii', len(keys), capacity) + b''.join(struct.pack('<q', key) for key in keys)
            case = ScalarCase(1, max(4, len(keys)), raw)
            cases += [case, *edits(case)]
    cases += invalid_cases()
    for count in (10000, 100000):
        cases.append(ScalarCase(0, count, struct.pack('<i', count) + bytes(count * 4)))
    return cases


def invalid_cases() -> list[ScalarCase]:
    """Expected invalidity is fixed before either independent reader is invoked."""
    cases = []
    frames = ((0, struct.pack('<iii', 2, -1, 0x7fffffff)), (1, struct.pack('<iiqq', 2, 1000, -1, -0x8000000000000000)))
    for profile, raw in frames:
        invalid = [raw[:end] for end in range(len(raw))] + [raw + b'\0']
        cases += [ScalarCase(profile, 2, malformed, accepted=False) for malformed in invalid]
        cases.append(ScalarCase(profile, 1, raw, accepted=False))
    cases += [ScalarCase(1, 10, struct.pack('<ii', used, capacity), accepted=False) for used, capacity in ((-1, 0), (0, -1), (1, 0), (0x7fffffff, 0x7fffffff))]
    cases.append(ScalarCase(0, 10, struct.pack('<i', -1), accepted=False))
    return cases
