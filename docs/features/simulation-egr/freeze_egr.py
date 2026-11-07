"""Freeze complete conditional EGR metadata tables and existing-live-field edits (#124)."""
from pathlib import Path
import json
import struct
import sys
import tempfile
import unittest
import zlib

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'simulation-images'))
import freeze_images as shared
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from owned_egr import TableCase, table_cases, table_image, descriptor_pairs, padded, field_edit_golden
import name_reference
import projection_reference
span, words, sha = shared.span, shared.words, shared.sha


def report(name: str, value: object) -> None:
    """Require explicit review before changing frozen reference results."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed baseline'


def fixture(name: str, packet: bytes) -> None:
    """Ship independently assembled synthetic images, without native fixture claims."""
    path = ROOT / 'tests/fixtures/simulation-egr' / name
    raw = zlib.compress(packet, 9)
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(raw)
    assert path.read_bytes() == raw


def table_oracle(raw: bytes) -> bytes:
    """Bind complete framing, signed descriptors and physical partition to the actual reader."""
    import simulationegr as reader
    table = reader.metadata_flat_file(raw, constructor_bound_flat=True)
    assert table['raw'] == raw and table['framing_complete_for_file_profile'] and not table['complete_file_schema']
    assert b''.join(raw[part['offset']:part['end']] for part in table['spans']) == raw
    packet = words(table['header_words'])
    for key in ('header_padding', 'field_directory_alignment', 'directory_padding'):
        packet += span(table[key])
    packet += words([len(table['field_pairs'])]) + b''.join(words(pair) for pair in table['field_pairs'])
    packet += words([len(table['directory']), *table['directory'], table['allocated_record_count'], table['unallocated_capacity']])
    packet += words([len(table['physical_rows'])]) + b''.join(span(row) for row in table['physical_rows'])
    packet += words([len(table['rows'])]) + b''.join(live_row_oracle(row) for row in table['rows'])
    packet += words([len(table['free_records'])])
    for record in table['free_records']:
        packet += words([record['record_number'], record['next_record_number']]) + span(record['raw'])
    return packet


def live_row_oracle(row: dict[str, object]) -> bytes:
    """Keep variable-width fields raw and retain the complete unassigned row tail."""
    packet = words([row['logical_ordinal'], row['record_number'], row['body_offset'], len(row['fields'])])
    for field in row['fields']:
        packet += words([field['tag'], field['offset'], field['end']]) + span(field['raw'])
    assert b''.join(field['raw'] for field in row['fields']) + row['unassigned_tail'] == row['raw']
    packet += span(row['unassigned_tail']) + words([int('four_byte_row_view' in row)])
    if 'four_byte_row_view' in row:
        packet += span(standard_row_oracle(row['four_byte_row_view']))
    return packet


def standard_row_oracle(view: dict[str, object]) -> bytes:
    """Keep signed integer parameters separate from unknown runtime name lookup state."""
    status = {'known_name': 1, 'matched_null_source': 2, 'unmatched_no_write': 3}[view['name_lookup_status']]
    packet = words([view['group_identifier'], view['solver_type_code'], status, *view['group_parameter_int32']])
    name = view['solver_type_name']
    return packet + span(b'' if name is None else name.encode('ascii'))


def assert_independent_table(case: TableCase, raw: bytes) -> None:
    """Verify known region equations without deriving fixture layout from the reader."""
    import simulationegr as reader
    parsed = reader.metadata_body(raw)
    active = {number: ordinal for ordinal, number in enumerate(case.directory, 1) if number}
    start = 128 + padded(len(case.widths) * 8) + padded(len(case.directory) * 4)
    assert parsed['field_pairs'] == descriptor_pairs(case) and parsed['directory'] == list(case.directory)
    assert parsed['allocated_record_count'] == len(active) + len(case.free_chain)
    assert parsed['unallocated_capacity'] == len(case.directory) - len(active) - len(case.free_chain)
    assert [record['record_number'] for record in parsed['free_records']] == list(case.free_chain)
    assert [(row['logical_ordinal'], row['record_number']) for row in parsed['rows']] == [(ordinal, number) for ordinal, number in enumerate(case.directory, 1) if number]
    assert [row['body_offset'] for row in parsed['rows']] == [start + (row['record_number'] - 1) * 128 for row in parsed['rows']]


def freeze_tables() -> None:
    """Own complete standard, sparse/deleted, mixed-width and zero-field images."""
    import simulationegr as reader
    cases = table_cases()
    packet = bytearray(b'EGR1' + words([len(cases)]))
    for case in cases:
        raw = table_image(case)
        assert_independent_table(case, raw)
        table = reader.metadata_body(raw)
        descriptor = reader.descriptor_writer_view(raw)
        expected = b''.join(struct.pack('<2i', 1, width) for width in case.widths)
        assert descriptor['projected_descriptor_bytes'] == expected
        assert descriptor['original_descriptor_bytes'] == raw[128:128 + len(case.widths) * 8]
        assert descriptor['alignment_bytes'] == table['field_directory_alignment']
        assert not descriptor['complete_native_save']
        packet += span(raw) + span(table_oracle(raw)) + span(expected)
    fixture('tables.bin', bytes(packet))
    report('table-reference.json', dict(cases=len(cases), oracle_sha256=sha(bytes(packet)), native_egr_fixtures=0))


def freeze_fields() -> None:
    """Compare every existing live-field interval with independent complete-file edit goldens."""
    cases = table_cases()
    packet = bytearray(b'EGRF' + words([len(cases)]))
    total = 0
    for case in cases:
        raw = table_image(case)
        contexts = [(ordinal, first, last) for ordinal, number in enumerate(case.directory, 1) if number
                    for first in range(1, len(case.widths) + 1) for last in range(first, len(case.widths) + 1)]
        packet += span(raw) + words([len(contexts)])
        for ordinal, first, last in contexts:
            packet += field_oracle(raw, case, ordinal, first, last)
        total += len(contexts)
    fixture('fields.bin', bytes(packet))
    report('field-reference.json', dict(tables=len(cases), accepted_live_ranges=total, oracle_sha256=sha(bytes(packet))))


def field_oracle(raw: bytes, case: TableCase, logical: int, first: int, last: int) -> bytes:
    """Use actual loaded-width range selection and independently patched full-file output."""
    import simulationegr as reader
    view = reader.live_field_range(raw, logical_ordinal=logical, first_field=first, last_field=last)
    start, end = sum(case.widths[:first - 1]), sum(case.widths[:last])
    assert (view['row_offset'], view['row_end'], view['byte_count']) == (start, end, end - start)
    replacement, golden = field_edit_golden(raw, case, logical, first, last)
    changed = reader.live_field_range(golden, logical_ordinal=logical, first_field=first, last_field=last)
    assert changed['raw'] == replacement and not changed['native_cache_state_projected']
    assert reader.metadata_flat_file(golden, constructor_bound_flat=True)['raw'] == golden
    return words([logical, first, last, view['record_number'], start, end]) + span(view['raw']) + span(replacement) + span(golden)


def freeze_guards() -> None:
    """Reject invalid extent, allocation partition and explicit transport assumptions."""
    import simulationegr as reader
    raw = table_image(table_cases()[3])
    for offset, value in ((0, 127), (4, -1), (8, 33), (16, -1), (20, -1), (24, -1), (32, 5), (132, 0), (384, 1), (640, 2), (640, 1), (640, 4), (640, -1)):
        invalid = bytearray(raw)
        struct.pack_into('<i', invalid, offset, value)
        with unittest.TestCase().assertRaises(ValueError):
            reader.metadata_body(invalid)
    for prefix in (raw[:-1], raw + b'\0'):
        with unittest.TestCase().assertRaises(ValueError):
            reader.metadata_body(prefix)
    for flag in (False, 0, 1, None):
        with unittest.TestCase().assertRaises(ValueError):
            reader.metadata_flat_file(raw, constructor_bound_flat=flag)
    with unittest.TestCase().assertRaises(ValueError):
        reader.live_field_range(raw, logical_ordinal=2, first_field=1, last_field=1)


def native_census() -> None:
    """Recheck the three retained carriers without claiming absence outside their actual stream names."""
    from cfb import CFB
    facts = []
    for source, carrier, _, _ in shared.native_pairs():
        document = CFB(source['path'])
        names = [entry.path for entry in document.walk() if entry.type == 2]
        assert not any('egr' in name.casefold() for name in names)
        facts.append(dict(path=source['path'], sha256=sha(carrier), stream_names=names))
    report('native-census-reference.json', dict(retained_cwr_files=3, records=facts, named_egr_streams=0, complete_native_egr_fixtures=0, scope_is_not_all_installed_or_external_files=True))


def freeze_names() -> None:
    """Use unchanged complete solver-name reader code and independent literal mappings."""
    name_reference.freeze_names(fixture, report, words, span, sha)


def freeze_groups() -> None:
    """Use the actual conditional EGROUP helper with explicit caller contexts."""
    name_reference.freeze_groups(fixture, report, words, span, sha)


def schema() -> None:
    """Keep the complete current EGR/name clauses as the bounded acceptance scope."""
    text = (ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md').read_text(encoding='utf-8').replace('\r\n', '\n')
    start = text.index('### 14.514 ')
    end = text.find('### 14.523 ', start)
    raw = (text[start:end if end >= 0 else len(text)].rstrip() + '\n').encode()
    path = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw
    report('schema-reference.json', dict(issue=124, clauses=[f'14.{number}' for number in range(514, 523)], sha256=sha(raw)))


def verify_rust() -> None:
    """Bind the complete independently emitted Rust metadata/range/name/group packet to actual readers."""
    if '--verify-rust' not in sys.argv:
        return
    expected = projection_reference.complete_packet(table_oracle, field_oracle, words, span)
    actual = (ROOT / 'target/simulation-egr-projections.bin').read_bytes()
    assert actual == expected
    report('rust-projection-reference.json', dict(bytes=len(actual), sha256=sha(actual), tables=14, live_field_ranges=3992, name_contexts=183, egroup_contexts=936, inputs_are_owned_synthetic=True, native_egr_fixtures=0))


def run() -> bool:
    """Import unchanged complete reader modules only after copying and hashing their sources."""
    with tempfile.TemporaryDirectory(prefix='bessemer-simulation-egr-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        shared.compound.pin_reader(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(test) for test in (freeze_tables, freeze_fields, freeze_guards, freeze_names, freeze_groups, native_census, schema, verify_rust))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('cfb', 'simulationegr', 'simulationelementnames'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
