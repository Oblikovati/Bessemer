"""Recheck every static table target and complete constant-write tail without executing provider code (#166)."""
import struct
from typing import cast
import pefile
from dialog_bindings import dialog_images
from native_carriers import digest
from vector_evidence import vector_reports


def constant_write_bytes(offset: int, word: int) -> bytes:
    """Encode only the two C-879 MOV-immediate forms, rejecting every other destination; e.g. constant_write_bytes(4, 0x3f800000)."""
    assert offset in (0, 4, 8, 12), offset
    prefix = bytes.fromhex('41c702') if offset == 0 else bytes.fromhex('41c742') + bytes([offset])
    return prefix + struct.pack('<I', word)


def verify_constant_tail(image: pefile.PE, tail: dict[str, object]) -> str:
    """Require contiguous actual MOV bytes followed by RET and the exact reconstructed raw output; e.g. verify_constant_tail(image, tail)."""
    cursor = int(cast(str, tail['target']), 16)
    words = [0, 0, 0, 0]
    writes = cast(list[dict[str, object]], tail['writes'])
    assert len(writes) <= 11, writes
    for write in writes:
        assert int(cast(str, write['address']), 16) == cursor, write
        offset, word = cast(int, write['output_byte_offset']), cast(int, write['value_u32'])
        raw = constant_write_bytes(offset, word)
        assert image.get_data(cursor - image.OPTIONAL_HEADER.ImageBase, len(raw)) == raw, write
        words[offset // 4] = word
        cursor += len(raw)
    assert cursor == int(cast(str, tail['return_address']), 16)
    assert image.get_data(cursor - image.OPTIONAL_HEADER.ImageBase, 1) == b'\xc3'
    output = struct.pack('<4I', *words).hex()
    assert words == tail['output_words_u32'] and output == tail['output_raw_hex'], tail
    return output


def verify_vector_static_tables() -> None:
    """Check all 23 contiguous tables, 262 targets and 11 complete literal tails against installed bytes; e.g. verify_vector_static_tables()."""
    report = vector_reports()[1]
    images = dialog_images()
    try:
        image = images['cwpostcommon.dll']
        assert digest(image.__data__) == report['module_sha256']
        tails = cast(list[dict[str, object]], report['constant_tails'])
        outputs = {int(cast(str, tail['target']), 16): verify_constant_tail(image, tail) for tail in tails}
        verify_table_target_ranges(image, report, outputs)
    finally:
        for image in images.values(): image.close()


def verify_table_target_ranges(image: pefile.PE, report: dict[str, object], outputs: dict[int, str]) -> None:
    """Match exact native u32 targets, instruction boundaries and each entry's full output; e.g. verify_table_target_ranges(image, report, outputs)."""
    code = cast(dict[str, object], report['code'])
    boundaries = {int(instruction['address'], 16) for instruction in cast(list[dict[str, str]], code['instructions'])}
    cursor, total = 0x2eff4, 0
    for table in cast(list[dict[str, object]], report['tables']):
        count = cast(int, table['count'])
        assert int(cast(str, table['rva']), 16) == cursor, table['rva']
        raw = image.get_data(cursor, count * 4)
        assert raw.hex() == table['raw_hex'] and digest(raw) == table['sha256'], table['rva']
        entries = cast(list[dict[str, object]], table['entries'])
        assert len(entries) == count
        for index, (target, entry) in enumerate(zip(struct.unpack('<' + 'I' * count, raw), entries, strict=True)):
            address = image.OPTIONAL_HEADER.ImageBase + target
            assert address in boundaries and target == int(cast(str, entry['target_rva']), 16), entry
            assert (entry['component_index'], entry['output_raw_hex']) == (index, outputs[address]), entry
        cursor, total = cursor + count * 4, total + count
    assert (cursor, total) == (0x2f40c, 262), (cursor, total)
