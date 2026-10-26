"""Independent complete sweeps, shared conditional dimensions and reference helpers (#54)."""
from collections.abc import Callable
from dataclasses import dataclass
import struct
from fillet_fixture_bytes import WireMap, body
from body_feature_fixture_bytes import direction
from freeze_component_references import base as component_base
from freeze_model_references import wrapper
from freeze_geometric_references import wire as geometric
from display_dimension_fixture_bytes import body as stripped, shared, suffix


@dataclass(frozen=True)
class SweepOptions:
    enabled: int = 0
    second_mode: int = 3
    radial: int = 0
    angular: int = 7
    first_direction: bool = False
    second_direction: bool = False


def shared_object(mapping: WireMap, identities: dict[str, int], key: str, name: str,
                   contents: Callable[[], bytes]) -> bytes:
    """Reserve identity before nested objects and retain aliases; e.g. shared_object(mapping, ids, 'profile', 'moCompProfile_c', body)."""
    if key in identities:
        return struct.pack('<H', identities[key])
    def reserved() -> bytes:
        identities[key] = mapping.slot
        return contents()
    return mapping.new(name, reserved)


def profile(mapping: WireMap, version: int, identities: dict[str, int]) -> bytes:
    """Reuse the complete component-feature base without a profile suffix; e.g. profile(mapping, 19000, ids)."""
    return shared_object(mapping, identities, 'profile', 'moCompProfile_c',
        lambda: component_base(version, False)+struct.pack('<iI', -2147483648, 0x89abcdef))


def pointer_array(mapping: WireMap, version: int, identities: dict[str, int], count: int, escape: bool) -> bytes:
    """Keep direct arrays/lists independent of the MFC map; e.g. pointer_array(mapping, 19000, ids, 2, True)."""
    raw = struct.pack('<HI', 65535, count) if escape else struct.pack('<H', count)
    return raw+b''.join(profile(mapping, version, identities) for _ in range(count))


def dimension(mapping: WireMap, version: int, identities: dict[str, int], name: str) -> bytes:
    """Share complete dimensions across separate occurrences; e.g. dimension(mapping, 19000, ids, 'moDisplayAngularDim_c')."""
    return shared_object(mapping, identities, name, name,
        lambda: shared(mapping, version, name, False, mapping.slot)+suffix(mapping, version, name, False))


def endpoint_body(mapping: WireMap, version: int, identities: dict[str, int], rich: bool) -> bytes:
    """Retain the full point-reference base before the endpoint suffix; e.g. endpoint_body(mapping, 19000, ids, True)."""
    component = profile(mapping, version, identities) if rich else bytes(2)
    return component+wrapper(version, False)[2:]+struct.pack('<4di', 1, 2, 3, -0.0, -2147483648)


def endpoint(mapping: WireMap, version: int, identities: dict[str, int], rich: bool) -> bytes:
    """Allocate an endpoint once for cached point occurrences; e.g. endpoint(mapping, 19000, ids, True)."""
    return shared_object(mapping, identities, 'endpoint', 'moEndPointRef_w',
        lambda: endpoint_body(mapping, version, identities, rich))


def curve(mapping: WireMap, version: int, identities: dict[str, int]) -> bytes:
    """Use a complete supported curve rather than guessed curve bytes; e.g. curve(mapping, 19000, ids)."""
    return shared_object(mapping, identities, 'curve', 'moGeneralCurveRef_w',
        lambda: stripped(geometric('moGeneralCurveRef_w', version, False, 0)))


