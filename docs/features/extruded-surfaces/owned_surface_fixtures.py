"""Complete extruded surfaces with actual inherited context and verified pointer identities (#54)."""
from pathlib import Path
from typing import NotRequired, Protocol, TypedDict, cast
import hashlib
import struct
import sys
from freeze_blank_part import source_module
from primitive_inventory import observe_primitives
from surface_fixture_bytes import surface_object, SurfaceMode

WORKSPACE = Path(__file__).resolve().parents[3]
for folder in ('resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_owned_features import ReplacedFeatureDocument, STREAM
from owned_circular_fixtures import write_observation


class SurfaceMap(Protocol):
    map: list[object]


class KeeperFacts(TypedDict):
    _class: str


class SurfaceBody(TypedDict):
    has_id_keeper: int
    id_keeper: NotRequired[KeeperFacts]


class OrderedFacts(TypedDict):
    array_08: list[object]
    raw_words_48: list[int]
    arrays_28: list[list[object]]
    array_68: list[object]
    raw_words_a8: list[int]
    arrays_88: list[list[object]]


class SurfacePayload(TypedDict):
    end_spec_800: object
    ordered_faces_808: OrderedFacts | None
    raw_810: int
    raw_814: int
    raw_818: int
    array_820: list[object]


class SurfaceRoot(TypedDict):
    body: SurfaceBody
    payload: SurfacePayload


class SurfaceRead(TypedDict):
    end: int
    tail: bytes
    archive: SurfaceMap
    items: list[SurfaceRoot]


def verify_ordered(helper: OrderedFacts | None, mode: SurfaceMode) -> None:
    """Retain unsigned words, unequal arrays, nulls and repeated faces; e.g. verify_ordered(helper, 'populated')."""
    if mode == 'null':
        if helper is not None:
            raise ValueError('null ordered helper; expected absent object')
        return
    if helper is None:
        raise ValueError(f'ordered mode {mode!r}; expected complete helper')
    if mode == 'empty':
        if any((helper['array_08'], helper['raw_words_48'], helper['arrays_28'],
                helper['array_68'], helper['raw_words_a8'], helper['arrays_88'])):
            raise ValueError('empty ordered helper; expected empty direct arrays and groups')
        return
    verify_groups(helper)


def verify_groups(helper: OrderedFacts) -> None:
    """Require words-before-arrays semantics and stable aliases; e.g. verify_groups(helper)."""
    if helper['raw_words_48'] != [0xffffffff, 0x80000000] or helper['raw_words_a8'] != [0xffffff85]:
        raise ValueError(f'ordered words {(helper["raw_words_48"], helper["raw_words_a8"])}; expected unsigned extrema')
    initial, first, second = helper['array_08'], helper['arrays_28'], helper['arrays_88']
    lengths = (len(initial), [len(items) for items in first], len(helper['array_68']), [len(items) for items in second])
    if lengths != (3, [2, 0], 1, [1]):
        raise ValueError(f'ordered array lengths {lengths}; expected {(3, [2, 0], 1, [1])}')
    if initial[0] is not initial[1] or initial[2] is not None or first[0][0] is not initial[0] or second[0][0] is not initial[0]:
        raise ValueError('ordered face identities; expected repeated face and preserved null')
    if first[0][1] is not helper or helper['array_68'][0] is not helper:
        raise ValueError('ordered helper identities; expected cached self references')


def verify_surface(root: SurfaceRoot, mode: SurfaceMode, keeper: bool, edited: bool, header: bool) -> None:
    """Require the inherited keeper and both suffix pointer aliases; e.g. verify_surface(root, 'populated', True, False, True)."""
    payload, body = root['payload'], root['body']
    if (payload['raw_810'], payload['raw_814'], payload['raw_818']) != (7 if edited else -1, -2147483648, 2147483647):
        raise ValueError('surface raw words; expected unchanged signed extrema and selected edit')
    verify_ordered(payload['ordered_faces_808'], mode)
    items = payload['array_820']
    count = (2 if mode == 'null' else 3)+int(keeper)
    if len(items) != count or items[0] is not payload['end_spec_800'] or items[-2 if keeper else -1] is not items[0]:
        raise ValueError(f'surface array length/aliases {len(items)}; expected {count} items sharing the end specification')
    if mode != 'null' and items[1] is not payload['ordered_faces_808']:
        raise ValueError('surface ordered-helper pointer; expected shared tagged identity')
    verify_keepers(body, items, keeper, header)


def verify_keepers(body: SurfaceBody, items: list[object], keeper: bool, header: bool) -> None:
    """Keep distinct zero-byte objects distinct across roles; e.g. verify_keepers(body, items, True, True)."""
    expected_class = 'moExtruRefSurfIdKeeper_c'
    if body['has_id_keeper'] != int(header):
        raise ValueError(f'inherited keeper flag {body["has_id_keeper"]}; expected {int(header)}')
    if header and body.get('id_keeper', {}).get('_class') != expected_class:
        raise ValueError(f'inherited keeper {body.get("id_keeper")}; expected {expected_class}')
    if not header and 'id_keeper' in body:
        raise ValueError('absent inherited keeper; expected no object')
    if keeper:
        last = cast(KeeperFacts, items[-1])
        if last['_class'] != expected_class or (header and last is body['id_keeper']):
            raise ValueError('suffix keeper; expected a distinct complete binary-empty object')


def owned_surface_stream(first_slot: int, mode: SurfaceMode, header: bool, edited: bool) -> tuple[bytes, int]:
    """Construct complete streams and expected edits independently; e.g. owned_surface_stream(96, 'populated', True, True)."""
    raw, slots = surface_object(19000, mode != 'null', mode, mode != 'null', first_slot, edited, header)
    return struct.pack('<iH', -123, 1)+raw+bytes(4), slots


def freeze_owned_case(reference: Path, destination: Path, mode: SurfaceMode, header: bool, edited: bool) -> dict[str, object]:
    """Check actual context, exact EOF and semantic aliases; e.g. freeze_owned_case(source, output, 'populated', True, False)."""
    container, features, prefix, archive = (source_module(name, reference) for name in
        ('swcontainer', 'resolvedfeatures', 'modelprefix', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, imported = prefix.read(document, 'Contents/Config-0', through_part_resolved=True)
    context = cast(SurfaceMap, imported)
    raw, slots = owned_surface_stream(len(context.map)-1, mode, header, edited)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = cast(SurfaceRead, features.read_supported(ReplacedFeatureDocument(document, raw), STREAM))
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'surface end/tail/map {(parsed["end"], parsed["tail"], len(parsed["archive"].map))}; expected {(len(raw), b"", slots)}')
    verify_surface(parsed['items'][0], mode, mode != 'null', edited, header)
    label = 'owned-'+mode+('-header' if header else '')+('-edited' if edited else '')
    return write_observation(destination, label, raw, reads) | dict(slots=slots,
        imported_slots=len(context.map), baseline_sha256=hashlib.sha256(baseline).hexdigest())


def freeze_owned(reference: Path, destination: Path) -> list[dict[str, object]]:
    """Freeze null/empty/populated helpers and both keeper roles; e.g. freeze_owned(source, output)."""
    return [freeze_owned_case(reference, destination, mode, header, edited)
            for mode, header, edited in [('null', False, False), ('empty', False, False),
                ('populated', False, False), ('populated', True, False), ('populated', True, True)]]
