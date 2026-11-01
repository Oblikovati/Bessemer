"""Independent all-channel comparison cases, retaining bitwise and native-model equality separately (#166)."""
from collections.abc import Iterator
from types import ModuleType
from typing import cast
import importlib
import struct
from color_comparison_evidence import color_comparison_report
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets

PATTERNS = (0, 0x80000000, 1, 0x80000001, 0x007fffff, 0x00800000, 0x3f800000, 0xbf800000,
    0x3f000000, 0x40000000, 0x7f800000, 0xff800000, 0x7fc00042, 0xffc00042, 0x7fa00042, 0x7f7fffff)


def owned_comparison_pairs() -> Iterator[tuple[bytes, bytes]]:
    """Exercise each RGBA position, all representative bit-class pairs and independent mixed values; e.g. list(owned_comparison_pairs())."""
    for left in PATTERNS:
        for right in PATTERNS:
            yield from positioned_comparison_pairs(left, right)
    for index in range(1024):
        left = tuple((index * factor + offset) & 0xffffffff for factor, offset in
            ((0x9e3779b9, 0), (0x85ebca6b, 1), (0xc2b2ae35, 2), (0x27d4eb2f, 3)))
        right = tuple((value ^ (1 << (index % 32))) & 0xffffffff for value in left)
        yield struct.pack('<4I', *left), struct.pack('<4I', *right)


def positioned_comparison_pairs(left: int, right: int) -> Iterator[tuple[bytes, bytes]]:
    """Keep other channels equal so alpha and each RGB branch are independently observed; e.g. list(positioned_comparison_pairs(0, 1))."""
    for channel in range(4):
        first, second = [0, 0, 0, 0x3f800000], [0, 0, 0, 0x3f800000]
        first[channel], second[channel] = left, right
        yield struct.pack('<4I', *first), struct.pack('<4I', *second)


def comparison_record(helper: ModuleType, left: bytes, right: bytes) -> bytes:
    """Retain both native-model predicates beside the complete unchanged source colors; e.g. comparison_record(helper, left, right)."""
    equal = helper.equal(left, right, environment='masked_exceptions_daz_off')
    different = helper.different(left, right, environment='masked_exceptions_daz_off')
    assert equal in (0, 1) and different == 1 - equal, (equal, different)
    return left + right + struct.pack('<II', equal, different)


def comparison_packet(helper: ModuleType, pairs: list[tuple[bytes, bytes]]) -> bytes:
    """Publish every complete paired input and independent expected predicates; e.g. comparison_packet(helper, pairs)."""
    return struct.pack('<II', 1, len(pairs)) + b''.join(comparison_record(helper, *pair) for pair in pairs)


def verify_color_comparison_packets() -> None:
    """Cover synthetic bit classes and every ordered pair of distinct native raw colors; e.g. verify_color_comparison_packets()."""
    helper = importlib.import_module('cosworks_color_comparison')
    report = color_comparison_report()
    counts = cast(dict[str, int], report['raw_color_counts'])
    assert sum(counts.values()) == report['native_color_records'] == 4836, counts
    colors = [bytes.fromhex(raw) for raw in sorted(counts)]
    assert len(colors) == report['unique_native_raw_colors'] == 26, len(colors)
    owned, native = list(owned_comparison_pairs()), [(left, right) for left in colors for right in colors]
    packets = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'rgba-comparison.bin': comparison_packet(helper, owned)})
    packets.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-rgba-comparison-native.bin': comparison_packet(helper, native)}))
    dialog_baseline('rgba-comparison-packets-reference.json', dict(owned_pairs=len(owned), native_pairs=len(native),
        native_distinct_colors=26, native_original_observations=4836, additional_stored_byte_credit=0, packets=packets))
