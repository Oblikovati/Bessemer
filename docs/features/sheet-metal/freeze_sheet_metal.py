"""Independent full sheet-metal fixtures with explicit state and raw-array provenance (#54)."""
from functools import partial
from pathlib import Path
from typing import cast
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
REFERENCE = WORKSPACE.parent/'inventor-interface/solidworks'
for folder in ('model-archives', 'reference-axes', 'sweeps', 'resolved-features', 'circular-patterns'):
    sys.path.insert(0, str(WORKSPACE/'docs/features'/folder))
from freeze_blank_part import reference_snapshot
from sheet_fixture_bytes import NAMES, SheetFixtureOptions, sheet_object
from sheet_reference_observer import observe_sheet_object
from sheet_owned_fixtures import freeze_owned_sheet

VERSIONS = (0x27cc, 0x2b8e, 0x2b8f, 0x2bae, 0x2baf, 0x2bb9, 0x2bba, 0x2bc4, 0x2bc5, 0x2f85, 0x2f86, 19000)


def verify_sheet(root: dict[str, object], kind: int, version: int, options: SheetFixtureOptions) -> None:
    """Check state-sensitive grammar independently of the Rust decoder; e.g. verify_sheet(root, 1, 19000, options)."""
    if root['_class'] != NAMES[kind]:
        raise ValueError(f'sheet root {root["_class"]}; expected {NAMES[kind]}')
    if kind == 3:
        if 'feature_id' in root or root['raw_e8'] != -2147483648:
            raise ValueError(f'component sheet root {root}; expected reference-only base and one signed suffix')
        return
    if kind == 2:
        verify_fixed(root, options)
        return
    payload = cast(dict[str, object], root['payload'])
    common = cast(dict[str, object], payload['common_bend_800'])
    if ('sheet_feature_20' in common) != options.extended or ('raw_978' in payload) != options.extended:
        raise ValueError(f'sheet helper {options.extended}; expected both independent helper-selected occurrences')
    if ('raw_97c' in payload) != (options.extended and version >= 0x2bc5):
        raise ValueError(f'sheet version/helper {(version, options.extended)}; expected the independent 0x2BC5 tail gate')
    verify_payload(payload, common, version, options)


def verify_payload(payload: dict[str, object], common: dict[str, object], version: int, options: SheetFixtureOptions) -> None:
    """Pin shared dimensions, raw presence and complete optional bodies; e.g. verify_payload(payload, common, 19000, options)."""
    if common['has_allowance'] != (255 if options.allowance else 0):
        raise ValueError(f'allowance presence {common["has_allowance"]}; expected original raw nonzero byte')
    if options.rich and (payload['dimension_8f8'] is not common['dimension_10'] or payload['dimension_908'] is not common['dimension_10']):
        raise ValueError('sheet dimensions; expected three occurrences of the same complete object')
    if options.allowance:
        verify_allowance(cast(dict[str, object], common['allowance_08']), version, options)
    if options.rich:
        verify_fixed(cast(dict[str, object], payload['fixed_face_910']), options)


def verify_allowance(allowance: dict[str, object], version: int, options: SheetFixtureOptions) -> None:
    """Counts determine table shape regardless of raw stride; e.g. verify_allowance(allowance, 19000, options)."""
    expected = (2, 1, 3) if options.allowance == 2 else ((2, 1, 0) if options.allowance == 3 else ((0, 2, 1) if options.allowance == 4 else (0, 0, 0)))
    if allowance['table_shape'] != expected:
        raise ValueError(f'allowance table shape {allowance["table_shape"]}; expected {expected} from independently framed array counts')
    if ('raw_material_compat' in allowance) != (0x2bae < version < 0x2bba):
        raise ValueError(f'allowance material version {version}; expected the 0x2BAF..0x2BB9 compatibility interval')
    if options.allowance != 2:
        return
    table = cast(list[float], allowance['raw_table'])
    expected_bits = struct.pack('<d', 123.5) if options.edited else struct.pack('<Q', 0x8000000000000000)
    if len(table) != 6 or struct.pack('<d', table[3]) != expected_bits:
        raise ValueError(f'allowance table {table}; expected six preserved values and the independent selected edit')


