"""Complete C907 route/context/source/alpha case matrix using pinned Source expectations (#166)."""
from itertools import product
from collections import Counter
import importlib
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets
from preference_color_evidence import PREFERENCE_COLOR_ROUTES
from preference_color_operation_oracle import preference_case_source, preference_operation_oracle

PREFERENCE_RUNTIME_FACTS = ((0, 0, 0), (0, 1, 73), (1, 0, 0), (1, 1, 73), (2, 0, 0),
    (2, 1, 0), (2, 1, 70), (2, 1, 73), (2, 1, 74), (2, 1, 0xffffffff), (2, 1, 0x80000000), (2, 1, 0x7fffffff))
PREFERENCE_ALPHA_BITS = (0, 0x80000000, 1, 0x7f800000, 0xff800000, 0x7fc00042, 0x7fa00042, 0x3f800000, 0x3f000000, 0xffffffff)


def preference_operation_case(route: tuple[int, int, int | None], runtime: tuple[int, int, int], length: int | None,
                              packed: int, original_alpha: int, alpha: int, seed: int = 0x55) -> list[int]:
    """Tag all known/unknown inputs and independently retained route descriptors; e.g. preference_operation_case(route, runtime, None, 0, 0, 0)."""
    member, packed_offset, alpha_offset = route
    return [member, 0x7fa00042, 0x80000000, 0xffffffff, original_alpha, *runtime,
        int(length is not None), length or 0, packed, alpha, seed, packed_offset, int(alpha_offset is not None), alpha_offset or 0]


def preference_boundary_cases() -> list[list[int]]:
    """Exercise both source extents, skipped/missing facts and raw alpha classes for every route; e.g. preference_boundary_cases()."""
    cases: list[list[int]] = []
    for route in PREFERENCE_COLOR_ROUTES:
        _, packed, alpha = route
        required = max(packed, alpha or 0) + 4
        lengths = (None, *sorted({0, packed - 1, packed, packed + 3, packed + 4, required - 1, required, required + 1, 0x15d4}))
        for runtime, length, original, replacement in product(PREFERENCE_RUNTIME_FACTS, lengths, (0, 0x80000000, 0x7fa00042), (0, 0x80000000, 0xffc00042, 0x7f800000)):
            cases.append(preference_operation_case(route, runtime, length, 0xaa123456, original, replacement))
    return cases


def preference_rounding_cases() -> list[list[int]]:
    """Cover every packed byte in each channel and high byte across all eight routes; e.g. preference_rounding_cases()."""
    cases: list[list[int]] = []
    for ordinal, route in enumerate(PREFERENCE_COLOR_ROUTES):
        for byte, channel in product(range(256), range(3)):
            packed = ((0x335577 & ~(255 << (8 * channel))) | (byte << (8 * channel))) | ((byte ^ 0xa5) << 24)
            original = PREFERENCE_ALPHA_BITS[(byte + channel + ordinal) % len(PREFERENCE_ALPHA_BITS)]
            alpha = PREFERENCE_ALPHA_BITS[(byte * 3 + channel + ordinal) % len(PREFERENCE_ALPHA_BITS)]
            cases.append(preference_operation_case(route, (1, 1, 73), 0x15d4, packed, original, alpha))
    return cases


def preference_equal_projection_cases() -> list[list[int]]:
    """Retain projection completion even when all resulting bits equal the original; e.g. preference_equal_projection_cases()."""
    cases: list[list[int]] = []
    model = importlib.import_module('cosworks_style_preference_colors')
    for route, packed, alpha in product(PREFERENCE_COLOR_ROUTES, (0, 0xaa112233, 0xffffffff), PREFERENCE_ALPHA_BITS):
        words = preference_operation_case(route, (2, 1, 70), 0x15d4, packed, alpha, alpha)
        result = model.project(struct.pack('<4I', *words[1:5]), style_offset=route[0], preference_bytes=preference_case_source(words), local_block_applies=True, rounding_mode='nearest_even')
        words[1:5] = struct.unpack('<4I', result['projected_raw'])
        cases.append(words)
    return cases


def preference_invalid_route_cases() -> list[list[int]]:
    """Keep unknown members rejected even when a local block would otherwise skip; e.g. preference_invalid_route_cases()."""
    return [preference_operation_case((member, 0, None), runtime, length, 0, alpha, 0)
        for member, runtime, length, alpha in product((0, 0x1bb, 0x1bd, 0x1cb, 0x1cd, 0x21f, 0x221, 0x2ab, 0x2ad, 0xffffffff),
            PREFERENCE_RUNTIME_FACTS, (None, 0, 0x15d4), (0, 0x80000000, 0x7fa00042))]


def verify_preference_operation_packets() -> None:
    """Freeze independent complete arithmetic/history/context outputs for every operation case; e.g. verify_preference_operation_packets()."""
    model = importlib.import_module('cosworks_style_preference_colors')
    groups = [preference_boundary_cases(), preference_rounding_cases(), preference_equal_projection_cases(), preference_invalid_route_cases()]
    cases = [words for group in groups for words in group]
    expected = [preference_operation_oracle(model, words) for words in cases]
    packet = struct.pack('<2I', 1, len(cases)) + b''.join(struct.pack('<16I', *words) + result for words, result in zip(cases, expected, strict=True))
    packets = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'preference-color-operations.bin': packet})
    dialog_baseline('preference-color-operation-reference.json', dict(cases=len(cases), group_counts=[len(group) for group in groups],
        statuses=dict(sorted(Counter(str(struct.unpack_from('<I', result)[0]) for result in expected).items())),
        packets=packets, nearest_even_context_explicit=True, target_executed=False, additional_stored_byte_credit=0))
