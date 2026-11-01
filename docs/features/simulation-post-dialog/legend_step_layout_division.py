"""Exact signed32 division certificate for C851's bound IMUL/SAR/correction sequence (#166)."""
from typing import cast
from dialog_evidence import dialog_baseline
from legend_step_bindings import legend_step_methods


def layout_division_certificate() -> dict[str, int]:
    """Bound every signed32 quotient/remainder class without enumerating2^32 inputs; e.g. layout_division_certificate()."""
    multiplier, denominator = 0x66666667, 1 << 35
    excess = multiplier * 20 - denominator
    largest_quotient = (1 << 31) // 20
    largest_remainder_numerator = excess * largest_quotient + multiplier * 19
    assert excess == 12 and 0 < largest_remainder_numerator < denominator
    assert (1 << 31) * multiplier < 1 << 63
    return dict(multiplier=multiplier, denominator=denominator, excess=excess,
        largest_magnitude=1 << 31, largest_quotient=largest_quotient,
        largest_remainder_numerator=largest_remainder_numerator)


def native_layout_divide_twenty(value: int) -> int:
    """Emulate the exact signed high-product, SAR3 and logical sign correction; e.g. native_layout_divide_twenty(-21)."""
    assert type(value) is int and -2147483648 <= value <= 2147483647, (value, 'expected signed32 integer')
    high_product = (value * 0x66666667) // (1 << 32)
    shifted = high_product // 8
    correction = (shifted & 0xffffffff) >> 31
    return shifted + correction


def verify_step_layout_arithmetic_bindings() -> None:
    """Bind the complete quotient sequence and prove its signed32 divide-by20 identity; e.g. verify_step_layout_arithmetic_bindings()."""
    lines = cast(list[str], legend_step_methods()[3]['instructions'])
    first = lines.index('1800de53e mov ecx, dword ptr [rcx + 0x12c]')
    expected = ['1800de53e mov ecx, dword ptr [rcx + 0x12c]', '1800de544 sub ecx, dword ptr [r15 + 0x124]',
        '1800de54b mov eax, 0x66666667', '1800de550 imul ecx', '1800de552 mov edi, edx',
        '1800de554 sar edi, 3', '1800de557 mov eax, edi', '1800de559 shr eax, 0x1f', '1800de55c add edi, eax']
    assert lines[first:first + len(expected)] == expected
    certificate = layout_division_certificate()
    values = list(range(-81, 82)) + [-2147483648, -2147483647, 2147483646, 2147483647]
    seed = 0x85120
    for _ in range(8192):
        seed = (1664525 * seed + 1013904223) % (1 << 32)
        values.append(seed if seed < 1 << 31 else seed - (1 << 32))
    for value in values:
        expected_quotient = (abs(value) // 20) * (-1 if value < 0 else 1)
        assert native_layout_divide_twenty(value) == expected_quotient, value
    dialog_baseline('legend-step-layout-division-reference.json', dict(certificate=certificate,
        regression_cases=len(values), complete_signed32_identity_proved=True, target_executed=False,
        proof='For |n|=20q+r, 0<=r<=19, the excess numerator12q+Mr is below2^35, and positive for nonzero n; floor/negative ceiling plus sign correction yields truncation toward zero.'))
