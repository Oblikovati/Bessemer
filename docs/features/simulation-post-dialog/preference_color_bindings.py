"""Complete C905/C907 callers, exact provider leaves and all preference routes (#166)."""
from typing import cast
import json
import re
import struct
import pefile
from native_carriers import digest
from dialog_bindings import dialog_images, dialog_export, dialog_import
from dialog_evidence import FEATURE, dialog_baseline
from dialog_field_bindings import verify_field_leaf
from style_preferences_disassembly import StylePreferenceInstruction, decoded_style_preference_method
from style_preferences_bindings import style_transfer_pairs
from preference_color_evidence import preference_color_reports, PREFERENCE_COLOR_ROUTES
from colorref_import_evidence import colorref_import_reports, verify_assignment_leaf


def preference_color_method(image: pefile.PE, probe: dict[str, object]) -> dict[str, object]:
    """Select the complete original interval only after whole-image pinning; e.g. preference_color_method(image, probe)."""
    start, end = int(cast(str, probe['address']), 16), int(cast(str, probe['end']), 16)
    raw = image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)
    return dict(start=hex(start), end=hex(end), sha256=digest(raw), instructions=probe['instructions'])


def preference_color_calls(instructions: list[StylePreferenceInstruction]) -> dict[int, int]:
    """Track original LEA argument chains into every packed-assignment slot; e.g. preference_color_calls(instructions)."""
    source: int | None = None
    target: int | None = None
    calls: dict[int, int] = {}
    for instruction in instructions:
        operands = instruction.operands
        if instruction.mnemonic == 'lea' and len(operands) == 2:
            incoming = re.fullmatch(r'\[rdi\+(\d+)\]', operands[1])
            outgoing = re.fullmatch(r'\[rbx\+(\d+)\]', operands[1])
            if operands[0] == 'rdx' and incoming is not None: source = int(incoming[1])
            if operands[0] == 'rcx' and outgoing is not None: target = int(outgoing[1])
        if instruction.mnemonic != 'call' or not operands[0].startswith('qwordptr[rip+'): continue
        assert source is not None and target is not None
        assert instruction.raw[:2] == b'\xff\x15'
        assert instruction.address + 6 + struct.unpack_from('<i', instruction.raw, 2)[0] == 0x1802cb378
        calls[target] = source
        source, target = None, None
    return calls


def verify_preference_color_guard(instructions: list[StylePreferenceInstruction]) -> None:
    """Retain the legend call before nullable-pointer/type73 skipping; e.g. verify_preference_color_guard(instructions)."""
    actual = {instruction.address: (instruction.mnemonic, instruction.operands) for instruction in instructions}
    assert actual[0x180045060] == ('call', (str(0x1800e3a90),))
    assert actual[0x180045065] == ('mov', ('rax', 'qwordptr[rbx+720]'))
    assert actual[0x18004506c] == ('test', ('rax', 'rax'))
    assert actual[0x18004506f] == ('je', (str(0x18004507b),))
    assert actual[0x180045071] == ('cmp', ('dwordptr[rax+16]', '73'))
    assert actual[0x180045075] == ('je', (str(0x1800451b7),))


def verify_preference_color_routes(instructions: list[StylePreferenceInstruction]) -> None:
    """Bind all eight packed arguments and exactly five raw alpha copy pairs; e.g. verify_preference_color_routes(instructions)."""
    calls = preference_color_calls(instructions)
    assert calls == {member: packed for member, packed, _ in PREFERENCE_COLOR_ROUTES}
    copies = style_transfer_pairs(instructions, 'rdi', 'rbx')
    for member, _, alpha in PREFERENCE_COLOR_ROUTES:
        if alpha is None:
            assert member + 12 not in copies
            continue
        assert copies[member + 12]['source'] == alpha
    dialog_baseline('preference-color-route-reference.json', dict(packed_calls={hex(key): hex(value) for key, value in calls.items()},
        alpha_copies={hex(member): copies[member + 12] for member, _, alpha in PREFERENCE_COLOR_ROUTES if alpha is not None},
        legend_call_precedes_guard=True, skipped_block_does_not_skip_legend_call=True))


def verify_preference_color_providers(image: pefile.PE, legend: dict[str, object]) -> None:
    """Retain the complete RGB constructor and all-four-word assignment exports; e.g. verify_preference_color_providers(image, legend)."""
    assert digest(image.__data__) == legend['provider_sha256']
    names = ('??0COGLColor@@QEAA@MMM@Z', '??4COGLColor@@QEAAAEAV0@AEBV0@@Z')
    for leaf, name in zip(cast(list[dict[str, str]], legend['color_leaves']), names, strict=True):
        raw = bytes.fromhex(leaf['raw_hex'])
        qualified = leaf | dict(end=hex(int(leaf['address'], 16) + len(raw)), sha256=digest(raw), name=name)
        verify_field_leaf(image, qualified)
    assignment = colorref_import_reports()[1]
    assert assignment['provider_sha256'] == legend['provider_sha256']
    verify_assignment_leaf(image, assignment)
    assert image.get_data(0x11870, 4).hex() == assignment['scale_raw_hex'] == '8180803b'


