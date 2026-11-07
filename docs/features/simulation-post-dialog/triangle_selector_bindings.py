"""Complete C-889 installed callers/helper and exact color/triangle boundaries (#166)."""
from typing import cast
import json
import pefile
from native_carriers import digest
from dialog_bindings import dialog_images, dialog_export, dialog_import
from dialog_evidence import FEATURE, dialog_baseline
from aux_triangle_bindings import aux_file_bytes, verify_aux_triangle_vtables
from aux_triangle_evidence import aux_triangle_report
from style_preferences_disassembly import decoded_style_preference_method
from triangle_selector_evidence import triangle_selector_report


def verify_triangle_selector_callers(report: dict[str, object]) -> None:
    """Pin the actual branches and complete argument chains, not a guessed UI policy; e.g. verify_triangle_selector_callers(report)."""
    methods = cast(list[dict[str, object]], report['functions'])
    hidden = cast(list[str], methods[0]['instructions'])
    model = cast(list[str], methods[2]['instructions'])
    assert {'180100e75 add rcx, 0x3a0', '180100e7f call 0x180044540',
        '180100e84 test eax, eax', '180100e86 je 0x180100fbc',
        '180100e91 mov esi, dword ptr [rbx + 0x590]', '180100e97 cmp esi, 1',
        '180100e9a ja 0x180100fb7', '180100ea0 cmp dword ptr [rbx + 0x678], 0',
        '180100ea7 je 0x180100f85', '180100f85 lea rdx, [rbx + 0x594]',
        '180100f91 call qword ptr [rip + 0x1ca379]', '180100f97 xor r9d, r9d',
        '180100f9a mov r8d, 1', '180100fa0 cmp esi, 1',
        '180100fae sete r9b', '180100fb2 call 0x1801578e0'} <= set(hidden)
    assert {'180100ff8 cmp eax, 5', '180100ffb je 0x18010115a',
        '180101004 call 0x1800ff240', '180101009 test eax, eax', '18010100b jne 0x18010115a',
        '180101019 cmp dword ptr [rdi + 0x5bc], eax', '18010101f je 0x18010105a',
        '180101023 lea rdx, [rdi + 0x5c0]', '18010102a cmp dword ptr [rdi + 0x5d0], 1',
        '180101036 sete bl', '180101039 call qword ptr [rip + 0x1ca2d1]',
        '18010103f mov r9d, ebx', '180101042 xor r8d, r8d', '180101050 call 0x1801578e0'} <= set(model)


def verify_triangle_selector_helper(report: dict[str, object]) -> None:
    """Retain signed iteration, group-class/lookup filters, RGB origins and opaque forwarding; e.g. verify_triangle_selector_helper(report)."""
    methods = cast(list[dict[str, object]], report['functions'])
    helper = set(cast(list[str], methods[1]['instructions']))
    assert {'1801578ff mov rax, qword ptr [rcx + 0x2520]', '180157906 mov r12d, r9d',
        '180157914 mov dword ptr [rsp + 0x34], r8d', '18015791f je 0x180157ae3',
        '180157929 movsxd rbx, dword ptr [rax + 0x40c]', '180157934 mov rbp, qword ptr [rax + 0x410]',
        '18015793b test ebx, ebx', '18015793d je 0x180157ad3', '180157946 je 0x180157ad3',
        '18015795b mov r15d, dword ptr [rax + 0x8c8]', '180157981 minss xmm0, dword ptr [rip + 0x180163]',
        '1801579f1 jle 0x180157a9e', '1801579f7 lea rbx, [rbp + 8]',
        '180157a00 cmp dword ptr [rsp + 0x34], 0', '180157a07 je 0x180157a0f',
        '180157a0b je 0x180157a8b', '180157a11 jne 0x180157a8b',
        '180157a1f mov r8d, dword ptr [rbx + 0x38]', '180157a31 call 0x1800aafc0',
        '180157a3d cmp rax, qword ptr [rcx + 0x8d8]', '180157a44 jne 0x180157a8b',
        '180157a46 cmp dword ptr [rbx + 0x24], 0', '180157a4a jle 0x180157a8b',
        '180157a4c test r12d, r12d', '180157a58 imul rcx, rax, 0xb8',
        '180157a64 add rcx, 0x1c', '180157a6a add rcx, 0xc',
        '180157a71 call qword ptr [rip + 0x1739c1]', '180157a7d mov r9, qword ptr [rbx + 0x30]',
        '180157a81 mov r8, qword ptr [rbx + 0x28]', '180157a85 mov edx, dword ptr [rbx + 0x24]',
        '180157a88 call qword ptr [rax + 0x58]', '180157a8d add rbx, 0xb8',
        '180157a94 sub r14, 1', '180157a98 jne 0x180157a00'} <= helper


