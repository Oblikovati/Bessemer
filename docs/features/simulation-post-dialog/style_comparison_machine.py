"""Restricted interpreter for the exact validated107-instruction C863 body; no native execution (#166)."""
from dataclasses import dataclass, field
from typing import cast
import importlib
import re
import struct
from style_comparison_inputs import StyleFrame, STYLE_INPUT_PATHS, STYLE_MEMORY_PATHS, STYLE_COLOR_OFFSETS

StyleInstruction = tuple[str, tuple[str, ...], int]
StyleProgram = dict[int, StyleInstruction]
StyleRegister = int | tuple[int, int]
PAIR_WORD_CHECKS = dict(zip((0x180044040, 0x1800440b4, 0x1800440df, 0x180044124,
    0x180044136, 0x180044148, 0x18004415a, 0x180044188, 0x18004419a, 0x1800441c0, 0x1800441d2), range(11), strict=True))
COLOR_CALLS = {0x18004405f: 0, 0x180044084: 0, 0x1800440a0: 1, 0x1800440fd: 2,
    0x180044174: 3, 0x1800441b0: 4, 0x1800441e8: 5}


@dataclass(frozen=True)
class StyleOracleFailure(Exception):
    status: int
    member: int
    side: int


def comparison_native_program() -> StyleProgram:
    """Use the complete original probe whose entire body is checked against GNU and Ghidra; e.g. comparison_native_program()."""
    from style_comparison_bindings import style_comparison_methods
    lines = cast(list[str], style_comparison_methods()[0]['instructions'])
    addresses = [int(line.split()[0], 16) for line in lines]
    program: StyleProgram = {}
    for index, line in enumerate(lines):
        parts = line.split(' ', 2); mnemonic = parts[1]
        operands = tuple(part.strip().replace(' ', '') for part in parts[2].split(',')) if len(parts) == 3 and parts[2] else ()
        program[addresses[index]] = (mnemonic, operands, addresses[index + 1] if index + 1 < len(lines) else 0)
    return program


@dataclass
class StyleComparisonMachine:
    frames: tuple[StyleFrame, StyleFrame]
    program: StyleProgram
    registers: dict[str, StyleRegister] = field(default_factory=lambda: {'rcx': (0, 0), 'rdx': (1, 0)})
    checks: list[int] = field(default_factory=list)
    zero: bool = False
    visited: set[int] = field(default_factory=set)

    def pointer(self, operand: str) -> tuple[int, int]:
        """Resolve only the four verified object-base pointer registers and offsets; e.g. machine.pointer('[rbx+0x1bc]')."""
        match = re.fullmatch(r'\[(r(?:cx|dx|bx|di))(?:\+(0x[0-9a-f]+|\d+))?\]', operand)
        assert match is not None, operand
        pointer = self.registers[match[1]]; assert isinstance(pointer, tuple), (operand, pointer)
        return pointer[0], pointer[1] + int(match[2] or '0', 0)

    def operand(self, operand: str) -> StyleRegister:
        """Read actual optional patterns or exact integer/register values; e.g. machine.operand('dwordptr[rbx+0x1b8]')."""
        if operand in self.registers: return self.registers[operand]
        if not operand.startswith('dwordptr['): return int(operand, 0)
        side, offset = self.pointer(operand.removeprefix('dwordptr'))
        path = STYLE_MEMORY_PATHS[offset]; raw = self.frames[side].get(path)
        if raw is None: raise StyleOracleFailure(2, STYLE_INPUT_PATHS.index(path), side)
        assert len(raw) == 4, (path, raw)
        return int.from_bytes(raw, 'little')

    def execute(self) -> int:
        """Stop at the exact zero/difference result blocks, rejecting unsupported flow; e.g. machine.execute()."""
        cursor = 0x180044034
        for _ in range(128):
            if cursor in (0x180044203, 0x180044207): return int(cursor == 0x180044207)
            self.visited.add(cursor)
            mnemonic, operands, following = self.program[cursor]
            cursor = self.step(cursor, mnemonic, operands, following)
        raise AssertionError((hex(cursor), 'expected an original result block within128 steps'))

    def step(self, address: int, mnemonic: str, operands: tuple[str, ...], following: int) -> int:
        """Interpret only instructions reached in this exact body; e.g. machine.step(address, opcode, operands, following)."""
        if mnemonic in ('je', 'jne'):
            taken = self.zero if mnemonic == 'je' else not self.zero
            return int(operands[0], 0) if taken else following
        if mnemonic in ('mov', 'movss'): self.registers[operands[0]] = self.operand(operands[1])
        elif mnemonic == 'lea': self.registers[operands[0]] = self.pointer(operands[1])
        elif mnemonic == 'add': self.add_pointer(operands)
        elif mnemonic in ('cmp', 'test', 'ucomiss'): self.compare(address, mnemonic, operands)
        elif mnemonic == 'call': self.call(address, operands[0])
        else: raise AssertionError((hex(address), mnemonic, 'expected a bound C863 semantic instruction'))
        return following

    def add_pointer(self, operands: tuple[str, ...]) -> None:
        """Apply only the verified object-member address additions; e.g. machine.add_pointer(('rdx','0x1bc'))."""
        original = self.registers[operands[0]]; assert isinstance(original, tuple), operands
        self.registers[operands[0]] = (original[0], original[1] + int(operands[1], 0))

    def compare(self, address: int, mnemonic: str, operands: tuple[str, ...]) -> None:
        """Compute integer/SSE zero flags without host floating arithmetic; e.g. machine.compare(address, 'cmp', operands)."""
        left, right = self.operand(operands[0]), self.operand(operands[1])
        assert isinstance(left, int) and isinstance(right, int), (address, operands)
        self.zero = (left & right) == 0 if mnemonic == 'test' else left == right
        if mnemonic == 'ucomiss': self.zero = scalar_source_equal(left, right)
        if address in PAIR_WORD_CHECKS: self.checks.append(PAIR_WORD_CHECKS[address])
        if address in (0x1800440cc, 0x1800441fa): self.checks.append(11 if address == 0x1800440cc else 12)

    def call(self, address: int, target: str) -> None:
        """Delegate only the exact legend target or seven bound color call sites; e.g. machine.call(address, target)."""
        if address == 0x180044111:
            assert target == '0x1800d9950'
            assert self.registers['rcx'] == (0, 0) and self.registers['rdx'] == (1, 0)
            result = legend_source_different(*self.frames); self.checks.append(19)
        else:
            color = COLOR_CALLS[address]; offset = STYLE_COLOR_OFFSETS[color]
            assert self.registers['rcx'] == (0, offset) and self.registers['rdx'] == (1, offset)
            result = color_source_different(self.frames, color); self.checks.append(13 + color)
        self.registers['eax'] = result
        for name in ('rcx', 'rdx'): self.registers.pop(name, None)


