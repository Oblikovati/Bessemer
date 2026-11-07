"""C854 complete renderer/title bodies, exact width constants and font/color boundaries (#166)."""
from typing import cast
import json
import struct
import pefile
from native_carriers import RESEARCH, digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images, dialog_export
from dialog_field_bindings import verify_field_leaf
from colorref_import_evidence import colorref_import_reports
from legend_3d_bindings import bind_3d_indirect_call
from style_preferences_disassembly import decoded_style_preference_method
from legend_body_evidence import legend_body_capture
from legend_width_evidence import legend_width_report


def legend_width_methods() -> list[dict[str, object]]:
    """Retain both complete original probe intervals and immutable byte/decompilation pins; e.g. legend_width_methods()."""
    probes = {row['address']: row for prefix in ('cosworks-post-legend-main', 'cosworks-post-legend-title2d')
        for row in json.loads((RESEARCH / (prefix + '-probe.json')).read_bytes())}
    methods: list[dict[str, object]] = []
    for row in cast(list[dict[str, object]], legend_width_report()['functions']):
        probe = probes[row['address']]
        assert row['end'] == probe['end']
        methods.append(row | dict(start=row['address'], instructions=probe['instructions']))
    return methods


def verify_legend_width_bindings() -> None:
    """Require all1235 instructions, both named exports and exact width/font boundaries; e.g. verify_legend_width_bindings()."""
    report = legend_width_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        methods = legend_width_methods()
        counts = [len(decoded_style_preference_method(image, method, normalize_address_spelling=True)) for method in methods]
        assert counts == [1144, 91], counts
        verify_width_exports_and_decompilations(image, methods)
        verify_width_constants(image, report)
        verify_width_font_constructor(image, images['dspogl.dll'])
        verify_width_policy_anchors(methods[0])
        verify_width_title_anchors(methods[1])
        dialog_baseline('legend-width-installed-reference.json', dict(complete_instruction_counts=counts,
            exact_decoder_qualifications=0, target_executed=False, floating_output_modeled=False,
            stored_color100_identity_inferred=False, universal_width_units_inferred=False))
    finally:
        for image in images.values(): image.close()


def verify_width_exports_and_decompilations(image: pefile.PE, methods: list[dict[str, object]]) -> None:
    """Require exact named exports and both complete decompilation hashes; e.g. verify_width_exports_and_decompilations(image, methods)."""
    folders = ('cosworks-post-legend-main', 'cosworks-post-legend-title2d')
    for folder, method in zip(folders, methods, strict=True):
        address = int(cast(str, method['start']), 16)
        dialog_export(image, cast(str, method['export']).encode(), address)
        path = RESEARCH / 'decomp' / folder / f'FUN_{address:x}.c'
        assert digest(path.read_bytes()) == method['decompilation_sha256']


def verify_width_constants(image: pefile.PE, report: dict[str, object]) -> None:
    """Bind each MULSS RIP target to its exact four-byte constant; e.g. verify_width_constants(image, report)."""
    expected = ((0x1800e2899, 0x1802db520, '6f12833a', b'\xf3\x0f\x59\x3d'),
        (0x1800e28a4, 0x1802de5fc, '0000203f', b'\xf3\x0f\x59\x35'),
        (0x1800e28c8, 0x1802da658, '00008040', b'\xf3\x0f\x59\x35'))
    constants = cast(list[dict[str, object]], report['constants'])
    assert len(constants) == len(expected)
    for (instruction, address, raw_hex, prefix), constant in zip(expected, constants, strict=True):
        raw = image.get_data(instruction - image.OPTIONAL_HEADER.ImageBase, 8)
        assert raw[:4] == prefix and instruction + 8 + struct.unpack_from('<i', raw, 4)[0] == address
        actual = image.get_data(address - image.OPTIONAL_HEADER.ImageBase, 4)
        assert constant == dict(address=hex(address), raw_hex=raw_hex, float_value=struct.unpack('<f', actual)[0])
        assert actual.hex() == raw_hex


