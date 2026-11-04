"""Use the existing native disassembler without adding a Python package dependency (#166)."""
from dataclasses import dataclass
from pathlib import Path
import os
import re
import subprocess
from typing import cast
import pefile
from native_carriers import digest
from dialog_bindings import dialog_export
from dialog_evidence import dialog_baseline


@dataclass(frozen=True)
class StylePreferenceInstruction:
    address: int
    mnemonic: str
    operands: tuple[str, ...]
    raw: bytes


def canonical_style_assembly(text: str) -> str:
    """Normalize only presentation whitespace and integer spelling across decoders; e.g. canonical_style_assembly('eax, 0x1')."""
    text = text.split('#', 1)[0].split('<', 1)[0].strip().lower()
    text = re.sub(r'0x[0-9a-f]+', lambda match: str(int(match[0], 16)), text)
    return re.sub(r'\s+', '', text)


def canonical_style_operands(mnemonic: str, text: str) -> str:
    """Normalize redundant address spelling only for NOP; e.g. canonical_style_operands('nop', 'dword ptr [rax+0]')."""
    normalized = canonical_style_assembly(text)
    if mnemonic != 'nop': return normalized
    return normalized.replace('+0]', ']').replace('*1+', '+').replace('*1]', ']')


def canonical_style_opcode(mnemonic: str, operands: str, raw: bytes) -> tuple[str, str]:
    """Normalize only byte-checked redundant prefixes and the exact two-byte NOP alias; e.g. canonical_style_opcode('xchg', 'ax,ax', bytes.fromhex('6690'))."""
    if mnemonic == 'rex':
        assert raw[0] == 0x40, raw
        mnemonic, operands = operands.split(None, 1)
    prefixes = 0
    while mnemonic == 'data16':
        assert raw[prefixes] == 0x66, raw
        prefixes += 1
        mnemonic, operands = operands.split(None, 1)
        assert mnemonic in ('data16', 'nop'), (mnemonic, raw)
    if mnemonic == 'xchg' and canonical_style_assembly(operands) == 'ax,ax':
        assert raw == bytes.fromhex('6690'), raw
        return 'nop', ''
    return mnemonic, operands


def parsed_style_instructions(output: str) -> list[StylePreferenceInstruction]:
    """Keep decoded bytes and addresses beside normalized operands; e.g. parsed_style_instructions(output)."""
    pattern = re.compile(r'^\s*([0-9a-f]+):\s+((?:[0-9a-f]{2}\s+)+)\s*([a-z][a-z0-9]*)\s*(.*)$')
    instructions: list[StylePreferenceInstruction] = []
    for line in output.splitlines():
        match = pattern.match(line)
        if match is None: continue
        address, raw, mnemonic, operands = match.groups()
        mnemonic, operands = canonical_style_opcode(mnemonic, operands, bytes.fromhex(raw))
        normalized = canonical_style_operands(mnemonic, operands)
        instructions.append(StylePreferenceInstruction(int(address, 16), mnemonic,
            tuple(normalized.split(',')) if normalized else (), bytes.fromhex(raw)))
    return instructions


def native_style_disassembly(image: pefile.PE, start: int, end: int) -> list[StylePreferenceInstruction]:
    """Run an existing read-only tool against the exact installed method interval; e.g. native_style_disassembly(image, start, end)."""
    executable = Path(os.environ.get('BESSEMER_OBJDUMP', 'C:/msys64/mingw64/bin/objdump.exe'))
    assert executable.is_file(), f'{executable}; expected existing GNU objdump or BESSEMER_OBJDUMP'
    module = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/Simulation/DgxPost.dll')
    assert digest(module.read_bytes()) == digest(image.__data__)
    arguments = [str(executable), '-d', '-w', '-Mintel', '--insn-width=16',
        f'--start-address={start:#x}', f'--stop-address={end:#x}', str(module)]
    output = subprocess.run(arguments, check=True, capture_output=True, text=True, encoding='utf-8').stdout
    dialog_baseline('raw-preferences-disassembler-reference.json', dict(path=str(executable),
        sha256=digest(executable.read_bytes()), adds_project_dependency=False))
    return parsed_style_instructions(output)


def canonical_style_address_operands(mnemonic: str, text: str) -> str:
    """Normalize only exact zero displacement and unit-scale address identities; e.g. canonical_style_address_operands('lea', 'rax,[rbp+0]')."""
    normalized = canonical_style_operands(mnemonic, text)
    return normalized.replace('+0]', ']').replace('*1+', '+').replace('*1]', ']').replace('*1-', '-')


def decoded_style_preference_method(image: pefile.PE, method: dict[str, object], normalize_address_spelling: bool = False) -> list[StylePreferenceInstruction]:
    """Match complete instructions, contiguous bytes, digest and any recorded export; e.g. decoded_style_preference_method(image, method)."""
    start, end = int(cast(str, method['start']), 16), int(cast(str, method['end']), 16)
    raw = image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)
    assert digest(raw) == method['sha256'], method['start']
    instructions = native_style_disassembly(image, start, end)
    normalizer = canonical_style_address_operands if normalize_address_spelling else canonical_style_operands
    actual = [(i.address, i.mnemonic, normalizer(i.mnemonic, ','.join(i.operands))) for i in instructions]
    expected = [line.split(' ', 2) for line in cast(list[str], method['instructions'])]
    original = [(int(parts[0], 16), parts[1], normalizer(parts[1], parts[2] if len(parts) == 3 else '')) for parts in expected]
    assert actual == original, [(a, b) for a, b in zip(actual, original) if a != b]
    assert b''.join(instruction.raw for instruction in instructions) == raw, method['start']
    assert instructions[0].address == start and instructions[-1].address + len(instructions[-1].raw) == end
    if 'export' in method: dialog_export(image, cast(str, method['export']).encode(), start)
    return instructions


def style_memory_operand(operand: str, base: str) -> tuple[int, int] | None:
    """Recognize only the explicitly sized base-relative operands used by this audit; e.g. style_memory_operand('dwordptr[rbx+576]', 'rbx')."""
    match = re.fullmatch(r'(dword|qword)ptr\[' + re.escape(base) + r'(?:\+(\d+))?\]', operand)
    if match is None: return None
    width, displacement = match.groups()
    return int(displacement or '0'), 4 if width == 'dword' else 8
