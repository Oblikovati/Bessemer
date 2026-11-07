"""Independent complete LBC words and exact one-field splice cases (#128)."""
from dataclasses import dataclass
import struct
from owned_lbc_property import BOUNDARIES, LAYOUTS


@dataclass(frozen=True)
class LbcCase:
    kind: int
    fixed_writer: bool
    version: int
    raw: bytes
    ordinal: int
    replacement: int
    accepted: bool


def field_cases(original: LbcCase) -> list[LbcCase]:
    """Toggle each exact field's high bit and retain every authentic no-op."""
    cases = []
    for ordinal, (value,) in enumerate(struct.iter_unpack('<i', original.raw)):
        changed = value ^ -0x80000000
        for replacement in (value, changed):
            cases.append(LbcCase(1, original.fixed_writer, original.version, original.raw, ordinal, replacement, True))
    return cases


def owned_cases() -> list[LbcCase]:
    """Exercise explicit literal contexts with constant and nonuniform word patterns."""
    cases = []
    contexts = [(False, version, size) for version, size in BOUNDARIES] + [(True, 0, 216)]
    for fixed, version, size in contexts:
        patterns = [[value] * (size // 4) for value in (0, -1, -0x80000000, 0x7fffffff)]
        patterns.append([((index * 0x9e3779b9) & 0xffffffff) for index in range(size // 4)])
        for values in patterns:
            raw = b''.join(struct.pack('<I', value & 0xffffffff) for value in values)
            original = LbcCase(0, fixed, version, raw, 0, 0, True)
            cases += [original, *field_cases(original)]
    for size in (0, 52, 204, 208, 212, 215, 217):
        cases.append(LbcCase(0, True, 0, b'\0' * size, 0, 0, False))
    assert all(len(order) * 4 == size for size, order in LAYOUTS.items())
    return cases
