"""Trace the installed local gate instructions as an independent raw-preference oracle (#166)."""
from typing import cast
import struct
from dialog_bindings import dialog_images
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets
from render_policy_packets import signed_gate_values
from color_comparison_packets import PATTERNS
from style_preferences_evidence import style_preferences_report
from style_preferences_disassembly import StylePreferenceInstruction, decoded_style_preference_method


def style_gate_operand(operand: str, pointed: int | None, registers: dict[str, int]) -> int:
    """Evaluate only the two bound memory sources and operation-local registers; e.g. style_gate_operand('rax', None, registers)."""
    if operand in registers: return registers[operand]
    if operand == 'qwordptr[rbx+720]': return int(pointed is not None)
    assert operand == 'dwordptr[rax+16]' and pointed is not None, (operand, pointed)
    return pointed & 0xffffffff


def style_gate_step(instruction: StylePreferenceInstruction, pointed: int | None, registers: dict[str, int], zero: bool) -> tuple[int, bool]:
    """Interpret only instructions reached between the bound pointer load and local block entries; e.g. style_gate_step(instruction, None, registers, False)."""
    operands = instruction.operands
    following = instruction.address + len(instruction.raw)
    if instruction.mnemonic == 'mov':
        assert operands[0] in ('rax', 'eax'), operands
        registers[operands[0]] = style_gate_operand(operands[1], pointed, registers)
        return following, zero
    if instruction.mnemonic == 'test':
        assert operands == ('rax', 'rax'), operands
        return following, registers['rax'] == 0
    if instruction.mnemonic == 'cmp':
        return following, style_gate_operand(operands[0], pointed, registers) == int(operands[1])
    assert instruction.mnemonic in ('je', 'jne'), instruction
    taken = zero if instruction.mnemonic == 'je' else not zero
    return (int(operands[0]) if taken else following), zero


def trace_style_gate(instructions: list[StylePreferenceInstruction], start: int, terminals: tuple[int, int], pointed: int | None) -> int:
    """Stop at a verified local block entry, rejecting unsupported or looping control flow; e.g. trace_style_gate(instructions, start, terminals, None)."""
    program = {instruction.address: instruction for instruction in instructions}
    cursor, zero = start, False
    registers: dict[str, int] = {}
    for _ in range(16):
        if cursor in terminals: return cursor
        assert cursor in program, (hex(cursor), 'expected a complete bound gate instruction')
        cursor, zero = style_gate_step(program[cursor], pointed, registers, zero)
    raise AssertionError((hex(cursor), 'expected a local terminal within sixteen gate instructions'))


def style_gate_outcomes() -> list[tuple[int | None, bool, bool]]:
    """Derive branch decisions from installed bytes rather than the Rust branch expressions; e.g. style_gate_outcomes()."""
    images = dialog_images()
    try:
        methods = cast(list[dict[str, object]], style_preferences_report()['functions'])
        decoded = {int(cast(str, method['start']), 16): decoded_style_preference_method(images['dgxpost.dll'], method)
            for method in methods if method['start'] in ('0x180045050', '0x180043f40')}
        outcomes: list[tuple[int | None, bool, bool]] = []
        for pointed in [None, *sorted(set(signed_gate_values()) | {69, 70, 71, 72, 73, 74})]:
            setter = trace_style_gate(decoded[0x180045050], 0x180045065, (0x18004507b, 0x1800451b7), pointed)
            copier = trace_style_gate(decoded[0x180043f40], 0x180043f9b, (0x180043fb4, 0x180043fc3), pointed)
            outcomes.append((pointed, setter == 0x18004507b, copier == 0x180043fb4))
        return outcomes
    finally:
        for image in images.values(): image.close()


def raw_style_presence_profiles() -> list[int]:
    """Cover all-present/all-missing and each independently present/missing raw word; e.g. raw_style_presence_profiles()."""
    return sorted({0, 0xfff, *(1 << index for index in range(12)), *(0xfff ^ (1 << index) for index in range(12))})


def raw_style_policy_case(pointed: int | None, enters: bool, clears: bool, mask: int, phase: int) -> bytes:
    """Keep exact raw inputs beside instruction-derived decisions, with no float interpretation; e.g. raw_style_policy_case(None, True, False, 0, 0)."""
    packet = bytearray(struct.pack('<IIII', int(pointed is not None), 0 if pointed is None else pointed & 0xffffffff,
        int(enters), int(clears)))
    patterns = (*PATTERNS, 0xffffffff, 0x7fffffff, 0x55555555, 0xaaaaaaaa)
    for member in range(12):
        present = (mask >> member) & 1
        packet.extend(struct.pack('<II', present, patterns[(phase + member) % len(patterns)] if present else 0))
    arguments = [patterns[(phase + 12 + index) % len(patterns)] for index in range(6)]
    return bytes(packet) + struct.pack('<6I', *arguments)


def verify_style_preferences_policy() -> None:
    """Freeze signed pointer contexts and every optional-field profile against actual gate instructions; e.g. verify_style_preferences_policy()."""
    outcomes = style_gate_outcomes()
    assert (None, True, False) in outcomes
    assert (70, True, True) in outcomes and (73, False, True) in outcomes
    assert sum(not entry[1] for entry in outcomes) == 1 and sum(entry[2] for entry in outcomes) == 2
    profiles = raw_style_presence_profiles()
    cases = [raw_style_policy_case(pointed, enters, clears, mask, context + profile)
        for context, (pointed, enters, clears) in enumerate(outcomes) for profile, mask in enumerate(profiles)]
    packet = struct.pack('<II', 1, len(cases)) + b''.join(cases)
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'raw-preferences-policy.bin': packet})
    dialog_baseline('raw-preferences-policy-reference.json', dict(pointer_contexts=len(outcomes),
        presence_profiles=len(profiles), cases=len(cases), setter_enter_contexts=sum(entry[1] for entry in outcomes),
        copy_zero_contexts=sum(entry[2] for entry in outcomes), instructions_executed='Only bound local gate instructions; no SolidWorks execution',
        additional_stored_byte_credit=0, packets=summaries))
