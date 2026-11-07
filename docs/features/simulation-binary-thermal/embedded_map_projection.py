"""Observe actual unmodified MFC object calls and resolved map identities (#134)."""
import struct
import sys
from types import FrameType
from typing import Protocol, cast
from owned_embedded import owned_embedded_cases
from owned_components import owned_holders
from owned_chains import owned_chain_cases
from owned_references import owned_reference_cases


class ObservedArchive(Protocol):
    p: int
    b: bytes
    map: list[object]


def alias_class(slots: list[object], index: int) -> tuple[str, int]:
    """Require an unambiguous actual class schema for a resolved alias; e.g. alias_class(slots, 2)."""
    name = cast(dict[str, object], slots[index])['_class']
    schemas = {cast(tuple[str, str, int], slot)[2] for slot in slots[:index]
               if isinstance(slot, tuple) and slot[1] == name}
    assert len(schemas) == 1, (name, schemas)
    return cast(str, name), next(iter(schemas))


def object_event(frame: FrameType, start: int, prior_slots: int) -> bytes:
    """Use actual resolved frame locals and source map, retaining exact framing; e.g. object_event(frame, 4, 1)."""
    archive = cast(ObservedArchive, frame.f_locals['self'])
    tag = cast(int, frame.f_locals['tag'])
    extension = frame.f_locals['big']
    if tag == 0:
        meaning, index, name, schema = 0, 0, '', 0
    elif tag == 0xffff or 'obj' in frame.f_locals:
        meaning, index = 1, prior_slots + int(tag == 0xffff)
        name, schema = cast(str, frame.f_locals['name']), cast(int, frame.f_locals['schema'])
    else:
        meaning, index = 2, cast(int, frame.f_locals['idx'])
        name, schema = alias_class(archive.map, index)
    encoded = name.encode('ascii')
    assert struct.unpack_from('<H', archive.b, start)[0] == tag
    return struct.pack('<IIIIII', tag, 0xffffffff if extension is None else cast(int, extension), meaning, index, schema, len(encoded)) + encoded


class ObjectCallObserver:
    def __init__(self) -> None:
        """Create operation-local profiling state; e.g. ObjectCallObserver()."""
        from mfcobjects import ObjectArchive
        self.code = ObjectArchive.object.__code__
        self.pending: dict[int, tuple[int, int]] = {}
        self.events: list[tuple[int, bytes]] = []
        self.slots: int = 0

    def __call__(self, frame: FrameType, event: str, argument: object) -> None:
        """Observe the original object implementation without replacement; e.g. sys.setprofile(observer)."""
        if frame.f_code is not self.code: return
        archive = cast(ObservedArchive, frame.f_locals['self'])
        if event == 'call':
            self.pending[id(frame)] = archive.p, len(archive.map)
        if event == 'return':
            start, prior_slots = self.pending.pop(id(frame))
            self.events.append((start, object_event(frame, start, prior_slots)))
            self.slots = max(self.slots, len(archive.map))


def embedded_map_case(raw: bytes) -> bytes:
    """Replay the actual complete additive callback while observing object calls; e.g. embedded_map_case(raw)."""
    from cosworks_binary_thermal_transform_audit import archive_reader
    observer = ObjectCallObserver()
    prior = sys.getprofile()
    try:
        sys.setprofile(observer)
        end, _ = archive_reader(raw, 0)
    finally:
        sys.setprofile(prior)
    assert end == len(raw) and not observer.pending
    events = [record for _, record in sorted(observer.events)]
    return struct.pack('<I', len(raw)) + raw + struct.pack('<II', observer.slots, len(events)) + b''.join(events)


def owned_map_packet() -> bytes:
    """Cover every declared class and historical owned root; e.g. owned_map_packet()."""
    roots = [bytes.fromhex(case['raw_hex']) for case in owned_embedded_cases()]
    roots += [raw for _, raw in owned_holders()]
    roots += [raw for _, _, raw in owned_chain_cases()]
    roots += [raw for _, _, raw in owned_reference_cases()]
    assert len(roots) == 1595
    return b''.join(embedded_map_case(raw) for raw in roots)


def native_map_packet(packet: bytes) -> bytes:
    """Retain all native embedded roots in their original order; e.g. native_map_packet(packet)."""
    output = bytearray()
    offset = 0
    count = 0
    while offset < len(packet):
        size = struct.unpack_from('<I', packet, offset)[0]
        offset += 4
        output.extend(embedded_map_case(packet[offset:offset + size]))
        offset += size
        count += 1
    assert offset == len(packet) and count == 242
    return bytes(output)
