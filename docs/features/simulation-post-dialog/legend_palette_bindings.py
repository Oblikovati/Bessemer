"""Complete C848 mapper bodies, dispatch targets and raw-control argument bindings (#166)."""
from typing import cast
import json
import pefile
from native_carriers import RESEARCH, digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images, dialog_export, dialog_import
from legend_palette_qualifications import decoded_palette_methods
from legend_body_evidence import legend_body_capture
from legend_palette_evidence import legend_palette_report


def legend_palette_methods() -> list[dict[str, object]]:
    """Attach complete original probe instructions to exact recorded byte/decompilation digests; e.g. legend_palette_methods()."""
    probes = json.loads((RESEARCH / 'cosworks-post-legend-color-policy-probe.json').read_bytes())
    methods: list[dict[str, object]] = []
    for recorded, probe in zip(cast(list[dict[str, object]], legend_palette_report()['functions']), probes, strict=True):
        assert (recorded['address'], recorded['end']) == (probe['address'], probe['end'])
        methods.append(recorded | {'start': recorded['address'], 'instructions': probe['instructions']})
    return methods


def verify_legend_palette_bindings() -> None:
    """Match all620 instructions and exact dispatcher/copy/provider identities; e.g. verify_legend_palette_bindings()."""
    report = legend_palette_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        methods = legend_palette_methods()
        counts, qualifications = decoded_palette_methods(image, methods)
        verify_legend_palette_anchors(methods)
        legend_palette_targets(image, methods)
        dialog_baseline('legend-palette-installed-reference.json', dict(module_sha256=report['module_sha256'],
            complete_instruction_counts=counts, exact_decoder_qualifications=1, qualifications=qualifications, target_executed=False,
            interpolation_modeled=False, actual_runtime_plot_known=False, gray_output_established=False))
    finally:
        for image in images.values(): image.close()


def legend_palette_targets(image: pefile.PE, methods: list[dict[str, object]]) -> None:
    """Retain exact helper targets, provider imports and decompilation pins; e.g. legend_palette_targets(image, methods)."""
    for name, address in (('GetColorMapping_ShowGrayAboveYield', 0x1800db0c0),
            ('GetColorMapping_ShellMaterial', 0x1800dab20), ('GetColorMapping_ShellThickness', 0x1800dadf0)):
        dialog_export(image, ('?' + name + '@CDspLegend@@').encode(), address)
    dialog_import(image, b'dspogl.dll', b'??0COGLColor@@QEAA@XZ', 0x1802cb300)
    dialog_import(image, b'dspogl.dll', b'??4COGLColor@@QEAAAEAV0@AEBV0@@Z', 0x1802cb320)
    for method in methods:
        path = RESEARCH / 'decomp/cosworks-post-legend-color-policy' / f"FUN_{int(cast(str, method['address']), 16):x}.c"
        assert digest(path.read_bytes()) == method['decompilation_sha256']


def verify_legend_palette_anchors(methods: list[dict[str, object]]) -> None:
    """Bind early-gray continuation, mode4 tail dispatch and count/reversal source arguments; e.g. verify_legend_palette_anchors(methods)."""
    dispatch = cast(list[str], methods[0]['instructions'])
    anchors = ['1800da1cf cmp dword ptr [rcx + 0xd4], 0', '1800da1de je 0x1800da1e9',
        '1800da1e0 call 0x1800db0c0', '1800da1e5 test eax, eax', '1800da1e7 jne 0x1800da231',
        '1800da1e9 cmp dword ptr [rbx + 8], 4', '1800da1f5 jne 0x1800da22c',
        '1800da1f7 mov rax, qword ptr [rbx + 0x1a8]', '1800da1fe cmp dword ptr [rax + 0x18], 3',
        '1800da202 jne 0x1800da218', '1800da213 jmp 0x1800dab20',
        '1800da227 jmp 0x1800dadf0', '1800da22c call 0x1800da250']
    positions = [dispatch.index(anchor) for anchor in anchors]
    assert positions == sorted(set(positions)), positions
    regular = set(cast(list[str], methods[1]['instructions']))
    assert {'1800da2a8 mov ebp, dword ptr [r15 + 0x10]', '1800da2ac lea r14, [r15 + 0x18]',
        '1800da2bb mov eax, 0x10', '1800da2c0 mul rdi', '1800da2fe test ebp, ebp',
        '1800da300 jle 0x1800da367', '1800da302 cmp dword ptr [r15 + 0xc], 0',
        '1800da307 je 0x1800da33c', '1800da30c lea eax, [rbp - 1]', '1800da312 shl rdi, 4',
        '1800da316 add rdi, r14', '1800da32c sub rdi, 0x10', '1800da35d add rdi, 0x10'} <= regular


def verify_legend_palette_rea_capture() -> None:
    """Require all620 independent Ghidra locations and both complete contiguous bodies; e.g. verify_legend_palette_rea_capture()."""
    module_hash = cast(str, legend_palette_report()['module_sha256'])
    captures = [legend_body_capture(FEATURE / f'legend-palette-rea-{name}.json', method, module_hash)
        for name, method in zip(('dispatch', 'regular'), legend_palette_methods(), strict=True)]
    dialog_baseline('legend-palette-rea-reference.json', dict(captures=captures,
        independent_instruction_count=620, target_executed=False))