def verify_preference_color_bindings() -> None:
    """Verify all194 DGXPOST instructions and exact provider/import chains without target execution; e.g. verify_preference_color_bindings()."""
    legend, preferences = preference_color_reports()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == legend['module_sha256'] == preferences['module_sha256']
        methods = [preference_color_method(image, cast(dict[str, object], report['function'])) for report in (legend, preferences)]
        decoded = [decoded_style_preference_method(image, method, normalize_address_spelling=True) for method in methods]
        assert [len(method) for method in decoded] == [129, 65]
        verify_preference_color_legend(image, decoded[0], legend)
        verify_preference_color_guard(decoded[1])
        verify_preference_color_routes(decoded[1])
        verify_preference_color_providers(images['dspogl.dll'], legend)
        dialog_import(image, b'dspogl.dll', b'??4COGLColor@@QEAAAEAV0@AEBK@Z', 0x1802cb378)
        dialog_baseline('preference-color-installed-reference.json', dict(module_sha256=legend['module_sha256'], methods=methods,
            provider_sha256=legend['provider_sha256'], provider_complete_instructions=[6, 10], target_executed=False,
            initializer_is_wire_default=False, actual_preference_invocation_observed=False))
    finally:
        for image in images.values(): image.close()


def verify_preference_color_legend(image: pefile.PE, instructions: list[StylePreferenceInstruction], legend: dict[str, object]) -> None:
    """Bind the blue literal, complete constructor/assignment calls and unresolved member100; e.g. verify_preference_color_legend(image, instructions, legend)."""
    dialog_export(image, b'?Initialize@CDspLegend@@QEAAXXZ', 0x1800dc530)
    assert image.get_data(0x2d7aec, 4) == bytes.fromhex('0000803f')
    lines = {instruction.address: instruction for instruction in instructions}
    assert lines[0x1800dc71d].mnemonic == 'xorps' and lines[0x1800dc71d].operands == ('xmm2', 'xmm2')
    assert lines[0x1800dc726].mnemonic == 'xorps' and lines[0x1800dc726].operands == ('xmm1', 'xmm1')
    assert lines[0x1800dc558].operands == ('xmm6', 'dwordptr[rip+2078092]')
    assert 0x1800dc560 + 0x1fb58c == 0x1802d7aec
    assert lines[0x1800dc714].operands == ('xmm3', 'xmm6')
    assert lines[0x1800dc737].operands == ('rcx', '[rsi+256]')
    for call in cast(list[dict[str, str]], legend['calls']):
        instruction = lines[int(call['address'], 16)]
        assert instruction.raw[:2] == b'\xff\x15'
        assert instruction.address + 6 + struct.unpack_from('<i', instruction.raw, 2)[0] == int(call['iat'], 16)
        dialog_import(image, b'dspogl.dll', call['name'].encode(), int(call['iat'], 16))
    assert legend['initialized_rgba'] == [0, 0, 1, 1]



def verify_preference_color_rea_capture() -> None:
    """Require all194 independent caller locations and complete contiguous body extents; e.g. verify_preference_color_rea_capture()."""
    legend, preferences = preference_color_reports()
    captures: list[dict[str, object]] = []
    for name, report in (('legend', legend), ('setter', preferences)):
        path = FEATURE / f'preference-color-rea-{name}.json'
        record = json.loads(path.read_bytes())
        assert record['subject']['digest']['sha256'] == report['module_sha256']
        assert record['provider'] == dict(id='ghidra', name='Ghidra', version='12.1.4')
        assert record['authority'] == 'shipped-artifact' and record['confidence'] == 'observed'
        assert record['raw_result']['value'] == record['normalized_result']
        method = cast(dict[str, object], report['function'])
        lines = record['normalized_result']['instructions']
        assert [line.split(':', 1)[0] for line in lines] == ['0x' + line.split()[0] for line in cast(list[str], method['instructions'])]
        preference_color_rea_extent(record['normalized_result'], method)
        captures.append(dict(path=path.name, sha256=digest(path.read_bytes()), evidence_id=record['evidence_id'],
            instruction_count=len(lines), limitations=record['limitations']))
    dialog_baseline('preference-color-rea-reference.json', dict(captures=captures, independent_caller_instructions=194,
        provider_leaves_verified_by_exact_bytes_and_exports=True, target_executed=False, original_binary_written=False))


def preference_color_rea_extent(normalized: dict[str, object], method: dict[str, object]) -> None:
    """Preserve exact analyzed intervals rather than treating a partial caller as complete; e.g. preference_color_rea_extent(normalized, method)."""
    procedure = cast(dict[str, object], normalized['procedure'])
    body = cast(dict[str, object], procedure['body'])
    start, end = int(cast(str, method['address']), 16), int(cast(str, method['end']), 16)
    assert body['available'] is True and body['contains_entry'] is True and body['non_contiguous'] is False
    assert body['ranges'] == [dict(start=hex(start), end=hex(end - 1))]
    assert body['total_bytes'] == body['span_bytes'] == end - start