def synchronization_body(mapping: WireMap, version: int, identities: dict[str, int], rich: bool) -> bytes:
    """Preserve separate points, raw words and repeated curve pointers; e.g. synchronization_body(mapping, 19000, ids, True)."""
    raw = endpoint_body(mapping, version, identities, rich)+struct.pack('<iIId', -1, 0xffffffff, 0x80000000, 1.25)
    raw += curve(mapping, version, identities) if rich else bytes(2)
    raw += struct.pack('<2i', -2147483648, 2147483647)
    raw += endpoint(mapping, version, identities, rich) if rich else bytes(2)
    raw += struct.pack('<10d', -0.0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
    return raw+(curve(mapping, version, identities)*2 if rich else bytes(4))


def synchronization(mapping: WireMap, version: int, identities: dict[str, int], rich: bool) -> bytes:
    """Keep tagged synchronization identity distinct from its inherited endpoint; e.g. synchronization(mapping, 19000, ids, True)."""
    return shared_object(mapping, identities, 'synch', 'moLoftSynchRef_c',
        lambda: synchronization_body(mapping, version, identities, rich))


def data_prefix(mapping: WireMap, version: int, identities: dict[str, int], rich: bool, options: SweepOptions) -> bytes:
    """Write independent direction tags only for exact control value two; e.g. data_prefix(mapping, 19000, ids, True, options)."""
    raw = b''.join(pointer_array(mapping, version, identities, count if rich else 0, rich) for count in (2, 1, 1))
    raw += struct.pack('<BBidBBii', 255, 254, options.angular, 0.5, 253, 252,
        2 if options.first_direction else -1, 2 if options.second_direction else 0)
    for enabled in (options.first_direction, options.second_direction):
        if enabled:
            raw += shared_object(mapping, identities, 'direction', 'moDirectionSpec_c', lambda: direction(mapping, rich))
    raw += pointer_array(mapping, version, identities, int(rich), rich)
    return raw+pointer_array(mapping, version, identities, 2 if rich else 0, rich)


def sweep_data(mapping: WireMap, version: int, identities: dict[str, int], rich: bool, options: SweepOptions) -> bytes:
    """Retain the complete direct data body and versioned final word; e.g. sweep_data(mapping, 19000, ids, True, options)."""
    raw = data_prefix(mapping, version, identities, rich, options)
    raw += struct.pack('<iBB2d3i2diBBid', -1, 251, options.enabled, 1.5, 2.5, options.second_mode,
        -2, -3, 3.5, 4.5, -4, 250, 249, options.radial, 5.5)
    raw += bytes(2)+(endpoint(mapping, version, identities, True) if rich else bytes(2))+bytes([248])
    raw += pointer_array(mapping, version, identities, int(rich), rich)+pointer_array(mapping, version, identities, 0, rich)
    return raw+(struct.pack('<i', -6) if version > 0x27d9 else b'')


def sweep_suffix(mapping: WireMap, version: int, identities: dict[str, int], rich: bool,
                  options: SweepOptions, edited: bool) -> bytes:
    """Construct each optional dimension predicate independently; e.g. sweep_suffix(mapping, 19000, ids, True, options, False)."""
    raw = sweep_data(mapping, version, identities, rich, options)+struct.pack('<i5B', 12345 if edited else -5, 247, 246, 245, 244, 243)
    if options.enabled:
        raw += dimension(mapping, version, identities, 'moDisplayDistanceDim_c')
        if options.second_mode == 3:
            raw += dimension(mapping, version, identities, 'moDisplayDistanceDim_c')
    raw += bytes([242, 241])
    if options.angular in (8, 9):
        raw += dimension(mapping, version, identities, 'moDisplayAngularDim_c')
        raw += dimension(mapping, version, identities, 'moDisplayAngularDim_c')
    if options.radial:
        raw += dimension(mapping, version, identities, 'moDisplayRadialDim_c')
    return raw


def sweep_object(version: int, rich: bool, options: SweepOptions, first_slot: int = 0, edited: bool = False,
                 name: str = 'moSweep_c') -> tuple[bytes, int]:
    """Compose a complete feature in either inherited context; e.g. sweep_object(19000, True, SweepOptions(), 96)."""
    if first_slot < 0 or not 0 <= options.enabled <= 255:
        raise ValueError(f'sweep slot/enabled {(first_slot, options.enabled)}; expected nonnegative slot and u8 enablement')
    if name not in ('moSweep_c', 'moSweepCut_c'):
        raise ValueError(f'sweep class {name!r}; expected moSweep_c or moSweepCut_c')
    mapping = WireMap();mapping.slot = first_slot;identities: dict[str, int] = {}
    def contents() -> bytes:
        owner = mapping.slot
        return body(mapping, version, rich, owner)+sweep_suffix(mapping, version, identities, rich, options, edited)
    raw = mapping.new(name, contents)
    return raw, mapping.slot+1


def contact_body(mapping: WireMap, version: int, identities: dict[str, int], rich: bool) -> bytes:
    """Preserve the direct array and mixed-width contact suffix; e.g. contact_body(mapping, 19000, ids, True)."""
    raw = pointer_array(mapping, version, identities, 2 if rich else 0, rich)+struct.pack('<di', -0.0, -2147483648)
    raw += dimension(mapping, version, identities, 'moDisplayAngularDim_c') if rich else bytes(2)
    raw += struct.pack('<d4i', 1.25, -1, 0, 2147483647, -2147483648)
    raw += synchronization(mapping, version, identities, rich) if rich else bytes(2)
    raw += struct.pack('<3id', 7, -7, 0, -0.0)
    return raw+(curve(mapping, version, identities) if rich else bytes(2))


def helper_object(name: str, version: int, rich: bool) -> tuple[bytes, int]:
    """Build complete standalone dependency objects; e.g. helper_object('moWireContact_c', 19000, True)."""
    mapping = WireMap();identities: dict[str, int] = {}
    builders: dict[str, Callable[[], bytes]] = {
        'moWireContact_c': lambda: contact_body(mapping, version, identities, rich),
        'moEndPointRef_w': lambda: endpoint_body(mapping, version, identities, rich),
        'moLoftSynchRef_c': lambda: synchronization_body(mapping, version, identities, rich),
        'moCompProfile_c': lambda: component_base(version, False)+struct.pack('<iI', -2147483648, 0x89abcdef),
    }
    if name not in builders:
        raise ValueError(f'sweep dependency {name!r}; expected one of {tuple(builders)}')
    raw = mapping.new(name, builders[name])
    return raw, mapping.slot+1
