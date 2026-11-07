"""Freeze complete connection/route features and real dimension callers (#54)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
for folder in ('model-archives', 'reference-axes', 'reference-axis-owners', 'sweeps', 'reference-points',
               'configuration-toolbox', 'resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import ReferenceObjectCursor, object_cursor, observe_object_body
from primitive_inventory import observe_primitives
from freeze_owned_features import ReplacedFeatureDocument, STREAM
from owned_circular_fixtures import write_observation
from connection_fixture_bytes import NAMES, PARAMETERS, feature_object, helper_object

VERSIONS = (0x27cc, 0x2f85, 0x2f86, 0x371d, 0x371e, 0x42ee, 0x42ef, 19000)
FLAGS = (0, 2, 4, 8, 16, 30)


def install_references(cursor: ReferenceObjectCursor) -> None:
    """Use the ordinary resolved-list wrapper registrations; e.g. install_references(cursor)."""
    import referenceplanes, sketchprefix
    referenceplanes.install_scoped(cursor)
    sketchprefix.install_scoped_readers(cursor)


def verify_feature(root: dict[str, object], name: str, rich: bool, edited: bool = False) -> None:
    """Retain distinct inheritance and repeated pointer identities; e.g. verify_feature(root, NAMES[0], True)."""
    if root['_class'] != name or 'data_6f8' in root:
        raise ValueError(f'connection/route class {root["_class"]}; expected {name} with sketch-point inheritance')
    if (root['point_reference_6f0'] is not None) != rich:
        raise ValueError('inherited point reference; expected selected null/populated tag')
    if name == NAMES[1]:
        return
    payload = cast(dict[str, object], root['payload'])
    if payload['line_reference_710'] is not payload['line_reference_718']:
        raise ValueError('connection line pointers; expected shared identity')
    first = payload['dimension_740']
    if any(payload['dimension_'+offset] is not first for offset in ('748', '750', '788')):
        raise ValueError('connection dimension pointers; expected four occurrences of one object')
    verify_parameters(payload, rich, edited)


def verify_parameters(payload: dict[str, object], rich: bool, edited: bool) -> None:
    """Pin direct member order and nondefault compatibility strings; e.g. verify_parameters(payload, True, False)."""
    for offset, name in zip(('798', '7f8', '7d8', '7b8'), PARAMETERS):
        parameter = cast(dict[str, object], payload['parameter_'+offset])
        if (parameter['raw_10'], parameter['raw_18']) != ((name, 'Ω '+name) if rich else ('', '')):
            raise ValueError(f'parameter {offset}: {parameter}; expected the ordered direct string pair')
    expected = ('Bessemer compatibility Ω' if edited else 'compatibility Ω') if rich else ''
    if payload['compatibility_string'] != expected:
        raise ValueError(f'compatibility string {payload["compatibility_string"]!r}; expected {expected!r}')
    if struct.pack('<d', cast(float, payload['compatibility_double_1'])) != struct.pack('<Q', 0x7ff8000000001234 if rich else 0):
        raise ValueError('compatibility double; expected the complete original NaN/zero word')


def observe_feature(reference: Path, version: int, name: str, rich: bool, style: int) -> tuple[bytes, dict[str, object]]:
    """Require complete source decoding and slots; e.g. observe_feature(source, 19000, NAMES[0], True, 1)."""
    raw, slots = feature_object(name, version, rich, style)
    cursor = object_cursor(reference, raw, version, install_references)
    verify_feature(cast(dict[str, object], cursor.object()), name, rich)
    return observe_object_body(reference, raw, slots, NAMES.index(name), version,
        f'feature-{version:04x}-{NAMES.index(name)}-{int(rich)}-{style}', install_references)


def freeze_records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Cover inheritance, geometric flags and direct/tagged parameters; e.g. freeze_records(source)."""
    records = [observe_feature(reference, version, name, rich, style) for version in VERSIONS for name in NAMES
        for rich, style in [(False, 0), (True, 0), (True, 1)]]
    records += [observe_object_body(reference, *helper_object(0, version, rich), 2, version,
        f'parameter-{version:04x}-{int(rich)}', install_references)
        for version in VERSIONS for rich in (False, True)]
    records += [observe_object_body(reference, *helper_object(1, version, flags=flags), 3, version,
        f'radial-{version:04x}-{flags}', install_references) for version in VERSIONS for flags in FLAGS]
    return records+[observe_object_body(reference, *helper_object(2, version, flags=flags), 4, version,
        f'handle-{version:04x}-{flags}', install_references) for version in (0x27cc, 19000) for flags in FLAGS]


def freeze_owned(reference: Path, destination: Path, name: str, rich: bool, style: int, edited: bool) -> dict[str, object]:
    """Use real inherited slots, CMgr and complete helper continuation; e.g. freeze_owned(source, output, NAMES[0], True, 1, False)."""
    container, prefix, features, archive = (source_module(name, reference) for name in
        ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, imported = prefix.read(document, 'Contents/Config-0', through_part_resolved=True)
    context = cast(ReferenceObjectCursor, imported)
    feature, slots = feature_object(name, 19000, rich, style, len(context.map)-1, edited)
    raw = struct.pack('<iH', -123, 1)+feature+bytes(4)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(ReplacedFeatureDocument(document, raw), STREAM)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'connection/route end/tail/map; expected {(len(raw), b"", slots)}')
    verify_feature(parsed['items'][0], name, rich, edited)
    label = f'owned-{NAMES.index(name)}-{int(rich)}-{style}'+('-edited' if edited else '')
    return write_observation(destination, label, raw, reads) | dict(slots=slots,
        imported_slots=len(context.map), baseline_sha256=hashlib.sha256(baseline).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Keep reader hashes stable while observing every byte; e.g. freeze(source, output)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    records = freeze_records(reference)
    packet = b'CPF1'+struct.pack('<I', len(records))
    packet += b''.join(struct.pack('<I', row['slots']) for _, row in records)+b''.join(raw for raw, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    owned = [freeze_owned(reference, destination, *options) for options in
        [(NAMES[0], False, 0, False), (NAMES[1], False, 0, False), (NAMES[0], True, 0, False),
         (NAMES[0], True, 1, False), (NAMES[1], True, 1, False), (NAMES[0], True, 1, True)]]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    scripts = sorted(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[row for _, row in records], owned=owned, source_hashes=before,
        packet_sha256=hashlib.sha256(packet).hexdigest(),
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in scripts},
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/connection-points')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
