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


def parsed_style_instructions(output: str) -> list[StylePreferenceInstruction]:
    """Keep decoded bytes and addresses beside normalized operands; e.g. parsed_style_instructions(output)."""
    pattern = re.compile(r'^\s*([0-9a-f]+):\s+((?:[0-9a-f]{2}\s+)+)\s*([a-z][a-z0-9]*)\s*(.*)$')
    instructions: list[StylePreferenceInstruction] = []
    for line in output.splitlines():
        match = pattern.match(line)
        if match is None: continue
        address, raw, mnemonic, operands = match.groups()
        if mnemonic == 'rex':
            assert bytes.fromhex(raw)[0] == 0x40, raw
            mnemonic, operands = operands.split(None, 1)
        normalized = canonical_style_assembly(operands)
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


def decoded_style_preference_method(image: pefile.PE, method: dict[str, object]) -> list[StylePreferenceInstruction]:
    """Match complete decoded instructions, contiguous bytes, digest and exact named export; e.g. decoded_style_preference_method(image, method)."""
    start, end = int(cast(str, method['start']), 16), int(cast(str, method['end']), 16)
    raw = image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)
    assert digest(raw) == method['sha256'], method['start']
    instructions = native_style_disassembly(image, start, end)
    actual = [(i.address, i.mnemonic, ','.join(i.operands)) for i in instructions]
    expected = [line.split(' ', 2) for line in cast(list[str], method['instructions'])]
    original = [(int(parts[0], 16), parts[1], canonical_style_assembly(parts[2] if len(parts) == 3 else '')) for parts in expected]
    assert actual == original, [(a, b) for a, b in zip(actual, original) if a != b]
    assert b''.join(instruction.raw for instruction in instructions) == raw, method['start']
    assert instructions[0].address == start and instructions[-1].address + len(instructions[-1].raw) == end
    dialog_export(image, cast(str, method['export']).encode(), start)
    return instructions


def style_memory_operand(operand: str, base: str) -> tuple[int, int] | None:
    """Recognize only the explicitly sized base-relative operands used by this audit; e.g. style_memory_operand('dwordptr[rbx+576]', 'rbx')."""
    match = re.fullmatch(r'(dword|qword)ptr\[' + re.escape(base) + r'(?:\+(\d+))?\]', operand)
    if match is None: return None
    width, displacement = match.groups()
    return int(displacement or '0'), 4 if width == 'dword' else 8
