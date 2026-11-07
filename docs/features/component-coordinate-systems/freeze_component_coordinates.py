"""Independent complete component-coordinate archives and fixed-word edits (#54)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
for folder in ('model-archives', 'reference-axes', 'reference-axis-owners', 'sweeps', 'reference-points',
               'coordinate-systems', 'resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import object_cursor, observe_object_body
from primitive_inventory import observe_primitives
from freeze_coordinates import install_references
from freeze_owned_features import ReplacedFeatureDocument, STREAM
from owned_circular_fixtures import write_observation
from component_coordinate_bytes import component_coordinate_object

VERSIONS = (0x27cc, 0x2b60, 0x2b61, 0x2bb8, 0x2bb9, 0x2f55, 0x2f56, 0x2f85, 0x2f86,
            0x371a, 0x371b, 0x371d, 0x371e, 0x3b27, 0x3b28, 0x3ef5, 0x3ef6, 19000)


def component_root(root: dict[str, object], mode: int) -> dict[str, object]:
    """Resolve only documented containing frames; e.g. component_root(root, 2)."""
    if mode == 3:
        return cast(dict[str, object], root['component_reference'])
    if mode != 2:
        return root
    items = cast(list[dict[str, object]], root['items'])
    if items[1]['reference_geometry'] is not items[0]:
        raise ValueError('component coordinate geometry; expected alias to the first complete coordinate object')
    return items[1]


def verify_component(root: dict[str, object], version: int, rich: bool, mode: int,
                     kind: int = 11, edited: bool = False) -> None:
    """Check concrete types and retained fixed words; e.g. verify_component(root, 19000, True, 3)."""
    component = component_root(root, mode)
    if (component['_class'], component['feature_id'], component['feature_time']) != ('moCompCoordSys_c', -2147483648, 0xfedcba98):
        raise ValueError(f'component coordinate identity {component}; expected concrete class and original signed ID/u32 time')
    geometry = component['reference_geometry']
    if (geometry is not None) != (mode != 0) or geometry is not None and cast(dict[str, object], geometry)['_class'] != 'moCoordSys_c':
        raise ValueError(f'component coordinate target {geometry}; expected selected null or complete coordinate system')
    if ('transform_100' in component) != (version > 0x3b27):
        raise ValueError(f'component coordinate version {version}; expected the 0x3B27 fixed-word boundary')
    if version <= 0x3b27:
        return
    words = cast(list[int], component['transform_100'])
    expected_first = 12345 if edited else (0xffffffff if rich else 0)
    if len(words) != 30 or words[0] != expected_first or words[28] != (0x12340000 if rich else 0)+kind:
        raise ValueError(f'component coordinate transform {words}; expected 30 original u32 words and selected kind')


def observe_component(reference: Path, version: int, rich: bool, mode: int,
                      kind: int = 11, mask_case: bool = False) -> tuple[bytes, dict[str, object]]:
    """Require complete reader consumption and alias checks; e.g. observe_component(source, 19000, True, 2)."""
    raw, slots = component_coordinate_object(version, rich, mode, kind=kind)
    root = cast(dict[str, object], object_cursor(reference, raw, version, install_references).object())
    verify_component(root, version, rich, mode, kind)
    label = f'component-coordinate-{version:04x}-{int(rich)}-{mode}-{kind}'
    return observe_object_body(reference, raw, slots, 4 if mask_case else mode, version, label, install_references)


def freeze_owned(reference: Path, destination: Path, rich: bool, mode: int, edited: bool) -> dict[str, object]:
    """Import the real model/CMgr map and continuation; e.g. freeze_owned(source, output, True, 3, False)."""
    container, prefix, features, archive = (source_module(name, reference) for name in
        ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, context = prefix.read(document, 'Contents/Config-0', through_part_resolved=True)
    feature, slots = component_coordinate_object(19000, rich, mode, len(context.map)-1, edited=edited)
    raw = struct.pack('<iH', -123, 1)+feature+bytes(4)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(ReplacedFeatureDocument(document, raw), STREAM)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'component coordinate end/tail/map; expected {(len(raw), b"", slots)}')
    verify_component(parsed['items'][0], 19000, rich, mode, edited=edited)
    label = f'owned-{int(rich)}-{mode}'+('-edited' if edited else '')
    return write_observation(destination, label, raw, reads) | dict(slots=slots,
        imported_slots=len(context.map), baseline_sha256=hashlib.sha256(baseline).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Freeze every inherited boundary, container and supported kind; e.g. freeze(source, output)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    records = [observe_component(reference, version, rich, mode)
        for version in VERSIONS for rich in (False, True) for mode in range(4)]
    records += [observe_component(reference, 19000, False, 0, kind, True) for kind in range(16)]
    packet = b'CCR1'+struct.pack('<I', len(records))
    packet += b''.join(struct.pack('<I', row['slots']) for _, row in records)+b''.join(raw for raw, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    owned = [freeze_owned(reference, destination, *options) for options in
        [(False, 0, False), (True, 0, False), (True, 1, False), (True, 2, False), (True, 3, False), (True, 1, True)]]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    return fixture_report(records, owned, before, packet)


def fixture_report(records: list[tuple[bytes, dict[str, object]]], owned: list[dict[str, object]],
                   hashes: dict[str, str], packet: bytes) -> dict[str, object]:
    """Bind local callers and reused construction helpers; e.g. fixture_report(records, owned, hashes, packet)."""
    scripts = sorted(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[row for _, row in records], owned=owned, source_hashes=hashes,
        packet_sha256=hashlib.sha256(packet).hexdigest(),
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in scripts},
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    result = freeze(REFERENCE, WORKSPACE/'tests/fixtures/component-coordinate-systems')
    (Path(__file__).parent/'reference.json').write_text(json.dumps(result, indent=2)+'\n', encoding='utf-8')
