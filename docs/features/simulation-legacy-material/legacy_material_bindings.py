"""Read-only recorded current-reader intervals; no historical body binding is inferred (#175)."""
from pathlib import Path
from typing import cast
from operator import itemgetter
import hashlib
import json
import struct
import sys
import pefile
from legacy_material_evidence import FEATURE, RESEARCH, legacy_material_reports


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


def record_legacy_material_installed(value: dict[str, object]) -> None:
    """Require unchanged current-route observations outside initialization; e.g. record_legacy_material_installed(report)."""
    path = FEATURE / 'installed-current-reader-reference.json'
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected exact unchanged current-reader evidence'


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
        literal = 'MaterialList\0'.encode('utf-16le')
        observed = image.get_data(0x182064060 - image.OPTIONAL_HEADER.ImageBase, len(literal))
        assert observed == literal, f'{observed.hex()}; expected complete MaterialList UTF16 literal'
        record_legacy_material_installed(dict(module_sha256=actual, recorded_intervals=intervals,
            current_serializer_direct_calls=calls, material_list_literal_address='0x182064060',
            historical_candidate_body_bound=False, target_executed=False, completed_schema_records=0,
            inspection_provider='Existing pefile reader; REA objdump returned ERR_REA_WINDOWS_NATIVE'))
    finally:
        image.close()
