"""Freeze complete sweep streams using the actual imported document map (#54)."""
from pathlib import Path
from typing import Protocol, TypedDict, cast
import hashlib
import struct
import sys
from freeze_blank_part import source_module
from primitive_inventory import observe_primitives
from sweep_fixture_bytes import SweepOptions, sweep_object

WORKSPACE = Path(__file__).resolve().parents[3]
for folder in ('resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_owned_features import ReplacedFeatureDocument, STREAM
from owned_circular_fixtures import write_observation


class SweepMap(Protocol):
    map: list[object]


class SweepPayload(TypedDict):
    raw_c04: int


class SweepRoot(TypedDict):
    payload: SweepPayload


class SweepRead(TypedDict):
    end: int
    tail: bytes
    archive: SweepMap
    items: list[SweepRoot]


def freeze_owned_case(reference: Path, destination: Path, rich: bool, enabled: bool, edited: bool,
                      cut: bool = False) -> dict[str, object]:
    """Require complete independent decoding and actual context; e.g. freeze_owned_case(source, output, True, True, False)."""
    container, features, prefix, archive = (source_module(name, reference) for name in
        ('swcontainer', 'resolvedfeatures', 'modelprefix', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, imported = prefix.read(document, 'Contents/Config-0', through_part_resolved=True)
    context = cast(SweepMap, imported)
    options = SweepOptions(255, 3, -1, 9, True, True) if enabled else SweepOptions()
    name = 'moSweepCut_c' if cut else 'moSweep_c'
    feature, slots = sweep_object(19000, rich, options, len(context.map)-1, edited, name)
    raw = struct.pack('<iH', -123, 1)+feature+bytes(4)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = cast(SweepRead, features.read_supported(ReplacedFeatureDocument(document, raw), STREAM))
    observed = (parsed['end'], parsed['tail'], len(parsed['archive'].map))
    if observed != (len(raw), b'', slots):
        raise ValueError(f'sweep end/tail/map {observed}; expected {(len(raw), b"", slots)}')
    expected_word = 12345 if edited else -5
    if parsed['items'][0]['payload']['raw_c04'] != expected_word:
        raise ValueError(f'sweep edited word; expected {expected_word}')
    label = f'owned-{"populated" if rich else "minimal"}-{"enabled" if enabled else "disabled"}'+('-edited' if edited else '')
    label += '-cut' if cut else ''
    return write_observation(destination, label, raw, reads) | dict(slots=slots,
        imported_slots=len(context.map), baseline_sha256=hashlib.sha256(baseline).hexdigest())


def freeze_owned(reference: Path, destination: Path) -> list[dict[str, object]]:
    """Cover both collections and option states, plus an independent edit; e.g. freeze_owned(source, output)."""
    return [freeze_owned_case(reference, destination, rich, enabled, edited)
        for rich, enabled, edited in [(False, False, False), (False, True, False),
            (True, False, False), (True, True, False), (True, True, True)]]+[
        freeze_owned_case(reference, destination, rich, rich, False, True) for rich in (False, True)]
