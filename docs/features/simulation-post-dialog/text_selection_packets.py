"""C890/C855 explicit assignment, current selection and composition matrices (#166)."""
from collections import Counter
from itertools import product
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets
from text_selection_oracle import post_text_frame, post_optional_text, post_text_operation_oracle

WIRES = (b'', b'A\0', bytes.fromhex('00004200'), 'A\0B'.encode('utf-16le'), bytes.fromhex('00d8'), '😀'.encode('utf-16le'))
PRIORS = (None, b'', 'prior\0tail'.encode('utf-16le'))
LINES = ((None,) * 7, (b'',) * 7, (b'A\0', bytes.fromhex('00d8'), b'', '😀'.encode('utf-16le'), 'x\0y'.encode('utf-16le'), b'Z\0', bytes.fromhex('00dc')))


def post_text_case(mode: int, primary: int, lines_flag: int, wire: bytes, prior: bytes | None,
                   lines: tuple[bytes | None, ...], maximum: int) -> tuple[bytes, int]:
    """Frame inputs and independently expected bytes separately; e.g. post_text_case(0, 0, 0, b'', None, LINES[0], 512)."""
    assert len(lines) == 7
    payload = struct.pack('<4I', mode, primary, lines_flag, maximum) + post_text_frame(wire)
    payload += post_optional_text(prior) + b''.join(post_optional_text(line) for line in lines)
    expected = post_text_operation_oracle(mode, primary, lines_flag, wire, prior, lines, maximum)
    return struct.pack('<I', len(payload)) + payload + struct.pack('<I', len(expected)) + expected, struct.unpack_from('<I', expected)[0]


def post_text_boundary_wires() -> list[bytes]:
    """Keep bounded Source tests separate from larger explicit-budget copy-loop cases; e.g. post_text_boundary_wires()."""
    wires: set[bytes] = set()
    for count in (0, 1, 2, 511, 512, 513):
        raw = b'A\0' * count
        wires.add(raw)
        for index in {0, count // 2, count - 1}:
            if count == 0: continue
            wires.add(raw[:2 * index] + bytes(2) + raw[2 * index + 2:])
    return sorted(wires, key=lambda raw: (len(raw), raw))


def post_text_operation_cases() -> list[tuple[bytes, int]]:
    """Exercise raw control classes, unknown/empty state, every line position and resource boundaries; e.g. post_text_operation_cases()."""
    flags = (0, 1, 2, 0xffffffff, 0x80000000, 0x7fffffff)
    cases = [post_text_case(mode, primary, lines_flag, wire, prior, lines, maximum)
        for mode, primary, lines_flag, wire, prior, lines, maximum in product(
            (0, 1, 2), flags, flags, WIRES, PRIORS, LINES, (0, 2, 32))]
    for index, mode, wire, prior in product(range(7), (1, 2), (WIRES[0], WIRES[3]), (None, WIRES[3])):
        lines = tuple(None if position == index else WIRES[3] for position in range(7))
        cases.append(post_text_case(mode, 1, 1, wire, prior, lines, 32))
    cases.extend(post_text_case(mode, 1, 0, wire, WIRES[3], LINES[0], maximum)
        for mode, wire, maximum in product((0, 1, 2), post_text_boundary_wires(), (0, 1, 2, 3, 7, 8, 15, 16, 511, 512, 513, 1024)))
    return cases


def verify_post_text_operation_packets() -> None:
    """Freeze exact expected local units without native contents/layout credit; e.g. verify_post_text_operation_packets()."""
    cases = post_text_operation_cases()
    packet = struct.pack('<2I', 1, len(cases)) + b''.join(row for row, _ in cases)
    packets = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'post-text-operations.bin': packet})
    dialog_baseline('text-selection-operation-reference.json', dict(cases=len(cases),
        statuses=dict(sorted(Counter(str(status) for _, status in cases).items())), packets=packets,
        source_view_limit_units=512, larger_explicit_budgets_oracle='Reviewed native bounded-copy loop; not native oversize rejection or accepted-file coverage',
        runtime_string_contents_observed=False, target_executed=False, additional_stored_byte_credit=0))
