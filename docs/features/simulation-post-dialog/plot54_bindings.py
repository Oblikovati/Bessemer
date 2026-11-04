"""C891 complete installed caller/helper, exact copy/read imports and alpha branch (#166)."""
from typing import cast
import json
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images, dialog_import
from aux_triangle_bindings import aux_file_bytes
from style_preferences_disassembly import decoded_style_preference_method
from plot54_evidence import plot54_report


def plot54_caller_anchors(report: dict[str, object]) -> None:
    """Bind unsigned mode-first admission and the separately copied R8/R9 colors; e.g. plot54_caller_anchors(report)."""
    methods = cast(list[dict[str, object]], report['functions'])
    lines = set(cast(list[str], methods[0]['instructions']))
    assert {'18015956a mov eax, dword ptr [rcx + 0x2c68]', '180159573 sub eax, 3',
        '180159579 cmp eax, 1', '18015957c ja 0x1801595c9',
        '18015957e cmp dword ptr [rcx + 0x170], 0x36', '180159585 jne 0x1801595c9',
        '180159587 lea rdx, [rcx + 0x65c]', '180159598 call qword ptr [rip + 0x171d72]',
        '18015959e lea rdx, [rdi + 0x64c]', '1801595ad call qword ptr [rip + 0x171d5d]',
        '1801595b3 mov r9, rbx', '1801595b9 mov r8, rax', '1801595bf call 0x18015a820'} <= lines
    assert 0x18015959e + 0x171d72 == 0x1801595b3 + 0x171d5d == 0x1802cb310


def plot54_helper_anchors(report: dict[str, object]) -> None:
    """Bind both pre-read helper guards and preserve COMISS/JAE rather than the decompiler's ordered-only condition; e.g. plot54_helper_anchors(report)."""
    methods = cast(list[dict[str, object]], report['functions'])
    lines = set(cast(list[str], methods[1]['instructions']))
    assert {'18015a83d cmp qword ptr [rcx + 0x2520], 0', '18015a845 mov rbx, r8',
        '18015a853 je 0x18015ad70', '18015a859 mov rcx, qword ptr [rcx + 0x2c20]',
        '18015a865 mov edx, 0xa', '18015a86a mov qword ptr [rsp + 0x48], 0',
        '18015a873 call 0x1801ac860', '18015a878 cmp qword ptr [rsp + 0x48], 0',
        '18015a87e je 0x18015ad70', '18015a8b0 call qword ptr [r8 + 0x88]',
        '18015a8c1 movss xmm6, dword ptr [rip + 0x17d223]', '18015a8f1 call qword ptr [rax + 0x90]',
        '18015a8f7 lea rdx, [rdi + 0x20]', '18015a8fb mov rcx, rbx',
        '18015a8fe call qword ptr [rip + 0x17091c]', '18015a904 comiss xmm6, dword ptr [rdi + 0x2c]',
        '18015a908 jae 0x18015a911', '18015a90a mov dword ptr [rdi + 0x2c], 0x3f800000'} <= lines
    assert 0x18015a8c9 + 0x17d223 == 0x1802d7aec
    assert 0x18015a904 + 0x17091c == 0x1802cb220


def verify_plot54_bindings() -> None:
    """Require both complete native methods and all copied/read component bindings without geometry claims; e.g. verify_plot54_bindings()."""
    report = plot54_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        methods = cast(list[dict[str, object]], report['functions'])
        assert [len(decoded_style_preference_method(image, method, normalize_address_spelling=True)) for method in methods] == [29, 294]
        plot54_caller_anchors(report)
        plot54_helper_anchors(report)
        dialog_import(image, b'dspogl.dll', b'??0COGLColor@@QEAA@AEBV0@@Z', 0x1802cb310)
        dialog_import(image, b'dspogl.dll', b'?GetColor4fv@COGLColor@@QEAAXQEAM@Z', 0x1802cb220)
        assert report['color_read_imports'] == [dict(address='0x18015a8fe', iat='0x1802cb220', dll='DSPOGL.dll', symbol='?GetColor4fv@COGLColor@@QEAAXQEAM@Z')]
        assert aux_file_bytes(image, 0x1802d7aec, 4).hex() == '0000803f'
        dialog_baseline('plot54-installed-reference.json', dict(module_sha256=report['module_sha256'],
            complete_method_instruction_counts=[29, 294], alpha_constant_address='0x1802d7aec', alpha_constant_hex='0000803f',
            target_executed=False, geometry_callback_abi_bound=False, forwarded_2bc_purpose_bound=False))
    finally:
        for image in images.values(): image.close()


def verify_plot54_rea_capture() -> None:
    """Retain independent complete caller/helper locations and literal bytes with their provider limitations; e.g. verify_plot54_rea_capture()."""
    report = plot54_report()
    methods = cast(list[dict[str, object]], report['functions'])
    captures: list[dict[str, object]] = []
    for name, method in zip(('caller', 'helper'), methods, strict=True):
        path = FEATURE / f'plot54-rea-{name}.json'
        record = json.loads(path.read_bytes())
        assert record['subject']['digest']['sha256'] == report['module_sha256']
        assert record['provider'] == dict(id='ghidra', name='Ghidra', version='12.1.4')
        assert record['authority'] == 'shipped-artifact' and record['confidence'] == 'observed'
        assembly = cast(str, record['normalized_result'])
        assert record['raw_result']['value'] == assembly
        assert [line.split(':', 1)[0] for line in assembly.splitlines()] == [
            '0x' + line.split()[0] for line in cast(list[str], method['instructions'])]
        captures.append(dict(path=path.name, sha256=digest(path.read_bytes()), evidence_id=record['evidence_id'],
            instruction_count=len(assembly.splitlines()), limitations=record['limitations']))
    constant = plot54_rea_constant(report)
    dialog_baseline('plot54-rea-reference.json', dict(captures=captures, constant=constant,
        target_executed=False, original_binary_written=False))


def plot54_rea_constant(report: dict[str, object]) -> dict[str, object]:
    """Keep exact literal bytes without treating loaded-memory evidence as raw-file evidence alone; e.g. plot54_rea_constant(report)."""
    path = FEATURE / 'plot54-rea-constant.json'
    record = json.loads(path.read_bytes())
    assert record['normalized_result'] == dict(address='0x1802d7aec', requested_bytes=4, returned_bytes=4, bytes_hex='0000803f', complete=True)
    assert record['subject']['digest']['sha256'] == report['module_sha256']
    assert record['provider'] == dict(id='ghidra', name='Ghidra', version='12.1.4')
    assert record['authority'] == 'shipped-artifact' and record['confidence'] == 'observed'
    return dict(path=path.name, sha256=digest(path.read_bytes()), evidence_id=record['evidence_id'],
        bytes_hex='0000803f', limitations=record['limitations'])
