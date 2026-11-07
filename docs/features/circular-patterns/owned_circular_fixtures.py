"""Complete circular streams against the owned model's actual inherited object map (#54)."""
from pathlib import Path
from typing import Protocol, TypedDict, cast
import hashlib
import struct
import sys
from freeze_blank_part import source_module
from primitive_inventory import PrimitiveRead, observe_primitives, verify_primitive_coverage
from circular_fixture_bytes import circular_object

WORKSPACE = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WORKSPACE/'docs/features/resolved-features'))
from freeze_owned_features import ReplacedFeatureDocument, STREAM


class CircularMap(Protocol):
    map: list[object]


class CircularPayload(TypedDict):
    dimension_1130: object
    dimension_1138: object | None
    dimension_1140: object
    dimension_1198: object | None
    dimension_11a0: object | None
    raw_1160: int
    map_11d0: list[tuple[int, list[int]]]
    raw_words_1200: list[int]


class CircularRoot(TypedDict):
    payload: CircularPayload


class CircularRead(TypedDict):
    end: int
    tail: bytes
    archive: CircularMap
    items: list[CircularRoot]


def owned_circular_stream(first_slot: int, rich: bool, alternate: bool, edited: bool) -> tuple[bytes, int]:
    """Construct each expected stream independently; e.g. owned_circular_stream(96, True, True, True)."""
    raw, slots = circular_object(19000, rich, alternate, populated=rich,
        line='line' if rich else 'null', flag=7 if edited else 255, first_slot=first_slot)
    return struct.pack('<iH', -123, 1)+raw+bytes(4), slots


def verify_payload(payload: CircularPayload, rich: bool, alternate: bool, edited: bool) -> None:
    """Preserve dimension aliases and ordered duplicate keys; e.g. verify_payload(payload, True, True, False)."""
    expected_map = [(-1, [-2147483648, -1]), (-1, [123])] if rich else []
    expected_words = [456, -2] if rich else []
    if payload['map_11d0'] != expected_map or payload['raw_words_1200'] != expected_words:
        raise ValueError(f'circular map/vector {(payload["map_11d0"], payload["raw_words_1200"])}; expected {(expected_map, expected_words)}')
    if payload['raw_1160'] != (7 if edited else 255):
        raise ValueError(f'circular flag {payload["raw_1160"]}; expected {7 if edited else 255}')
    if payload['dimension_1130'] is not payload['dimension_1140']:
        raise ValueError('circular dimensions 1130/1140; expected one shared distance dimension')
    if payload['dimension_1198'] is None or payload['dimension_1198'] is not payload['dimension_11a0']:
        raise ValueError('circular dimensions 1198/11a0; expected one shared nonnull angular dimension')
    if alternate and payload['dimension_1138'] is not None:
        raise ValueError('circular alternate dimension 1138; expected null')
    if not alternate and payload['dimension_1138'] is not payload['dimension_1198']:
        raise ValueError('circular primary dimensions 1138/1198; expected shared identity')


def write_observation(destination: Path, label: str, raw: bytes, reads: list[PrimitiveRead]) -> dict[str, object]:
    """Bind each owned stream to its complete primitive oracle; e.g. write_observation(output, 'owned', raw, reads)."""
    reads = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, reads)
    oracle = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in reads).encode()
    (destination/(label+'.bin')).write_bytes(raw)
    (destination/(label+'.tsv')).write_bytes(oracle)
    return dict(label=label, bytes=len(raw), reads=len(reads), sha256=hashlib.sha256(raw).hexdigest(),
                oracle_sha256=hashlib.sha256(oracle).hexdigest())


def freeze_owned_case(reference: Path, destination: Path, rich: bool, alternate: bool, edited: bool) -> dict[str, object]:
    """Require complete framing and the actual inherited map; e.g. freeze_owned_case(source, output, True, True, False)."""
    container, features, prefix, archive = (source_module(name, reference) for name in
        ('swcontainer', 'resolvedfeatures', 'modelprefix', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, imported = prefix.read(document, 'Contents/Config-0', through_part_resolved=True)
    context = cast(CircularMap, imported)
    raw, slots = owned_circular_stream(len(context.map)-1, rich, alternate, edited)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = cast(CircularRead, features.read_supported(ReplacedFeatureDocument(document, raw), STREAM))
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'circular end/tail/map {(parsed["end"], parsed["tail"], len(parsed["archive"].map))}; expected {(len(raw), b"", slots)}')
    verify_payload(parsed['items'][0]['payload'], rich, alternate, edited)
    label = f'owned-{"populated" if rich else "minimal"}-{"alternate" if alternate else "primary"}'+('-edited' if edited else '')
    return write_observation(destination, label, raw, reads) | dict(slots=slots,
        imported_slots=len(context.map), baseline_sha256=hashlib.sha256(baseline).hexdigest())


def freeze_owned(reference: Path, destination: Path) -> list[dict[str, object]]:
    """Freeze both alternatives with empty/populated collections and one edit; e.g. freeze_owned(source, output)."""
    return [freeze_owned_case(reference, destination, rich, alternate, edited)
            for rich, alternate, edited in [(False, False, False), (False, True, False),
                (True, False, False), (True, True, False), (True, True, True)]]
