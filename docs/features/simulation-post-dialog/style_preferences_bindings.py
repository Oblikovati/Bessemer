"""Complete installed C-885 methods, split initialization stores and raw transfer pairs (#166)."""
from typing import cast
from native_carriers import digest
from dialog_bindings import dialog_images
from style_preferences_evidence import style_preferences_report, RAW_STYLE_INITIAL, RAW_STYLE_TRANSFERS
from style_preferences_disassembly import (StylePreferenceInstruction, decoded_style_preference_method,
    style_memory_operand)


def style_initializer_store(instruction: StylePreferenceInstruction, zero_known: bool) -> dict[int, dict[str, object]]:
    """Split only verified RBX-relative immediate/known-zero stores into raw words; e.g. style_initializer_store(instruction, True)."""
    if instruction.mnemonic != 'mov' or len(instruction.operands) != 2: return {}
    destination, source = instruction.operands
    memory = style_memory_operand(destination, 'rbx')
    if memory is None: return {}
    if source.isdecimal(): value = int(source)
    elif source == 'rdi' and zero_known: value = 0
    else: return {}
    displacement, width = memory
    return {displacement + offset: dict(value=(value >> (8 * offset)) & 0xffffffff,
        instruction=hex(instruction.address), store_width=width)
        for offset in range(0, width, 4) if displacement + offset in RAW_STYLE_INITIAL}


def verify_style_initialization(instructions: list[StylePreferenceInstruction], report: dict[str, object]) -> None:
    """Keep the original producing instruction and width for all twelve initial values; e.g. verify_style_initialization(instructions, report)."""
    zero_known = False
    assignments: dict[int, dict[str, object]] = {}
    for instruction in instructions:
        if instruction.mnemonic == 'xor' and instruction.operands == ('edi', 'edi'): zero_known = True
        assignments.update(style_initializer_store(instruction, zero_known))
    assert {member: entry['value'] for member, entry in assignments.items()} == RAW_STYLE_INITIAL
    assert {hex(member): entry for member, entry in assignments.items()} == report['initialization']


def style_transfer_pairs(instructions: list[StylePreferenceInstruction], source_base: str, destination_base: str) -> dict[int, dict[str, object]]:
    """Track exact four-byte EAX loads/stores, invalidating the candidate across calls; e.g. style_transfer_pairs(instructions, 'rdi', 'rbx')."""
    pairs: dict[int, dict[str, object]] = {}
    pending: tuple[int, str] | None = None
    for instruction in instructions:
        if instruction.mnemonic == 'call': pending = None
        if instruction.mnemonic != 'mov' or len(instruction.operands) != 2: continue
        destination, source = instruction.operands
        if destination == 'eax':
            memory = style_memory_operand(source, source_base)
            pending = (memory[0], hex(instruction.address)) if memory is not None and memory[1] == 4 else None
            continue
        memory = style_memory_operand(destination, destination_base)
        if pending is not None and source == 'eax' and memory is not None and memory[1] == 4:
            pairs[memory[0]] = dict(source=pending[0], load=pending[1], store=hex(instruction.address))
    return pairs


def verify_style_transfers(decoded: dict[int, list[StylePreferenceInstruction]], report: dict[str, object]) -> None:
    """Bind each preference load and export to its distinct runtime structure offset; e.g. verify_style_transfers(decoded, report)."""
    loads = style_transfer_pairs(decoded[0x180045050], 'rdi', 'rbx')
    exports = style_transfer_pairs(decoded[0x180044350], 'rbx', 'rdi')
    expected = cast(dict[str, dict[str, object]], report['preference_transfers'])
    for member, (source, destination) in RAW_STYLE_TRANSFERS.items():
        assert loads[member]['source'] == source and exports[destination]['source'] == member
        assert expected[hex(member)] == dict(source_preference=hex(source), destination_plot_adv_preference=hex(destination),
            load=loads[member], export=exports[destination])


def verify_style_copy_assignments(instructions: list[StylePreferenceInstruction]) -> None:
    """Bind the split zero store and both exact supplied stack-word assignments; e.g. verify_style_copy_assignments(instructions)."""
    actual = {instruction.address: (instruction.mnemonic, instruction.operands) for instruction in instructions}
    expected = {
        0x180043fb4: ('xor', ('eax', 'eax')),
        0x180043fb6: ('mov', ('qwordptr[rbx+564]', 'rax')),
        0x180043fc3: ('mov', ('eax', 'dwordptr[rsp+184]')),
        0x180043fca: ('mov', ('ecx', 'dwordptr[rsp+192]')),
        0x180043fd1: ('mov', ('dwordptr[rbx+564]', 'eax')),
        0x180043fd7: ('mov', ('dwordptr[rbx+568]', 'ecx')),
    }
    assert {address: actual[address] for address in expected} == expected


def verify_style_preferences_bindings() -> None:
    """Rematch every complete method before interpreting initialization or transfers; e.g. verify_style_preferences_bindings()."""
    report = style_preferences_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        decoded = {int(cast(str, method['start']), 16): decoded_style_preference_method(image, method)
            for method in cast(list[dict[str, object]], report['functions'])}
        verify_style_initialization(decoded[0x180044580], report)
        verify_style_transfers(decoded, report)
        verify_style_copy_assignments(decoded[0x180043f40])
    finally:
        for image in images.values(): image.close()
