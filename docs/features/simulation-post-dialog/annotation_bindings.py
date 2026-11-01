"""Full C-887 consumers, exact UTF16 literals and distinct annotation/material branches (#166)."""
from typing import cast
import re
import pefile
from native_carriers import digest
from dialog_bindings import dialog_images, dialog_export
from style_preferences_disassembly import StylePreferenceInstruction
from annotation_decoder_qualification import annotation_decoded_methods, verify_annotation_qualification_boundaries
from annotation_evidence import annotation_report


def verify_annotation_literals(image: pefile.PE, decoded: dict[int, list[StylePreferenceInstruction]], report: dict[str, object]) -> None:
    """Match every literal byte and actual RIP-relative reference; e.g. verify_annotation_literals(image, decoded, report)."""
    expected = {0x1802d9a98: 'Min', 0x1802d9aa0: 'Max', 0x1802d9ad0: 'Min: ', 0x1802d9ae0: 'Max: '}
    literals = cast(dict[str, dict[str, str]], report['literals'])
    assert set(literals) == {hex(address) for address in expected}
    for address, text in expected.items():
        raw = text.encode('utf-16le') + b'\0\0'
        assert literals[hex(address)] == dict(value=text, raw_hex=raw.hex())
        assert image.get_data(address - image.OPTIONAL_HEADER.ImageBase, len(raw)) == raw
    for reference in cast(list[dict[str, str]], report['literal_references']):
        instruction = next(i for i in decoded[int(reference['function'], 16)] if i.address == int(reference['instruction'], 16))
        assert instruction.mnemonic == 'lea'
        match = re.fullmatch(r'\[rip\+(\d+)\]', instruction.operands[1])
        assert match is not None, instruction
        assert instruction.address + len(instruction.raw) + int(match[1]) == int(reference['literal'], 16)
    assert {reference['literal'] for reference in cast(list[dict[str, str]], report['literal_references'])} == set(literals)


def verify_annotation_role_paths(decoded: dict[int, list[StylePreferenceInstruction]]) -> None:
    """Pin independent Min/Max guards and both glyph calls after the material-only gate; e.g. verify_annotation_role_paths(decoded)."""
    instructions = {instruction.address: (instruction.mnemonic, instruction.operands) for method in decoded.values() for instruction in method}
    expected = {
        0x18010e1c6: ('mov', ('esi', 'dwordptr[rcx+1492]')),
        0x18010e1cc: ('mov', ('edi', 'dwordptr[rcx+1496]')),
        0x18010e1eb: ('call', (str(0x1800fe300),)),
        0x18010e1f0: ('test', ('esi', 'esi')),
        0x18010e1f2: ('je', (str(0x18010e22a),)),
        0x18010e22a: ('test', ('edi', 'edi')),
        0x18010e22c: ('je', (str(0x18010e264),)),
        0x1800849b4: ('test', ('edx', 'edx')),
        0x1800849b6: ('je', (str(0x180084a26),)),
        0x1800849c3: ('je', (str(0x180084a26),)),
        0x1800849c5: ('cmp', ('dwordptr[rax+1492]', 'ebx')),
        0x1800849cd: ('cmp', ('dwordptr[rax+1496]', 'ebx')),
        0x180084a36: ('call', (str(0x180084aa0),)),
        0x180084a4b: ('call', (str(0x180084e80),)),
    }
    assert {address: instructions[address] for address in expected} == expected


def verify_annotation_bindings() -> None:
    """Require complete consumer/helper byte coverage before using the local predicates; e.g. verify_annotation_bindings()."""
    report = annotation_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        verify_annotation_qualification_boundaries()
        decoded = annotation_decoded_methods(image, cast(list[dict[str, object]], report['functions']))
        dialog_export(image, b'?Render@CcwLBGlyphManager@@', 0x180084980)
        verify_annotation_literals(image, decoded, report)
        verify_annotation_role_paths(decoded)
    finally:
        for image in images.values(): image.close()
