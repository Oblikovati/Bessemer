"""Complete C852 native bodies, exact call identities and argument boundaries (#166)."""
from typing import cast
import json
import struct
import pefile
from native_carriers import RESEARCH, digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images, dialog_export, dialog_import
from dialog_field_bindings import verify_field_leaf
from colorref_import_evidence import colorref_import_reports
from style_preferences_disassembly import decoded_style_preference_method
from legend_body_evidence import legend_body_capture
from legend_3d_evidence import legend_3d_report


def legend_3d_methods() -> list[dict[str, object]]:
    """Preserve both complete original probe intervals and their byte pins; e.g. legend_3d_methods()."""
    probes = {row['address']: row for prefix in ('cosworks-post-legend-background', 'cosworks-post-legend-body3d')
        for row in json.loads((RESEARCH / (prefix + '-probe.json')).read_bytes())}
    methods: list[dict[str, object]] = []
    for row in cast(list[dict[str, object]], legend_3d_report()['functions']):
        probe = probes[row['address']]
        assert row['end'] == probe['end']
        methods.append(row | {'start': row['address'], 'instructions': probe['instructions']})
    return methods


def verify_legend_3d_bindings() -> None:
    """Require all432 instructions, both exports/decompilations and exact color/GL calls; e.g. verify_legend_3d_bindings()."""
    report = legend_3d_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        methods = legend_3d_methods()
        counts = [len(decoded_style_preference_method(image, method, normalize_address_spelling=True)) for method in methods]
        assert counts == [185, 247], counts
        verify_3d_exports_and_decompilations(image, methods)
        verify_3d_color_calls(image, images['dspogl.dll'])
        verify_3d_gl_calls(image)
        verify_3d_background_anchors(methods[0])
        verify_3d_body_anchors(methods[1])
        dialog_baseline('legend-3d-installed-reference.json', dict(complete_instruction_counts=counts,
            exact_decoder_qualifications=0, target_executed=False, renderer_modeled=False,
            stored_background_color_inferred=False, stored_contour_type_inferred=False, provider_getter_body_modeled=False))
    finally:
        for image in images.values(): image.close()


def verify_3d_exports_and_decompilations(image: pefile.PE, methods: list[dict[str, object]]) -> None:
    """Require both exact named exports and complete decompilation hashes; e.g. verify_3d_exports_and_decompilations(image, methods)."""
    for method in methods:
        address = int(cast(str, method['start']), 16)
        dialog_export(image, cast(str, method['export']).encode(), address)
        path = RESEARCH / 'decomp/cosworks-post-legend-background' / f'FUN_{address:x}.c'
        assert digest(path.read_bytes()) == method['decompilation_sha256']


def bind_3d_indirect_call(image: pefile.PE, address: int, module: bytes, symbol: bytes, slot: int) -> None:
    """Require the exact RIP-relative call instruction and named IAT identity; e.g. bind_3d_indirect_call(image, address, module, symbol, slot)."""
    raw = image.get_data(address - image.OPTIONAL_HEADER.ImageBase, 6)
    assert raw[:2] == b'\xff\x15', (hex(address), raw.hex(), 'expected indirect RIP-relative CALL')
    actual = address + 6 + struct.unpack_from('<i', raw, 2)[0]
    assert actual == slot, (hex(address), hex(actual), hex(slot))
    dialog_import(image, module, symbol, slot)


def verify_3d_color_calls(caller: pefile.PE, provider: pefile.PE) -> None:
    """Bind the packed argument constructor and named float getter without a stored-color alias; e.g. verify_3d_color_calls(caller, provider)."""
    original = colorref_import_reports()[0]
    assert digest(provider.__data__) == original['module_sha256']
    constructor = cast(dict[str, str], original['constructor'])
    verify_field_leaf(provider, constructor | dict(name=constructor['export']))
    bind_3d_indirect_call(caller, 0x1800e1ca6, b'DSPOGL.dll', constructor['export'].encode(), 0x1802cb318)
    getter = b'?GetColorfv@COGLColor@@QEAAXQEAM@Z'
    dialog_export(provider, getter, 0x180003cf0)
    bind_3d_indirect_call(caller, 0x1800e1cc0, b'DSPOGL.dll', getter, 0x1802cb510)


