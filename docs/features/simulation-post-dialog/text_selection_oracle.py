"""Independent C855 assignment and C890 selection over explicit current CString units (#166)."""
import importlib
import struct


def post_text_frame(raw: bytes) -> bytes:
    """Keep code-unit counts distinct from bytes and Unicode scalar values; e.g. post_text_frame(bytes.fromhex('00d8'))."""
    assert len(raw) % 2 == 0
    return struct.pack('<I', len(raw) // 2) + raw


def post_optional_text(raw: bytes | None) -> bytes:
    """Distinguish unknown state from known empty text; e.g. post_optional_text(None)."""
    return struct.pack('<I', int(raw is not None)) + (post_text_frame(raw) if raw is not None else b'')


def post_assignment_prefix(raw: bytes) -> bytes:
    """Use the unchanged bounded Source view and reviewed copy loop for larger explicit budgets; e.g. post_assignment_prefix(b'A\0\0\0B\0')."""
    if len(raw) <= 1024:
        view = importlib.import_module('cosworks_post_style_text_view').decode_view(raw)
        assert view['assignment_performed'] is True
        return bytes.fromhex(view['native_prefix_raw_hex'])
    units = struct.unpack('<' + 'H' * (len(raw) // 2), raw)
    retained: list[int] = []
    for unit in units:
        if unit == 0: break
        retained.append(unit)
    return struct.pack('<' + 'H' * len(retained), *retained)


def post_assignment_oracle(wire: bytes, previous: bytes | None, maximum: int) -> tuple[int, int, bytes]:
    """Resolve only known primary state; count0 unknown remains a typed missing result; e.g. post_assignment_oracle(b'', None, 512)."""
    if len(wire) // 2 > maximum: return 10, 0, b''
    if not wire and previous is None: return 11, 0, b''
    result = post_assignment_prefix(wire) if wire else previous
    assert result is not None
    if len(result) // 2 > maximum: return 12, 0, b''
    return 0, 1 if wire else 2, result


def post_selection_sources(primary_flag: int, line_flag: int, primary: bytes | None,
                           lines: tuple[bytes | None, ...]) -> tuple[int, list[tuple[int, bytes]]]:
    """Require primary first, then each enabled line position, retaining explicitly supplied NUL units; e.g. post_selection_sources(0, 0, None, (None,) * 7)."""
    if primary_flag != 0 and primary is None: return 13, []
    selected = [(1, primary)] if primary_flag != 0 else [(0, b'')]
    for index, line in enumerate(lines):
        if line_flag != 0 and line is None: return 14 + index, []
        selected.append((2 + index, line) if line_flag != 0 else (0, b''))
    assert all(raw is not None for _, raw in selected)
    return 0, selected


def post_text_operation_oracle(mode: int, primary_flag: int, line_flag: int, wire: bytes,
                               previous: bytes | None, lines: tuple[bytes | None, ...], maximum: int) -> bytes:
    """Compose assignment only in its explicit mode, without partial success; e.g. post_text_operation_oracle(0, 0, 0, b'', None, (None,) * 7, 512)."""
    kind, assigned = 0, b''
    if mode in (0, 2):
        status, kind, assigned = post_assignment_oracle(wire, previous, maximum)
        if status: return struct.pack('<I', status)
    if mode == 0:
        return struct.pack('<5I', 0, len(assigned) // 2, int(not wire), int(bool(assigned)), kind) + post_text_frame(assigned)
    status, selected = post_selection_sources(primary_flag, line_flag, assigned if mode == 2 else previous, lines)
    if status: return struct.pack('<I', status)
    units = sum(len(raw) // 2 for _, raw in selected)
    if units > maximum: return struct.pack('<I', 12)
    header = struct.pack('<5I', mode, units, int(primary_flag != 0) + 7 * int(line_flag != 0), int(units != 0), kind)
    history = post_text_frame(assigned) if mode == 2 else b''
    return header + history + b''.join(struct.pack('<I', origin) + post_text_frame(raw) for origin, raw in selected)
