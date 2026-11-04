"""C851 complete consumers and exact probe-tail/function-body separation (#166)."""
from typing import cast
import json
import pefile
from native_carriers import RESEARCH, digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images, dialog_export
from style_preferences_disassembly import decoded_style_preference_method
from legend_body_evidence import legend_body_capture
from legend_step_evidence import legend_step_report

STEP_PROBE_TAILS = {'0x1800dba50': 0x1800dba86, '0x1800dbc30': 0x1800dbc66, '0x1800db850': 0x1800dba4f}


def legend_step_methods() -> list[dict[str, object]]:
    """Retain full original probe intervals, including their three INT3 tails; e.g. legend_step_methods()."""
    probes = {row['address']: row for prefix in ('cosworks-post-legend-step', 'cosworks-post-legend-mode-render')
        for row in json.loads((RESEARCH / (prefix + '-probe.json')).read_bytes())}
    methods: list[dict[str, object]] = []
    for row in cast(list[dict[str, object]], legend_step_report()['functions']):
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


def verify_legend_step_bindings() -> None:
    """Bind all555 probe instructions, exact exports and complete availability returns; e.g. verify_legend_step_bindings()."""
    report = legend_step_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        methods = legend_step_methods()
        counts = [len(decoded_style_preference_method(image, method, normalize_address_spelling=True)) for method in methods]
        assert counts == [14, 14, 129, 373, 25], counts
        verify_step_exports_and_decompilations(image, methods)
        verify_step_layout_anchors(methods[3])
        verify_step_text_anchors(methods[2])
        verify_step_availability_anchors(methods[4])
        tails = step_probe_tail_pins(image, methods)
        dialog_baseline('legend-step-installed-reference.json', dict(complete_probe_instruction_counts=counts,
            exact_decoder_qualifications=0, probe_tail_bytes=tails, target_executed=False,
            complete_renderer_modeled=False, localized_labels_known=False, availability_from_fc=False))
    finally:
        for image in images.values(): image.close()


def verify_step_exports_and_decompilations(image: pefile.PE, methods: list[dict[str, object]]) -> None:
    """Bind all five named exports and four exact decompilation files; e.g. verify_step_exports_and_decompilations(image, methods)."""
    for method in methods:
        address = int(cast(str, method['start']), 16)
        if 'exports' in method:
            for name in cast(list[str], method['exports']):
                dialog_export(image, name.encode(), address)
        else:
            dialog_export(image, b'?IsModeShapeOrPlotStepAvailable@CDspLegend@@', address)
        if 'decompilation_sha256' not in method: continue
        path = RESEARCH / 'decomp/cosworks-post-legend-step' / f'FUN_{address:x}.c'
        assert digest(path.read_bytes()) == method['decompilation_sha256']


def step_ordered_anchors(method: dict[str, object], anchors: list[str]) -> None:
    """Require every selected dependency in original instruction order; e.g. step_ordered_anchors(method, anchors)."""
    lines = cast(list[str], method['instructions'])
    positions = [lines.index(anchor) for anchor in anchors]
    assert positions == sorted(set(positions)), (anchors, positions)


def verify_step_layout_anchors(method: dict[str, object]) -> None:
    """Keep the enable gate, both FC reads and later runtime spacing multiplier separate; e.g. verify_step_layout_anchors(method)."""
    step_ordered_anchors(method, ['1800de4c0 test edx, edx', '1800de4c2 je 0x1800deaf8',
        '1800de535 cmp dword ptr [rcx + 0xfc], 0', '1800de53c je 0x1800de567',
        '1800de53e mov ecx, dword ptr [rcx + 0x12c]', '1800de544 sub ecx, dword ptr [r15 + 0x124]',
        '1800de54b mov eax, 0x66666667', '1800de550 imul ecx', '1800de554 sar edi, 3',
        '1800de55e sub edi, dword ptr [r8 + 4]', '1800de562 add edi, dword ptr [rbx + 4]',
        '1800de567 mov edi, dword ptr [rbx + 4]', '1800de56a sub edi, dword ptr [r8 + 4]',
        '1800de577 call 0x1800dbc30', '1800de587 call 0x1800db850', '1800de594 call 0x1800dba50',
        '1800de5a4 cmp dword ptr [r15 + 0xfc], 0', '1800de5ae lea eax, [rax*2 - 4]',
        '1800de5b5 cmp dword ptr [r15 + 0x1b0], 0', '1800de5bf cmp dword ptr [r15 + 0x1b4], 0',
        '1800de5c9 lea eax, [rax + rax*2]', '1800de5cc sub edi, eax'])


