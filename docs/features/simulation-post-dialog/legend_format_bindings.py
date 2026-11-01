"""Complete named format methods, exact literals and independently analyzed body extents (#166)."""
import json
import pefile
from typing import cast
from native_carriers import RESEARCH, digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images
from style_preferences_disassembly import decoded_style_preference_method
from native_decoder_qualifications import qualify_captured_native_method, qualify_captured_native_line
from legend_format_evidence import legend_format_report

FORMAT_COMISD = {0x1800db216: ('1800db216 comisd xmm0, xmmword ptr [rip + 0x1ff6ca]',
    '1800db216 comisd xmm0, qword ptr [rip + 0x1ff6ca]', '660f2f05caf61f00', 'captured COMISD scalar-double width typo; #169')}


def legend_format_methods() -> list[dict[str, object]]:
    """Attach complete immutable probe text to each original body digest; e.g. legend_format_methods()."""
    report = legend_format_report()
    probes = json.loads((RESEARCH / 'cosworks-post-legend-number-format-probe.json').read_bytes())
    methods: list[dict[str, object]] = []
    for recorded, probe in zip(cast(list[dict[str, object]], report['functions']), probes, strict=True):
        assert (recorded['address'], recorded['end']) == (probe['address'], probe['end'])
        methods.append(recorded | {'start': recorded['address'], 'instructions': probe['instructions']})
    return methods


def verify_legend_format_bindings() -> None:
    """Match all146 instructions, complete bytes, exports, constants and literals; e.g. verify_legend_format_bindings()."""
    report = legend_format_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        qualifications: list[dict[str, str]] = []
        counts: list[int] = []
        for method in legend_format_methods():
            qualified, changes = qualify_captured_native_method(image, method, FORMAT_COMISD)
            counts.append(len(decoded_style_preference_method(image, qualified, normalize_address_spelling=True)))
            qualifications.extend(changes)
        assert counts == [67, 79] and len(qualifications) == 1, (counts, qualifications)
        legend_format_constants(image, report)
        verify_legend_format_qualification_boundaries()
        dialog_baseline('legend-format-installed-reference.json', dict(module_sha256=report['module_sha256'],
            complete_instruction_counts=counts, qualifications=qualifications, target_executed=False))
    finally:
        for image in images.values(): image.close()


def legend_format_constants(image: pefile.PE, report: dict[str, object]) -> None:
    """Bind all original literal/constant bytes and decompilations without executing audits; e.g. legend_format_constants(image, report)."""
    for row in cast(list[dict[str, str]], report['literals']) + cast(list[dict[str, str]], report['constants']):
        expected = bytes.fromhex(row['raw_hex'])
        assert image.get_data(int(row['address'], 16) - image.OPTIONAL_HEADER.ImageBase, len(expected)) == expected
    for method in cast(list[dict[str, str]], report['functions']):
        path = RESEARCH / 'decomp/cosworks-post-legend-number-format' / f"FUN_{int(method['address'], 16):x}.c"
        assert digest(path.read_bytes()) == method['decompilation_sha256']


def require_legend_format_qualification_rejection(line: str, raw: bytes) -> None:
    """Reject changed instruction text or any changed byte; e.g. require_legend_format_qualification_rejection(line, raw)."""
    try:
        qualify_captured_native_line(line, raw, FORMAT_COMISD)
    except AssertionError:
        return
    raise AssertionError((line, raw.hex(), 'expected exact captured COMISD qualification rejection'))


def verify_legend_format_qualification_boundaries() -> None:
    """Limit the scalar-width qualification to one captured location/text/eight-byte tuple; e.g. verify_legend_format_qualification_boundaries()."""
    original, canonical, encoded, _ = FORMAT_COMISD[0x1800db216]
    raw = bytes.fromhex(encoded)
    assert qualify_captured_native_line(original, raw, FORMAT_COMISD)[0] == canonical
    for index in range(len(raw)):
        changed = bytearray(raw)
        changed[index] ^= 1
        require_legend_format_qualification_rejection(original, bytes(changed))
    require_legend_format_qualification_rejection(original.replace('xmm0', 'xmm1'), raw)
    unknown = original.replace('1800db216', '1800db217')
    assert qualify_captured_native_line(unknown, raw, FORMAT_COMISD) == (unknown, None)


def verify_legend_format_rea_capture() -> None:
    """Require all146 independent Ghidra locations and both complete contiguous body extents; e.g. verify_legend_format_rea_capture()."""
    captures: list[dict[str, object]] = []
    for name, method in zip(('primary', 'alternate'), legend_format_methods(), strict=True):
        path = FEATURE / f'legend-format-rea-{name}.json'
        record = json.loads(path.read_bytes())
        assert record['subject']['digest']['sha256'] == legend_format_report()['module_sha256']
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
    dialog_baseline('legend-format-rea-reference.json', dict(captures=captures, independent_instruction_count=146, target_executed=False))
