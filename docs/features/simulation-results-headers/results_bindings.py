"""Read-only full-image, function, threshold, call and virtual-slot checks for Results headers (#164)."""
from pathlib import Path
from typing import cast
import struct
import pefile
from native_carriers import digest
from freeze_shell import verify_shell_image
from results_evidence import results_reports


def verify_results_calls(image: pefile.PE) -> None:
    """Bind the two documented category/plot constructors at their exact caller instructions; e.g. verify_results_calls(image)."""
    base = image.OPTIONAL_HEADER.ImageBase
    for address, constructor in ((0x18118bec6, 0x1809dea60), (0x18118c01b, 0x1809dedc0)):
        raw = image.get_data(address - base, 5)
        assert raw[0] == 0xe8, f'{address:x}/{raw.hex()}; expected the pinned direct constructor call'
        target = address + 5 + struct.unpack_from('<i', raw, 1)[0]
        assert target == constructor, f'{target:x}; expected constructor {constructor:x}'
    gate = image.get_data(0x1814ab220 - base, 7)
    assert gate == bytes.fromhex('4181fe53aa0c00'), f'{gate.hex()}; expected the pinned plot compare with inclusive 830035'


def verify_results_installed_bindings() -> None:
    """Hash the complete installed image before verifying all three hierarchy readers; e.g. verify_results_installed_bindings()."""
    reports = results_reports()
    raw = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/Simulation/cosworks.dll').read_bytes()
    actual = digest(raw)
    assert all(actual == report['module_sha256'] for report in reports), f'{actual}; expected original Results image hash'
    image = pefile.PE(data=raw, fast_load=True)
    try:
        for report in reports: verify_shell_image(image, report)
        root = reports[0]
        slot, table = int(cast(str, root['reader_slot']), 16), int(cast(str, root['vtable']), 16)
        assert slot == table + 0x48, f'{slot:x}/{table:x}; expected root primary +48 slot'
        pointer = struct.unpack('<Q', image.get_data(slot - image.OPTIONAL_HEADER.ImageBase, 8))[0]
        assert pointer == int(cast(str, root['reader']), 16), f'{pointer:x}; expected the recorded root serializer'
        verify_results_calls(image)
    finally:
        image.close()
