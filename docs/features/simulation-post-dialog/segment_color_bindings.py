"""C892 exact installed bodies and original RGB argument/import chains (#166)."""
import json
import re
import struct
from collections.abc import Callable
from typing import cast
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images, dialog_import
from segment_color_qualifications import CAPTURED_SEGMENT_TEXT, segment_color_decoded_methods, verify_segment_qualification_boundaries
from style_preferences_disassembly import canonical_style_address_operands
from segment_color_evidence import segment_color_report


def segment_color_argument_anchors(methods: list[dict[str, object]]) -> None:
    """Bind stored1BC to the same twelve-byte local and both helper argument registers; e.g. segment_color_argument_anchors(methods)."""
    caller = set(cast(list[str], methods[0]['instructions']))
    assert {'18015e889 mov eax, dword ptr [rcx + 0x2c68]', '18015e88f sub eax, 3',
        '18015e892 cmp eax, 1', '18015e895 ja 0x18015ec8e',
        '18015e924 lea rcx, [r15 + 0x55c]', '18015e92b lea rdx, [rbp + 0x7c0]',
        '18015ec43 lea r9, [rbp + 0x7c0]', '18015ec50 call 0x1800bed10',
        '18015ec5d lea r8, [rbp + 0x7c0]', '18015ec68 call 0x1800beaa0'} <= caller
    first = set(cast(list[str], methods[1]['instructions']))
    second = set(cast(list[str], methods[2]['instructions']))
    assert {'1800beac2 mov r15, r8', '1800bec58 mov rcx, r15',
        '1800bec5b call qword ptr [rip + 0x20cf57]'} <= first
    assert {'1800bed45 mov rsi, r9', '1800beeb7 mov rcx, rsi',
        '1800beeba call qword ptr [rip + 0x20ccf8]', '1800bef1a mov rcx, rsi',
        '1800bef1d call qword ptr [rip + 0x20cc95]'} <= second


def verify_segment_color_bindings() -> None:
    """Match all556 complete instructions, exact RGB imports and every retained call slot; e.g. verify_segment_color_bindings()."""
    report = segment_color_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        methods = cast(list[dict[str, object]], report['functions'])
        verify_segment_qualification_boundaries()
        assert [len(method) for method in segment_color_decoded_methods(image, methods)] == [275, 136, 145]
        segment_color_argument_anchors(methods)
        segment_rgb_guard_anchors(methods)
        dialog_import(image, b'dspogl.dll', b'?GetColor3fv@COGLColor@@QEAAXQEAM@Z', 0x1802cb438)
        dialog_import(image, b'opengl32.dll', b'glColor3fv', 0x1802cbbb8)
        segment_color_call_slots(report, image.get_data)
        dialog_baseline('segment-color-installed-reference.json', dict(module_sha256=report['module_sha256'],
            complete_method_instruction_counts=[275, 136, 145], color_read_components=3,
            color_read_import_calls=1, primitive_color_import_calls=3, target_executed=False,
            alpha_purpose_bound=False, geometry_validated=False, actual_invocation_observed=False))
    finally:
        for image in images.values(): image.close()


def segment_color_call_slots(report: dict[str, object], read_bytes: Callable[[int, int], bytes]) -> None:
    """Validate every original RIP slot rather than treating a symbol elsewhere as its binding; e.g. segment_color_call_slots(report, image.get_data)."""
    calls = cast(list[dict[str, str]], report['import_bindings'])
    assert [(row['address'], row['iat']) for row in calls] == [('0x18015e932', '0x1802cb438'),
        ('0x1800bec5b', '0x1802cbbb8'), ('0x1800beeba', '0x1802cbbb8'), ('0x1800bef1d', '0x1802cbbb8')]
    for row in calls:
        address = int(row['address'], 16)
        raw = read_bytes(address - 0x180000000, 6)
        assert raw[:2] == b'\xff\x15'
        assert address + 6 + struct.unpack_from('<i', raw, 2)[0] == int(row['iat'], 16)



