"""C-883 complete recorded blocks and file-backed import bindings; never query the registry (#166)."""
import struct
from typing import cast
import pefile
from native_carriers import digest
from dialog_bindings import dialog_images
from unit_evidence import unit_reports


def descriptor_thunks(image: pefile.PE, lookup: int, first: int, dll: str) -> dict[int, tuple[str, str]]:
    """Decode actual terminated file-backed imports, including ordinal entries; e.g. descriptor_thunks(image, lookup, first, dll)."""
    bindings: dict[int, tuple[str, str]] = {}
    for index in range(10000):
        thunk = struct.unpack('<Q', image.get_data(lookup + index * 8, 8))[0]
        if thunk == 0: return bindings
        label = str(thunk & 0xffff) if thunk >> 63 else image.get_string_at_rva(thunk + 2).decode()
        bindings[image.OPTIONAL_HEADER.ImageBase + first + index * 8] = (dll, label)
    raise AssertionError(f'{dll}/{lookup:#x}; expected a terminated import thunk array within10000 entries')


def complete_import_bindings(image: pefile.PE) -> dict[int, tuple[str, str]]:
    """Bypass the bounded convenience parser that omits C-883 descriptors in this image; e.g. complete_import_bindings(image)."""
    directory = image.OPTIONAL_HEADER.DATA_DIRECTORY[1]
    bindings: dict[int, tuple[str, str]] = {}
    for offset in range(0, directory.Size, 20):
        descriptor = struct.unpack('<5I', image.get_data(directory.VirtualAddress + offset, 20))
        if not any(descriptor): return bindings
        original, _, _, name, first = descriptor
        dll = image.get_string_at_rva(name).decode()
        bindings.update(descriptor_thunks(image, original or first, first, dll))
    raise AssertionError(f'{directory.VirtualAddress:#x}/{directory.Size}; expected a terminated file-backed import directory')


def verify_registry_blocks(image: pefile.PE, report: dict[str, object]) -> None:
    """Require every complete recorded conversion/query block and each actual imported CALL target; e.g. verify_registry_blocks(image, report)."""
    imports = complete_import_bindings(image)
    for block in cast(list[dict[str, object]], report['blocks']):
        start, end = int(cast(str, block['start']), 16), int(cast(str, block['end']), 16)
        raw = image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)
        assert raw.hex() == block['raw_hex'] and digest(raw) == block['sha256'], block['start']
        for call in cast(list[dict[str, object]], block['indirect_calls']):
            address = int(cast(str, call['address']), 16)
            instruction = image.get_data(address - image.OPTIONAL_HEADER.ImageBase, 6)
            assert instruction[:2] == b'\xff\x15', call
            target = address + 6 + struct.unpack_from('<i', instruction, 2)[0]
            assert target == int(cast(str, call['iat']), 16), call
            assert list(imports[target]) == call['import_binding'], call


def verify_unit_registry_bindings() -> None:
    """Retain the exact registry-name/caller dependency without resolving any file's runtime flag; e.g. verify_unit_registry_bindings()."""
    report = unit_reports()[2]
    assert report['actual_runtime_flag'] is None and report['retained_missing_context_records'] == 56
    images = dialog_images()
    try:
        image = images['cosworks.dll']
        assert digest(image.__data__) == report['module_sha256']
        verify_registry_blocks(image, report)
        references = cast(list[dict[str, str]], report['registry_value_references'])
        assert len(references) == 2
        for reference in references:
            assert reference['value'] == 'ActualFrequencyDisplacements', reference
            raw = (reference['value'] + '\0').encode('utf-16le')
            assert image.get_data(int(reference['address'], 16) - image.OPTIONAL_HEADER.ImageBase, len(raw)) == raw
    finally:
        for image in images.values(): image.close()
