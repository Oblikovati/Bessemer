"""Installed C-886 complete wrappers/targets, object links, field writes and color-copy imports (#166)."""
from typing import cast
import struct
import pefile
from native_carriers import digest
from dialog_bindings import dialog_images, dialog_export, dialog_import
from style_preferences_disassembly import StylePreferenceInstruction, decoded_style_preference_method
from vector_style_evidence import vector_style_report


def verify_vector_style_instructions(decoded: dict[int, list[StylePreferenceInstruction]]) -> None:
    """Bind both object-base links and exact argument-preserving tail/store instructions; e.g. verify_vector_style_instructions(decoded)."""
    instructions = {instruction.address: instruction for method in decoded.values() for instruction in method}
    expected = {
        0x1800c8096: ('jmp', (str(0x18015d740),)),
        0x1800c80d6: ('jmp', (str(0x18015d7a0),)),
        0x1800c8102: ('jmp', (str(0x18015d8c0),)),
        0x18015d811: ('lea', ('rcx', '[rbx+352]')),
        0x18015d818: ('call', (str(0x180090fd0),)),
        0x1800957df: ('lea', ('rcx', '[r15+576]')),
        0x18015d755: ('mov', ('dwordptr[rcx+1592]', 'edx')),
        0x18015d776: ('lea', ('rcx', '[rbx+1596]')),
        0x18015d7b6: ('mov', ('dwordptr[rcx+1584]', 'edx')),
        0x18015d7bf: ('mov', ('dwordptr[rcx+1588]', 'r8d')),
    }
    actual = {address: (instructions[address].mnemonic, instructions[address].operands) for address in expected}
    assert actual == expected, (actual, expected)


def verify_vector_style_imports(image: pefile.PE, report: dict[str, object]) -> None:
    """Keep packed construction distinct from whole COGLColor copy assignment; e.g. verify_vector_style_imports(image, report)."""
    bindings = cast(list[dict[str, str]], report['color_import_bindings'])
    expected = ('??0COGLColor@@QEAA@AEBK@Z', '??4COGLColor@@QEAAAEAV0@AEBV0@@Z')
    assert tuple(binding['symbol'] for binding in bindings) == expected
    for binding in bindings:
        address, iat = int(binding['address'], 16), int(binding['iat'], 16)
        raw = image.get_data(address - image.OPTIONAL_HEADER.ImageBase, 6)
        assert raw[:2] == b'\xff\x15' and address + 6 + struct.unpack_from('<i', raw, 2)[0] == iat
        dialog_import(image, b'dspogl.dll', binding['symbol'].encode(), iat)


def verify_vector_style_bindings() -> None:
    """Require every complete chain function before validating exact stores, bases and color imports; e.g. verify_vector_style_bindings()."""
    report = vector_style_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        decoded = {int(cast(str, method['start']), 16): decoded_style_preference_method(image, method)
            for method in cast(list[dict[str, object]], report['functions'])}
        verify_vector_style_instructions(decoded)
        for address, name in ((0x1800c8060, 'ChangeVectorColor'), (0x1800c80a0, 'ChangeVectorDensity'),
                              (0x1800c80e0, 'ChangeVectorSize'), (0x180090fd0, 'IsContactPressure')):
            dialog_export(image, ('?' + name + '@').encode(), address)
        leaf = image.get_data(0x15d8c0, 12)
        assert leaf.hex() == report['size_leaf_raw_hex'] == '89912c060000b801000000c3'
        verify_vector_style_imports(image, report)
    finally:
        for image in images.values(): image.close()
