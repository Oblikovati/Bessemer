"""Five exact converter imm8 sign-extension spelling qualifications; shared rules stay strict (#166)."""
from typing import cast
import pefile

GROUPING_IMMEDIATE_SPELLINGS: tuple[tuple[str, str, str], ...] = (
    ('1800c83ab cmp ebx, -1', '83fbff', '1800c83ab cmp ebx, 0xffffffff'),
    ('1800c844f cmp ebx, -1', '83fbff', '1800c844f cmp ebx, 0xffffffff'),
    ('1800c84a5 cmp eax, -1', '83f8ff', '1800c84a5 cmp eax, 0xffffffff'),
    ('1800c84bc cmp eax, -1', '83f8ff', '1800c84bc cmp eax, 0xffffffff'),
    ('1800c84d4 add ecx, -4', '83c1fc', '1800c84d4 add ecx, 0xfffffffc'),
)


def grouping_converter_gnu_spelling(image: pefile.PE, method: dict[str, object]) -> dict[str, object]:
    """Qualify only five byte-bound imm8/32-bit operand spellings in this exact body; e.g. grouping_converter_gnu_spelling(image, method)."""
    assert (method['start'], method['end']) == ('0x1800c8320', '0x1800c8592')
    lines = list(cast(list[str], method['instructions']))
    for original, raw_hex, gnu in GROUPING_IMMEDIATE_SPELLINGS:
        assert lines.count(original) == 1 and gnu not in lines, (original, gnu)
        address = int(original.split()[0], 16)
        raw = bytes.fromhex(raw_hex)
        assert image.get_data(address - image.OPTIONAL_HEADER.ImageBase, len(raw)) == raw, original
        lines[lines.index(original)] = gnu
    return method | dict(instructions=lines)
