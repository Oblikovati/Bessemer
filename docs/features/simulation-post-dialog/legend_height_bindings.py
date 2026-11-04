"""Complete C849 height/title/tick consumers and exact word/font/count relationships (#166)."""
from typing import cast
import json
import pefile
from native_carriers import RESEARCH, digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images, dialog_export
from style_preferences_disassembly import decoded_style_preference_method
from legend_body_evidence import legend_body_capture
from legend_palette_bindings import legend_palette_methods
from legend_palette_evidence import legend_palette_report
from legend_height_evidence import legend_height_report


def legend_height_methods() -> list[dict[str, object]]:
    """Normalize only original probe/structured-leaf text beside all original byte pins; e.g. legend_height_methods()."""
    probes = {row['address']: row for row in json.loads((RESEARCH / 'cosworks-post-legend-ticks-probe.json').read_bytes())}
    methods: list[dict[str, object]] = []
    for row in cast(list[dict[str, object]], legend_height_report()['functions']):
        if 'raw_hex' in row:
            assert digest(bytes.fromhex(cast(str, row['raw_hex']))) == row['sha256']
            original = cast(list[dict[str, str]], row['instructions'])
            lines = [f"{int(item['address'], 16):x} {item['mnemonic']} {item['operands']}".rstrip() for item in original]
        else:
            probe = probes[row['address']]
            assert row['end'] == probe['end']
            lines = probe['instructions']
        methods.append(row | {'start': row['address'], 'instructions': lines})
    return methods


def verify_legend_height_bindings() -> None:
    """Match all152 instructions, exact constants/exports and independent runtime-count arguments; e.g. verify_legend_height_bindings()."""
    report = legend_height_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        methods = legend_height_methods()
        counts = [len(decoded_style_preference_method(image, method, normalize_address_spelling=True)) for method in methods]
        assert counts == [98, 25, 23, 6], counts
        height_constants_and_targets(image, methods, report)
        verify_height_argument_anchors(methods)
        verify_height_tick_anchors(methods[0])
        verify_height_palette_cap(image)
        dialog_baseline('legend-height-installed-reference.json', dict(module_sha256=report['module_sha256'],
            complete_instruction_counts=counts, exact_decoder_qualifications=0, target_executed=False,
            font_metrics_known=False, pixel_outputs_validated=False, complete_tick_adjustment_modeled=False))
    finally:
        for image in images.values(): image.close()


def height_constants_and_targets(image: pefile.PE, methods: list[dict[str, object]], report: dict[str, object]) -> None:
    """Bind the full sign mask, title scale, called helpers and all three decompilations; e.g. height_constants_and_targets(image, methods, report)."""
    assert cast(dict[str, str], report['sign_mask'])['raw_hex'] == 'ffffff7fffffff7fffffff7fffffff7f'
    assert cast(dict[str, str], report['title_scale'])['raw_hex'] == '000000c0'
    for key in ('sign_mask', 'title_scale'):
        row = cast(dict[str, str], report[key])
        expected = bytes.fromhex(row['raw_hex'])
        assert image.get_data(int(row['address'], 16) - image.OPTIONAL_HEADER.ImageBase, len(expected)) == expected
    for name, address in (('GetLegendCellHeightInPixel', 0x1800db650),
            ('GetLegendUpperLeftInPixel', 0x1800db720), ('AdjustValuesAtTick', 0x1800d9560)):
        dialog_export(image, ('?' + name + '@CDspLegend@@').encode(), address)
    for method in methods:
        if 'decompilation_sha256' not in method: continue
        path = RESEARCH / 'decomp/cosworks-post-legend-ticks' / f"FUN_{int(cast(str, method['address']), 16):x}.c"
        assert digest(path.read_bytes()) == method['decompilation_sha256']


