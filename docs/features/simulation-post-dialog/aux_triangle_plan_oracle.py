"""Independent C-888 helper expectations follow the reviewed pointer-stride loop (#166)."""
import importlib
import struct
from collections.abc import Iterator
from typing import TypeAlias
from itertools import product
from color_comparison_packets import PATTERNS

AuxGroup: TypeAlias = tuple[int, int, int, tuple[int, int, int, int]]
AuxContext: TypeAlias = tuple[int, int, int, int, int]
AuxControls: TypeAlias = tuple[int, int, tuple[int, int, int, int]]


def aux_plan_status(controls: AuxControls, context: AuxContext, available: int, maximum: int) -> tuple[int, int]:
    """Keep native admission order separate from library completeness/resource checks; e.g. aux_plan_status(controls, context, 3, 3)."""
    helper = importlib.import_module('cosworks_aux_triangle_gate')
    enable, selector, rgba = controls
    if not helper.request(enable, selector, struct.pack('<I', rgba[3]), masked_exceptions=True)['request_helper']: return 0, 0
    core, plot, raw_count, array, renderer = context
    if core == 0: return 1, 0
    if plot == 51: return 2, 0
    if raw_count == 0: return 3, 0
    if array == 0: return 4, 0
    if ((renderer - 3) & 0xffffffff) <= 2: return 5, 0
    count = max(0, struct.unpack('<i', struct.pack('<I', raw_count))[0])
    if count > maximum: return 7, 0
    if count > available: return 8, 0
    return 6, count


def aux_group_memory(groups: tuple[AuxGroup, ...]) -> dict[int, int | tuple[int, int, int, int]]:
    """Lay out only the verified color/count/payload members at the native stride; e.g. aux_group_memory(groups)."""
    memory: dict[int, int | tuple[int, int, int, int]] = {}
    for index, (count, first, second, color) in enumerate(groups):
        base = 0x1000 + index * 0xb8
        memory.update({base + 0xc: color, base + 0x2c: count, base + 0x30: first, base + 0x38: second})
    return memory


def aux_oracle_calls(groups: tuple[AuxGroup, ...], count: int, supplied: tuple[int, int, int, int], per_group: bool) -> list[bytes]:
    """Walk native payload addresses rather than the Rust iterator representation; e.g. aux_oracle_calls(groups, 3, color, True)."""
    memory = aux_group_memory(groups)
    pointer, remaining = 0x1038, count
    calls: list[bytes] = []
    while remaining > 0:
        triangles = memory[pointer - 0xc]
        assert isinstance(triangles, int)
        if triangles > 0:
            color = memory[pointer - 0x2c] if per_group else supplied
            assert isinstance(color, tuple)
            rgba = (*color[:3], supplied[3])
            calls.append(struct.pack('<IiQQ4II', (pointer - 0x1038) // 0xb8, triangles,
                memory[pointer - 8], memory[pointer], *rgba, int(per_group)))
        pointer, remaining = pointer + 0xb8, remaining - 1
    return calls


def aux_plan_expected(controls: AuxControls, context: AuxContext, groups: tuple[AuxGroup, ...], maximum: int) -> bytes:
    """Return explicit empty stages or complete owned calls, never partial success on missing data; e.g. aux_plan_expected(controls, context, groups, 3)."""
    status, count = aux_plan_status(controls, context, len(groups), maximum)
    if status != 6: return struct.pack('<8I', status, 0, 0, 0, 0, 0, 0, 0)
    alpha = struct.unpack('<f', struct.pack('<I', controls[2][3]))[0]
    capped = struct.unpack('<I', struct.pack('<f', min(alpha, 1.0)))[0]
    supplied = (*controls[2][:3], capped)
    calls = aux_oracle_calls(groups, count, supplied, controls[1] == 1)
    return struct.pack('<8I', status, count, 1, *supplied, len(calls)) + b''.join(calls)


def aux_plan_record(controls: AuxControls, context: AuxContext, groups: tuple[AuxGroup, ...], maximum: int) -> bytes:
    """Frame every input independently of published runtime types; e.g. aux_plan_record(controls, context, groups, 3)."""
    enable, selector, rgba = controls
    core, plot, count, array, renderer = context
    packet = struct.pack('<ii4I', enable, selector, *rgba)
    packet += struct.pack('<IiIIiII', core, plot, count, array, renderer, maximum, len(groups))
    packet += b''.join(struct.pack('<iQQ4I', triangles, first, second, *color) for triangles, first, second, color in groups)
    return packet + aux_plan_expected(controls, context, groups, maximum)


def aux_plan_groups(phase: int) -> tuple[AuxGroup, ...]:
    """Include skipped groups, maximum positive counts, zero payload identities and ignored group alpha; e.g. aux_plan_groups(0)."""
    counts = ((-1, 0, 1), (1, 2, 3), (0, 0, 0), (-2147483648, 2147483647, -1))[phase % 4]
    return tuple((count, 0 if index == 0 else (phase << 32) + index,
        0xffffffffffffffff - index, tuple(PATTERNS[(phase + index * 3 + channel) % len(PATTERNS)] for channel in range(4)))
        for index, count in enumerate(counts))


def aux_plan_cases() -> Iterator[bytes]:
    """Cross runtime rejection boundaries and admitted alpha domains before separate completeness failures; e.g. list(aux_plan_cases())."""
    contexts = product((0, 1), (0, 51, -1), (0, 1, 3, 0x80000000, 0xffffffff), (0, 1),
        (-2147483648, -1, 0, 2, 3, 4, 5, 6, 2147483647))
    for phase, context in enumerate(contexts):
        for selector, alpha in product((0, 1, 2), (0x3f7d70a4, 0x3f800000, 0x40000000, 0x7f800000)):
            controls = (2, selector, (0x7fa00042, 0x80000000, phase & 0xffffffff, alpha))
            yield aux_plan_record(controls, context, aux_plan_groups(phase + selector), 3)
    yield from aux_plan_completeness_cases()


def aux_plan_completeness_cases() -> Iterator[bytes]:
    """Require resource/missing errors only when group iteration is actually admitted; e.g. list(aux_plan_completeness_cases())."""
    for selector, count, maximum, available in product((0, 1, 2), (1, 3, 2147483647), (0, 1, 2, 3), (0, 1, 2, 3)):
        controls = (1, selector, (0x3f000000, 0x7fc00042, 0xffffffff, 0x3f800000))
        yield aux_plan_record(controls, (1, 0, count, 1, 0), aux_plan_groups(selector)[:available], maximum)
    for enable, alpha in ((0, 0x3f800000), (1, 0x3f7d70a3), (1, 0x7fa00042), (-1, 0xff800000)):
        yield aux_plan_record((enable, 1, (1, 2, 3, alpha)), (1, 0, 2147483647, 1, 0), (), 0)
