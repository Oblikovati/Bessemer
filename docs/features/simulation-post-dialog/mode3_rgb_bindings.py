"""C893 complete caller/array bodies, copied candidate and exact forwarding thunk (#166)."""
import json
import struct
from typing import cast
from native_carriers import digest
from dialog_bindings import dialog_images, dialog_import
from dialog_evidence import FEATURE, dialog_baseline
from mode3_rgb_evidence import mode3_rgb_report
from style_preferences_disassembly import canonical_style_address_operands, decoded_style_preference_method


def verify_mode3_address_identities() -> None:
    """Admit only arithmetic unit-scale spellings and preserve changed scale/offsets; e.g. verify_mode3_address_identities()."""
    normalize = canonical_style_address_operands
    assert normalize('mov', 'dword ptr [rcx+rdx*1-8],eax') == normalize('mov', 'dword ptr [rcx+rdx-8],eax')
    assert normalize('mov', 'dword ptr [rcx+rdx*2-8],eax') != normalize('mov', 'dword ptr [rcx+rdx-8],eax')
    assert normalize('mov', 'dword ptr [rcx+rdx*1-4],eax') != normalize('mov', 'dword ptr [rcx+rdx-8],eax')
    assert normalize('mov', 'dword ptr [rcx+rdx*10-8],eax') != normalize('mov', 'dword ptr [rcx+rdx-8],eax')


def mode3_candidate_anchors(methods: list[dict[str, object]]) -> None:
    """Pin exact mode3 substitution without conflating the separately retained alpha role; e.g. mode3_candidate_anchors(methods)."""
    lines = set(cast(list[str], methods[0]['instructions']))
    assert {'18015c2c3 mov dword ptr [rbp + 0x790], 0x3f000000', '18015c2cd mov dword ptr [rbp + 0x794], 0x3f000000',
        '18015c2d7 mov dword ptr [rbp + 0x798], 0x3f000000', '18015c2e1 mov ebx, dword ptr [rsi + 0x558]',
        '18015c2e7 lea rdx, [rsi + 0x57c]', '18015c2fd cmp ebx, 3', '18015c300 jne 0x18015c332',
        '18015c307 movss dword ptr [rbp + 0x790], xmm0', '18015c314 movss dword ptr [rbp + 0x794], xmm1',
        '18015c321 movss dword ptr [rbp + 0x798], xmm0', '18015c47f cmp qword ptr [rsp + 0x68], 0',
        '18015c485 jne 0x18015c491', '18015c487 cmp dword ptr [rsp + 0x58], 0x405', '18015c48f jne 0x18015c4ff',
        '18015c4b3 cmp dword ptr [rsp + 0x58], 0x404', '18015c4bb jne 0x18015c4d6',
        '18015c58f lea r9, [rbp + 0x790]', '18015c596 mov edx, 1', '18015c59b mov r8d, 5',
        '18015c5a8 call 0x18018ae40'} <= lines


def verify_mode3_rgb_bindings() -> None:
    """Match all504 complete instructions, exact copy import and five-byte jump; e.g. verify_mode3_rgb_bindings()."""
    report = mode3_rgb_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        verify_mode3_address_identities()
        methods = cast(list[dict[str, object]], report['functions'])
        assert [len(decoded_style_preference_method(image, method, normalize_address_spelling=True)) for method in methods] == [371, 133]
        mode3_candidate_anchors(methods)
        dialog_import(image, b'dspogl.dll', b'??0COGLColor@@QEAA@AEBV0@@Z', 0x1802cb310)
        copy_call = image.get_data(0x15c2f2, 6)
        assert copy_call[:2] == b'\xff\x15' and 0x18015c2f8 + struct.unpack_from('<i', copy_call, 2)[0] == 0x1802cb310
        assert report['thunk'] == dict(start='0x18018ae40', target='0x18018a870', raw_hex='e92bfaffff')
        assert image.get_data(0x18ae40, 5) == bytes.fromhex('e92bfaffff')
        dialog_baseline('mode3-rgb-installed-reference.json', dict(module_sha256=report['module_sha256'],
            complete_method_instruction_counts=[371, 133], thunk=report['thunk'], target_executed=False,
            count1_index5_offset_bytes=60, actual_array_update_observed=False, complete_rendering_bound=False))
    finally:
        for image in images.values(): image.close()



def verify_mode3_rgb_rea_capture() -> None:
    """Retain every independent instruction and the exact five-byte thunk body; e.g. verify_mode3_rgb_rea_capture()."""
    report = mode3_rgb_report()
    methods = cast(list[dict[str, object]], report['functions']) + [dict(start='0x18018ae40', end='0x18018ae45', instructions=['18018ae40 jmp 0x18018a870'])]
    captures: list[dict[str, object]] = []
    for name, method in zip(('caller', 'array', 'thunk'), methods, strict=True):
        path = FEATURE / f'mode3-rgb-rea-{name}.json'
        record = json.loads(path.read_bytes())
        assert record['subject']['digest']['sha256'] == report['module_sha256']
        assert record['provider'] == dict(id='ghidra', name='Ghidra', version='12.1.4')
        assert record['authority'] == 'shipped-artifact' and record['confidence'] == 'observed'
        assert record['raw_result']['value'] == record['normalized_result']
        lines = record['normalized_result']['instructions']
        assert [line.split(':', 1)[0] for line in lines] == ['0x' + line.split()[0] for line in cast(list[str], method['instructions'])]
        verify_mode3_rea_extent(name, record['normalized_result'], method)
        captures.append(dict(path=path.name, sha256=digest(path.read_bytes()), evidence_id=record['evidence_id'],
            instruction_count=len(lines), limitations=record['limitations']))
    dialog_baseline('mode3-rgb-rea-reference.json', dict(captures=captures, target_executed=False,
        independent_method_instructions=504, independent_thunk_instructions=1, original_binary_written=False))


def verify_mode3_rea_extent(name: str, normalized: dict[str, object], method: dict[str, object]) -> None:
    """Require contiguous complete body intervals and the exact target of the forwarding thunk; e.g. verify_mode3_rea_extent(name, normalized, method)."""
    procedure = cast(dict[str, object], normalized['procedure'])
    body = cast(dict[str, object], procedure['body'])
    start, end = int(cast(str, method['start']), 16), int(cast(str, method['end']), 16)
    assert body['available'] is True and body['contains_entry'] is True and body['non_contiguous'] is False
    assert body['ranges'] == [dict(start=hex(start), end=hex(end - 1))]
    assert body['total_bytes'] == body['span_bytes'] == end - start
    if name != 'thunk': return
    classification = cast(dict[str, object], procedure['classification'])
    assert classification['thunk'] is True and classification['thunk_target'] == '0x18018a870'
    assert normalized['instructions'] == ['0x18018ae40: JMP 0x18018a870']
