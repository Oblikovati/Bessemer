"""Independent complete sketch fixtures for persisted equation roles (#199)."""
import struct
from fillet_fixture_bytes import WireMap
from sketch_tail_fixture_bytes import ordinary
from sketch_spline_fixture_bytes import collection

EQUATION_FLAGS = (4, 5, 0x80000004, 0x80000005, 0x34, 0x24, 0, 1)
EQUATION_OPTIONS = (-2**31, -1, 0, 1, 2, 2**31 - 1, 0, 0)
SAVED_STRINGS = [f'Ω equation {index}' for index in range(6)]


def equation_collection(kind: int, style: int, edited: bool = False) -> bytes:
    """Assemble a direct spline with a literal stored-state oracle; e.g. equation_collection(0,0)."""
    flags = EQUATION_FLAGS[style] ^ (0x10 if edited else 0)
    mask = 1 if kind == 0 else 0x80
    raw = collection(None, False, flags, 0, False, mask=mask)
    if not flags & 4:
        return raw
    original = struct.pack('<ii', -2**31, 2**31 - 1)
    assert raw.count(original) == 1, 'expected one independently assembled compatibility/option pair'
    return raw.replace(original, struct.pack('<ii', -2**31, EQUATION_OPTIONS[style]), 1)


def equation_sketch(kind: int, version: int, style: int, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Use the existing complete sketch body and inherited map; e.g. equation_sketch(0,19000,0)."""
    mapping = WireMap()
    mapping.slot = first_slot
    def sketch_body() -> bytes:
        raw = ordinary(mapping, version, False)
        assert raw[:32] == bytes(32), 'expected original eight empty direct collections'
        return raw[:28] + equation_collection(kind, style, edited) + raw[32:]
    raw = mapping.new('sgSketch', sketch_body)
    return raw, mapping.slot + 1


def expected_equation_state(kind: int, style: int, edited: bool = False) -> dict[str, object]:
    """Return literal expected roles before consulting a reader; e.g. expected_equation_state(1,4)."""
    flags = EQUATION_FLAGS[style] ^ (0x10 if edited else 0)
    return dict(prototype=kind, flags_32c=flags, equation_present=bool(flags & 4),
                strings=SAVED_STRINGS.copy() if flags & 4 else None,
                compatibility_word=-2**31 if flags & 4 else None,
                saved_option=EQUATION_OPTIONS[style] if flags & 4 else None,
                lock_start=bool(flags & 0x10) if flags & 4 else None,
                lock_end=bool(flags & 0x20) if flags & 4 else None)
