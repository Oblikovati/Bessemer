"""Installed shell/midsurface dispatch and scoped delegate bindings without loading code (#159)."""
from pathlib import Path
from typing import cast
import struct
import pefile
from native_carriers import digest
from freeze_solid_manager import verify_image
from freeze_body import verify_model_bindings
from shell_property_evidence import shell_reports


def verify_refined_shell_image(image: pefile.PE, report: dict[str, object]) -> None:
    """Bind delegate functions, virtual +50 targets and exact layer lookup bytes; e.g. verify_refined_shell_image(image, report)."""
    base = image.OPTIONAL_HEADER.ImageBase
    for function in cast(list[dict[str, str]], report['functions']):
        start, end = int(function['address'], 16), int(function['end'], 16)
        assert digest(image.get_data(start - base, end - start)) == function['raw_sha256']
    for binding in cast(list[dict[str, str]], report['virtual_bindings']):
        location = int(binding['vtable'], 16) + int(binding['slot'], 16) - base
        raw = image.get_data(location, 8)
        assert raw.hex() == binding['raw_hex']
        assert struct.unpack('<Q', raw)[0] == int(binding['target'], 16)
    lookup = cast(dict[str, str], report['layer_lookup'])
    expected = bytes.fromhex(lookup['raw_hex'])
    assert image.get_data(int(lookup['address'], 16) - base, len(expected)) == expected
    assert digest(expected) == lookup['sha256']
    assert struct.unpack('<d', image.get_data(0x181f61f88 - base, 8))[0] == 820011


def verify_shell_bindings() -> None:
    """Inspect both concrete dispatches and the exact scoped ID callback; e.g. verify_shell_bindings()."""
    reports = shell_reports()
    root = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS')
    for filename, key in (('Simulation/cosworks.dll', 'module_sha256'), ('sldmodu.dll', 'model_sha256')):
        raw = (root / filename).read_bytes()
        image = pefile.PE(data=raw, fast_load=True)
        try:
            for report in reports[:2]:
                assert digest(raw) == report[key]
                if key == 'module_sha256': verify_image(image, report)
                else: verify_model_bindings(image, report)
            if key == 'module_sha256':
                assert digest(raw) == reports[-1][key]
                verify_refined_shell_image(image, reports[-1])
        finally:
            image.close()