def verify_fixed(root: dict[str, object], options: SheetFixtureOptions) -> None:
    """Distinguish an ID from an MFC tag without byte probing; e.g. verify_fixed(root, options)."""
    if ('raw_bend_id_10' in root) != options.extended or ('bend_08' in root) == options.extended:
        raise ValueError(f'fixed face fields {root}; expected exactly the explicit helper-selected ID or pointer')
    if options.extended and root['raw_bend_id_10'] != -2147483648:
        raise ValueError(f'fixed bend ID {root["raw_bend_id_10"]}; expected complete signed i32 minimum')


def observe_record(reference: Path, kind: int, version: int, options: SheetFixtureOptions) -> tuple[bytes, dict[str, object]]:
    """Observe a complete tagged graph and the exact state; e.g. observe_record(source, 1, 19000, options)."""
    raw, slots = sheet_object(kind, version, options)
    verify = partial(verify_sheet, kind=kind, version=version, options=options)
    record, facts = observe_sheet_object(reference, raw, slots, kind+4*int(options.extended), version, int(options.extended), verify)
    return record, facts | dict(label=f'sheet-{kind}-{version:04x}-{int(options.extended)}-{int(options.rich)}-{options.allowance}')


def freeze_records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Cover both states and every independent allocation branch; e.g. freeze_records(source)."""
    records = [observe_record(reference, kind, version, SheetFixtureOptions(extended, rich, allowance))
        for version in VERSIONS for kind in (0, 1) for extended in (False, True)
        for rich, allowance in ((False, 0), (False, 1), (True, 2), (True, 3), (False, 4))]
    records += [observe_record(reference, 2, version, SheetFixtureOptions(extended, rich, 0))
        for version in VERSIONS for extended in (False, True) for rich in (False, True)]
    records += [observe_record(reference, 3, version, SheetFixtureOptions(False, rich, 0))
        for version in VERSIONS for rich in (False, True)]
    return records+[observe_record(reference, kind, 19000, SheetFixtureOptions(extended, False, 2))
        for kind in (0, 1) for extended in (False, True)]


def freeze_owned(reference: Path, destination: Path) -> list[dict[str, object]]:
    """Use both actual saved manager states and a separately built edit; e.g. freeze_owned(source, output)."""
    choices = [(0, False, False, 0, False), (0, False, True, 2, False), (0, True, True, 2, False),
               (1, False, True, 2, False), (1, True, True, 2, False), (1, True, True, 3, False), (0, True, True, 2, True)]
    return [freeze_owned_sheet(reference, destination, kind, SheetFixtureOptions(extended, rich, allowance, edited),
        partial(verify_sheet, kind=kind, version=19000, options=SheetFixtureOptions(extended, rich, allowance, edited)))
        for kind, extended, rich, allowance, edited in choices]


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Pin source hashes across complete generation; e.g. freeze(source, output)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    records = freeze_records(reference)
    packet = b'SMT1'+struct.pack('<I', len(records))
    packet += b''.join(struct.pack('<I', row['slots']) for _, row in records)+b''.join(frame for frame, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    owned = freeze_owned(reference, destination)
    invalid_manager = bytearray((destination/'manager-0.bin').read_bytes())
    struct.pack_into('<i', invalid_manager, cast(int, owned[0]['state_position']), 2)
    (destination/'manager-invalid.bin').write_bytes(invalid_manager)
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    return fixture_report(records, owned, before, packet)


def fixture_report(records: list[tuple[bytes, dict[str, object]]], owned: list[dict[str, object]],
                   hashes: dict[str, str], packet: bytes) -> dict[str, object]:
    """Retain both original and lossless projected observations; e.g. fixture_report(records, owned, hashes, packet)."""
    scripts = sorted(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[facts for _, facts in records], owned=owned, source_hashes=hashes,
        packet_sha256=hashlib.sha256(packet).hexdigest(),
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in scripts},
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    report = freeze(REFERENCE, WORKSPACE/'tests/fixtures/sheet-metal')
    (Path(__file__).parent/'reference.json').write_text(json.dumps(report, indent=2)+'\n', encoding='utf-8')
