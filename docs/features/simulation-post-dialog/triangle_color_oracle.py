"""Independent C889 pointer-offset and ordered-comparison oracle (#166)."""
import struct


def triangle_caller_oracle(words: list[int]) -> int | None:
    """Retain lazy external-call dependencies and unsigned hidden selector admission; e.g. triangle_caller_oracle(words)."""
    kind, selector, override = words[:3]
    first_tag, first, second_tag, second = words[7:11]
    if kind == 1 and second == 5: return 3
    if first_tag == 0: return 12
    if kind == 0 and first == 0: return 0
    if kind == 1 and first != 0: return 4
    if kind == 0 and selector > 1: return 1
    if kind == 1 and override == 0: return 5
    if kind == 1: return None
    if second_tag == 0: return 12
    return 2 if second != 0 else None


def triangle_alpha_oracle(bits: int) -> int:
    """Use the manual's ordered comparison rather than the Rust bit classification; e.g. triangle_alpha_oracle(0xffc00042)."""
    value = struct.unpack('<f', struct.pack('<I', bits))[0]
    return bits if value < 1.0 else 0x3f800000


def triangle_group_oracle(words: list[int], groups: bytes, initial: list[int]) -> tuple[int, list[bytes]] | None:
    """Walk native strideB8 and group offsets, preserving lookup-before-count dependency; e.g. triangle_group_oracle(words, groups, initial)."""
    count = words[12] if words[12] < 0x80000000 else 0
    lookups, calls = 0, []
    for index in range(count):
        base = index * 0xb8
        flag = struct.unpack_from('<i', groups, base + 8)[0]
        if bool(flag) != (words[0] == 0): continue
        if words[14] != 0:
            lookups += 1
            outcome = struct.unpack_from('<I', groups, base + 0x44)[0]
            if outcome == 0: return None
            if outcome == 1: continue
        triangles = struct.unpack_from('<i', groups, base + 0x2c)[0]
        if triangles <= 0: continue
        origin = 0 if words[1] != 1 else (1 if words[14] == 0 else 2)
        rgb = initial[:3] if origin == 0 else list(struct.unpack_from('<3I', groups, base + (0xc if origin == 1 else 0x1c)))
        calls.append(struct.pack('<Ii', index, triangles) + groups[base + 0x30:base + 0x40] + struct.pack('<5I', *rgb, initial[3], origin))
    return lookups, calls


def triangle_operation_oracle(words: list[int], groups: bytes) -> bytes:
    """Return complete local output or one failure class with no partial plan; e.g. triangle_operation_oracle(words, groups)."""
    status = triangle_caller_oracle(words)
    if status is None:
        status = 6 if words[11] == 0 else 7 if words[12] == 0 else 8 if words[13] == 0 else 9
    inspected = words[12] if status == 9 and words[12] < 0x80000000 else 0
    if inspected > words[15]: status = 10
    elif inspected > words[16]: status = 11
    if status != 9: return struct.pack('<9I', status, 0, 0, 0, 0, 0, 0, 0, 0)
    initial = words[3:6] + [triangle_alpha_oracle(words[6])]
    selected = triangle_group_oracle(words, groups, initial)
    if selected is None: return struct.pack('<9I', 13, 0, 0, 0, 0, 0, 0, 0, 0)
    lookups, calls = selected
    return struct.pack('<9I', 9, inspected, lookups, 1, *initial, len(calls)) + b''.join(calls)
