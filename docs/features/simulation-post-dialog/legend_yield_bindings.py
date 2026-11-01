"""Complete C850 consumers with explicit serialized/runtime separation (#166)."""
from typing import cast
import json
import pefile
from native_carriers import RESEARCH, digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images, dialog_export, dialog_import
from dialog_field_bindings import verify_field_leaf
from legend_body_evidence import legend_body_capture
from legend_yield_evidence import legend_yield_report
from legend_yield_qualifications import decoded_yield_methods
from preference_color_evidence import preference_color_reports
from colorref_import_evidence import colorref_import_reports


def legend_yield_methods() -> list[dict[str, object]]:
    """Retain both complete intervals and their original instruction spellings; e.g. legend_yield_methods()."""
    probes = json.loads((RESEARCH / 'cosworks-post-legend-yield-probe.json').read_bytes())
    assert len(probes) == 1
    methods: list[dict[str, object]] = []
    for row in cast(list[dict[str, object]], legend_yield_report()['functions']):
        if 'raw_hex' in row:
            assert digest(bytes.fromhex(cast(str, row['raw_hex']))) == row['sha256']
            original = cast(list[dict[str, str]], row['instructions'])
            lines = [f"{int(item['address'], 16):x} {item['mnemonic']} {item['operands']}".rstrip() for item in original]
        else:
            probe = probes[0]
            assert (row['address'], row['end']) == (probe['address'], probe['end'])
            lines = probe['instructions']
        methods.append(row | {'start': row['address'], 'instructions': lines})
    return methods


def verify_legend_yield_bindings() -> None:
    """Require all119 instructions, both constructors and the exact sentinel qualification; e.g. verify_legend_yield_bindings()."""
    report = legend_yield_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        methods = legend_yield_methods()
        counts, qualifications = decoded_yield_methods(image, methods)
        for method, name in zip(methods, ('GetColorMapping_ShowGrayAboveYield', 'AdjustValuesAtTick'), strict=True):
            dialog_export(image, ('?' + name + '@CDspLegend@@').encode(), int(cast(str, method['start']), 16))
        verify_yield_constructor_bindings(image, images['dspogl.dll'])
        verify_yield_mapping_order(methods[0])
        verify_yield_adjustment_extent(methods[1])
        path = RESEARCH / 'decomp/cosworks-post-legend-yield/FUN_1800db0c0.c'
        assert digest(path.read_bytes()) == methods[0]['decompilation_sha256']
        dialog_baseline('legend-yield-installed-reference.json', dict(complete_instruction_counts=counts,
            exact_decoder_qualifications=qualifications, target_executed=False, runtime_split_serialized=False,
            complete_tick_adjustment_modeled=False, mapping_helper_modeled=False))
    finally:
        for image in images.values(): image.close()


def verify_yield_constructor_bindings(caller: pefile.PE, provider: pefile.PE) -> None:
    """Bind the copy-constructor alias and packed-white constructor to complete provider bytes; e.g. verify_yield_constructor_bindings(caller, provider)."""
    legend = preference_color_reports()[0]
    packed = colorref_import_reports()[0]
    assert digest(provider.__data__) == legend['provider_sha256'] == packed['module_sha256']
    copy = cast(list[dict[str, str]], legend['color_leaves'])[1]
    raw = bytes.fromhex(copy['raw_hex'])
    name = '??0COGLColor@@QEAA@AEBV0@@Z'
    verify_field_leaf(provider, copy | dict(end=hex(int(copy['address'], 16) + len(raw)), sha256=digest(raw), name=name))
    dialog_import(caller, b'DSPOGL.dll', name.encode(), 0x1802cb310)
    constructor = cast(dict[str, str], packed['constructor'])
    verify_field_leaf(provider, constructor | dict(name=constructor['export']))
    dialog_import(caller, b'DSPOGL.dll', constructor['export'].encode(), 0x1802cb318)
    scale = cast(dict[str, str], packed['inverse_scale'])
    assert scale['raw_hex'] == '8180803b'
    assert provider.get_data(int(scale['address'], 16) - provider.OPTIONAL_HEADER.ImageBase, 4).hex() == scale['raw_hex']