def verify_triangle_selector_imports(image: pefile.PE) -> None:
    """Bind copied RGBA and three-component-only group replacements by exact imported names; e.g. verify_triangle_selector_imports(image)."""
    dialog_import(image, b'dspogl.dll', b'??0COGLColor@@QEAA@AEBV0@@Z', 0x1802cb310)
    dialog_import(image, b'dspogl.dll', b'?GetColor4fv@COGLColor@@QEAAXQEAM@Z', 0x1802cb220)
    dialog_import(image, b'dspogl.dll', b'?GetColor3fv@COGLColor@@QEAAXQEAM@Z', 0x1802cb438)
    assert 0x180100f97 + 0x1ca379 == 0x18010103f + 0x1ca2d1 == 0x1802cb310
    assert 0x180157968 + 0x1738b8 == 0x180157976 + 0x1738aa == 0x1802cb220
    assert 0x180157a77 + 0x1739c1 == 0x1802cb438


def verify_triangle_selector_bindings() -> None:
    """Require all three complete native bodies, named boundaries and the file-backed alpha constant; e.g. verify_triangle_selector_bindings()."""
    report = triangle_selector_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        methods = cast(list[dict[str, object]], report['functions'])
        counts = [len(decoded_style_preference_method(image, method, normalize_address_spelling=True)) for method in methods]
        assert counts == [81, 132, 92]
        accessor = dialog_export(image, b'?HiddenPartsOptionOn@CDspPostStyle@@', 0x180044540)
        assert aux_file_bytes(image, 0x180157989 + 0x180163, 4).hex() == '0000803f'
        verify_aux_triangle_vtables(image, aux_triangle_report())
        verify_triangle_selector_imports(image)
        verify_triangle_selector_callers(report)
        verify_triangle_selector_helper(report)
        dialog_baseline('triangle-selector-installed-reference.json', dict(module_sha256=report['module_sha256'],
            complete_method_instruction_counts=counts, hidden_accessor=accessor.decode(),
            alpha_constant_address='0x1802d7aec', alpha_constant_hex='0000803f', target_executed=False))
    finally:
        for image in images.values(): image.close()


def verify_triangle_selector_rea_capture() -> None:
    """Keep independent Ghidra captures, exact instruction locations and stated limitations; e.g. verify_triangle_selector_rea_capture()."""
    report = triangle_selector_report()
    methods = cast(list[dict[str, object]], report['functions'])
    captures: list[dict[str, object]] = []
    for name, method in zip(('hidden', 'helper', 'model'), methods, strict=True):
        path = FEATURE / f'triangle-selector-rea-{name}.json'
        record = json.loads(path.read_bytes())
        assert record['subject']['digest']['sha256'] == report['module_sha256']
        assert record['provider'] == dict(id='ghidra', name='Ghidra', version='12.1.4')
        assert record['authority'] == 'shipped-artifact' and record['confidence'] == 'observed'
        assert record['raw_result']['value'] == record['normalized_result']
        assembly = cast(str, record['normalized_result']).splitlines()
        addresses = [line.split(':', 1)[0] for line in assembly]
        assert addresses == ['0x' + line.split()[0] for line in cast(list[str], method['instructions'])]
        captures.append(dict(path=path.name, sha256=digest(path.read_bytes()), evidence_id=record['evidence_id'],
            instruction_count=len(assembly), limitations=record['limitations']))
    constant = triangle_selector_rea_constant(report)
    dialog_baseline('triangle-selector-rea-reference.json', dict(captures=captures,
        constant=constant, target_executed=False, original_binary_written=False))


def triangle_selector_rea_constant(report: dict[str, object]) -> dict[str, object]:
    """Retain exact constant bytes and the independent decoder's full provenance; e.g. triangle_selector_rea_constant(report)."""
    path = FEATURE / 'triangle-selector-rea-constant.json'
    record = json.loads(path.read_bytes())
    assert record['normalized_result'] == dict(address='0x1802d7aec', requested_bytes=4, returned_bytes=4, bytes_hex='0000803f', complete=True)
    assert record['subject']['digest']['sha256'] == report['module_sha256']
    assert record['authority'] == 'shipped-artifact' and record['confidence'] == 'observed'
    assert record['provider'] == dict(id='ghidra', name='Ghidra', version='12.1.4')
    return dict(path=path.name, sha256=digest(path.read_bytes()), evidence_id=record['evidence_id'],
        limitations=record['limitations'], bytes_hex='0000803f')
