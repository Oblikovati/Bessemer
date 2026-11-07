"""Independent typed nested fields observed from original source calls (#134)."""
import struct
import sys
from collections.abc import Callable
from types import FrameType
from typing import cast
from embedded_map_projection import ObservedArchive

FieldEvent = tuple[int, int, int, int]


def raw_scalar_kind(frame: FrameType) -> tuple[int, int]:
    """Expand only explicitly documented raw scalar groups; e.g. raw_scalar_kind(frame)."""
    caller = frame.f_back
    assert caller is not None
    name = caller.f_code.co_name
    if name == 'feature_wrapper': return 3, 4
    if name == 'boundary_prefix':
        values = cast(dict[str, object], caller.f_locals['out'])
        return (4, 4) if 'face_count' in values else (7, 8)
    assert name in ('first_component', 'line_reference', 'endpoint_reference', 'line_4281',
                    'endpoint_4281', 'edge_component', 'curve_wrapper'), (name, caller.f_code.co_filename)
    return 7, 8


def object_header_end(raw: bytes, start: int) -> int:
    """Bound one native pointer header independently of its body; e.g. object_header_end(raw, 4)."""
    tag = struct.unpack_from('<H', raw, start)[0]
    if tag == 0xffff: return start + 6 + struct.unpack_from('<H', raw, start + 4)[0]
    return start + (6 if tag == 0x7fff else 2)


def returned_fields(frame: FrameType, start: int, archive: ObservedArchive) -> list[FieldEvent]:
    """Project complete typed calls without replacing their implementations; e.g. returned_fields(frame, 4, archive)."""
    name, end = frame.f_code.co_name, archive.p
    if name == 'object': return [(start, object_header_end(archive.b, start), 9, 3)]
    if name == 'cstring': return [(start, end, 8, 2)]
    if name == 'read_mgXform_c': return [(start, end, 10, 2)]
    if name == '_take':
        tag = {'B': 0, 'H': 1, 'h': 2, 'i': 3, 'I': 4, 'q': 5, 'f': 6, 'd': 7}[cast(str, frame.f_locals['fmt'])]
        return [(start, end, tag, 0)]
    caller = frame.f_back
    assert caller is not None
    if caller.f_code.co_name in ('cstring', 'object'): return []
    tag, width = raw_scalar_kind(frame)
    assert (end - start) % width == 0, (start, end, width)
    return [(offset, offset + width, tag, 0) for offset in range(start, end, width)]


class FieldCallObserver:
    def __init__(self) -> None:
        """Keep all source observation state operation-local; e.g. FieldCallObserver()."""
        from swarchive import Archive
        from mfcobjects import ObjectArchive
        from dl_ui import read_mgXform_c
        functions = (Archive._take, Archive.raw, Archive.cstring, ObjectArchive.object, read_mgXform_c)
        self.codes = {function.__code__ for function in functions}
        self.pending: dict[int, tuple[int, ObservedArchive]] = {}
        self.events: list[FieldEvent] = [(0, 4, 4, 0)]

    def __call__(self, frame: FrameType, event: str, argument: object) -> None:
        """Observe the original primitive and aggregate calls; e.g. sys.setprofile(observer)."""
        if frame.f_code not in self.codes: return
        if event == 'call':
            archive = cast(ObservedArchive, frame.f_locals.get('self', frame.f_locals.get('a')))
            self.pending[id(frame)] = archive.p, archive
        if event == 'return':
            start, archive = self.pending.pop(id(frame))
            self.events.extend(returned_fields(frame, start, archive))


def selected_fields(events: list[FieldEvent], size: int) -> list[FieldEvent]:
    """Remove component calls contained by one logical stored value; e.g. selected_fields(events, len(raw))."""
    selected: list[FieldEvent] = []
    for event in sorted(events, key=lambda item: (item[0], -item[3], -item[1])):
        if selected and event[0] < selected[-1][1]:
            assert event[1] <= selected[-1][1], (event, selected[-1])
            continue
        assert event[0] == (selected[-1][1] if selected else 0), (event, selected[-1:])
        selected.append(event)
    assert selected[-1][1] == size
    return selected


def embedded_field_case(raw: bytes, parser: Callable[[bytes, int], tuple[int, dict[str, object]]] | None = None) -> bytes:
    """Replay the full actual additive reader before exporting every logical typed field; e.g. embedded_field_case(raw)."""
    if parser is None:
        from cosworks_binary_thermal_transform_audit import archive_reader
        parser = archive_reader
    observer, prior = FieldCallObserver(), sys.getprofile()
    try:
        sys.setprofile(observer)
        end, _ = parser(raw, 0)
    finally:
        sys.setprofile(prior)
    assert end == len(raw) and not observer.pending
    fields = selected_fields(observer.events, len(raw))
    packet = b''.join(struct.pack('<III', start, end, tag) + raw[start:end] for start, end, tag, _ in fields)
    return struct.pack('<I', len(raw)) + raw + struct.pack('<I', len(fields)) + packet


def owned_field_packet() -> bytes:
    """Check every stored owned root through independent observed field calls; e.g. owned_field_packet()."""
    from owned_embedded import owned_embedded_cases
    from owned_components import owned_holders
    from owned_chains import owned_chain_cases
    from owned_references import owned_reference_cases
    roots = [bytes.fromhex(case['raw_hex']) for case in owned_embedded_cases()]
    roots += [raw for _, raw in owned_holders()]
    roots += [raw for _, _, raw in owned_chain_cases()]
    roots += [raw for _, _, raw in owned_reference_cases()]
    assert len(roots) == 1595
    return b''.join(embedded_field_case(raw) for raw in roots)


def native_field_packet(packet: bytes) -> bytes:
    """Preserve all native roots without scanning for guessed boundaries; e.g. native_field_packet(packet)."""
    output = bytearray()
    offset, count = 0, 0
    while offset < len(packet):
        size = struct.unpack_from('<I', packet, offset)[0]
        offset += 4
        output.extend(embedded_field_case(packet[offset:offset + size]))
        offset += size
        count += 1
    assert offset == len(packet) and count == 242
    return bytes(output)
