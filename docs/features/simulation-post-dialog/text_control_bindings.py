"""Complete C890 display-source consumer and retained C855 assignment native bodies (#166)."""
from typing import cast
import json
import pefile
from native_carriers import RESEARCH, digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images
from aux_triangle_bindings import aux_file_bytes
from style_preferences_disassembly import decoded_style_preference_method
from text_control_evidence import text_control_report, text_assignment_report


def text_selection_anchors(report: dict[str, object]) -> None:
    """Pin both branches, all seven empty constructors and their selected object arrays; e.g. text_selection_anchors(report)."""
    consumer = cast(dict[str, object], report['consumer'])
    instructions = set(cast(list[str], consumer['instructions']))
    assert {'18010aa36 xor esi, esi', '18010aa3c cmp dword ptr [rcx + 0x5dc], esi',
        '18010aa42 je 0x18010aa5e', '18010aa44 lea rdx, [rcx + 0x5e8]',
        '18010aa50 call qword ptr [rip + 0x1c2142]', '18010aa5e lea rdx, [rip + 0x1cd1ab]',
        '18010aa6a call qword ptr [rip + 0x1c2120]', '18010aa82 call qword ptr [rip + 0x1c2110]',
        '18010ab4b cmp dword ptr [rdi + 0x5e0], 0', '18010ab52 lea r12, [rdi + 0x5f0]',
        '18010ab59 jne 0x18010ab62', '18010ab5b lea r12, [rbp + 0xb0]'} <= instructions
    constructors = ((0x18010aab8, 0x1cd151, 0x18010aac6, 0x1c20c4),
        (0x18010aacd, 0x1cd13c, 0x18010aadb, 0x1c20af), (0x18010aae2, 0x1cd127, 0x18010aaf0, 0x1c209a),
        (0x18010aaf7, 0x1cd112, 0x18010ab05, 0x1c2085), (0x18010ab0c, 0x1cd0fd, 0x18010ab1a, 0x1c2070),
        (0x18010ab21, 0x1cd0e8, 0x18010ab2f, 0x1c205b), (0x18010ab36, 0x1cd0d3, 0x18010ab44, 0x1c2046))
    for lea, displacement, call, iat in constructors:
        assert f'{lea:x} lea rdx, [rip + {displacement:#x}]' in instructions
        assert f'{call:x} call qword ptr [rip + {iat:#x}]' in instructions
        assert lea + 7 + displacement == 0x1802d7c10 and call + 6 + iat == 0x1802ccb90


def text_cstring_imports(image: pefile.PE) -> list[dict[str, object]]:
    """Preserve actual ordinal imports rather than inventing imported CString symbol names; e.g. text_cstring_imports(image)."""
    image.parse_data_directories(directories=[1])
    rows = [dict(module=descriptor.dll.decode(), iat=hex(item.address), name=item.name, ordinal=item.ordinal)
        for descriptor in image.DIRECTORY_ENTRY_IMPORT for item in descriptor.imports
        if item.address in (0x1802ccb90, 0x1802ccb98)]
    assert rows == [dict(module='mfc140u.dll', iat='0x1802ccb90', name=None, ordinal=286),
        dict(module='mfc140u.dll', iat='0x1802ccb98', name=None, ordinal=280)]
    assert 0x18010aa56 + 0x1c2142 == 0x18010aa88 + 0x1c2110 == 0x1802ccb98
    assert 0x18010aa70 + 0x1c2120 == 0x1802ccb90
    return rows


def text_assignment_methods(image: pefile.PE) -> list[dict[str, object]]:
    """Rematch all original assignment/helper instructions without replacing their prefix qualifications; e.g. text_assignment_methods(image)."""
    report = text_assignment_report()
    assert digest(image.__data__) == report['module_sha256']
    probes = json.loads((RESEARCH / 'cosworks-post-style-text-probe.json').read_bytes())
    by_address = {probe['address']: probe for probe in probes}
    methods = cast(list[dict[str, str]], report['functions'])
    assert set(by_address) == {method['address'] for method in methods}
    summaries: list[dict[str, object]] = []
    for method in methods:
        probe = by_address[method['address']]
        assert probe['end'] == method['end']
        qualified = dict(start=method['address'], end=method['end'], sha256=method['sha256'], instructions=probe['instructions'])
        decoded = decoded_style_preference_method(image, qualified, normalize_address_spelling=True)
        summaries.append(dict(start=qualified['start'], end=qualified['end'], sha256=qualified['sha256'], instructions=len(decoded)))
    return summaries


def verify_text_control_bindings() -> None:
    """Bind the complete display consumer, source-selection instructions, empty literal and earlier assignment bodies; e.g. verify_text_control_bindings()."""
    report = text_control_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        consumer = cast(dict[str, object], report['consumer'])
        assert len(decoded_style_preference_method(image, consumer, normalize_address_spelling=True)) == 315
        assert report['empty_literal'] == dict(address='0x1802d7c10', raw_hex='0000')
        assert aux_file_bytes(image, 0x1802d7c10, 2).hex() == '0000'
        text_selection_anchors(report)
        imports = text_cstring_imports(image)
        assignment = text_assignment_methods(image)
        dialog_baseline('text-control-installed-reference.json', dict(module_sha256=report['module_sha256'],
            complete_consumer_instructions=315, assignment_methods=assignment, cstring_ordinal_imports=imports,
            empty_literal=report['empty_literal'], target_executed=False,
            limitations=['Ordinal import identities are observed; imported CString symbol names are absent.', 'No complete font/layout or renderer execution claim.']))
    finally:
        for image in images.values(): image.close()


def verify_text_control_rea_capture() -> None:
    """Retain independent Ghidra instruction locations, empty-literal bytes and provider limitations; e.g. verify_text_control_rea_capture()."""
    consumer_path, literal_path = [FEATURE / f'text-control-rea-{name}.json' for name in ('consumer', 'empty')]
    consumer, literal = [json.loads(path.read_bytes()) for path in (consumer_path, literal_path)]
    report = text_control_report()
    assert consumer['subject'] == literal['subject']
    assert consumer['subject']['digest']['sha256'] == report['module_sha256']
    assert consumer['provider'] == literal['provider'] == dict(id='ghidra', name='Ghidra', version='12.1.4')
    assert consumer['authority'] == literal['authority'] == 'shipped-artifact'
    assert consumer['confidence'] == literal['confidence'] == 'observed'
    assembly = cast(str, consumer['normalized_result'])
    assert consumer['raw_result']['value'] == assembly
    method = cast(dict[str, object], report['consumer'])
    assert [line.split(':', 1)[0] for line in assembly.splitlines()] == [
        '0x' + line.split()[0] for line in cast(list[str], method['instructions'])]
    assert literal['normalized_result'] == dict(address='0x1802d7c10', requested_bytes=2, returned_bytes=2, bytes_hex='0000', complete=True)
    captures = [dict(path=path.name, sha256=digest(path.read_bytes()), evidence_id=record['evidence_id'], limitations=record['limitations'])
        for path, record in zip((consumer_path, literal_path), (consumer, literal), strict=True)]
    dialog_baseline('text-control-rea-reference.json', dict(captures=captures, target_executed=False, original_binary_written=False))
