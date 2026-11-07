"""Freeze complete helices, branch boundaries and wire-edge identifiers (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
for folder in ('model-archives', 'sweeps', 'sheet-metal', 'resolved-features', 'circular-patterns', 'reference-axes', 'reference-axis-owners', 'radiated-edge-identifiers', 'helices'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import observe_object_body
from primitive_inventory import observe_primitives, verify_primitive_coverage
from sheet_owned_fixtures import SavedSheetMetalDocument, saved_manager
from freeze_owned_features import STREAM
from owned_circular_fixtures import write_observation
from helix_bytes import HelixOptions, THRESHOLDS, VERSIONS, WORDS, double_bits, helix_object, wire_identifier_object

SCENARIOS = (
    HelixOptions(table_presence=1),
    HelixOptions(variable_presence=1, table_presence=1, rows=1),
    HelixOptions(variable_presence=128, array_style=1, table_presence=255, rows=2),
    HelixOptions(double_bits(-2.0), 255, 2, 1, 2, True, True),
    HelixOptions(0x7ff8000000001234, 255, 3, 1, 2, True, True),
    HelixOptions(double_bits(2.0), 128, 2, 2, 2, True, False, True),
    HelixOptions(variable_presence=1, array_style=1, table_presence=1, dimensions=True, wire=True),
    HelixOptions(table_presence=1, rows=2, wire=True),
)


def owned(reference: Path, destination: Path, label: str, options: HelixOptions, alias: bool = False, wire_style: int | None = None, edited: bool = False) -> dict[str, object]:
    """Use the actual inherited document context before assigning new slots; e.g. owned(source, output, 'helix', HelixOptions())."""
    container, prefix, features, archive = (source_module(name, reference) for name in ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
    original = container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    manager, _ = saved_manager(reference, original, 0)
    _, context = prefix.read(SavedSheetMetalDocument(original, manager, b''), 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    first_slot = len(context.map)-1
    feature, slots = helix_object(19000, options, alias, edited, first_slot) if wire_style is None else wire_identifier_object(19000, wire_style, WORDS[0], edited, first_slot)
    raw = struct.pack('<iH', -123, 1)+feature
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(SavedSheetMetalDocument(original, manager, raw), STREAM)
    if (parsed['end'], parsed['tail'], len(parsed['archive'].map)) != (len(raw), b'', slots):
        raise ValueError(f'helix end/tail/map; expected {(len(raw), b"", slots)}')
    verify_primitive_coverage(raw, reads)
    return write_observation(destination, label, raw, [read for read in reads if read.end > read.start]) | dict(slots=slots)


def bare_cases(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Cross strict threshold bits with exact-one table flags; e.g. bare_cases(source)."""
    matrix = [HelixOptions(bits, table_presence=flag, rows=2, dimensions=True) for bits in THRESHOLDS for flag in (0, 1, 2, 255)]
    cases = [observe_object_body(reference, *helix_object(version, options), 0, version, f'threshold/{options.pitch_bits:016x}/{options.table_presence}') for version in VERSIONS for options in matrix]
    cases += [observe_object_body(reference, *helix_object(version, options, alias), 1 if alias else 2, version, f'options/{index}/{int(alias)}')
        for version in VERSIONS for index, options in enumerate(SCENARIOS) for alias in (False, True)]
    cases += [observe_object_body(reference, *wire_identifier_object(version, style, word), 3+style, version, f'wire/{style}/{word}')
        for version in VERSIONS for style in range(3) for word in WORDS]
    return cases


def owned_cases(reference: Path, destination: Path) -> list[dict[str, object]]:
    """Keep independent edit goldens for equal helices and identifiers; e.g. owned_cases(source, output)."""
    options = (HelixOptions(),)+SCENARIOS
    cases = [owned(reference, destination, f'owned-helix-{index}', selected) for index, selected in enumerate(options)]
    cases += [owned(reference, destination, 'owned-helix-alias', SCENARIOS[3], True)]
    cases += [owned(reference, destination, f'owned-wire-{style}', HelixOptions(), wire_style=style) for style in range(3)]
    cases += [owned(reference, destination, 'owned-helix-alias-edited', SCENARIOS[3], True, edited=True)]
    cases += [owned(reference, destination, 'owned-wire-2-edited', HelixOptions(), wire_style=2, edited=True)]
    return cases


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Bind all complete observations to stable approved sources; e.g. freeze(source, fixtures)."""
    before = reference_snapshot(reference);sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    cases = bare_cases(reference)
    packet = b'HLX1'+struct.pack('<I', len(cases))+b''.join(struct.pack('<I', row['slots']) for _, row in cases)+b''.join(frame for frame, _ in cases)
    (destination/'cases.bin').write_bytes(packet)
    saved = owned_cases(reference, destination)
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    return dict(cases=[row for _, row in cases], owned=saved, source_hashes=before, packet_sha256=hashlib.sha256(packet).hexdigest(), reads=sum(row['reads'] for _, row in cases))


if __name__ == '__main__':
    facts = freeze(REFERENCE, WORKSPACE/'tests/fixtures/helices')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts, indent=2)+'\n', encoding='utf-8', newline='\n')
