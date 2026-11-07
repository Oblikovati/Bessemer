"""Observe complete coordinate-system and axis-data fixtures (#54)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
for folder in ('model-archives', 'reference-axes', 'reference-axis-owners', 'sweeps', 'reference-points',
               'resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import object_cursor, observe_object_body, ReferenceObjectCursor
from primitive_inventory import observe_primitives
from freeze_owned_features import ReplacedFeatureDocument, STREAM
from owned_circular_fixtures import write_observation
from coordinate_fixture_bytes import AXES, coordinate_object

VERSIONS = (0x27cc, 0x2f85, 0x2f86, 0x371a, 0x371b, 0x371d, 0x371e, 19000)
SELECTORS = ((0, 65535), (1, 1), (1, 2), (2, 1), (3, 1), (2, 3), (3, 3), (65535, 2))


def install_references(cursor: ReferenceObjectCursor) -> None:
    """Install the ordinary scoped geometric readers; e.g. install_references(cursor)."""
    import referenceplanes, sketchprefix
    referenceplanes.install_scoped(cursor)
    sketchprefix.install_scoped_readers(cursor)


def verify_root(root: dict[str, object], version: int, rich: bool, selectors: tuple[int, int]) -> None:
    """Check native equality and numeric ordering independently; e.g. verify_root(root, 19000, True, (3, 1))."""
    payload = cast(dict[str, object], root['payload'])
    axes = cast(list[dict[str, object]], payload['axes'])
    expected = [number for number in (1, 2, 3) if number in selectors]
    if [item['number'] for item in axes] != expected:
        raise ValueError(f'coordinate axis numbers {axes}; expected {expected} in numeric order')
    if axes and any(item['data'] is not axes[0]['data'] for item in axes):
        raise ValueError('coordinate axes; expected shared selected-axis identity')
    if (payload['origin_6f0'] is not None) != rich or ('raw_744' in payload) != (version > 0x371a):
        raise ValueError(f'coordinate presence/version {(rich, version)}; expected selected origin and 0x371A tail gate')
    for item in axes:
        if item['data'] is not None:
            verify_axis(cast(dict[str, object], item['data']), rich)


def verify_axis(root: dict[str, object], rich: bool) -> None:
    """Retain direct-array sharing and exact nonunit vector bits; e.g. verify_axis(root, True)."""
    references = cast(list[object], root['references_08'])
    if len(references) != (3 if rich else 0):
        raise ValueError(f'coordinate axis references {len(references)}; expected selected direct-array length')
    if rich and (references[0] is not references[1] or references[2] is not None):
        raise ValueError('coordinate axis references; expected repeated reference and null')
    expected = struct.pack('<3Q', 0x8000000000000000, 0x4000000000000000 if rich else 0, 0x7ff8000000001234 if rich else 0)
    if struct.pack('<3d', *cast(tuple[float, float, float], root['vector_28'])) != expected:
        raise ValueError(f'coordinate axis vector {root["vector_28"]}; expected original signed-zero/nonunit/NaN bits')


def observe_coordinate(reference: Path, version: int, rich: bool, selectors: tuple[int, int],
                       style: int, helper: bool = False) -> tuple[bytes, dict[str, object]]:
    """Require full source decoding before primitive observation; e.g. observe_coordinate(source, 19000, True, (3, 1), 2)."""
    raw, slots = coordinate_object(version, rich, selectors, style, helper=helper)
    root = cast(dict[str, object], object_cursor(reference, raw, version, install_references).object())
    if helper:
        verify_axis(root, rich)
    else:
        verify_root(root, version, rich, selectors)
    label = f'coordinate-{version:04x}-{int(rich)}-{selectors[0]}-{selectors[1]}-{style}-{int(helper)}'
    return observe_object_body(reference, raw, slots, style+1 if helper else 0, version, label, install_references)


def freeze_owned(reference: Path, destination: Path, rich: bool, selectors: tuple[int, int],
                 style: int, edited: bool) -> dict[str, object]:
    """Keep the actual CMgr and imported identities; e.g. freeze_owned(source, output, True, (3, 1), 2, False)."""
    container, prefix, features, archive = (source_module(name, reference) for name in
        ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, context = prefix.read(document, 'Contents/Config-0', through_part_resolved=True)
    feature, slots = coordinate_object(19000, rich, selectors, style, len(context.map)-1, edited)
    raw = struct.pack('<iH', -123, 1)+feature+bytes(4)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(ReplacedFeatureDocument(document, raw), STREAM)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'coordinate end/tail/map; expected {(len(raw), b"", slots)}')
    verify_root(parsed['items'][0], 19000, rich, selectors)
    label = f'owned-{int(rich)}-{selectors[0]}-{selectors[1]}-{style}'+('-edited' if edited else '')
    return write_observation(destination, label, raw, reads) | dict(slots=slots,
        imported_slots=len(context.map), baseline_sha256=hashlib.sha256(baseline).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind exact bytes and all selector modes to stable readers; e.g. freeze(source, output)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    records = [observe_coordinate(reference, version, rich, selectors, style)
        for version in VERSIONS for selectors in SELECTORS for rich, style in ((False, 0), (True, 0), (True, 1), (True, 2))]
    records += [observe_coordinate(reference, version, rich, (1, 2), style, True)
        for version in VERSIONS for style in range(3) for rich in (False, True)]
    packet = b'CSY1'+struct.pack('<I', len(records))
    packet += b''.join(struct.pack('<I', row['slots']) for _, row in records)+b''.join(raw for raw, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    owned = [freeze_owned(reference, destination, *options) for options in
        [(False, (1, 2), 0, False), (True, (3, 1), 0, False), (True, (3, 1), 1, False),
         (True, (3, 1), 2, False), (True, (1, 1), 1, False), (True, (0, 65535), 2, False), (True, (3, 1), 2, True)]]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    return fixture_report(records, owned, before, packet)


def fixture_report(records: list[tuple[bytes, dict[str, object]]], owned: list[dict[str, object]],
                   hashes: dict[str, str], packet: bytes) -> dict[str, object]:
    """Bind the shipped builders and primitive observations; e.g. fixture_report(records, owned, hashes, packet)."""
    scripts = sorted(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[row for _, row in records], owned=owned, source_hashes=hashes,
        packet_sha256=hashlib.sha256(packet).hexdigest(),
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in scripts},
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    result = freeze(REFERENCE, WORKSPACE/'tests/fixtures/coordinate-systems')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
