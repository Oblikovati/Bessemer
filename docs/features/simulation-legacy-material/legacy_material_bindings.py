"""Read-only recorded current-reader intervals; no historical body binding is inferred (#175)."""
from pathlib import Path
from typing import cast
from operator import itemgetter
import hashlib
import json
import struct
import pefile
from legacy_material_evidence import RESEARCH, legacy_material_reports, legacy_material_baseline


def verify_legacy_material_historical_capture() -> None:
    """Keep the static historical-helper finding distinct from a body binding; e.g. verify_legacy_material_historical_capture()."""
    folder = Path(__file__).resolve().parent
    capture = json.loads((folder / 'historical-helper-capture.json').read_bytes())
    module = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/Simulation/cosworks.dll')
    raw = module.read_bytes()
    assert hashlib.sha256(raw).hexdigest() == capture['module_sha256'], module
    image = pefile.PE(data=raw, fast_load=True)
    try:
        start, end = int(capture['begin'], 16), int(capture['end'], 16)
        observed = image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)
        assert observed.hex() == capture['raw_hex'] and len(observed) == 404, capture['begin']
        for constant in capture['constants']:
            value = image.get_data(int(constant['address'], 16) - image.OPTIONAL_HEADER.ImageBase, 8)
            assert value.hex() == constant['raw_hex'] and struct.unpack('<d', value)[0] == constant['value'], constant
        listing = (folder / 'historical-helper-objdump.txt').read_bytes()
        assert hashlib.sha256(listing).hexdigest() == capture['output_sha256'], folder
        assert capture['historical_candidate_body_bound'] is False and capture['completed_schema_records'] == 0, capture
    finally:
        image.close()


def legacy_material_function_rows() -> list[dict[str, str]]:
    """Retain every distinct original current-route interval without conflicting pins; e.g. legacy_material_function_rows()."""
    report = legacy_material_reports()[0]
    prior = json.loads((RESEARCH / 'cosworks-material-native-evidence.json').read_bytes())
    rows = cast(list[dict[str, str]], prior['functions'] + report['legacy_bindings'] + report['body_storage_bindings'])
    unique: dict[str, dict[str, str]] = {}
    for row in rows:
        previous = unique.get(row['address'])
        assert previous is None or previous == row, f'{row}; expected the same repeated function pin {previous}'
        unique[row['address']] = row
    return sorted(unique.values(), key=itemgetter('address'))


def verify_legacy_material_call_targets(image: pefile.PE) -> int:
    """Require every recorded direct call to reach the current shared serializer; e.g. verify_legacy_material_call_targets(image)."""
    report = json.loads((RESEARCH / 'cosworks-material-callers.json').read_bytes())
    base = image.OPTIONAL_HEADER.ImageBase
    for row in report['callers']:
        start = int(row['call'], 16)
        raw = image.get_data(start - base, 5)
        assert len(raw) == 5 and raw[0] == 0xe8, f'{start:x}/{raw.hex()}; expected a five-byte direct call'
        target = start + 5 + struct.unpack_from('<i', raw, 1)[0]
        assert target == 0x1816e15a0, f'{start:x}->{target:x}; expected current serializer1816e15a0'
    return len(report['callers'])


def verify_legacy_material_intervals(image: pefile.PE) -> list[dict[str, object]]:
    """Match full bytes of the recorded intervals without widening their boundaries; e.g. verify_legacy_material_intervals(image)."""
    observed: list[dict[str, object]] = []
    for row in legacy_material_function_rows():
        start, end = int(row['address'], 16), int(row['end'], 16)
        raw = image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)
        actual = hashlib.sha256(raw).hexdigest()
        assert len(raw) == end - start and actual == row['raw_sha256'], f'{row["address"]}/{actual}; expected {row["raw_sha256"]}'
        observed.append(dict(address=row['address'], end=row['end'], bytes=len(raw), sha256=actual))
    return observed


def verify_legacy_material_literal_pool(image: pefile.PE) -> int:
    """Bind all nine spelling pointers without assigning wire property IDs; e.g. verify_legacy_material_literal_pool(image)."""
    report = legacy_material_reports()[2]
    matches = cast(dict[str, dict[str, object]], report['literal_matches'])
    base = image.OPTIONAL_HEADER.ImageBase
    for name, binding in matches.items():
        address = int(cast(str, binding['pointer_address']), 16)
        assert address == 0x18254f1b8 + 8 * cast(int, binding['pointer_table_slot']), binding
        pointer = image.get_data(address - base, 8)
        assert pointer.hex() == binding['pointer_raw_hex'], f'{name}/{pointer.hex()}; expected {binding}'
        target = struct.unpack('<Q', pointer)[0]
        assert hex(target) == binding['literal_address'], f'{name}/{target:x}; expected {binding["literal_address"]}'
        literal = image.get_data(target - base, len(name) + 1)
        assert literal.hex() == binding['literal_raw_hex'] and literal == name.encode('ascii') + b'\0', binding
    return len(matches)


def verify_legacy_material_installed() -> None:
    """Verify the entire installed image before resolving known route bytes; e.g. verify_legacy_material_installed()."""
    module = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/Simulation/cosworks.dll')
    raw = module.read_bytes()
    expected = legacy_material_reports()[0]['module_sha256']
    actual = hashlib.sha256(raw).hexdigest()
    assert actual == expected, f'{module}/{actual}; expected {expected}'
    image = pefile.PE(data=raw, fast_load=True)
    try:
        intervals = verify_legacy_material_intervals(image)
        calls = verify_legacy_material_call_targets(image)
        spellings = verify_legacy_material_literal_pool(image)
        literal = 'MaterialList\0'.encode('utf-16le')
        observed = image.get_data(0x182064060 - image.OPTIONAL_HEADER.ImageBase, len(literal))
        assert observed == literal, f'{observed.hex()}; expected complete MaterialList UTF16 literal'
        legacy_material_baseline('installed-current-reader-reference.json', dict(module_sha256=actual, recorded_intervals=intervals,
            current_serializer_direct_calls=calls, material_list_literal_address='0x182064060', corroborated_identifier_spellings=spellings,
            historical_candidate_body_bound=False, target_executed=False, completed_schema_records=0,
            inspection_provider='Existing pefile reader; REA objdump returned ERR_REA_WINDOWS_NATIVE'))
    finally:
        image.close()