def scalar_source_equal(left: int, right: int) -> bool:
    """Reuse the original independent C864 SSE policy for one scalar; e.g. scalar_source_equal(0, 0x80000000)."""
    equal = importlib.import_module('cosworks_color_comparison').equal
    return bool(equal(struct.pack('<4I', left, 0, 0, 0), struct.pack('<4I', right, 0, 0, 0), environment='masked_exceptions_daz_off'))


def color_source_different(frames: tuple[StyleFrame, StyleFrame], color: int) -> int:
    """Respect the bound right-first RGBA reads and early JNE before later unknown channels; e.g. color_source_different(frames, 0)."""
    for channel in range(4):
        path = f'style.color_{STYLE_COLOR_OFFSETS[color]:x}.component_{channel}'
        values: list[int] = []
        for side in (1, 0):
            raw = frames[side].get(path)
            if raw is None: raise StyleOracleFailure(2, STYLE_INPUT_PATHS.index(path), side)
            assert len(raw) == 4, (path, raw); values.append(int.from_bytes(raw, 'little'))
        if not scalar_source_equal(*values): return 1
    return 0


def legend_source_different(left: StyleFrame, right: StyleFrame) -> int:
    """Compose the existing original C871 model without filling absent loaded legend facts; e.g. legend_source_different(left, right)."""
    try:
        return int(importlib.import_module('cosworks_legend_comparison').different(left, right, environment='masked_exceptions_daz_off'))
    except ValueError as error:
        message = str(error)
        assert 'explicit loaded' in message or 'palette count0..9' in message, message
        raise StyleOracleFailure(3 if 'palette count0..9' in message else 2, 39, 2) from error


def style_comparison_expected(left: StyleFrame, right: StyleFrame, program: StyleProgram, coverage: set[int] | None = None) -> tuple[int, ...]:
    """Return exact completion/first-difference/error identity and up to20 ordered checks; e.g. style_comparison_expected(left, right, program)."""
    machine = StyleComparisonMachine((left, right), program)
    try:
        outcome = machine.execute(); checks = machine.checks
        assert len(checks) <= 20 and len(set(checks)) == len(checks), checks
        first = checks[-1] if outcome else 0xffffffff
        return (outcome, first, 0xffffffff, 0xffffffff, len(checks), *checks, *([0xffffffff] * (20 - len(checks))))
    except StyleOracleFailure as error:
        return (error.status, 0xffffffff, error.member, error.side, 0, *([0xffffffff] * 20))
    finally:
        if coverage is not None: coverage.update(machine.visited)
