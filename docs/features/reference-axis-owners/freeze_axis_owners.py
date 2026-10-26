"""Freeze complete axis-owner regressions without component-feature extensions (#46)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
for folder in ('model-archives', 'reference-axes', 'resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import ReferenceObjectCursor, object_cursor, observe_object_body
from primitive_inventory import observe_primitives
from freeze_owned_features import ReplacedFeatureDocument, STREAM
from owned_circular_fixtures import write_observation
from axis_owner_bytes import axis_object


def verify_owners(root: dict[str, object], rich: bool) -> None:
    """Require four exact self-owner identities; e.g. verify_owners(axis, True)."""
    geometry = cast(dict[str, object], root['reference_geometry'])
    choosers = cast(dict[str, object], geometry['atom_choosers'])
    for name in ('chooser_4a0', 'chooser_4a8', 'chooser_4b8', 'chooser_4c0'):
        chooser = cast(dict[str, object], choosers[name])
        if chooser['presence'] != (65535 if rich else 0):
            raise ValueError(f'axis chooser {name} presence {chooser["presence"]}; expected {65535 if rich else 0}')
        body = cast(dict[str, object] | None, chooser['body'])
        if rich and (body is None or body['owner'] is not root):
            raise ValueError(f'axis chooser {name} owner; expected the enclosing axis identity')
        if not rich and body is not None:
            raise ValueError(f'axis chooser {name} body; expected absent body')


def freeze_bare(reference: Path, destination: Path, version: int, rich: bool) -> dict[str, object]:
    """Observe complete axes including self-owner aliases; e.g. freeze_bare(source, output, 19000, True)."""
    raw, slots = axis_object(version, rich)
    cursor = object_cursor(reference, raw, version, None)
    verify_owners(cast(dict[str, object], cursor.object()), rich)
    frame, facts = observe_object_body(reference, raw, slots, int(rich), version, f'axis-{version:04x}-{int(rich)}')
    oracle_start = 13+len(raw)
    (destination/(facts['label']+'.bin')).write_bytes(raw)
    (destination/(facts['label']+'.tsv')).write_bytes(frame[oracle_start:])
    return facts


def freeze_owned(reference: Path, destination: Path, rich: bool, edited: bool) -> dict[str, object]:
    """Require actual CMgr/helper framing and exact EOF; e.g. freeze_owned(source, output, True, False)."""
    container, prefix, features, archive = (source_module(name, reference) for name in
        ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, imported = prefix.read(document, 'Contents/Config-0', through_part_resolved=True)
    context = cast(ReferenceObjectCursor, imported)
    feature, slots = axis_object(19000, rich, len(context.map)-1, edited)
    raw = struct.pack('<iH', -123, 1)+feature+bytes(4)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(ReplacedFeatureDocument(document, raw), STREAM)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'owned axis end/tail/map; expected {(len(raw), b"", slots)}')
    flags = parsed['items'][0]['reference_geometry']['flags_6d4']
    verify_owners(parsed['items'][0], rich)
    if flags != (0x87654321 if edited else 0x89abcdef):
        raise ValueError(f'axis flags {flags:#x}; expected selected raw word')
    label = 'owned-'+('populated' if rich else 'minimal')+('-edited' if edited else '')
    return write_observation(destination, label, raw, reads) | dict(slots=slots,
        imported_slots=len(context.map), baseline_sha256=hashlib.sha256(baseline).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Keep full independent primitive partitions and stable readers; e.g. freeze(source, output)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    cases = [freeze_bare(reference, destination, version, rich) for version in (0x27cc, 19000) for rich in (False, True)]
    owned = [freeze_owned(reference, destination, rich, edited) for rich, edited in
        [(False, False), (True, False), (True, True)]]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    scripts = sorted(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=cases, owned=owned, source_hashes=before,
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in scripts},
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/reference-axis-owners')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
