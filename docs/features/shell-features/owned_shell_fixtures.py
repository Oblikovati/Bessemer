"""Complete shell streams in the owned model's actual inherited object map (#54)."""
from pathlib import Path
from typing import Protocol, TypedDict, cast
import hashlib
import struct
import sys
from freeze_blank_part import source_module
from primitive_inventory import observe_primitives, verify_primitive_coverage
from shell_fixture_bytes import shell_object, ShellMode

WORKSPACE = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WORKSPACE/'docs/features/resolved-features'))
from freeze_owned_features import ReplacedFeatureDocument, STREAM


class ShellMap(Protocol):
    map: list[object]


class ShellList(TypedDict):
    items: list[object]


class ShellPayload(TypedDict):
    list_820: ShellList | None
    list_828: ShellList | None
    list_830: ShellList | None
    raw_818: int


class ShellRoot(TypedDict):
    payload: ShellPayload


class ShellRead(TypedDict):
    end: int
    tail: bytes
    archive: ShellMap
    items: list[ShellRoot]


def owned_shell_stream(first_slot: int, mode: ShellMode, edited: bool) -> tuple[bytes, int]:
    """Generate an expected flag edit independently; e.g. owned_shell_stream(96, 'populated', True)."""
    raw, slots = shell_object(19000, mode != 'null', mode,
                             flag=7 if edited else None, first_slot=first_slot)
    return struct.pack('<iH', -123, 1)+raw+bytes(4), slots


def verify_owned_shell(parsed: ShellRead, width: int, slots: int, mode: ShellMode, edited: bool) -> None:
    """Require EOF, shared identities and unequal lengths; e.g. verify_owned_shell(parsed, 2620, 113, 'populated', False)."""
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (width, b'', slots):
        raise ValueError(f'shell end/tail/map {(parsed["end"], parsed["tail"], len(parsed["archive"].map))}; expected {(width, b"", slots)}')
    payload = parsed['items'][0]['payload']
    expected_flag = 7 if edited else (0 if mode == 'null' else 255)
    if payload['raw_818'] != expected_flag:
        raise ValueError(f'shell flag {payload["raw_818"]}; expected {expected_flag}')
    if mode in ('shared', 'populated') and not payload['list_820'] is payload['list_828'] is payload['list_830']:
        raise ValueError(f'shell mode {mode!r}; expected three aliases of the same list')
    if mode == 'unequal':
        lengths = [len(entry['items']) if entry is not None else 0
                   for entry in (payload['list_828'], payload['list_830'])]
        if lengths != [3, 1]:
            raise ValueError(f'shell list lengths {lengths}; expected preserved [3, 1]')


def freeze_owned_case(reference: Path, destination: Path, mode: ShellMode, edited: bool) -> dict[str, object]:
    """Read complete streams against actual model context; e.g. freeze_owned_case(source, output, 'unequal', False)."""
    container, features, prefix, archive = (source_module(name, reference) for name in
        ('swcontainer', 'resolvedfeatures', 'modelprefix', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, imported = prefix.read(document, 'Contents/Config-0', through_part_resolved=True)
    context = cast(ShellMap, imported)
    raw, slots = owned_shell_stream(len(context.map)-1, mode, edited)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = cast(ShellRead, features.read_supported(ReplacedFeatureDocument(document, raw), STREAM))
    verify_owned_shell(parsed, len(raw), slots, mode, edited)
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
    """Freeze four graph modes and a separately generated edit; e.g. freeze_owned(source, fixture_root)."""
    return [freeze_owned_case(reference, destination, mode, edited)
            for mode, edited in [('null', False), ('shared', False), ('unequal', False),
                                 ('populated', False), ('populated', True)]]
