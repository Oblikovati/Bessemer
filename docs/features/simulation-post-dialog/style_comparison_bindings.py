"""C863 complete comparison/copy bodies and exact delegated consumer bindings (#166)."""
from typing import cast
import json
import pefile
from native_carriers import RESEARCH, digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images, dialog_export
from style_preferences_disassembly import decoded_style_preference_method
from legend_body_evidence import legend_body_capture
from legend_3d_bindings import bind_3d_indirect_call
from color_comparison_evidence import verify_color_comparison_bindings
from legend_comparison_evidence import verify_legend_comparison_bindings
from style_comparison_evidence import style_comparison_report


def style_comparison_methods() -> list[dict[str, object]]:
    """Retain both complete original probe ranges and body/decompilation pins; e.g. style_comparison_methods()."""
    probes = {row['address']: row for row in json.loads((RESEARCH / 'cosworks-post-style-compare-probe.json').read_bytes())}
    methods: list[dict[str, object]] = []
    for row in cast(list[dict[str, object]], style_comparison_report()['functions']):
        probe = probes[row['address']]; assert row['end'] == probe['end']
        methods.append(row | dict(start=row['address'], instructions=probe['instructions']))
    return methods


def verify_style_comparison_bindings() -> None:
    """Bind all145 original instructions, both complete return paths and exact delegated consumers; e.g. verify_style_comparison_bindings()."""
    images = dialog_images()
    try:
        image = images['dgxpost.dll']; report = style_comparison_report()
        assert digest(image.__data__) == report['module_sha256']
        methods = style_comparison_methods()
        counts = [len(decoded_style_preference_method(image, method, normalize_address_spelling=True)) for method in methods]
        assert counts == [107, 38], counts
        verify_style_comparison_symbols(image, methods)
        verify_style_comparison_dependencies(methods[0])
        verify_style_copy_dependencies(methods[1])
        verify_style_comparison_delegates(image)
        dialog_baseline('style-comparison-installed-reference.json', dict(complete_instruction_counts=counts,
            complete_body_bytes=676, exact_decoder_qualifications=0, target_executed=False,
            color_call_sites=7, full_style_comparator_modeled=False, wire_words_as_loaded_state=False))
    finally:
        for image in images.values(): image.close()


def verify_style_comparison_symbols(image: pefile.PE, methods: list[dict[str, object]]) -> None:
    """Require exact named exports and both original decompilation hashes; e.g. verify_style_comparison_symbols(image, methods)."""
    for method in methods:
        address = int(cast(str, method['start']), 16)
        dialog_export(image, cast(str, method['export']).encode(), address)
        path = RESEARCH / 'decomp/cosworks-post-style-compare' / f'FUN_{address:x}.c'
        assert digest(path.read_bytes()) == method['decompilation_sha256']


def style_dependency_anchors(method: dict[str, object], anchors: list[str]) -> None:
    """Keep original field reads, branches and delegation in native order; e.g. style_dependency_anchors(method, anchors)."""
    lines = cast(list[str], method['instructions'])
    positions = [lines.index(anchor) for anchor in anchors]
    assert positions == sorted(set(positions)), (anchors, positions)


def verify_style_comparison_dependencies(method: dict[str, object]) -> None:
    """Bind all controlling conditions, both scalar UCOMISS/JNE pairs and return destinations; e.g. verify_style_comparison_dependencies(method)."""
    style_dependency_anchors(method, ['180044034 mov eax, dword ptr [rcx + 0x1b8]',
        '180044040 cmp eax, dword ptr [rdx + 0x1b8]', '180044046 jne 0x180044207',
        '18004404c cmp eax, 1', '18004404f jne 0x18004406d',
        '18004406d cmp dword ptr [rbx + 0x1b8], 2', '180044074 jne 0x1800440ae',
        '1800440ae mov eax, dword ptr [rbx + 0x21c]', '1800440c2 je 0x18004410b',
        '1800440cc ucomiss xmm0, dword ptr [rbx + 0x22c]', '1800440d3 jne 0x180044207',
        '1800440d9 mov eax, dword ptr [rbx + 0x230]', '1800440ed jne 0x18004410b',
        '180044111 call 0x1800d9950', '180044124 cmp dword ptr [rbx + 0x28c], eax',
        '180044136 cmp dword ptr [rbx + 0x290], eax', '180044148 cmp dword ptr [rbx + 0x294], eax',
        '18004415a cmp dword ptr [rbx + 0x298], eax', '180044182 mov eax, dword ptr [rbx + 0x1ec]',
        '180044192 je 0x1800441ba', '18004419a cmp dword ptr [rbx + 0x1f0], eax',
        '1800441ba mov eax, dword ptr [rbx + 0x204]', '1800441ca je 0x1800441f2',
        '1800441d2 cmp dword ptr [rbx + 0x208], eax',
        '1800441fa ucomiss xmm0, dword ptr [rbx + 0x2b8]', '180044201 jne 0x180044207',
        '180044203 xor eax, eax', '180044207 mov eax, 1', '180044220 ret '])


def verify_style_copy_dependencies(method: dict[str, object]) -> None:
    """Keep the earlier legend call and exact70/73 qword clear versus supplied scalar stores; e.g. verify_style_copy_dependencies(method)."""
    style_dependency_anchors(method, ['180043f96 call 0x1800d98d0',
        '180043f9b mov rax, qword ptr [rbx + 0x2d0]', '180043fa2 test rax, rax', '180043fa5 je 0x180043fc3',
        '180043fa7 mov eax, dword ptr [rax + 0x10]', '180043faa cmp eax, 0x46', '180043fad je 0x180043fb4',
        '180043faf cmp eax, 0x49', '180043fb2 jne 0x180043fc3', '180043fb4 xor eax, eax',
        '180043fb6 mov qword ptr [rbx + 0x234], rax', '180043fc2 ret ',
        '180043fc3 mov eax, dword ptr [rsp + 0xb8]', '180043fca mov ecx, dword ptr [rsp + 0xc0]',
        '180043fd1 mov dword ptr [rbx + 0x234], eax', '180043fd7 mov dword ptr [rbx + 0x238], ecx', '180043fe2 ret '])


def verify_style_comparison_delegates(image: pefile.PE) -> None:
    """Bind every exact color IAT call and the named legend compare/copy destinations; e.g. verify_style_comparison_delegates(image)."""
    for address in (0x18004405f, 0x180044084, 0x1800440a0, 0x1800440fd, 0x180044174, 0x1800441b0, 0x1800441e8):
        bind_3d_indirect_call(image, address, b'DSPOGL.dll', b'??9COGLColor@@QEAAHAEBV0@@Z', 0x1802cb380)
    dialog_export(image, b'?DiffOGLRender@CDspLegend@@', 0x1800d9950)
    dialog_export(image, b'?CopyPreferences@CDspLegend@@', 0x1800d98d0)
    verify_color_comparison_bindings()
    verify_legend_comparison_bindings()


def verify_style_comparison_rea() -> None:
    """Require all145 independent locations and both complete contiguous method bodies; e.g. verify_style_comparison_rea()."""
    module_hash = cast(str, style_comparison_report()['module_sha256'])
    captures = [legend_body_capture(FEATURE / f'style-comparison-rea-{name}.json', method, module_hash)
        for name, method in zip(('diff', 'copy'), style_comparison_methods(), strict=True)]
    dialog_baseline('style-comparison-rea-reference.json', dict(captures=captures,
        independent_instruction_count=145, complete_body_bytes=676, target_executed=False))
