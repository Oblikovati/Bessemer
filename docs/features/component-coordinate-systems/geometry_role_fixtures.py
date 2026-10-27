"""Independent coordinate-system targets for shared component geometry roles (#46)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from typing import cast

WORKSPACE = Path(__file__).resolve().parents[3]
for folder in ('model-archives', 'reference-axes', 'reference-axis-owners', 'sweeps', 'reference-points',
               'coordinate-systems', 'resolved-features'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from fillet_fixture_bytes import WireMap
from freeze_component_references import base
from axis_owner_bytes import geometry_header
from coordinate_fixture_bytes import payload
from object_fixture_observer import object_cursor, observe_object_body
from freeze_coordinates import install_references
from freeze_blank_part import reference_snapshot

VERSIONS = (0x27cc, 0x2f85, 0x2f86, 0x3b27, 0x3b28, 19000)


def geometry_role_object(version: int, point: bool, cached: bool) -> tuple[bytes, int]:
    """Keep genuine fresh/cached coordinate targets; e.g. geometry_role_object(19000, False, True)."""
    mapping = WireMap()
    def coordinate() -> bytes:
        return mapping.new('moCoordSys_c', lambda: geometry_header(mapping, version, False)
            +payload(mapping, version, {}, False, (1, 2), 0, False))
    def component(target: bytes | None = None) -> bytes:
        name = 'moCompRefPoint_c' if point else 'moCompRefAxis_c'
        return mapping.new(name, lambda: base(version, False)+struct.pack('<iI', -7, 0xfedcba98)
            +(coordinate() if target is None else target))
    def array() -> bytes:
        slot = mapping.slot+2;first = coordinate()
        return struct.pack('<H', 2)+first+component(struct.pack('<H', slot))
    raw = mapping.new('suObArray', array) if cached else component()
    return raw, mapping.slot+1


def observe_role(reference: Path, version: int, point: bool, cached: bool) -> tuple[bytes, dict[str, object]]:
    """Check the concrete target and alias before recording bytes; e.g. observe_role(source, 19000, False, True)."""
    raw, slots = geometry_role_object(version, point, cached)
    root = cast(dict[str, object], object_cursor(reference, raw, version, install_references).object())
    if cached:
        items = cast(list[dict[str, object]], root['items'])
        target = items[1]['reference_geometry']
        if target is not items[0]:
            raise ValueError('cached coordinate target; expected identity with the first array object')
    else:
        target = root['reference_geometry']
    if cast(dict[str, object], target)['_class'] != 'moCoordSys_c':
        raise ValueError(f'coordinate target {target}; expected moCoordSys_c')
    return observe_object_body(reference, raw, slots, int(point)+2*int(cached), version,
        f'geometry-role-{version:04x}-{int(point)}-{int(cached)}', install_references)


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Pin complete field and slot observations; e.g. freeze(source, destination)."""
    before = reference_snapshot(reference);destination.mkdir(parents=True, exist_ok=True)
    records = [observe_role(reference, version, point, cached)
        for version in VERSIONS for point in (False, True) for cached in (False, True)]
    packet = b'CGR1'+struct.pack('<I', len(records))
    packet += b''.join(struct.pack('<I', row['slots']) for _, row in records)+b''.join(raw for raw, _ in records)
    (destination/'geometry-roles.bin').write_bytes(packet)
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[row for _, row in records], source_hashes=before,
        packet_sha256=hashlib.sha256(packet).hexdigest(), generator_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    sys.path.insert(0, str(WORKSPACE.parent/'inventor-interface/solidworks'))
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/component-coordinate-systems')
    (Path(__file__).parent/'geometry-roles-reference.json').write_text(json.dumps(result, indent=2)+'\n', encoding='utf-8')
