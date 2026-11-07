"""Freeze complete documented comment objects and independent primitive oracles (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from typing import cast

WORKSPACE = Path(__file__).resolve().parents[3]
for folder in ('model-archives', 'resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import object_cursor, observe_object_body
from primitive_inventory import observe_primitives
from freeze_owned_features import ReplacedFeatureDocument, STREAM
from owned_circular_fixtures import write_observation
from comment_fixture_bytes import NAMES, comment_object

VERSIONS = (0x27cc, 0x332b, 0x332c, 0x3b0f, 0x3b10, 19000)


def verify_comment(root: dict[str, object], version: int, rich: bool) -> None:
    """Check stored kind and direct-list identities; e.g. verify_comment(root, 19000, True)."""
    if root['raw_28'] != (2 if rich else 0):
        raise ValueError(f'comment kind {root["raw_28"]}; expected the original stored kind')
    direct_list = cast(dict[str, object], root['comments_30'])
    items = cast(list[dict[str, object] | None], direct_list['items'])
    if len(items) != (4 if rich else 0):
        raise ValueError(f'comment items {len(items)}; expected selected direct-list length')
    if rich and (items[0] is not items[1] or items[2] is not None or items[3] is items[0]):
        raise ValueError('comment identities; expected repeated first, null and distinct second items')
    first = cast(dict[str, object], items[0]) if rich else {}
    if rich and first['time_18'] != 0xffffffff:
        raise ValueError(f'comment time {first["time_18"]}; expected full unsigned four-byte value')
    if ('feature_78' in root) != (version >= 0x3b10):
        raise ValueError(f'comment version {version}; expected the 0x3B10 owner boundary')


def observe_comment(reference: Path, kind: int, version: int, rich: bool, owners: bool) -> tuple[bytes, dict[str, object]]:
    """Observe each complete helper and primary body; e.g. observe_comment(source, 0, 19000, True, True)."""
    raw, slots = comment_object(kind, version, rich, owners)
    if kind == 0:
        verify_comment(cast(dict[str, object], object_cursor(reference, raw, version, None).object()), version, rich)
    return observe_object_body(reference, raw, slots, kind, version,
        f'comment-{kind}-{version:04x}-{int(rich)}-{int(owners)}')


def freeze_owned(reference: Path, destination: Path, rich: bool, owners: bool, edited: bool) -> dict[str, object]:
    """Use the real blank part's imported map; e.g. freeze_owned(source, output, True, True, False)."""
    container, prefix, features, archive = (source_module(name, reference) for name in
        ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, context = prefix.read(document, 'Contents/Config-0', through_part_resolved=True)
    feature, slots = comment_object(0, 19000, rich, owners, len(context.map)-1, edited)
    raw = struct.pack('<iH', -123, 1)+feature+bytes(4)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(ReplacedFeatureDocument(document, raw), STREAM)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'owned comment end/tail/map; expected {(len(raw), b"", slots)}')
    verify_comment(parsed['items'][0], 19000, rich)
    label = f'owned-{int(rich)}-{int(owners)}'+('-edited' if edited else '')
    return write_observation(destination, label, raw, reads) | dict(slots=slots,
        imported_slots=len(context.map), baseline_sha256=hashlib.sha256(baseline).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind all observed cases and generators to stable readers; e.g. freeze(source, output)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    records = [observe_comment(reference, kind, version, rich, owners)
        for version in VERSIONS for kind in range(3) for rich in (False, True)
        for owners in ((False, True) if kind == 0 else (False,))]
    packet = b'FCM1'+struct.pack('<I', len(records))
    packet += b''.join(struct.pack('<I', row['slots']) for _, row in records)+b''.join(raw for raw, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    owned = [freeze_owned(reference, destination, *options) for options in
        [(False, False, False), (True, False, False), (True, True, False), (True, True, True)]]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    return fixture_report(records, owned, before, packet)


def fixture_report(records: list[tuple[bytes, dict[str, object]]], owned: list[dict[str, object]],
                   hashes: dict[str, str], packet: bytes) -> dict[str, object]:
    """Bind the shipped fixture callers and reused builders; e.g. fixture_report(records, owned, hashes, packet)."""
    scripts = sorted(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[row for _, row in records], owned=owned, source_hashes=hashes,
        packet_sha256=hashlib.sha256(packet).hexdigest(),
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in scripts},
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/feature-comments')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
