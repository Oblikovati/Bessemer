"""Independent C907 operation expectations from the unchanged pinned Source projection (#166)."""
from types import ModuleType
from typing import cast
import struct
from preference_color_evidence import PREFERENCE_COLOR_ROUTES


def preference_case_source(words: list[int]) -> bytes | None:
    """Construct only the declared synthetic source, retaining partial-word bytes; e.g. preference_case_source(words)."""
    if words[8] == 0: return None
    source = bytearray([words[12] & 255]) * words[9]
    fields = [(words[13], words[10])]
    if words[14]: fields.append((words[15], words[11]))
    for offset, value in fields:
        if offset >= len(source): continue
        count = min(4, len(source) - offset)
        source[offset:offset + count] = struct.pack('<I', value)[:count]
    return bytes(source)


def preference_guard_decision(words: list[int]) -> bool | int:
    """Keep unknown pointer/type separate from null and exact type73; e.g. preference_guard_decision(words)."""
    if words[5] == 0: return 11
    if words[5] == 1: return True
    if words[6] == 0: return 12
    return words[7] != 73


def preference_operation_oracle(model: ModuleType, words: list[int]) -> bytes:
    """Use independent Source RGB arithmetic and raw-alpha policy with explicit guard facts; e.g. preference_operation_oracle(model, words)."""
    assert len(words) == 16
    route = next((row for row in PREFERENCE_COLOR_ROUTES if row[0] == words[0]), None)
    if route is None: return preference_operation_failure(10)
    applies = preference_guard_decision(words)
    if type(applies) is not bool: return preference_operation_failure(cast(int, applies))
    required = max(route[1], route[2] or 0) + 4
    source = preference_case_source(words)
    if applies and source is None: return preference_operation_failure(13)
    if applies and source is not None and len(source) < required: return preference_operation_failure(14)
    original = struct.pack('<4I', *words[1:5])
    projected = cast(dict[str, object], model.project(original, style_offset=words[0], preference_bytes=source,
        local_block_applies=applies, rounding_mode='nearest_even'))
    assert projected['original_raw'] == original and projected['complete_file_schema'] is False
    return preference_operation_success(words, applies, required, source, cast(bytes, projected['projected_raw']))


def preference_operation_success(words: list[int], applies: bool, required: int, source: bytes | None, projected: bytes) -> bytes:
    """Retain original/projected words and every actually consumed parent; e.g. preference_operation_success(words, False, required, None, projected)."""
    packed, alpha = 0, 0
    if applies:
        assert source is not None
        packed = struct.unpack_from('<I', source, words[13])[0]
        if words[14]: alpha = struct.unpack_from('<I', source, words[15])[0]
    replaced = applies and bool(words[14])
    return struct.pack('<18I', int(applies), *words[1:5], *struct.unpack('<4I', projected),
        int(applies), packed, int(replaced), alpha, int(replaced), words[15] if replaced else 0,
        int(source is not None), 0 if source is None else len(source), required)


def preference_operation_failure(status: int) -> bytes:
    """Serialize failure independently of legitimate known-zero words; e.g. preference_operation_failure(11)."""
    return struct.pack('<18I', status, *([0] * 17))
