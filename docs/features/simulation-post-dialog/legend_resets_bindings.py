"""C857 complete legend reader and exact zero-register/version/store boundaries (#166)."""
from typing import cast
import json
from native_carriers import RESEARCH, digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images, dialog_export
from style_preferences_disassembly import decoded_style_preference_method
from legend_body_evidence import legend_body_capture
from legend_resets_evidence import legend_resets_report


def legend_resets_method() -> dict[str, object]:
    """Retain the complete original reader interval and probe without padding exclusions; e.g. legend_resets_method()."""
    reader = cast(dict[str, object], legend_resets_report()['reader'])
    probes = json.loads((RESEARCH / 'cosworks-post-legend-probe.json').read_bytes())
    probe = next(row for row in probes if row['address'] == reader['address'])
    assert reader['end'] == probe['end']
    return reader | dict(start=reader['address'], instructions=probe['instructions'])


def verify_legend_resets_bindings() -> None:
    """Require all181 instructions, the named reader, hash and exact reset stores; e.g. verify_legend_resets_bindings()."""
    report = legend_resets_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        method = legend_resets_method()
        symbol = dialog_export(image, b'?ReadFromStream@CDspLegend@@', 0x1800dfa00)
        assert len(decoded_style_preference_method(image, method, normalize_address_spelling=True)) == 181
        path = RESEARCH / 'decomp/cosworks-post-dialog/FUN_1800dfa00.c'
        assert digest(path.read_bytes()) == method['decompilation_sha256']
        verify_legend_reset_gates(method)
        verify_legend_reset_stores(method)
        verify_legend_unreset_gates(method)
        dialog_baseline('legend-resets-installed-reference.json', dict(export=symbol.decode(), complete_instruction_count=181,
            complete_body_bytes=747, exact_decoder_qualifications=0, target_executed=False,
            wire_words_added=0, constructor_defaults_inferred=False))
    finally:
        for image in images.values(): image.close()


def ordered_reset_anchors(method: dict[str, object], anchors: list[str]) -> None:
    """Keep every exact native branch/read/store in original order; e.g. ordered_reset_anchors(method, anchors)."""
    lines = cast(list[str], method['instructions'])
    positions = [lines.index(anchor) for anchor in anchors]
    assert positions == sorted(set(positions)), (anchors, positions)


def verify_legend_reset_gates(method: dict[str, object]) -> None:
    """Bind raw-path exclusion and distinct840008/840010 signed gates; e.g. verify_legend_reset_gates(method)."""
    ordered_reset_anchors(method, ['1800dfa20 mov ebp, r8d', '1800dfa26 mov rdi, rcx',
        '1800dfa29 cmp r8d, 0xaae66', '1800dfa30 jge 0x1800dfa89',
        '1800dfa71 call 0x180042e70', '1800dfa84 jmp 0x1800dfccd',
        '1800dfa89 mov rax, qword ptr [rdx]', '1800dfbe4 xor r14d, r14d',
        '1800dfbe7 cmp ebp, 0xcd148', '1800dfbed jl 0x1800dfcb2',
        '1800dfbff lea rdx, [rdi + 0xc0]', '1800dfc09 call qword ptr [rax + 0x18]',
        '1800dfc0c lea rcx, [rdi + 0xc4]', '1800dfc13 lea rsi, [rdi + 0xc8]',
        '1800dfc1a cmp ebp, 0xcd14a', '1800dfc20 jl 0x1800dfcc7',
        '1800dfc38 call qword ptr [rax + 0x18]', '1800dfc4d call qword ptr [rax + 0x18]'])


def verify_legend_reset_stores(method: dict[str, object]) -> None:
    """Require the sole zero-register setup and all three exact32-bit assignments; e.g. verify_legend_reset_stores(method)."""
    lines = cast(list[str], method['instructions'])
    assert [line for line in lines if line.endswith('xor r14d, r14d')] == ['1800dfbe4 xor r14d, r14d']
    ordered_reset_anchors(method, ['1800dfbe4 xor r14d, r14d', '1800dfcb0 jmp 0x1800dfccd',
        '1800dfcb2 mov dword ptr [rdi + 0xc0], r14d', '1800dfcb9 lea rcx, [rdi + 0xc4]',
        '1800dfcc0 lea rsi, [rdi + 0xc8]', '1800dfcc7 mov dword ptr [rcx], r14d',
        '1800dfcca mov dword ptr [rsi], r14d', '1800dfccd mov rcx, qword ptr [rsp + 0x290]'])


def verify_legend_unreset_gates(method: dict[str, object]) -> None:
    """Retain skipped D4/D8 and E8/FC/EC/100 reads without manufacturing resets; e.g. verify_legend_unreset_gates(method)."""
    ordered_reset_anchors(method, ['1800dfba8 cmp ebp, 0xc5c19', '1800dfbae jl 0x1800dfbc9',
        '1800dfbb3 lea rdx, [rdi + 0xd4]', '1800dfbc6 call qword ptr [rax + 0x18]',
        '1800dfbc9 cmp ebp, 0xc5c23', '1800dfbcf jl 0x1800dfbe4',
        '1800dfbd1 lea rcx, [rdi + 0xd8]', '1800dfbde call qword ptr [rip + 0x1eb7b4]',
        '1800dfc50 cmp ebp, 0xd94f7', '1800dfc56 jl 0x1800dfccd',
        '1800dfc5b lea rdx, [rdi + 0xe8]', '1800dfc74 lea rdx, [rdi + 0xfc]',
        '1800dfc8a lea rcx, [rdi + 0xec]', '1800dfc9d lea rcx, [rdi + 0x100]'])


def verify_legend_resets_rea_capture() -> None:
    """Require all181 independent locations and the full contiguous747-byte body; e.g. verify_legend_resets_rea_capture()."""
    module_hash = cast(str, legend_resets_report()['module_sha256'])
    capture = legend_body_capture(FEATURE / 'legend-resets-rea-reader.json', legend_resets_method(), module_hash)
    dialog_baseline('legend-resets-rea-reference.json', dict(captures=[capture], independent_instruction_count=181,
        complete_body_bytes=747, target_executed=False))