def verify_step_text_anchors(method: dict[str, object]) -> None:
    """Bind runtime plot/analysis selection and resource IDs without localized text guesses; e.g. verify_step_text_anchors(method)."""
    lines = cast(list[str], method['instructions'])
    assert any('0x1587' in line for line in lines)
    assert any('0xca4' in line for line in lines)
    assert any('cmp dword ptr [rax + 0x10], 0x15' in line for line in lines)
    assert any('dword ptr [rax + 0xc]' in line for line in lines)
    assert not any('0xfc]' in line for line in lines)


def verify_step_availability_anchors(method: dict[str, object]) -> None:
    """Retain eager ordinary-gate reads, the later override and raw BEC return; e.g. verify_step_availability_anchors(method)."""
    lines = cast(list[str], method['instructions'])
    assert [line for line in lines if line.endswith(' ret')] == ['1800dc81b ret', '1800dc82b ret', '1800dc82e ret']
    step_ordered_anchors(method, ['1800dc7e0 mov rax, qword ptr [rcx + 0x1a8]',
        '1800dc7e7 mov edx, dword ptr [rax + 0x1c]', '1800dc7ed jle 0x1800dc7ff',
        '1800dc7ef cmp dword ptr [rax + 0xbd8], 0', '1800dc801 cmp dword ptr [rax + 0x10], 0x15',
        '1800dc80a jg 0x1800dc816', '1800dc80c test ecx, ecx', '1800dc80e je 0x1800dc82c',
        '1800dc810 cmp dword ptr [rax + 0xc], 8', '1800dc816 mov eax, 1',
        '1800dc81c cmp dword ptr [rax + 0xbe8], 2', '1800dc825 mov eax, dword ptr [rax + 0xbec]'])
    assert not any('0xfc]' in line for line in lines)


def step_probe_tail_pins(image: pefile.PE, methods: list[dict[str, object]]) -> list[dict[str, str]]:
    """Require exactly three separately pinned one-byte INT3 probe tails; e.g. step_probe_tail_pins(image, methods)."""
    tails: list[dict[str, str]] = []
    for method in methods:
        start = cast(str, method['start'])
        if start not in STEP_PROBE_TAILS: continue
        address = STEP_PROBE_TAILS[start]
        assert int(cast(str, method['end']), 16) == address + 1
        assert cast(list[str], method['instructions'])[-1].rstrip() == f'{address:x} int3'
        assert image.get_data(address - image.OPTIONAL_HEADER.ImageBase, 1) == b'\xcc'
        tails.append(dict(address=hex(address), raw_hex='cc', purpose='Original probe byte after the complete returned function body'))
    assert len(tails) == 3
    return tails


def step_function_body(method: dict[str, object]) -> dict[str, object]:
    """Select the exact observed function extent while retaining original probe metadata separately; e.g. step_function_body(method)."""
    start = cast(str, method['start'])
    lines = cast(list[str], method['instructions'])
    if start not in STEP_PROBE_TAILS: return method
    address = STEP_PROBE_TAILS[start]
    assert int(cast(str, method['end']), 16) == address + 1
    assert lines[-1].rstrip() == f'{address:x} int3' and lines[-2].rstrip().endswith(' ret')
    return dict(start=start, end=hex(address), instructions=lines[:-1])


def require_step_body_rejection(method: dict[str, object]) -> None:
    """Reject changed probe endpoints or instruction tails instead of trimming them; e.g. require_step_body_rejection(method)."""
    try:
        step_function_body(method)
    except AssertionError:
        return
    raise AssertionError((method, 'expected the exact captured one-byte INT3 tail shape'))


def verify_step_body_boundaries(methods: list[dict[str, object]]) -> None:
    """Keep tail trimming confined to the three exact captured probe intervals; e.g. verify_step_body_boundaries(methods)."""
    for method in methods:
        if method['start'] not in STEP_PROBE_TAILS:
            assert step_function_body(method) is method
            continue
        require_step_body_rejection(method | dict(end=hex(int(cast(str, method['end']), 16) + 1)))
        lines = cast(list[str], method['instructions'])
        require_step_body_rejection(method | dict(instructions=lines[:-1] + [lines[-1].replace('int3', 'nop')]))


def verify_legend_step_rea_capture() -> None:
    """Require all552 independent body locations, retaining three original probe tails separately; e.g. verify_legend_step_rea_capture()."""
    methods = legend_step_methods()
    verify_step_body_boundaries(methods)
    module_hash = cast(str, legend_step_report()['module_sha256'])
    captures = [legend_body_capture(FEATURE / f'legend-step-rea-{name}.json', step_function_body(method), module_hash)
        for name, method in zip(('next', 'previous', 'text', 'renderer', 'availability'), methods, strict=True)]
    assert sum(cast(int, capture['instructions']) for capture in captures) == 552
    dialog_baseline('legend-step-rea-reference.json', dict(captures=captures, independent_body_instruction_count=552,
        original_probe_instruction_count=555, separately_pinned_int3_tail_bytes=3, target_executed=False))