def yield_ordered_anchors(method: dict[str, object], anchors: list[str]) -> None:
    """Require argument construction and calls in native order; e.g. yield_ordered_anchors(method, anchors)."""
    lines = cast(list[str], method['instructions'])
    positions = [lines.index(anchor) for anchor in anchors]
    assert positions == sorted(set(positions)), (anchors, positions)


def verify_yield_mapping_order(method: dict[str, object]) -> None:
    """Keep unconditional color setup, sentinel forwarding and both partial calls; e.g. verify_yield_mapping_order(method)."""
    yield_ordered_anchors(method, ['1800db0dc lea rdx, [rcx + 0xd8]',
        '1800db0eb call qword ptr [rip + 0x1f021f]', '1800db0f6 mov dword ptr [rsp + 0x30], 0xffffff',
        '1800db103 call qword ptr [rip + 0x1f020f]', '1800db109 mov r9d, dword ptr [rbx + 0x178]',
        '1800db110 cmp r9d, -2', '1800db12b mov edx, edi', '1800db135 call 0x1800defd0',
        '1800db13a mov eax, 1', '1800db144 js 0x1800db1a6', '1800db149 jge 0x1800db1a6',
        '1800db14b lea eax, [r9 + r9*2]', '1800db14f sub edi, r9d', '1800db152 movsxd rcx, eax',
        '1800db16e lea r8, [rsi + rcx*4]', '1800db175 call 0x1800defd0',
        '1800db17a mov eax, dword ptr [rbx + 0xc]', '1800db17d lea rcx, [rbx + 0x18]',
        '1800db181 mov r9d, dword ptr [rbx + 0x10]', '1800db188 mov edx, dword ptr [rbx + 0x178]',
        '1800db19a call 0x1800defd0', '1800db19f mov eax, 1', '1800db1a6 xor eax, eax'])
    lines = cast(list[str], method['instructions'])
    assert sum('call 0x1800defd0' in line for line in lines) == 3
    assert not any('0xd4]' in line or '0x170]' in line for line in lines)


def verify_yield_adjustment_extent(method: dict[str, object]) -> None:
    """Retain runtime-only yield/split members and all three return instructions; e.g. verify_yield_adjustment_extent(method)."""
    lines = cast(list[str], method['instructions'])
    returns = [line for line in lines if line.rstrip().endswith(' ret')]
    assert returns == ['1800d95e5 ret', '1800d9619 ret', '1800d9624 ret'], returns
    yield_ordered_anchors(method, ['1800d9566 cmp eax, 1', '1800d9569 jle 0x1800d9624',
        '1800d956f cmp dword ptr [rcx + 0xd4], 0', '1800d9576 je 0x1800d9624',
        '1800d957c movsd xmm1, qword ptr [rcx + 0x170]', '1800d9584 cvtpd2ps xmm1, xmm1',
        '1800d9588 mov dword ptr [rcx + 0x178], 0xffffffff', '1800d959a jb 0x1800d961a',
        '1800d95a5 ja 0x1800d9624', '1800d95e5 ret',
        '1800d960c mov dword ptr [r10 + 0x178], edx', '1800d9613 movss dword ptr [r8 + rax*4], xmm1',
        '1800d961a mov dword ptr [rcx + 0x178], 0xfffffffe'])


def verify_legend_yield_rea_capture() -> None:
    """Match both full independent bodies and all119 instruction locations; e.g. verify_legend_yield_rea_capture()."""
    module_hash = cast(str, legend_yield_report()['module_sha256'])
    captures = [legend_body_capture(FEATURE / f'legend-yield-rea-{name}.json', method, module_hash)
        for name, method in zip(('mapping', 'adjustment'), legend_yield_methods(), strict=True)]
    dialog_baseline('legend-yield-rea-reference.json', dict(captures=captures,
        independent_instruction_count=119, target_executed=False))