def verify_width_font_constructor(caller: pefile.PE, provider: pefile.PE) -> None:
    """Require the font64 packed constructor call without assigning stored color100; e.g. verify_width_font_constructor(caller, provider)."""
    original = colorref_import_reports()[0]
    assert digest(provider.__data__) == original['module_sha256']
    constructor = cast(dict[str, str], original['constructor'])
    verify_field_leaf(provider, constructor | dict(name=constructor['export']))
    bind_3d_indirect_call(caller, 0x1800e28d8, b'DSPOGL.dll', constructor['export'].encode(), 0x1802cb318)


def ordered_width_anchors(method: dict[str, object], anchors: list[str]) -> None:
    """Require exact ordered field reads and native instructions; e.g. ordered_width_anchors(method, anchors)."""
    lines = cast(list[str], method['instructions'])
    positions = [lines.index(anchor) for anchor in anchors]
    assert positions == sorted(set(positions)), (anchors, positions)


def verify_width_policy_anchors(method: dict[str, object]) -> None:
    """Bind signed font conversion, B4 branches and separate font64 packed color; e.g. verify_width_policy_anchors(method)."""
    ordered_width_anchors(method, ['1800e2883 mov r13, rcx', '1800e2886 mov rcx, qword ptr [rbp + 0x1a0]',
        '1800e2891 movd xmm7, dword ptr [rcx + 0x68]', '1800e2896 cvtdq2ps xmm7, xmm7',
        '1800e2899 mulss xmm7, dword ptr [rip + 0x1f8c7f]', '1800e28a1 movaps xmm6, xmm7',
        '1800e28a4 mulss xmm6, dword ptr [rip + 0x1fbd50]', '1800e28ac mov eax, dword ptr [r13 + 0xb4]',
        '1800e28b3 cmp eax, 1', '1800e28b6 jne 0x1800e28c4', '1800e28b8 movaps xmm0, xmm6',
        '1800e28bb addss xmm0, xmm6', '1800e28bf movaps xmm6, xmm0', '1800e28c2 jmp 0x1800e28d0',
        '1800e28c4 test eax, eax', '1800e28c6 jne 0x1800e28d0',
        '1800e28c8 mulss xmm6, dword ptr [rip + 0x1f7d88]', '1800e28d0 lea rdx, [rcx + 0x64]',
        '1800e28d4 lea rcx, [rbp + 0x68]', '1800e28d8 call qword ptr [rip + 0x1e8a3a]'])


def verify_width_title_anchors(method: dict[str, object]) -> None:
    """Keep title font and packed arguments distinct from unresolved stored color100; e.g. verify_width_title_anchors(method)."""
    ordered_width_anchors(method, ['1800dff99 mov rsi, rdx', '1800dffa8 mov rax, qword ptr [rsp + 0x108]',
        '1800dffb0 movups xmm0, xmmword ptr [rax]', '1800dfff7 mov eax, dword ptr [rsp + 0xf0]',
        '1800dfffe mov dword ptr [rsp + 0x20], eax', '1800e000c call 0x1800dc100',
        '1800e0060 mov edx, dword ptr [rsp + 0x100]', '1800e0067 mov rcx, rsi', '1800e006a call r8'])


def verify_legend_width_rea_captures() -> None:
    """Require all1235 independent locations and both complete contiguous bodies; e.g. verify_legend_width_rea_captures()."""
    module_hash = cast(str, legend_width_report()['module_sha256'])
    captures = [legend_body_capture(FEATURE / f'legend-width-rea-{name}.json', method, module_hash)
        for name, method in zip(('main', 'title'), legend_width_methods(), strict=True)]
    dialog_baseline('legend-width-rea-reference.json', dict(captures=captures,
        independent_instruction_count=1235, complete_body_bytes=4974, target_executed=False))
