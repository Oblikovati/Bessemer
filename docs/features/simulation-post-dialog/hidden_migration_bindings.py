"""Complete C856 reader body, exact version gates and adjacent-word stores (#166)."""
from typing import cast
import json
from native_carriers import RESEARCH, digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_bindings import dialog_images, dialog_export
from style_preferences_disassembly import decoded_style_preference_method
from legend_body_evidence import legend_body_capture
from hidden_migration_evidence import hidden_migration_report


def hidden_migration_method() -> dict[str, object]:
    """Retain the original complete reader interval and every probe instruction; e.g. hidden_migration_method()."""
    reader = cast(dict[str, object], hidden_migration_report()['reader'])
    probes = json.loads((RESEARCH / 'cosworks-post-dialog-info-style-probe.json').read_bytes())
    probe = next(row for row in probes if row['address'] == reader['address'])
    assert reader['end'] == probe['end']
    return reader | dict(start=reader['address'], instructions=probe['instructions'])


def verify_hidden_migration_bindings() -> None:
    """Require all227 instructions, the named reader and complete decompilation; e.g. verify_hidden_migration_bindings()."""
    report = hidden_migration_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        method = hidden_migration_method()
        symbol = dialog_export(image, b'?ReadFromStream@CDspPostStyle@@', 0x180044ae0)
        assert len(decoded_style_preference_method(image, method, normalize_address_spelling=True)) == 227
        path = RESEARCH / 'decomp/cosworks-post-dialog/FUN_180044ae0.c'
        assert digest(path.read_bytes()) == method['decompilation_sha256']
        verify_hidden_reader_gates(method)
        verify_hidden_adjacent_stores(method)
        dialog_baseline('hidden-migration-installed-reference.json', dict(export=symbol.decode(), complete_instruction_count=227,
            complete_body_bytes=1010, exact_decoder_qualifications=0, target_executed=False,
            wire_words_added=0, full_visibility_policy_modeled=False))
    finally:
        for image in images.values(): image.close()


def ordered_hidden_anchors(method: dict[str, object], anchors: list[str]) -> None:
    """Require exact ordered native reads and writes, including operand widths; e.g. ordered_hidden_anchors(method, anchors)."""
    lines = cast(list[str], method['instructions'])
    positions = [lines.index(anchor) for anchor in anchors]
    assert positions == sorted(set(positions)), (anchors, positions)


def verify_hidden_reader_gates(method: dict[str, object]) -> None:
    """Bind signed version gates and separate raw/explicit/direct-field routes; e.g. verify_hidden_reader_gates(method)."""
    ordered_hidden_anchors(method, ['180044b02 mov esi, r8d', '180044b08 mov rdi, rcx',
        '180044b17 cmp esi, 0xaae66', '180044b1d jge 0x180044bae',
        '180044bae mov rax, qword ptr [rbx]', '180044c96 mov r8d, 4',
        '180044c9c lea rdx, [rdi + 0x1f0]', '180044ca6 call qword ptr [rax + 0x18]',
        '180044de6 cmp esi, 0xcd148', '180044dec jl 0x180044e69',
        '180044df1 lea rdx, [rdi + 0x1ec]', '180044dfb mov r8d, 4',
        '180044e04 call qword ptr [rax + 0x18]', '180044e67 jmp 0x180044ea3',
        '180044e69 mov eax, dword ptr [rdi + 0x1f0]'])


def verify_hidden_adjacent_stores(method: dict[str, object]) -> None:
    """Keep the unsigned decrement comparison and full64-bit store, not just option1; e.g. verify_hidden_adjacent_stores(method)."""
    ordered_hidden_anchors(method, ['180044e69 mov eax, dword ptr [rdi + 0x1f0]',
        '180044e6f test eax, eax', '180044e71 jne 0x180044e7b',
        '180044e73 mov dword ptr [rdi + 0x1ec], eax', '180044e79 jmp 0x180044ea3',
        '180044e7b dec eax', '180044e7d cmp eax, 2', '180044e80 jbe 0x180044e98',
        '180044e82 mov dword ptr [rdi + 0x1ec], 1', '180044e8c mov dword ptr [rdi + 0x1f0], 1',
        '180044e96 jmp 0x180044ea3', '180044e98 mov qword ptr [rdi + 0x1ec], 1'])


def verify_hidden_migration_rea_capture() -> None:
    """Require all227 independent locations and the complete contiguous1010-byte reader; e.g. verify_hidden_migration_rea_capture()."""
    module_hash = cast(str, hidden_migration_report()['module_sha256'])
    capture = legend_body_capture(FEATURE / 'hidden-migration-rea-reader.json', hidden_migration_method(), module_hash)
    dialog_baseline('hidden-migration-rea-reference.json', dict(captures=[capture],
        independent_instruction_count=227, complete_body_bytes=1010, target_executed=False))
