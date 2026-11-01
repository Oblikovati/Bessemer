"""Complete C853 consumer evidence with runtime activation distinct from stored words (#166)."""
from typing import cast
import json
from native_carriers import RESEARCH, digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images, dialog_export
from style_preferences_disassembly import decoded_style_preference_method
from legend_body_evidence import legend_body_capture
from legend_end_cell_evidence import legend_end_cell_report


def legend_end_cell_method() -> dict[str, object]:
    """Retain the complete original 2D function and every instruction; e.g. legend_end_cell_method()."""
    report = legend_end_cell_report()
    probes = json.loads((RESEARCH / 'cosworks-post-legend-2d-probe.json').read_bytes())
    methods = cast(list[dict[str, object]], report['functions'])
    assert len(methods) == len(probes) == 1
    method = methods[0]
    assert method['address'] == probes[0]['address'] and method['end'] == probes[0]['end']
    return method | dict(start=method['address'], instructions=probes[0]['instructions'])


def verify_legend_end_cell_bindings() -> None:
    """Bind all1638 instructions, the exact export and complete decompilation; e.g. verify_legend_end_cell_bindings()."""
    report = legend_end_cell_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        method = legend_end_cell_method()
        dialog_export(image, cast(str, method['export']).encode(), int(cast(str, method['start']), 16))
        path = RESEARCH / 'decomp/cosworks-post-legend-2d/FUN_1800e0100.c'
        assert digest(path.read_bytes()) == method['decompilation_sha256']
        assert len(decoded_style_preference_method(image, method, normalize_address_spelling=True)) == 1638
        verify_end_cell_runtime_anchors(method)
        verify_end_cell_rgb_anchors(method)
        dialog_baseline('legend-end-cell-installed-reference.json', dict(complete_instruction_count=1638,
            exact_decoder_qualifications=0, target_executed=False, renderer_modeled=False,
            runtime_plot_word_stored=False, alpha_compositing_inferred=False))
    finally:
        for image in images.values(): image.close()


def ordered_end_cell_anchors(method: dict[str, object], anchors: list[str]) -> None:
    """Require exact ordered native reads rather than textual member-name inference; e.g. ordered_end_cell_anchors(method, anchors)."""
    lines = cast(list[str], method['instructions'])
    positions = [lines.index(anchor) for anchor in anchors]
    assert positions == sorted(set(positions)), (anchors, positions)


def verify_end_cell_runtime_anchors(method: dict[str, object]) -> None:
    """Require runtime524 before E8 and distinct guards for both RGB paths; e.g. verify_end_cell_runtime_anchors(method)."""
    ordered_end_cell_anchors(method, ['1800e0154 mov rsi, rcx',
        '1800e017f xor r14d, r14d', '1800e01a8 mov rax, qword ptr [rsi + 0x1a8]',
        '1800e01af mov ecx, 1', '1800e01b4 cmp dword ptr [rax + 0x524], r14d',
        '1800e01bb je 0x1800e01c8', '1800e01bd cmp dword ptr [rsi + 0xe8], r14d',
        '1800e01c4 cmovne r14d, ecx', '1800e01c8 mov r15d, dword ptr [rsi + 0x14]',
        '1800e044f test r14d, r14d', '1800e0452 mov r14, qword ptr [rbp + 0x20]',
        '1800e0456 je 0x1800e05d2', '1800e0f05 mov rsi, qword ptr [rbp + 0x20]',
        '1800e0f09 test r14d, r14d', '1800e0f0c je 0x1800e1086'])


def verify_end_cell_rgb_anchors(method: dict[str, object]) -> None:
    """Bind RGB copies separately from preserved stored alpha and unmodeled geometry; e.g. verify_end_cell_rgb_anchors(method)."""
    ordered_end_cell_anchors(method, ['1800e0480 mov eax, dword ptr [r14 + 0x14]',
        '1800e050c movss xmm0, dword ptr [r14 + 0xec]',
        '1800e051d movss xmm1, dword ptr [r14 + 0xf0]',
        '1800e052e movss xmm0, dword ptr [r14 + 0xf4]', '1800e056e call 0x18029fb60',
        '1800e0f38 mov eax, dword ptr [rsi + 0x14]',
        '1800e0fc3 movss xmm0, dword ptr [rsi + 0xec]',
        '1800e0fd3 movss xmm1, dword ptr [rsi + 0xf0]',
        '1800e0fe3 movss xmm0, dword ptr [rsi + 0xf4]', '1800e1022 call 0x18029fb60'])


def verify_legend_end_cell_rea_capture() -> None:
    """Require all1638 independent locations and the complete6873-byte body; e.g. verify_legend_end_cell_rea_capture()."""
    method = legend_end_cell_method()
    module_hash = cast(str, legend_end_cell_report()['module_sha256'])
    capture = legend_body_capture(FEATURE / 'legend-end-cell-rea.json', method, module_hash)
    dialog_baseline('legend-end-cell-rea-reference.json', dict(captures=[capture],
        independent_instruction_count=1638, complete_body_bytes=6873, target_executed=False))
