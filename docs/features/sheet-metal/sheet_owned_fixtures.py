"""Complete owned sheet-metal streams with independently located CMgr state (#54)."""
from pathlib import Path
from collections.abc import Callable
from typing import cast
import hashlib
import struct
from freeze_blank_part import source_module
from freeze_owned_features import ReferenceDocument, STREAM
from owned_circular_fixtures import write_observation
from primitive_inventory import observe_primitives
from sheet_primitive_projection import dynamic_payload_ranges, project_payload_words
from sheet_fixture_bytes import SheetFixtureOptions, sheet_object

WORKSPACE = Path(__file__).resolve().parents[3]


class SavedSheetMetalDocument:
    def __init__(self, original: ReferenceDocument, manager: bytes, features: bytes) -> None:
        self.original = original
        self.manager = manager
        self.features = features

    def names(self) -> list[str]:
        return self.original.names()

    def read(self, name: str) -> bytes:
        if name == 'Contents/CMgr':
            return self.manager
        return self.features if name == STREAM else self.original.read(name)


def saved_manager(reference: Path, original: ReferenceDocument, state: int) -> tuple[bytes, int]:
    """Patch only the fully decoded state field, never a guessed tail; e.g. saved_manager(source, document, 1)."""
    manager = source_module('configurationmanager', reference)
    source = manager.sheet_metal_context(original)
    position = cast(int, source['position'])
    replacement = bytearray(original.read('Contents/CMgr'))
    struct.pack_into('<i', replacement, position, state)
    return bytes(replacement), position


def owned_sheet_bytes(reference: Path, kind: int, options: SheetFixtureOptions) -> tuple[SavedSheetMetalDocument, bytes, int, int]:
    """Bind frame selection to an independently validated manager; e.g. owned_sheet_bytes(source, 1, options)."""
    container, prefix = (source_module(name, reference) for name in ('swcontainer', 'modelprefix'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, position = saved_manager(reference, original, int(options.extended))
    probe = SavedSheetMetalDocument(original, manager, b'')
    _, context = prefix.read(probe, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    if (context.sheet_metal_helper_850, context.sheet_metal_helper_verified) != (int(options.extended), True):
        raise ValueError(f'saved sheet context {context.sheet_metal_helper_850}; expected verified {int(options.extended)}')
    feature, slots = sheet_object(kind, 19000, options, len(context.map)-1)
    raw = struct.pack('<iH', -123, 1)+feature+(bytes(4) if options.extended else b'')
    return SavedSheetMetalDocument(original, manager, raw), raw, slots, position


def freeze_owned_sheet(reference: Path, destination: Path, kind: int, options: SheetFixtureOptions,
                        verify: Callable[[dict[str, object]], None]) -> dict[str, object]:
    """Observe both real saved-state paths through exact EOF; e.g. freeze_owned_sheet(source, output, 1, options, check)."""
    features, archive = (source_module(name, reference) for name in ('resolvedfeatures', 'swarchive'))
    document, raw, slots, position = owned_sheet_bytes(reference, kind, options)
    with observe_primitives(archive.Archive, raw) as reads, dynamic_payload_ranges(reference, raw) as ranges:
        parsed = features.read_supported(document, STREAM)
        verify(cast(dict[str, object], parsed['items'][0]))
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'owned sheet end/tail/map; expected {(len(raw), b"", slots)}')
    label = f'owned-{kind}-{int(options.extended)}-{int(options.rich)}-{options.allowance}'+('-edited' if options.edited else '')
    (destination/f'manager-{int(options.extended)}.bin').write_bytes(document.manager)
    projected = project_payload_words(raw, reads, ranges)
    return write_observation(destination, label, raw, projected) | dict(slots=slots, imported_slots=97,
        state_position=position, manager_sha256=hashlib.sha256(document.manager).hexdigest(),
        source_reads=sum(read.end > read.start for read in reads), raw_payload_ranges=[list(span) for span in ranges])
