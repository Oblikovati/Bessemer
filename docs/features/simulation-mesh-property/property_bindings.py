"""Read-only installed Mesh constructors, thresholds and imported reference-container dispatch (#163)."""
from pathlib import Path
from typing import cast
import json
import struct
import pefile
from native_carriers import RESEARCH, digest
from freeze_solid_manager import verify_image
from freeze_body import verify_model_bindings
from property_evidence import property_reports


def property_import_report() -> dict[str, object]:
    """Read the source-pinned exact thunk and import identity; e.g. property_import_report()."""
    return cast(dict[str, object], json.loads((RESEARCH / 'cosworks-mesh-settings-import.json').read_bytes()))


def verify_property_dispatch(image: pefile.PE, report: dict[str, object]) -> None:
    """Bind primary +80 dispatch to the exact native thunk and IAT slot; e.g. verify_property_dispatch(image, report)."""
    imported = property_import_report()
    base = image.OPTIONAL_HEADER.ImageBase
    slot = int(cast(str, report['settings_slot']), 16)
    thunk = int(cast(str, imported['thunk']), 16)
    assert slot == int(cast(str, report['settings_vtable']), 16) + 0x80
    assert struct.unpack('<Q', image.get_data(slot - base, 8))[0] == thunk
    raw = bytes.fromhex(cast(str, imported['raw_hex']))
    assert image.get_data(thunk - base, len(raw)) == raw and raw[:2] == b'\xff\x25'
    target = thunk + 6 + struct.unpack_from('<i', raw, 2)[0]
    bindings = cast(list[dict[str, object]], imported['binding'])
    assert len(bindings) == 1 and target == int(cast(str, bindings[0]['iat']), 16)
    verify_property_import(image, bindings)


def verify_property_import(image: pefile.PE, expected: list[dict[str, object]]) -> None:
    """Require the exact DLL, decorated symbol, ordinal and IAT address; e.g. verify_property_import(image, expected)."""
    image.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_IMPORT']])
    targets = {int(cast(str, row['iat']), 16) for row in expected}
    actual = [dict(module=entry.dll.decode(), symbol=symbol.name.decode() if symbol.name else None,
                   ordinal=symbol.ordinal, iat=hex(symbol.address))
              for entry in image.DIRECTORY_ENTRY_IMPORT for symbol in entry.imports if symbol.address in targets]
    assert actual == expected, f'{actual}; expected exact Mesh settings reference-container import {expected}'


def verify_property_module(image: pefile.PE, raw: bytes) -> None:
    """Reuse complete function/constant checks for both static and final evidence; e.g. verify_property_module(image, raw)."""
    reports = property_reports()
    module_hash = digest(raw)
    assert all(module_hash == report['module_sha256'] for report in reports)
    final = reports[-1]
    dispatch = dict(reader_slot=final['settings_slot'], reader=property_import_report()['thunk'])
    for report in reports:
        functions = report.get('functions', [report.get('function')])
        verify_image(image, dict(report, functions=functions, **dispatch))
    literal = image.get_data(0x181f540a8 - image.OPTIONAL_HEADER.ImageBase, 10)
    assert literal == 'Mesh\0'.encode('utf-16le'), f'{literal.hex()}; expected the complete Mesh storage literal'
    verify_property_dispatch(image, final)


def verify_property_installed_bindings() -> None:
    """Verify installed full-image hashes before reading any bound address; e.g. verify_property_installed_bindings()."""
    report = property_reports()[-1]
    root = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS')
    for filename, key in (('Simulation/cosworks.dll', 'module_sha256'), ('sldmodu.dll', 'model_sha256')):
        raw = (root / filename).read_bytes()
        assert digest(raw) == report[key], f'{filename}; expected the recorded complete installed image hash'
        image = pefile.PE(data=raw, fast_load=True)
        try:
            if key == 'module_sha256': verify_property_module(image, raw)
            else: verify_model_bindings(image, report)
        finally:
            image.close()
