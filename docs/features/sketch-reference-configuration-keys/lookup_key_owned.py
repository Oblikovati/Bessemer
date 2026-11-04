"""Independently framed canonical lookup-key controls (#188)."""
from dataclasses import dataclass
import struct

LOOKUP_KEY_VALUES = (-2147483648, -2147483647, -12346, -12345, -12344, -1, 0, 1, 2147483646, 2147483647)
TRANSFORM_PROFILES = (
    ((0, False, 7), (0, False, 7)),
    ((65535, False, 0x2b), (0, False, 7)),
    ((0, False, 7), (2, True, 0x8d)),
    ((1, True, 0x1a), (65534, False, 0x4c)),
)

@dataclass(frozen=True)
class OwnedSketchLookupKeys:
    raw: bytes
    key_offsets: tuple[tuple[int, int], tuple[int, int]]
    keys: tuple[tuple[int, int], tuple[int, int]]
    profiles: tuple[int, int]


def checked_lookup_key(value: int) -> int:
    """Reject values outside the physical signed word; e.g. checked_lookup_key(-12345)."""
    if type(value) is not int or not -2147483648 <= value <= 2147483647:
        raise ValueError(f'lookup key {value!r}; expected a full signed i32 value')
    return value


def lookup_key_transform_bytes(profile: int) -> bytes:
    """Reuse the compact codec's owned transform builder; e.g. lookup_key_transform_bytes(3)."""
    if type(profile) is not int or not 0 <= profile < len(TRANSFORM_PROFILES):
        raise ValueError(f'transform profile {profile!r}; expected integer0..3')
    from freeze_views import transform
    encoded = bytearray()
    for presence, matrix, flags in TRANSFORM_PROFILES[profile]:
        encoded.extend(struct.pack('<H', presence))
        if presence:
            encoded.extend(transform(matrix, flags))
    return bytes(encoded)


def lookup_key_body(object_key: int, record_key: int, profile: int) -> tuple[bytes, int]:
    """Frame the full body with unrelated signed neighbors; e.g. lookup_key_body(-12345, 7, 0)."""
    object_key, record_key = checked_lookup_key(object_key), checked_lookup_key(record_key)
    transforms = lookup_key_transform_bytes(profile)
    prefix = struct.pack('<HHHii', 0, 65535, 0, -2147483646 + profile, 1879048192 + profile)
    key_offset = len(prefix) + len(transforms) + 8
    words = struct.pack('<5i', -2147483647 + profile, 2147483647 - profile, object_key, record_key, -13)
    suffix = struct.pack('<6ihH', -31 - profile, 27 + profile, -39 - profile, -43 - profile, -47 - profile, -53 - profile, (-2, -1, 0, 0)[profile], 0)
    return prefix + transforms + words + suffix, key_offset


def owned_lookup_key_root(object_key: int, record_key: int, profile: int) -> OwnedSketchLookupKeys:
    """Keep two physical definitions and the first alias; e.g. owned_lookup_key_root(-12345, 7, 2)."""
    first, first_offset = lookup_key_body(object_key, record_key, profile)
    second_profile = (profile + 1) % len(TRANSFORM_PROFILES)
    second, second_offset = lookup_key_body(~object_key, ~record_key, second_profile)
    name = b'moSketchExtRef_w'
    tag = struct.pack('<3H', 65535, 1, len(name)) + name
    first_start, second_start = 6 + len(tag), 6 + len(tag) + len(first) + 2
    raw = struct.pack('<iH', -71, 3) + tag + first + struct.pack('<H', 0x8061) + second + struct.pack('<HI', 98, 0)
    offsets = ((first_start + first_offset, first_start + first_offset + 4), (second_start + second_offset, second_start + second_offset + 4))
    return OwnedSketchLookupKeys(raw, offsets, ((object_key, record_key), (~object_key, ~record_key)), (profile, second_profile))