def verify_segment_color_rea_capture() -> None:
    """Keep all555 analyzed locations and the explicit terminal-call padding exclusion; e.g. verify_segment_color_rea_capture()."""
    report = segment_color_report()
    methods = cast(list[dict[str, object]], report['functions'])
    captures: list[dict[str, object]] = []
    for name, method in zip(('caller', 'first', 'second'), methods, strict=True):
        path = FEATURE / f'segment-color-rea-{name}.json'
        record = json.loads(path.read_bytes())
        assert record['subject']['digest']['sha256'] == report['module_sha256']
        assert record['provider'] == dict(id='ghidra', name='Ghidra', version='12.1.4')
        assert record['authority'] == 'shipped-artifact' and record['confidence'] == 'observed'
        assert record['raw_result']['value'] == record['normalized_result']
        lines = record['normalized_result']['instructions']
        expected = ['0x' + line.split()[0] for line in cast(list[str], method['instructions'])]
        assert [line.split(':', 1)[0] for line in lines] == (expected[:-1] if name == 'caller' else expected)
        segment_color_rea_boundaries(name, record, lines)
        captures.append(dict(path=path.name, sha256=digest(path.read_bytes()), evidence_id=record['evidence_id'],
            instruction_count=len(lines), limitations=record['limitations']))
    dialog_baseline('segment-color-rea-reference.json', dict(captures=captures, target_executed=False,
        independently_analyzed_instructions=555, full_gnu_instructions=556, caller_padding_nop_excluded='0x18015ece3'))


def segment_color_rea_boundaries(name: str, record: dict[str, object], lines: list[str]) -> None:
    """Retain the provider's no-return assumption and independently confirm six scalar qualifiers; e.g. segment_color_rea_boundaries(name, record, lines)."""
    normalized = cast(dict[str, object], record['normalized_result'])
    procedure = cast(dict[str, object], normalized['procedure'])
    body = cast(dict[str, object], procedure['body'])
    assert body['available'] is True and body['contains_entry'] is True and body['non_contiguous'] is False
    if name == 'caller':
        assert body['ranges'] == [dict(start='0x18015e830', end='0x18015ece2')]
        assert any('terminal call at 0x18015ecde' in value for value in cast(list[str], record['limitations']))
        return
    by_address = {int(line.split(':', 1)[0], 16): line.split(': ', 1)[1] for line in lines}
    for address, (_, canonical, _, _) in CAPTURED_SEGMENT_TEXT.items():
        if address not in by_address: continue
        verify_segment_scalar_capture(address, canonical, by_address[address])


def verify_segment_scalar_capture(address: int, canonical: str, actual: str) -> None:
    """Resolve the exact captured RIP operand before comparing provider text; e.g. verify_segment_scalar_capture(address, canonical, actual)."""
    _, mnemonic, operands = canonical.split(' ', 2)
    actual_mnemonic, actual_operands = actual.split(' ', 1)
    assert actual_mnemonic.lower() == mnemonic
    expected = canonical_style_address_operands(mnemonic, operands)
    displacement = re.search(r'rip\+(\d+)', expected)
    assert displacement is not None
    resolved = expected.replace(displacement[0], str(address + 8 + int(displacement[1])))
    assert canonical_style_address_operands(mnemonic, actual_operands) == resolved



def segment_rgb_guard_anchors(methods: list[dict[str, object]]) -> None:
    """Pin reached signed index and raw flag guards in exact instruction order; e.g. segment_rgb_guard_anchors(methods)."""
    expected = ('18015e889 mov eax, dword ptr [rcx + 0x2c68]', '18015e88f sub eax, 3',
        '18015e892 cmp eax, 1', '18015e895 ja 0x18015ec8e',
        '18015e89b mov eax, dword ptr [rcx + 0x274c]', '18015e8a1 test eax, eax', '18015e8a3 jle 0x18015ec8e',
        '18015e8a9 mov ecx, dword ptr [rcx + 0x2750]', '18015e8af test ecx, ecx', '18015e8b1 jle 0x18015ec8e',
        '18015e8b7 cmp ecx, eax', '18015e8b9 jl 0x18015ec8e', '18015e8bf mov eax, dword ptr [r15 + 0xda4]',
        '18015e8c6 test eax, eax', '18015e8c8 je 0x18015e8d2', '18015e8ca test al, 4', '18015e8cc je 0x18015ec8e')
    assert tuple(cast(list[str], methods[0]['instructions'])[21:38]) == expected
