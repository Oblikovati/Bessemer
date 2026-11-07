"""Complete C-881/C-882/C-883 evidence with unresolved runtime state retained (#166)."""
import json
import struct
import pefile
from typing import cast
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline
from dialog_bindings import dialog_images, dialog_export, dialog_import
from dialog_field_bindings import verify_field_leaf

UNIT_REPORTS = ('cosworks-component-unit-type-native-evidence.json',
    'cosworks-actual-freq-disp-flag-native-evidence.json', 'cosworks-actual-freq-registry-evidence.json')


def unit_reports() -> list[dict[str, object]]:
    """Retain all original rows and the explicitly unknown actual flag, without executing audit writers; e.g. unit_reports()."""
    reports = [cast(dict[str, object], json.loads((RESEARCH / name).read_bytes())) for name in UNIT_REPORTS]
    for name, report, evidence in zip(UNIT_REPORTS, reports, ('C-881', 'C-882', 'C-883'), strict=True):
        assert report['evidence'] == evidence and report['target_executed'] is False, evidence
        assert report['coverage_credit'] == 0, evidence
        dialog_baseline('unit-' + name, report)
    assert reports[0]['native_records'] == 224
    assert reports[1]['retained_missing_context_records'] == reports[2]['retained_missing_context_records'] == 56
    assert reports[2]['actual_runtime_flag'] is None
    return reports


def verify_unit_sources() -> None:
    """Require every original source, probe, decompilation and native-report pin unchanged; e.g. verify_unit_sources()."""
    for report in unit_reports():
        for original, expected in cast(dict[str, str], report['sources']).items():
            relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
            assert relative.startswith('solidworks/research/'), original
            actual = digest((SOURCE / relative).read_bytes())
            assert actual == expected, (relative, actual, expected)


def verify_unit_bindings() -> None:
    """Bind the complete numeric provider, wrapper and exact five-argument import/export; e.g. verify_unit_bindings()."""
    report = unit_reports()[0]
    images = dialog_images()
    try:
        provider, caller = images['cwpostcommon.dll'], images['dgxpost.dll']
        assert digest(provider.__data__) == report['module_sha256']
        assert digest(caller.__data__) == report['dgx_sha256']
        method = cast(dict[str, str], report['method'])
        start, end = int(method['address'], 16), int(method['end'], 16)
        assert digest(provider.get_data(start - provider.OPTIONAL_HEADER.ImageBase, end - start)) == method['sha256']
        wrappers = json.loads((RESEARCH / 'cosworks-post-vector-data-probe.json').read_bytes())
        wrapper = next(row for row in wrappers if int(row['address'], 16) == 0x180090610)
        a, b = int(wrapper['address'], 16), int(wrapper['end'], 16)
        assert digest(caller.get_data(a - caller.OPTIONAL_HEADER.ImageBase, b - a)) == report['wrapper_sha256']
        dialog_export(caller, b'?GetUnitsType@CcwPostDlgParam@@', a)
        symbol = b'?dgxPostCommonGetUnitType@@YAHHHHHH@Z'
        dialog_export(provider, symbol, start)
        dialog_import(caller, b'cwpostcommon.dll', symbol, 0x1802cc450)
    finally:
        for image in images.values(): image.close()


def verify_unit_flag_bindings() -> None:
    """Keep exact-one gating distinct from the raw setter, adjacent global and virtual zero-fill initialization; e.g. verify_unit_flag_bindings()."""
    report = unit_reports()[1]
    images = dialog_images()
    try:
        image = images['cwpostcommon.dll']
        assert digest(image.__data__) == report['module_sha256']
        targets: list[int] = []
        for leaf in cast(list[dict[str, str]], report['leaves']):
            verify_field_leaf(image, leaf | {'name': leaf['export']})
            raw = bytes.fromhex(leaf['raw_hex'])
            assert raw[:2] in (b'\x8b\x05', b'\x89\x0d') and raw[-1:] == b'\xc3' and len(raw) == 7
            target = int(leaf['address'], 16) + 6 + struct.unpack_from('<i', raw, 2)[0]
            assert target == int(leaf['global_address'], 16), leaf
            targets.append(target)
        assert targets == [0x180057b68, 0x180057b68, 0x180057b6c], targets
        verify_unit_zero_fill(image, cast(dict[str, object], report['static_global_location']))
    finally:
        for image in images.values(): image.close()


def verify_unit_zero_fill(image: pefile.PE, location: dict[str, object]) -> None:
    """Validate virtual versus file-backed bounds without assigning an application runtime value; e.g. verify_unit_zero_fill(image, location)."""
    rva = int(cast(str, location['global_address']), 16) - image.OPTIONAL_HEADER.ImageBase
    section = image.get_section_by_rva(rva)
    assert section is not None and location['inside_virtual_zero_fill'] is True
    expected = cast(dict[str, object], location['section'])
    assert (section.VirtualAddress, section.Misc_VirtualSize, section.SizeOfRawData) == (
        int(cast(str, expected['rva']), 16), expected['virtual_size'], expected['raw_size'])
    assert section.VirtualAddress + section.SizeOfRawData <= rva
    assert rva + 4 <= section.VirtualAddress + section.Misc_VirtualSize
