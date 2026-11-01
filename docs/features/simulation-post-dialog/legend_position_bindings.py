"""Complete C847 native leaves and independent instruction/body evidence (#166)."""
import json
from typing import cast
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images
from style_preferences_disassembly import decoded_style_preference_method
from legend_position_evidence import legend_position_report


def legend_position_methods() -> list[dict[str, object]]:
    """Retain complete structured Source instructions as exact decoder expectations; e.g. legend_position_methods()."""
    methods: list[dict[str, object]] = []
    for row in cast(list[dict[str, object]], legend_position_report()['functions']):
        original = cast(list[dict[str, str]], row['instructions'])
        lines = [f"{int(item['address'], 16):x} {item['mnemonic']} {item['operands']}".rstrip() for item in original]
        assert digest(bytes.fromhex(cast(str, row['raw_hex']))) == row['sha256']
        methods.append(row | {'start': row['address'], 'instructions': lines})
    return methods


def verify_legend_position_bindings() -> None:
    """Bind both complete leaves, exact exports and every factor/conversion/output argument; e.g. verify_legend_position_bindings()."""
    report = legend_position_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        methods = legend_position_methods()
        counts = [len(decoded_style_preference_method(image, method)) for method in methods]
        assert counts == [13, 12], counts
        verify_legend_position_argument_anchors(methods)
        dialog_baseline('legend-position-installed-reference.json', dict(module_sha256=report['module_sha256'],
            complete_instruction_counts=counts, exact_decoder_qualifications=0, target_executed=False,
            pixel_results_emulated=False, actual_runtime_dimensions_known=False, actual_fp_environment_known=False))
    finally:
        for image in images.values(): image.close()


def verify_legend_position_argument_anchors(methods: list[dict[str, object]]) -> None:
    """Keep signed dimensions, two separate factors and lower-only addition distinct; e.g. verify_legend_position_argument_anchors(methods)."""
    for method, base, lower in zip(methods, (0x1800db670, 0x1800db720), (True, False), strict=True):
        expected = [(0, 'movd xmm1, dword ptr [rsp + 0x28]'), (6, 'movd xmm0, r9d'),
            (11, 'cvtdq2ps xmm0, xmm0'), (14, 'cvtdq2ps xmm1, xmm1'),
            (17, 'mulss xmm0, dword ptr [rcx + 0xac]'), (25, 'cvttss2si eax, xmm0'),
            (29, 'mov dword ptr [rdx], eax'), (31, 'mulss xmm1, dword ptr [rcx + 0xb0]'),
            (39, 'cvttss2si eax, xmm1')]
        suffix = [(43, 'add eax, dword ptr [rsp + 0x30]'), (47, 'mov dword ptr [r8], eax'),
            (50, 'mov eax, 1'), (55, 'ret')] if lower else [(43, 'mov dword ptr [r8], eax'), (46, 'mov eax, 1'), (51, 'ret')]
        assert method['instructions'] == [f'{base + offset:x} {instruction}' for offset, instruction in expected + suffix]


def verify_legend_position_rea_capture() -> None:
    """Require all25 independent Ghidra locations and both complete contiguous leaf bodies; e.g. verify_legend_position_rea_capture()."""
    captures: list[dict[str, object]] = []
    for name, method in zip(('lower', 'upper'), legend_position_methods(), strict=True):
        path = FEATURE / f'legend-position-rea-{name}.json'
        record = json.loads(path.read_bytes())
        assert record['subject']['digest']['sha256'] == legend_position_report()['module_sha256']
        assert record['provider'] == dict(id='ghidra', name='Ghidra', version='12.1.4')
        assert record['authority'] == 'shipped-artifact' and record['confidence'] == 'observed'
        result = record['normalized_result']
        assert record['raw_result']['value'] == result
        assert [line.split(':', 1)[0] for line in result['instructions']] == ['0x' + line.split()[0] for line in cast(list[str], method['instructions'])]
        body = result['procedure']['body']
        assert body['available'] and body['contains_entry'] and not body['non_contiguous']
        assert body['ranges'] == [dict(start=method['start'], end=hex(int(cast(str, method['end']), 16) - 1))]
        assert body['total_bytes'] == body['span_bytes'] == int(cast(str, method['end']), 16) - int(cast(str, method['start']), 16)
        captures.append(dict(path=path.name, sha256=digest(path.read_bytes()), evidence_id=record['evidence_id'], instructions=len(result['instructions']), limitations=record['limitations']))
    dialog_baseline('legend-position-rea-reference.json', dict(captures=captures, independent_instruction_count=25, target_executed=False))
