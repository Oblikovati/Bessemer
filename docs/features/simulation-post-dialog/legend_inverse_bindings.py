"""C860 complete getter/setter bodies, zero assignments and bounded plot-type bypass (#166)."""
from typing import cast
import json
import pefile
from native_carriers import RESEARCH, digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images, dialog_export
from style_preferences_disassembly import decoded_style_preference_method
from legend_body_evidence import legend_body_capture
from legend_inverse_evidence import legend_inverse_report


def inverse_methods() -> list[dict[str, object]]:
    """Bind complete original probe ranges to both C860 function pins; e.g. inverse_methods()."""
    probes = {row['address']: row for row in json.loads((RESEARCH / 'cosworks-post-legend-preferences-probe.json').read_bytes())}
    methods: list[dict[str, object]] = []
    for row in cast(list[dict[str, object]], legend_inverse_report()['functions']):
        probe = probes[row['address']]; assert row['end'] == probe['end']
        methods.append(row | dict(start=row['address'], instructions=probe['instructions']))
    return methods


def verify_inverse_bindings() -> None:
    """Require all122 instructions and exact getter/setter/provider identities without execution; e.g. verify_inverse_bindings()."""
    images = dialog_images()
    try:
        image = images['dgxpost.dll']; report = legend_inverse_report()
        assert digest(image.__data__) == report['module_sha256']
        methods = inverse_methods()
        counts = [len(decoded_style_preference_method(image, method, normalize_address_spelling=True)) for method in methods]
        assert counts == [31, 91], counts
        verify_inverse_symbols(image, methods)
        verify_inverse_getter(methods[0])
        verify_inverse_setter(methods[1])
        dialog_baseline('legend-inverse-installed-reference.json', dict(complete_instruction_counts=counts,
            complete_body_bytes=486, exact_decoder_qualifications=0, target_executed=False,
            plot_type_bypass=[22, 57, 73], whole_preference_methods_emulated=False))
    finally:
        for image in images.values(): image.close()


def verify_inverse_symbols(image: pefile.PE, methods: list[dict[str, object]]) -> None:
    """Require exact named exports and both original decompilation hashes; e.g. verify_inverse_symbols(image, methods)."""
    for method in methods:
        address = int(cast(str, method['start']), 16)
        dialog_export(image, cast(str, method['export']).encode(), address)
        path = RESEARCH / 'decomp/cosworks-post-legend-preferences' / f'FUN_{address:x}.c'
        assert digest(path.read_bytes()) == method['decompilation_sha256']


def inverse_anchors(method: dict[str, object], expected: list[str]) -> None:
    """Require each original instruction in its exact native order; e.g. inverse_anchors(method, anchors)."""
    lines = cast(list[str], method['instructions'])
    positions = [lines.index(line) for line in expected]
    assert positions == sorted(set(positions)), (expected, positions)


def verify_inverse_getter(method: dict[str, object]) -> None:
    """Bind full zero-extension and A8-to-preference destination0 zero testing; e.g. verify_inverse_getter(method)."""
    inverse_anchors(method, ['1800dbbcf xor eax, eax', '1800dbbd5 cmp dword ptr [rcx + 0xa8], eax',
        '1800dbbe4 sete al', '1800dbbe7 mov dword ptr [rdx], eax'])


def verify_inverse_setter(method: dict[str, object]) -> None:
    """Bind nullable plot context, exact bounded mask and both reached zero-test stores; e.g. verify_inverse_setter(method)."""
    inverse_anchors(method, ['1800e3a9c mov rax, qword ptr [rcx + 0x1a8]', '1800e3aad test rax, rax',
        '1800e3ab0 je 0x1800e3b62', '1800e3ab6 mov edx, dword ptr [rcx]',
        '1800e3ab8 lea eax, [rdx - 0x16]', '1800e3abb cmp eax, 0x33',
        '1800e3abe ja 0x1800e3ad4', '1800e3ac0 movabs r8, 0x8000800000001',
        '1800e3aca bt r8, rax', '1800e3ace jb 0x1800e3bff',
        '1800e3ad4 cmp edx, 0x46', '1800e3ad7 jne 0x1800e3b62',
        '1800e3add xor ebx, ebx', '1800e3ae3 cmp dword ptr [rbp + 0x1528], ebx',
        '1800e3ae9 mov eax, ebx', '1800e3af2 sete al', '1800e3afc mov dword ptr [rdi + 0xa8], eax',
        '1800e3b62 xor ebx, ebx', '1800e3b69 cmp dword ptr [rbp + 0x1528], ebx',
        '1800e3b6f sete bl', '1800e3b72 mov dword ptr [rdi + 0xa8], ebx'])
    offsets = [index for index in range(52) if (0x8000800000001 >> index) & 1]
    assert [index + 22 for index in offsets] == [22, 57, 73]


def verify_inverse_rea() -> None:
    """Require all122 independent Ghidra locations and both complete contiguous bodies; e.g. verify_inverse_rea()."""
    module_hash = cast(str, legend_inverse_report()['module_sha256'])
    captures = [legend_body_capture(FEATURE / f'legend-inverse-rea-{name}.json', method, module_hash)
        for name, method in zip(('getter', 'setter'), inverse_methods(), strict=True)]
    dialog_baseline('legend-inverse-rea-reference.json', dict(captures=captures,
        independent_instruction_count=122, complete_body_bytes=486, target_executed=False))
