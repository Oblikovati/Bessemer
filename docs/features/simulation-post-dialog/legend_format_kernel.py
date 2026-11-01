"""Reviewed C846 control order and exact binary32 interval decision (#166)."""
import math
import struct
from typing import cast
from dialog_evidence import dialog_baseline
from legend_format_bindings import legend_format_methods
from legend_format_cases import legend_magnitude_patterns
from legend_format_evidence import legend_format_report


def verify_legend_format_kernel() -> None:
    """Bind guard order and independently prove the integer interval endpoints; e.g. verify_legend_format_kernel()."""
    verify_legend_format_guard_order()
    report = legend_format_report()
    constants = {row['address']: row['raw_hex'] for row in cast(list[dict[str, str]], report['constants'])}
    assert constants == {'0x1802de610': '00007a44', '0x1802da8e8': 'fca9f1d24d62503f', '0x1802d7bd0': 'ffffff7f'}
    lower = struct.unpack('<d', bytes.fromhex(constants['0x1802da8e8']))[0]
    upper = struct.unpack('<f', bytes.fromhex(constants['0x1802de610']))[0]
    decode = lambda word: struct.unpack('<f', struct.pack('<I', word))[0]
    assert decode(0x3a83126e) < lower <= decode(0x3a83126f)
    assert decode(0x447a0000) == upper < decode(0x447a0001)
    for bits in legend_magnitude_patterns():
        magnitude = abs(decode(bits))
        numeric = not math.isnan(magnitude) and lower <= magnitude <= upper
        assert numeric == (0x3a83126f <= bits & 0x7fffffff <= 0x447a0000), hex(bits)
    dialog_baseline('legend-format-kernel-reference.json', dict(behavior_version='C846-local-format-v1',
        first_admitted_positive_binary32='0x3a83126f', last_admitted_positive_binary32='0x447a0000',
        magnitude_patterns=len(legend_magnitude_patterns()), complete_guard_order_bound=True,
        assumptions=['Masked FP exceptions and DAZ disabled', 'Original binary32 input widened to binary64 for the lower comparison'],
        native_execution=False, actual_fp_environment_known=False))


def verify_legend_format_guard_order() -> None:
    """Require the exact native branch sequence before selector/precision reads; e.g. verify_legend_format_guard_order()."""
    methods = legend_format_methods()
    lines = cast(list[str], methods[1]['instructions'])
    anchors = ['1800db1d8 test r8, r8', '1800db1db je 0x1800db329',
        '1800db1e1 cmp dword ptr [rcx + 0xb8], 0', '1800db1e8 jne 0x1800db329',
        '1800db1ee cmp dword ptr [rcx + 0xc4], 0', '1800db1f5 je 0x1800db329',
        '1800db1fb andps xmm1, xmmword ptr [rip + 0x1fc9ce]',
        '1800db202 comiss xmm1, dword ptr [rip + 0x203407]', '1800db209 ja 0x1800db329',
        '1800db212 cvtss2sd xmm0, xmm1', '1800db216 comisd xmm0, xmmword ptr [rip + 0x1ff6ca]',
        '1800db21e jb 0x1800db329', '1800db241 cmp dword ptr [rbx + 0xc8], 0',
        '1800db25e mov r9d, dword ptr [rbx + 0xbc]', '1800db265 lea r8d, [r9 + 7]']
    positions = [lines.index(anchor) for anchor in anchors]
    assert positions == sorted(set(positions)), positions
    primary = cast(list[str], methods[0]['instructions'])
    assert '1800dbaf6 mov r9d, dword ptr [rbx + 0xbc]' in primary
