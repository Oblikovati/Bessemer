"""Complete mirror streams against the owned model's inherited map (#54)."""
from pathlib import Path
from typing import Protocol, TypedDict, cast
import hashlib
import struct
import sys
from freeze_blank_part import source_module
from primitive_inventory import observe_primitives, verify_primitive_coverage
from mirror_fixture_bytes import mirror_object, PlaneMode

WORKSPACE = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WORKSPACE/'docs/features/resolved-features'))
from freeze_owned_features import ReplacedFeatureDocument, STREAM


class ReferenceMap(Protocol):
    map: list[object]


class PlaneFacts(TypedDict):
    root_point: tuple[float, float, float]
    component_reference: object


class MirrorPayload(TypedDict):
    plane_11a8: PlaneFacts | None
    direct_plane_1138: PlaneFacts


class MirrorRoot(TypedDict):
    payload: MirrorPayload


class MirrorRead(TypedDict):
    end: int
    tail: bytes
    archive: ReferenceMap
    items: list[MirrorRoot]


def owned_mirror_stream(first_slot: int, mode: PlaneMode, edited: bool = False) -> tuple[bytes, int]:
    """Retain imported IDs in a complete stream; e.g. owned_mirror_stream(96, 'shared', True)."""
    raw, slots = mirror_object(19000, mode == 'shared', mode != 'null', mode, first_slot)
    raw = struct.pack('<iH', -123, 1)+raw+bytes(4)
    if not edited:
        return raw, slots
    if mode != 'shared':
        raise ValueError(f'edited plane mode {mode!r}; expected shared with a cached direct component')
    # The direct member has a two-byte cached component; the tagged member embeds its body.
    offset = len(raw)-4-200+94
    if raw[offset:offset+8] != struct.pack('<d', 1.0):
        raise ValueError(f'plane scalar {raw[offset:offset+8].hex()}; expected 1.0 at {offset}')
    return raw[:offset]+struct.pack('<d', 3.5)+raw[offset+8:], slots


def verify_owned_mirror(parsed: MirrorRead, width: int, slots: int, mode: PlaneMode,
                        edited: bool) -> None:
    """Require exact framing and shared-face semantics; e.g. verify_owned_mirror(parsed, 2177, 109, 'shared', False)."""
    if parsed['end'] != width or parsed['tail'] != b'' or len(parsed['archive'].map) != slots:
        raise ValueError(f'mirror end/tail/map {(parsed["end"], parsed["tail"], len(parsed["archive"].map))}; expected {(width, b"", slots)}')
    if mode != 'shared':
        return
    payload = parsed['items'][0]['payload']
    tagged, direct = payload['plane_11a8'], payload['direct_plane_1138']
    if tagged is None:
        raise ValueError('shared tagged plane None; expected a complete tagged plane reference')
    if tagged is direct or tagged['component_reference'] is not direct['component_reference']:
        raise ValueError('mirror plane identities; expected distinct planes sharing one face reference')
    expected = (3.5 if edited else 1.0, 2.0, 3.0)
    if tagged['root_point'] != (1.0, 2.0, 3.0) or direct['root_point'] != expected:
        raise ValueError(f'plane points {(tagged["root_point"], direct["root_point"])}; expected {((1.0, 2.0, 3.0), expected)}')


def freeze_owned_case(reference: Path, destination: Path, mode: PlaneMode, edited: bool) -> dict[str, object]:
    """Bind actual model/CMgr context and an expected edit; e.g. freeze_owned_case(source, output, 'shared', True)."""
    container, features, prefix, archive = (source_module(name, reference) for name in
        ('swcontainer', 'resolvedfeatures', 'modelprefix', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, imported = prefix.read(document, 'Contents/Config-0', through_part_resolved=True)
    context = cast(ReferenceMap, imported)
    raw, slots = owned_mirror_stream(len(context.map)-1, mode, edited)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = cast(MirrorRead, features.read_supported(ReplacedFeatureDocument(document, raw), STREAM))
    verify_owned_mirror(parsed, len(raw), slots, mode, edited)
    reads = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, reads)
    label = 'owned-'+mode+('-edited' if edited else '')
    oracle = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in reads).encode()
    (destination/(label+'.bin')).write_bytes(raw)
    (destination/(label+'.tsv')).write_bytes(oracle)
    return dict(label=label, bytes=len(raw), slots=slots, imported_slots=len(context.map), reads=len(reads),
        sha256=hashlib.sha256(raw).hexdigest(), oracle_sha256=hashlib.sha256(oracle).hexdigest(),
        baseline_sha256=hashlib.sha256(baseline).hexdigest())


def freeze_owned(reference: Path, destination: Path) -> list[dict[str, object]]:
    """Construct three graph variants and one independent edit; e.g. freeze_owned(source, owned_root)."""
    return [freeze_owned_case(reference, destination, mode, edited)
            for mode, edited in [('null', False), ('direct', False), ('shared', False), ('shared', True)]]
