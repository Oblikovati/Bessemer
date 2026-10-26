"""Complete Hole Wizard streams with actual inherited slots and an independent label edit (#54)."""
from pathlib import Path
from typing import NotRequired, Protocol, TypedDict, cast
import hashlib
import struct
import sys
from freeze_blank_part import source_module
from primitive_inventory import observe_primitives
from hole_fixture_bytes import hole_object

WORKSPACE = Path(__file__).resolve().parents[3]
for folder in ('resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_owned_features import ReplacedFeatureDocument, STREAM
from owned_circular_fixtures import write_observation


class HoleMap(Protocol):
    map: list[object]


class HoleList(TypedDict):
    items: list[object]


class SlotFacts(TypedDict):
    point_08: tuple[float, float, float]
    point_20: tuple[float, float, float]
    vector_38: tuple[float, float, float]


class HolePayload(TypedDict):
    object_810: HoleList | None
    raw_828: str
    raw_848: int
    raw_910: int
    raw_a48: float
    raw_83c: int
    raw_bf8: int
    pairs_958: list[tuple[int, float]]
    array_c08: list[object]
    dimension_a68: NotRequired[object]
    dimension_a80: NotRequired[object]
    slot_axes_bd8: NotRequired[list[SlotFacts]]


class HoleRoot(TypedDict):
    payload: HolePayload


class HoleRead(TypedDict):
    end: int
    tail: bytes
    archive: HoleMap
    items: list[HoleRoot]


def owned_hole_stream(first_slot: int, rich: bool, slot: bool, edited: bool) -> tuple[bytes, int]:
    """Generate each expected stream without patching offsets; e.g. owned_hole_stream(96, True, True, True)."""
    raw, slots = hole_object(19000, rich, slot, first_slot, edited)
    return struct.pack('<iH', -123, 1)+raw+bytes(4), slots


def verify_parameters(payload: HolePayload, rich: bool, edited: bool) -> None:
    """Preserve native-normalized values and NaN payload bits; e.g. verify_parameters(payload, True, True)."""
    label = 'Bessemer hole Ω' if edited else ('hole Ω 828' if rich else '')
    if payload['raw_828'] != label:
        raise ValueError(f'Hole Wizard label {payload["raw_828"]!r}; expected {label!r}')
    values = (payload['raw_848'], payload['raw_910'], payload['raw_83c'], payload['raw_bf8'])
    if values != (5, 99, -24, 255):
        raise ValueError(f'Hole Wizard raw values {values}; expected {(5, 99, -24, 255)}')
    bits = struct.pack('<d', payload['raw_a48'])
    if bits != struct.pack('<Q', 0x7ff8000000000042):
        raise ValueError(f'Hole Wizard NaN bits {bits.hex()}; expected 420000000000f87f')
    expected = [(-7, 2.5), (2147483647, -0.0)] if rich else []
    if payload['pairs_958'] != expected:
        raise ValueError(f'Hole Wizard pairs {payload["pairs_958"]}; expected {expected}')


def verify_slots(payload: HolePayload, rich: bool, slot: bool) -> None:
    """Distinguish absent slot blocks from independent direct bodies; e.g. verify_slots(payload, True, True)."""
    axes = payload.get('slot_axes_bd8')
    if not slot:
        if axes is not None or 'dimension_a68' in payload or 'dimension_a80' in payload:
            raise ValueError('disabled Hole Wizard slot block; expected absent axes and dimensions')
        return
    count = 3 if rich else 1
    if axes is None or len(axes) != count:
        raise ValueError(f'Hole Wizard axes {axes}; expected {count} complete direct bodies')
    if payload.get('dimension_a68') is None or payload['dimension_a68'] is not payload.get('dimension_a80'):
        raise ValueError('Hole Wizard dimensions a68/a80; expected one shared complete distance dimension')
    for index, axis in enumerate(axes):
        expected = ((float(index), 2.0, 3.0), (4.0, 5.0, 6.0), (0.0, 0.0, 1.0))
        actual = (axis['point_08'], axis['point_20'], axis['vector_38'])
        if actual != expected:
            raise ValueError(f'Hole Wizard axis {index} {actual}; expected {expected}')


def verify_aliases(root: HoleRoot, rich: bool) -> None:
    """Require self references in both collection framings; e.g. verify_aliases(root, True)."""
    payload = root['payload']
    if not rich:
        if payload['object_810'] is not None or payload['array_c08'] != []:
            raise ValueError('minimal Hole Wizard references; expected null pointer and empty array')
        return
    pointer = payload['object_810']
    collections = [pointer['items'] if pointer is not None else [], payload['array_c08']]
    if any(len(items) != 2 or any(item is not root for item in items) for items in collections):
        raise ValueError('Hole Wizard collection aliases; expected two root references in each collection')


def freeze_owned_case(reference: Path, destination: Path, rich: bool, slot: bool, edited: bool) -> dict[str, object]:
    """Check EOF and actual model/CMgr graph before freezing; e.g. freeze_owned_case(source, output, True, True, False)."""
    container, features, prefix, archive = (source_module(name, reference) for name in
        ('swcontainer', 'resolvedfeatures', 'modelprefix', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, imported = prefix.read(document, 'Contents/Config-0', through_part_resolved=True)
    context = cast(HoleMap, imported)
    raw, slots = owned_hole_stream(len(context.map)-1, rich, slot, edited)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = cast(HoleRead, features.read_supported(ReplacedFeatureDocument(document, raw), STREAM))
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'Hole Wizard end/tail/map {(parsed["end"], parsed["tail"], len(parsed["archive"].map))}; expected {(len(raw), b"", slots)}')
    root = parsed['items'][0]
    verify_parameters(root['payload'], rich, edited)
    verify_slots(root['payload'], rich, slot)
    verify_aliases(root, rich)
    label = f'owned-{"populated" if rich else "minimal"}-{"slot" if slot else "plain"}'+('-edited' if edited else '')
    return write_observation(destination, label, raw, reads) | dict(slots=slots,
        imported_slots=len(context.map), baseline_sha256=hashlib.sha256(baseline).hexdigest())


def freeze_owned(reference: Path, destination: Path) -> list[dict[str, object]]:
    """Freeze four graph/slot variants and one label edit; e.g. freeze_owned(source, output)."""
    return [freeze_owned_case(reference, destination, rich, slot, edited)
            for rich, slot, edited in [(False, False, False), (False, True, False),
                (True, False, False), (True, True, False), (True, True, True)]]
