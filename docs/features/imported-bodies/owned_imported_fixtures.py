"""Complete imported/base-body streams in the owned inherited map, including a label edit (#54)."""
from pathlib import Path
from typing import NotRequired, Protocol, TypedDict, cast
import hashlib
import struct
import sys
from freeze_blank_part import source_module
from primitive_inventory import observe_primitives
from imported_fixture_bytes import imported_object, ImportedClass, BodyMode

WORKSPACE = Path(__file__).resolve().parents[3]
for folder in ('resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_owned_features import ReplacedFeatureDocument, STREAM
from owned_circular_fixtures import write_observation


class ImportedMap(Protocol):
    map: list[object]


class BodyFacts(TypedDict):
    suppressed: int
    serialized_body: NotRequired[bool]
    format: NotRequired[str]


class ImportedFacts(TypedDict):
    file_name_800: str
    external_body_marker: int
    body_name_808: str
    body_810: NotRequired[BodyFacts]


class BaseFacts(TypedDict):
    raw_words_850: list[int]


class ImportedRoot(TypedDict):
    imported: ImportedFacts
    base_body: NotRequired[BaseFacts]


class ImportedRead(TypedDict):
    end: int
    tail: bytes
    archive: ImportedMap
    items: list[ImportedRoot]


def owned_imported_stream(first_slot: int, name: ImportedClass, rich: bool,
                           mode: BodyMode, edited: bool) -> tuple[bytes, int]:
    """Keep actual imported IDs and construct edits independently; e.g. owned_imported_stream(96, 'moBaseBody_c', True, 'compressed', True)."""
    raw, slots = imported_object(name, 19000, rich, mode, first_slot, edited)
    return struct.pack('<iH', -123, 1)+raw+bytes(4), slots


def verify_imported(payload: ImportedFacts, rich: bool, mode: BodyMode, edited: bool) -> None:
    """Distinguish missing, suppressed and fully serialized bodies; e.g. verify_imported(payload, True, 'compressed', False)."""
    expected = ('cad/Δ.x_t' if rich else '', 'Bessemer Body Ω' if edited else ('Body Ω' if rich else ''))
    actual = (payload['file_name_800'], payload['body_name_808'])
    if actual != expected:
        raise ValueError(f'imported strings {actual}; expected {expected}')
    marker = (-2147483648 if rich else 1) if mode == 'external' else 0
    if payload['external_body_marker'] != marker:
        raise ValueError(f'external marker {payload["external_body_marker"]}; expected {marker}')
    body = payload.get('body_810')
    if mode == 'external':
        if 'body_810' in payload:
            raise ValueError('external imported body; expected absent inline body')
        return
    if body is None or body['suppressed'] != (255 if mode == 'suppressed' else 0):
        raise ValueError(f'inline body {body}; expected {mode} framing')
    if mode == 'compressed' and (not body.get('serialized_body') or body.get('format') != 'framed_compressed_parasolid'):
        raise ValueError(f'inline format {body.get("format")}; expected complete framed_compressed_parasolid')


def verify_base(root: ImportedRoot, name: ImportedClass, rich: bool) -> None:
    """Keep vector presence tied to the concrete class; e.g. verify_base(root, 'moImported_c', False)."""
    if name == 'moImported_c':
        if 'base_body' in root:
            raise ValueError('imported class base suffix; expected absent vector layer')
        return
    suffix = root.get('base_body')
    expected = [-2147483648, 0, 2147483647] if rich else []
    if suffix is None or suffix['raw_words_850'] != expected:
        raise ValueError(f'base-body suffix {suffix}; expected signed vector {expected}')


def owned_label(name: ImportedClass, rich: bool, mode: BodyMode, edited: bool) -> str:
    """Give each persisted graph a stable fixture name; e.g. owned_label('moBaseBody_c', True, 'compressed', True)."""
    family = 'base' if name == 'moBaseBody_c' else 'imported'
    return f'owned-{family}-{"populated" if rich else "minimal"}-{mode}'+('-edited' if edited else '')


def freeze_owned_case(reference: Path, destination: Path, name: ImportedClass,
                      rich: bool, mode: BodyMode, edited: bool) -> dict[str, object]:
    """Verify complete framing in actual model/CMgr context; e.g. freeze_owned_case(source, output, 'moBaseBody_c', True, 'external', False)."""
    container, features, prefix, archive = (source_module(module, reference) for module in
        ('swcontainer', 'resolvedfeatures', 'modelprefix', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, imported = prefix.read(document, 'Contents/Config-0', through_part_resolved=True)
    context = cast(ImportedMap, imported)
    raw, slots = owned_imported_stream(len(context.map)-1, name, rich, mode, edited)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = cast(ImportedRead, features.read_supported(ReplacedFeatureDocument(document, raw), STREAM))
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'imported end/tail/map {(parsed["end"], parsed["tail"], len(parsed["archive"].map))}; expected {(len(raw), b"", slots)}')
    root = parsed['items'][0]
    verify_imported(root['imported'], rich, mode, edited)
    verify_base(root, name, rich)
    return write_observation(destination, owned_label(name, rich, mode, edited), raw, reads) | dict(
        slots=slots, imported_slots=len(context.map), baseline_sha256=hashlib.sha256(baseline).hexdigest())


def freeze_owned(reference: Path, destination: Path) -> list[dict[str, object]]:
    """Freeze both classes and all body modes with one independent edit; e.g. freeze_owned(source, output)."""
    cases: list[tuple[ImportedClass, bool, BodyMode, bool]] = [
        (name, True, mode, False) for name in ('moImported_c', 'moBaseBody_c')
        for mode in ('external', 'suppressed', 'compressed')]
    cases += [('moImported_c', False, 'external', False), ('moBaseBody_c', False, 'external', False),
              ('moBaseBody_c', True, 'compressed', True)]
    return [freeze_owned_case(reference, destination, *case) for case in cases]
