"""Exact C-888 helper guards, concrete file-backed triangle slots and bounded alpha cap (#166)."""
import struct
from typing import cast
import pefile
from native_carriers import digest
from dialog_evidence import dialog_baseline
from dialog_bindings import dialog_images, dialog_export
from annotation_decoder_qualification import qualified_annotation_method
from style_preferences_disassembly import StylePreferenceInstruction, decoded_style_preference_method
from aux_triangle_evidence import aux_triangle_report


def aux_file_bytes(image: pefile.PE, address: int, size: int) -> bytes:
    """Require the complete interval in both virtual and actual raw section bytes; e.g. aux_file_bytes(image, address, 8)."""
    rva = address - image.OPTIONAL_HEADER.ImageBase
    sections = [section for section in image.sections if section.VirtualAddress <= rva
        and rva + size <= section.VirtualAddress + min(section.SizeOfRawData, section.Misc_VirtualSize)]
    assert size > 0 and len(sections) == 1, (hex(address), size)
    section = sections[0]
    offset = section.PointerToRawData + rva - section.VirtualAddress
    raw = bytes(image.__data__[offset:offset + size])
    assert len(raw) == size and image.get_data(rva, size) == raw
    return raw


def verify_aux_triangle_vtables(image: pefile.PE, report: dict[str, object]) -> None:
    """Bind actual stored slot pointers and exact concrete triangle exports; e.g. verify_aux_triangle_vtables(image, report)."""
    slots = cast(list[dict[str, str]], report['triangles_slots'])
    assert len(slots) == 2
    for name, slot in zip(('CDrGL', 'CDrEDW3Plot'), slots, strict=True):
        table, target = int(slot['vtable'], 16), int(slot['target'], 16)
        assert slot['slot'] == '0x58'
        symbol = ('??_7' + name + '@@6B@').encode()
        assert dialog_export(image, symbol, table) == symbol
        assert struct.unpack('<Q', aux_file_bytes(image, table + 0x58, 8))[0] == target
        symbol = slot['export'].encode()
        assert dialog_export(image, symbol, target) == symbol
        aux_file_bytes(image, target, 1)


def aux_triangle_decoded(image: pefile.PE, report: dict[str, object]) -> dict[int, list[StylePreferenceInstruction]]:
    """Retain exactly the two inherited caller qualifier exceptions with complete method coverage; e.g. aux_triangle_decoded(image, report)."""
    decoded: dict[int, list[StylePreferenceInstruction]] = {}
    captures: list[dict[str, str]] = []
    methods = cast(list[dict[str, object]], report['functions'])
    assert {method['start'] for method in methods} == {'0x180100960', '0x1801581c0'}
    for method in methods:
        qualified, records = qualified_aux_method(image, method)
        decoded[int(cast(str, method['start']), 16)] = decoded_style_preference_method(image, qualified, normalize_address_spelling=True)
        captures.extend(records)
    assert {int(record['address'], 16) for record in captures} == {0x18010098b, 0x1801009b7, 0x18015822c}
    dialog_baseline('aux-triangle-decoder-qualifications.json', dict(issue=169, records=captures,
        scope='Two inherited caller qualifiers and one signed32 immediate spelling; complete bytes and other operands retained'))
    return decoded


def qualified_aux_method(image: pefile.PE, method: dict[str, object]) -> tuple[dict[str, object], list[dict[str, str]]]:
    """Keep signed/unsigned immediate notation equivalent only at the captured ADD instruction; e.g. qualified_aux_method(image, method)."""
    qualified, captures = qualified_annotation_method(image, method)
    original = '18015822c add eax, -3'
    canonical = '18015822c add eax, 0xfffffffd'
    lines = cast(list[str], qualified['instructions'])
    if original not in lines: return qualified, captures
    assert image.get_data(0x15822c, 3).hex() == '83c0fd'
    lines = [canonical if line == original else line for line in lines]
    captures.append(dict(address='0x18015822c', original=original, canonical=canonical,
        raw_hex='83c0fd', kind='signed32_immediate_spelling'))
    return qualified | {'instructions': lines}, captures


def verify_aux_caller_instructions(instructions: dict[int, StylePreferenceInstruction]) -> None:
    """Pin the nonzero enable, alpha source, exact-one selector and helper call chain; e.g. verify_aux_caller_instructions(instructions)."""
    expected = {
        0x1801009a3: ('cmp', ('dwordptr[rdi+1444]', '0')),
        0x1801009aa: ('je', (str(0x1801009f0),)),
        0x1801009ac: ('movss', ('xmm0', 'dwordptr[rdi+1464]')),
        0x1801009b7: ('comisd', ('xmm0', 'qwordptr[rip+1940553]')),
        0x1801009bf: ('jb', (str(0x1801009f0),)),
        0x1801009c3: ('lea', ('rdx', '[rdi+1452]')),
        0x1801009ca: ('cmp', ('dwordptr[rdi+1448]', '1')),
        0x1801009d6: ('sete', ('bl',)),
        0x1801009eb: ('call', (str(0x1801581c0),)),
    }
    assert {address: (instructions[address].mnemonic, instructions[address].operands) for address in expected} == expected


def verify_aux_helper_instructions(instructions: dict[int, StylePreferenceInstruction]) -> None:
    """Pin helper eligibility, signed group/count loops, color cap and triangle slots; e.g. verify_aux_helper_instructions(instructions)."""
    expected = {
        0x1801581d8: ('mov', ('rbp', 'qwordptr[rcx+9504]')),
        0x1801581f1: ('cmp', ('dwordptr[rcx+368]', '51')),
        0x180158203: ('movsxd', ('r14', 'dwordptr[rbp+1048]')),
        0x18015820a: ('mov', ('rbp', 'qwordptr[rbp+1056]')),
        0x18015822c: ('add', ('eax', '4294967293')),
        0x180158232: ('jbe', (str(0x180158395),)),
        0x180158270: ('minss', ('xmm0', 'dwordptr[rip+1570932]')),
        0x1801582db: ('jle', (str(0x18015835b),)),
        0x180158312: ('call', ('qwordptr[rax+88]',)),
        0x180158315: ('add', ('rsi', '184')),
        0x18015831c: ('add', ('rbx', '184')),
        0x18015834b: ('call', ('qwordptr[rax+88]',)),
    }
    assert {address: (instructions[address].mnemonic, instructions[address].operands) for address in expected} == expected


def verify_aux_triangle_bindings() -> None:
    """Bind complete caller/helper bytes, scalar constants and actual concrete renderer targets; e.g. verify_aux_triangle_bindings()."""
    report = aux_triangle_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        decoded = aux_triangle_decoded(image, report)
        instructions = {instruction.address: instruction for method in decoded.values() for instruction in method}
        assert aux_file_bytes(image, 0x1802da608, 8).hex() == report['threshold_raw_hex']
        assert aux_file_bytes(image, 0x1802d7aec, 4).hex() == '0000803f'
        verify_aux_triangle_vtables(image, report)
        verify_aux_caller_instructions(instructions)
        verify_aux_helper_instructions(instructions)
    finally:
        for image in images.values(): image.close()
