"""Independent complete sketch fixtures for persisted equation roles (#199)."""
from pathlib import Path
import struct
from fillet_fixture_bytes import WireMap
from sketch_tail_fixture_bytes import ordinary
from sketch_spline_fixture_bytes import collection
from sketch_geometry_fixture_bytes import segment
from owned_name_edit import short_wide_name

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


def equation_literal_positions(raw: bytes, kind: int, style: int) -> dict[str, object]:
    """Locate independently assembled field spans; e.g. equation_literal_positions(raw,0,0)."""
    direct = equation_collection(kind, style)
    assert raw.count(direct) == 1, 'expected one complete independent direct collection'
    start = raw.index(direct)
    flags = start + 6 + len(segment(False, False, True, 0xffff))
    result: dict[str, object] = dict(flags_offset=flags)
    if not EQUATION_FLAGS[style] & 4:
        return result
    strings = b''.join(short_wide_name(text) for text in SAVED_STRINGS)
    assert direct.count(strings) == 1, 'expected one complete six-CString equation block'
    result['option_offset'] = start + direct.index(strings) + len(strings) + 4
    result['strings'] = [literal_text_span(direct, start, text) for text in SAVED_STRINGS]
    return result


def literal_text_span(direct: bytes, start: int, text: str) -> tuple[int, int]:
    """Retain a literal complete CString extent; e.g. literal_text_span(raw,0,'Ω equation 0')."""
    frame = short_wide_name(text)
    assert direct.count(frame) == 1, f'{text!r}; expected unique independently assembled CString'
    return start + direct.index(frame), len(frame)


def write_literal_positions(destination: Path) -> None:
    """Save independent field extents beside generated wires; e.g. write_literal_positions(temporary)."""
    rows = []
    for kind in range(2):
        for style in range(8):
            raw = (destination / f'owned-{kind}-{style}.bin').read_bytes()
            positions = equation_literal_positions(raw, kind, style)
            flat = [kind, style, positions['flags_offset'], positions.get('option_offset', -1)]
            flat += [value for pair in positions.get('strings', []) for value in pair]
            rows.append('\t'.join(map(str, flat)))
    (destination / 'literal-positions.tsv').write_bytes(('\n'.join(rows) + '\n').encode())
