"""Exact C852 allocation-register and local precision-factor relationships (#166)."""
from typing import cast
from dialog_evidence import dialog_baseline
from legend_3d_bindings import legend_3d_methods, ordered_3d_anchors


def verify_3d_integer_request_bindings() -> None:
    """Bind both saturated allocation sequences and the by-value rendering argument reads; e.g. verify_3d_integer_request_bindings()."""
    methods = legend_3d_methods()
    body = cast(list[str], methods[1]['instructions'])
    ordered_3d_anchors(methods[1], ['1800e1f68 mov rdi, 0xffffffffffffffff',
        '1800e1fa4 mov eax, dword ptr [rcx + 0x14]', '1800e1fa7 inc eax', '1800e1fb2 lea eax, [rax + rax*2]',
        '1800e1fb5 add eax, eax', '1800e1fb7 movsxd rcx, eax', '1800e1fba mov eax, 4',
        '1800e1fbf mul rcx', '1800e1fc2 cmovb rax, rdi', '1800e1fc9 call 0x1802a1dd6',
        '1800e2061 lea ecx, [r14 + 1]', '1800e2065 lea edx, [rcx + rcx*2]', '1800e2068 movsxd rcx, edx',
        '1800e206b mul rcx', '1800e206e cmovb rax, rdi', '1800e2075 call 0x1802a1dd6'])
    argument_reads = [line for line in body if '[rbp + 0xe8]' in line]
    assert argument_reads == ['1800e204d cmp dword ptr [rbp + 0xe8], 2', '1800e208b cmp dword ptr [rbp + 0xe8], 2']
    positive_max = ((1 << 31) - 1) * 4
    negative_unsigned_min = (1 << 64) - (1 << 31)
    assert positive_max < 1 << 64 and negative_unsigned_min * 4 > (1 << 64) - 1
    dialog_baseline('legend-3d-integer-reference.json', dict(positive_byte_max=positive_max,
        negative_unsigned_min=negative_unsigned_min, signed32_saturation_identity_proved=True,
        argument_reads=argument_reads, native_allocations_performed=False, geometry_validated=False))
