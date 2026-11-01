"""C858 complete converter/caller bodies and conditional grouping boundaries (#166)."""
from typing import cast
import json
import pefile
from native_carriers import RESEARCH, digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images, dialog_export
from style_preferences_disassembly import decoded_style_preference_method
from legend_body_evidence import legend_body_capture
from legend_grouping_evidence import legend_grouping_report
from legend_grouping_decoder import grouping_converter_gnu_spelling


def grouping_methods() -> list[dict[str, object]]:
    """Retain both exact original complete probe intervals; e.g. grouping_methods()."""
    probes = {row['address']: row for name in ('cosworks-number-format-convert', 'cosworks-post-legend-2d')
        for row in json.loads((RESEARCH / (name + '-probe.json')).read_bytes())}
    methods: list[dict[str, object]] = []
    for row in cast(list[dict[str, object]], legend_grouping_report()['functions']):
        probe = probes[row['address']]; assert row['end'] == probe['end']
        methods.append(row | dict(start=row['address'], instructions=probe['instructions']))
    return methods


def verify_grouping_bindings() -> None:
    """Require1814 complete instructions, exact exports and conditional consumer gates; e.g. verify_grouping_bindings()."""
    images = dialog_images()
    try:
        image = images['dgxpost.dll']; report = legend_grouping_report()
        assert digest(image.__data__) == report['module_sha256']
        methods = grouping_methods()
        decoded_methods = [grouping_converter_gnu_spelling(image, methods[0]), methods[1]]
        counts = [len(decoded_style_preference_method(image, method, normalize_address_spelling=True)) for method in decoded_methods]
        assert counts == [176, 1638], counts
        verify_grouping_symbols(image, methods)
        verify_grouping_converter(methods[0])
        verify_grouping_calls(methods[1])
        dialog_baseline('legend-grouping-installed-reference.json', dict(complete_instruction_counts=counts,
            complete_body_bytes=7499, exact_decoder_qualifications=5, converter_call_sites=3,
            target_executed=False, locale_characters_inferred=False, full_locale_renderer_emulated=False))
    finally:
        for image in images.values(): image.close()


def verify_grouping_symbols(image: pefile.PE, methods: list[dict[str, object]]) -> None:
    """Bind exact named exports, locale providers and both decompilation hashes; e.g. verify_grouping_symbols(image, methods)."""
    for folder, method in zip(('cosworks-number-format-convert', 'cosworks-post-legend-2d'), methods, strict=True):
        address = int(cast(str, method['start']), 16)
        dialog_export(image, cast(str, method['export']).encode(), address)
        path = RESEARCH / 'decomp' / folder / f'FUN_{address:x}.c'
        assert digest(path.read_bytes()) == method['decompilation_sha256']
    dialog_export(image, b'?GetlocaleDecimalChar@CDspApp@@SA_WXZ', 0x1800caa70)
    dialog_export(image, b'?GetLocaleThousandSeparatorChar@CDspApp@@SA_WXZ', 0x1800ca230)


def grouping_anchors(method: dict[str, object], expected: list[str]) -> None:
    """Require each exact instruction in native order; e.g. grouping_anchors(method, anchors)."""
    lines = cast(list[str], method['instructions'])
    positions = [lines.index(anchor) for anchor in expected]
    assert positions == sorted(set(positions)), (expected, positions)


def verify_grouping_converter(method: dict[str, object]) -> None:
    """Bind independent decimal work, nonzero control, exponent bypass and three-position insertion; e.g. verify_grouping_converter(method)."""
    grouping_anchors(method, ['1800c8320 test rcx, rcx', '1800c8323 je 0x1800c8591',
        '1800c8340 mov r13d, edx', '1800c8381 mov edx, 0x2e',
        '1800c8395 call 0x1800caa70', '1800c839e call 0x1800ca230',
        '1800c83d8 cmp r12w, 0x2e', '1800c83dd je 0x1800c83f2',
        '1800c8442 test esi, esi', '1800c8444 jle 0x1800c852f',
        '1800c844a cmp ebx, 3', '1800c8452 jne 0x1800c852f',
        '1800c847c cmp ax, 0x20', '1800c848e test r13d, r13d', '1800c8491 je 0x1800c8509',
        '1800c8493 mov edx, 0x65', '1800c84a8 jne 0x1800c8509',
        '1800c84aa mov edx, 0x45', '1800c84bf jne 0x1800c8509',
        '1800c84c4 cmp r12w, 0x2d', '1800c84cc cmp ecx, 3', '1800c84cf jle 0x1800c8509',
        '1800c84d1 lea ebx, [rsi - 3]', '1800c84fa call qword ptr [rip + 0x2047e0]',
        '1800c8500 sub ebx, 3', '1800c8507 jne 0x1800c84f0'])


def verify_grouping_calls(method: dict[str, object]) -> None:
    """Bind all three C0 argument loads and converter calls across renderer branches; e.g. verify_grouping_calls(method)."""
    grouping_anchors(method, ['1800e0a74 mov edx, dword ptr [r12 + 0xc0]', '1800e0a80 call 0x1800c8320',
        '1800e1857 mov edx, dword ptr [rsi + 0xc0]', '1800e1861 call 0x1800c8320',
        '1800e19f5 mov edx, dword ptr [rsi + 0xc0]', '1800e19ff call 0x1800c8320'])
    calls = [line for line in cast(list[str], method['instructions']) if line.endswith('call 0x1800c8320')]
    assert len(calls) == 3, calls


def verify_grouping_rea() -> None:
    """Require independent Ghidra locations and both complete contiguous bodies; e.g. verify_grouping_rea()."""
    module_hash = cast(str, legend_grouping_report()['module_sha256'])
    captures = [legend_body_capture(FEATURE / f'legend-grouping-rea-{name}.json', method, module_hash)
        for name, method in zip(('converter', 'caller'), grouping_methods(), strict=True)]
    dialog_baseline('legend-grouping-rea-reference.json', dict(captures=captures,
        independent_instruction_count=1814, complete_body_bytes=7499, target_executed=False))
