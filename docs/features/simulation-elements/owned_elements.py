"""Independent complete element records and reviewed numeric branch expectations (#118)."""
import struct
from itertools import product


def element(family: int, count: int, variant: int, stored: int = 0x1234, activity: int = 1,
            ids: tuple[int, ...] = tuple(range(1, 12))) -> bytes:
    """Retain distinctive opaque regions beside explicitly fixed field offsets."""
    raw = bytearray((i * 17 + 43) & 255 for i in range(140))
    raw[0], raw[20], raw[23], raw[24] = activity, family, count, variant
    struct.pack_into('<H11i', raw, 26, stored, *([0] * 11))
    struct.pack_into('<11i', raw, 96, *ids)
    return bytes(raw)


def selected(family: int, count: int, variant: int) -> int:
    """Closed-form branch oracle, retaining the full sentinel separately from stored u16."""
    if family == 4:
        return 92 + 6 * (count == 4)
    direct = {(1, 1): 37, (2, 2): 8, (2, 3): 4}
    if family != 8:
        return direct.get((family, count), 0xffffffff)
    explicit = {(3, 0): 63, (3, 1): 43, (3, 2): 53, (4, 0): 64, (4, 1): 44,
                (4, 2): 54, (6, 0): 94, (6, 1): 90, (6, 2): 99}
    if (count, variant) in explicit:
        return explicit[count, variant]
    return 2 if variant >= 3 and count % 3 == 0 else 42 if variant >= 3 and count % 4 == 0 else 93


def legacy_cases() -> list[tuple[bytes, int, int, bytes]]:
    """Independently patch only the stored word for explicit visited-record contexts."""
    result = []
    domains = ((0, 1, 2, 4, 8, 255), (0, 1, 2, 3, 4, 6, 8, 10, 12, 255), (0, 1, 2, 3, 255),
               ((800050, 4), (820024, -1), (810000, 0), (820024, 0x7fffffff)))
    for family, count, variant, (version, analysis) in product(*domains):
        code = (28 if analysis == 4 else 37) if count == 2 else selected(family, count, variant) & 0xffff
        raw = element(family, count, variant, code if variant == 1 else 0x1234)
        golden = raw[:26] + struct.pack('<H', code) + raw[28:]
        result.append((raw, version, analysis, golden))
    return result


def topology_cases() -> list[tuple[bytes, int | None]]:
    """Own ordered duplicates, invalid signed IDs and optional upper-bound checks."""
    ordinary = tuple(range(1, 12))
    unusual = (0, -1, 3, 3, 11, 3, 0x7fffffff, -0x80000000, 1, 11, -999)
    return [(element(4, 10, 0, activity=activity, ids=ids), bound)
            for ids in (ordinary, unusual) for bound in (None, 0, 10, 0xffffffff)
            for activity in (1, 255)]