def verify_3d_gl_calls(image: pefile.PE) -> None:
    """Bind named lighting/RGB/primitive/shading calls without executing OpenGL; e.g. verify_3d_gl_calls(image)."""
    calls = ((0x1800e1e5e, b'glIsEnabled', 0x1802cbc88), (0x1800e1e6c, b'glDisable', 0x1802cbcb8),
        (0x1800e1e76, b'glColor3fv', 0x1802cbbb8), (0x1800e1e81, b'glBegin', 0x1802cbd00),
        (0x1800e1e8b, b'glNormal3fv', 0x1802cbb60), (0x1800e1e9f, b'glVertex3fv', 0x1802cbc48),
        (0x1800e1ec3, b'glEnd', 0x1802cbca8), (0x1800e1ed2, b'glEnable', 0x1802cbcb0),
        (0x1800e2099, b'glShadeModel', 0x1802cbb08), (0x1800e20a4, b'glBegin', 0x1802cbd00))
    for address, symbol, slot in calls:
        bind_3d_indirect_call(image, address, b'OPENGL32.dll', symbol, slot)


def ordered_3d_anchors(method: dict[str, object], anchors: list[str]) -> None:
    """Keep exact field reads, argument setup and helper calls in original order; e.g. ordered_3d_anchors(method, anchors)."""
    lines = cast(list[str], method['instructions'])
    positions = [lines.index(anchor) for anchor in anchors]
    assert positions == sorted(set(positions)), (anchors, positions)


def verify_3d_background_anchors(method: dict[str, object]) -> None:
    """Bind packed-stack color, precision+10 and conditional lighting restoration; e.g. verify_3d_background_anchors(method)."""
    ordered_3d_anchors(method, ['1800e1c93 call 0x1800db650', '1800e1c98 lea rdx, [rbp + 0xf8]',
        '1800e1ca2 lea rcx, [rbp - 0x70]', '1800e1ca6 call qword ptr [rip + 0x1e966c]',
        '1800e1cb4 lea rdx, [rbp - 0x10]', '1800e1cb8 lea rcx, [rbp - 0x70]',
        '1800e1cc0 call qword ptr [rip + 0x1e984a]', '1800e1d21 mov eax, dword ptr [rdi + 0xbc]',
        '1800e1d2f add eax, 0xa', '1800e1d69 movd xmm0, eax', '1800e1d6d cvtdq2ps xmm0, xmm0',
        '1800e1d74 mulss xmm0, xmm5', '1800e1e5e call qword ptr [rip + 0x1e9e24]',
        '1800e1e67 mov ecx, 0xb50', '1800e1e6c call qword ptr [rip + 0x1e9e46]',
        '1800e1e76 call qword ptr [rip + 0x1e9d3c]', '1800e1e7c mov ecx, 9', '1800e1ec9 test bl, bl',
        '1800e1ecb je 0x1800e1ed8', '1800e1ecd mov ecx, 0xb50', '1800e1ed2 call qword ptr [rip + 0x1e9dd8]'])
    lines = cast(list[str], method['instructions'])
    assert not any('[rdi + 0xec]' in line or '[rdi + 0x100]' in line for line in lines)


def verify_3d_body_anchors(method: dict[str, object]) -> None:
    """Bind count rereads, separate argument2 selection and later smooth reset; e.g. verify_3d_body_anchors(method)."""
    ordered_3d_anchors(method, ['1800e1fa4 mov eax, dword ptr [rcx + 0x14]',
        '1800e1fdd mov eax, dword ptr [r13 + 0x14]', '1800e1fe8 divss xmm2, xmm0',
        '1800e2015 addss xmm0, dword ptr [rbx + 4]', '1800e2028 addss xmm0, dword ptr [rbx]',
        '1800e2031 addss xmm1, dword ptr [rbx + 4]', '1800e2045 mov eax, dword ptr [r13 + 0x14]',
        '1800e204d cmp dword ptr [rbp + 0xe8], 2', '1800e2054 lea r14d, [rax + 1]',
        '1800e2058 cmovne r14d, eax', '1800e207d mov edx, r14d', '1800e2086 call 0x1800da1c0',
        '1800e208b cmp dword ptr [rbp + 0xe8], 2', '1800e2094 mov ecx, 0x1d01', '1800e209f mov ecx, 8',
        '1800e2101 mov ecx, 0x1d00', '1800e210c mov ecx, 7', '1800e22b9 mov ecx, 0x1d01'])


def verify_legend_3d_rea_capture() -> None:
    """Require all432 independent instruction locations and both full contiguous bodies; e.g. verify_legend_3d_rea_capture()."""
    module_hash = cast(str, legend_3d_report()['module_sha256'])
    captures = [legend_body_capture(FEATURE / f'legend-3d-rea-{name}.json', method, module_hash)
        for name, method in zip(('background', 'body'), legend_3d_methods(), strict=True)]
    dialog_baseline('legend-3d-rea-reference.json', dict(captures=captures,
        independent_instruction_count=432, complete_body_bytes=1831, target_executed=False))
