"""Independent counted MeshControl bodies with empty/populated shared bases (#161)."""
from typing import TypedDict
import struct
from owned_components import signed_words
from owned_embedded import patterned_doubles
from owned_thermal import persisted_literal, thermal_base
from owned_shell_property import empty_shell_base
from owned_mesh_headers import SIGNED_PATTERNS

RECORD_PROFILES: tuple[tuple[bool, ...], ...] = ((), (False,), (True,), (False, True), (True, False, True))
TEXT_PAYLOADS: tuple[bytes | None, ...] = (None, b'\0\0', b'\0\xd8\0\0', b'\x33\0\0')


class MeshRecordSpan(TypedDict):
    index: int
    start: int
    base_end: int
    end: int


class OwnedMeshRecords(TypedDict):
    version: int
    profile: int
    raw: bytes
    records: list[MeshRecordSpan]


def mesh_record_versions() -> list[int]:
    """Cross every shared-base/reference/optional and derived reader gate; e.g. mesh_record_versions()."""
    gates = (800001, 800002, 800027, 800032, 800037, 800066, 800075, 810001, 820011, 830000,
             840002, 850015, 850016, 850018, 870006, 870010, 870016, 880008, 890009, 890011, 890021, 890111)
    return sorted({0, 2147483647, *(gate + delta for gate in gates for delta in (-1, 0, 1))})


def mesh_record_doubles(count: int, rotation: int) -> bytes:
    """Cycle finite/zero/infinity/NaN/subnormal patterns without float conversion; e.g. mesh_record_doubles(2, 4)."""
    start = rotation % 6
    return patterned_doubles(count + start)[start * 8:]


def mesh_record_words(count: int, rotation: int) -> bytes:
    """Retain independent signed extremes in every derived word position; e.g. mesh_record_words(3, 1)."""
    return signed_words(*(SIGNED_PATTERNS[(rotation + ordinal) % len(SIGNED_PATTERNS)] for ordinal in range(count)))


def mesh_record_texts(count: int, rotation: int) -> bytes:
    """Keep null/empty/surrogate/odd raw text frames separate; e.g. mesh_record_texts(3, 2)."""
    return b''.join(persisted_literal(TEXT_PAYLOADS[(rotation + ordinal) % len(TEXT_PAYLOADS)]) for ordinal in range(count))


def mesh_record_suffix(version: int, rotation: int) -> bytes:
    """Encode the documented subobject-relative fields in wire order; e.g. mesh_record_suffix(890111, 3)."""
    raw = mesh_record_doubles(2, rotation) + mesh_record_words(3, rotation) + mesh_record_texts(3, rotation)
    if version < 800032: return raw
    raw += mesh_record_words(1, rotation + 3) + mesh_record_doubles(1, rotation + 2)
    if version >= 850015: raw += mesh_record_texts(1, rotation + 3)
    if version >= 850016: raw += mesh_record_words(2, rotation + 4)
    return raw + mesh_record_late_suffix(version, rotation)


def mesh_record_late_suffix(version: int, rotation: int) -> bytes:
    """Retain each late threshold without writing runtime defaults into old bodies; e.g. mesh_record_late_suffix(890111, 1)."""
    if version < 850018: return b''
    raw = mesh_record_words(1, rotation + 1)
    if version >= 870016: raw += mesh_record_words(2, rotation + 2)
    if version >= 890111: raw += mesh_record_doubles(2, rotation + 3) + mesh_record_words(5, rotation + 3)
    return raw


def mesh_collection_literal(version: int, profile: int) -> OwnedMeshRecords:
    """Compute every record/base extent from independent component literals; e.g. mesh_collection_literal(890111, 4)."""
    assert 0 <= profile < len(RECORD_PROFILES), f'{profile}; expected profile 0..{len(RECORD_PROFILES) - 1}'
    modes = RECORD_PROFILES[profile]
    packet = bytearray(struct.pack('<i', len(modes)))
    records: list[MeshRecordSpan] = []
    for index, populated in enumerate(modes):
        start = len(packet)
        base = thermal_base(version) if populated else empty_shell_base(version)
        packet.extend(base + mesh_record_suffix(version, profile + index))
        records.append(MeshRecordSpan(index=index, start=start, base_end=start + len(base), end=len(packet)))
    return OwnedMeshRecords(version=version, profile=profile, raw=bytes(packet), records=records)


def owned_mesh_records() -> list[OwnedMeshRecords]:
    """Cover completed-empty, single, mixed and repeated nonnull archive occurrences; e.g. owned_mesh_records()."""
    cases = [mesh_collection_literal(version, profile) for version in mesh_record_versions() for profile in range(len(RECORD_PROFILES))]
    cases.extend(mesh_empty_curve_mode(version, mode) for version in (890011, 890111, 2147483647)
                 for mode in (-2147483648, -101, 101, 2147483647))
    return cases


def mesh_empty_curve_mode(version: int, mode: int) -> OwnedMeshRecords:
    """Retain arbitrary modes when the final base table has no records; e.g. mesh_empty_curve_mode(890111, 2147483647)."""
    assert version >= 890011, f'{version}; expected a version with the final base curve table'
    assert -2147483648 <= mode <= 2147483647, f'{mode}; expected a signed four-byte mode'
    case = mesh_collection_literal(version, 1)
    raw = bytearray(case['raw'])
    struct.pack_into('<i', raw, case['records'][0]['base_end'] - 8, mode)
    return OwnedMeshRecords(version=version, profile=case['profile'], raw=bytes(raw), records=case['records'])