def verify_height_argument_anchors(methods: list[dict[str, object]]) -> None:
    """Keep the helper return/IMUL ordering and runtime-only font/title arguments exact; e.g. verify_height_argument_anchors(methods)."""
    total = cast(list[str], methods[2]['instructions'])
    call, multiply = '1800db6ff call 0x1800db650', '1800db704 imul eax, dword ptr [rbx + 0x14]'
    assert total.index(multiply) == total.index(call) + 1
    assert total[-1].rstrip() == '1800db710 ret', total[-1]
    cell = cast(list[str], methods[3]['instructions'])
    assert cell == ['1800db650 movd xmm0, dword ptr [rdx]', '1800db654 cvtdq2ps xmm0, xmm0',
        '1800db657 andps xmm0, xmmword ptr [rip + 0x1fc572]', '1800db65e addss xmm0, xmm0',
        '1800db662 cvttss2si eax, xmm0', '1800db666 ret']
    title = cast(list[str], methods[1]['instructions'])
    anchors = ['1800dc122 call 0x1800db720', '1800dc127 mov rax, qword ptr [rsp + 0x68]',
        '1800dc130 movd xmm0, dword ptr [rax]', '1800dc134 cvtdq2ps xmm0, xmm0',
        '1800dc137 andps xmm0, xmmword ptr [rip + 0x1fba92]', '1800dc13e mulss xmm0, dword ptr [rip + 0x2024ce]',
        '1800dc146 cvttss2si eax, xmm0', '1800dc14a add ecx, eax',
        '1800dc14c mov eax, dword ptr [rsp + 0x50]', '1800dc150 mov dword ptr [rbx], eax', '1800dc157 mov dword ptr [rdi], ecx']
    positions = [title.index(anchor) for anchor in anchors]
    assert positions == sorted(set(positions))


def verify_height_tick_anchors(method: dict[str, object]) -> None:
    """Bind separate requested count, null paths and the conditional final adjustment; e.g. verify_height_tick_anchors(method)."""
    lines = cast(list[str], method['instructions'])
    anchors = ['1800dc17a movsxd rdi, r8d', '1800dc183 cmp edi, 1', '1800dc186 jle 0x1800dc2b3',
        '1800dc1a4 call 0x1802a1dd6', '1800dc1ac test rax, rax', '1800dc1af je 0x1800dc2b3',
        '1800dc1b5 mov ecx, dword ptr [rbx]', '1800dc1bc mov dword ptr [rax], ecx',
        '1800dc1be lea ecx, [rdi - 1]', '1800dc1ca subss xmm2, dword ptr [rbx]', '1800dc1d1 divss xmm2, xmm0',
        '1800dc295 cmp dword ptr [rbp + 0xd4], 0', '1800dc29c je 0x1800dc2ae',
        '1800dc2a9 call 0x1800d9560', '1800dc2ae mov rax, r14', '1800dc2b3 xor eax, eax']
    positions = [lines.index(anchor) for anchor in anchors]
    assert positions == sorted(set(positions))
    assert not any('[rbp + 0x14]' in line for line in lines)


def verify_height_palette_cap(image: pefile.PE) -> None:
    """Keep the special256-level relationship conditional on earlier mapping guards; e.g. verify_height_palette_cap(image)."""
    report = legend_palette_report()
    probe = RESEARCH / 'cosworks-post-legend-color-policy-probe.json'
    pins = [value for path, value in cast(dict[str, str], report['sources']).items() if path.replace('\\', '/').endswith('/' + probe.name)]
    assert pins == [digest(probe.read_bytes())]
    method = legend_palette_methods()[1]
    start, end = (int(cast(str, method[key]), 16) for key in ('start', 'end'))
    assert digest(image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)) == method['sha256']
    lines = cast(list[str], method['instructions'])
    anchors = ['1800da384 cmp r13d, ebp', '1800da387 jl 0x1800da938', '1800da38d je 0x1800da938',
        '1800da393 cmp r13d, 0x100', '1800da39a jne 0x1800da3a9',
        '1800da39c mov ecx, dword ptr [r15 + 0x14]', '1800da3a0 inc ecx',
        '1800da3a2 cmp dword ptr [r15 + 0x10], ecx', '1800da3a6 cmovg ebp, ecx']
    positions = [lines.index(anchor) for anchor in anchors]
    assert positions == sorted(set(positions))


def verify_legend_height_rea_capture() -> None:
    """Require all152 independent locations and four complete contiguous bodies; e.g. verify_legend_height_rea_capture()."""
    module_hash = cast(str, legend_height_report()['module_sha256'])
    captures = [legend_body_capture(FEATURE / f'legend-height-rea-{name}.json', method, module_hash)
        for name, method in zip(('ticks', 'title', 'total', 'cell'), legend_height_methods(), strict=True)]
    dialog_baseline('legend-height-rea-reference.json', dict(captures=captures,
        independent_instruction_count=152, target_executed=False))
