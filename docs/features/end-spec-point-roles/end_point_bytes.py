"""Complete end-specification point-role regression wire graphs (#46)."""
from collections.abc import Callable
import struct
from fillet_fixture_bytes import WireMap
from body_feature_fixture_bytes import end_spec, revolution_end, header, atom_fields, body_suffix, suffix
from sweep_fixture_bytes import shared_object
from plane_point_fixture_bytes import POINTS, point_body

OWNERS = ('moEndSpec_c', 'moRevEndSpec_c')


def role_object(owner: int, point: int, version: int, cached: bool, first_slot: int = 0, edited: bool = False, feature: bool = False) -> tuple[bytes, int]:
    """Keep complete point bodies and share both side references; e.g. role_object(0, 1, 19000, True)."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def reference() -> bytes:
        return shared_object(mapping, ids, 'end-point', POINTS[point], lambda: point_body(mapping, version, ids, point, edited))
    def contents() -> bytes:
        if owner == 0:
            return end_spec(mapping, version, False, 3, 3, reference)
        return revolution_end(mapping, version, False, reference)
    def array() -> bytes:
        first = reference() if cached else bytes(2)
        second = feature_object(mapping, version, owner, contents) if feature else mapping.new(OWNERS[owner], contents)
        return struct.pack('<H', 2)+first+second
    raw = mapping.new('suObArray', array)
    return raw, mapping.slot+1


def feature_object(mapping: WireMap, version: int, owner: int, contents: Callable[[], bytes]) -> bytes:
    """Reuse complete inherited feature and suffix builders for the shipped caller; e.g. feature_object(mapping, 19000, 0, contents)."""
    name = ('moBoss_c', 'moRevolution_c')[owner]
    def body() -> bytes:
        base = header(mapping, version, False)+atom_fields(mapping, False, mapping.slot)+body_suffix(False)
        return base+suffix(mapping, version, name, False, lambda: mapping.new(OWNERS[owner], contents))
    return mapping.new(name, body)
