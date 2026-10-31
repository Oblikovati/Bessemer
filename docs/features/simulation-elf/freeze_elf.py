"""Pin actual ELF initial, selected-directory and scalar readers (#115)."""
from pathlib import Path
import json
import struct
import sys
import tempfile
import unittest
import zlib

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'simulation-out'))
import freeze_reports as shared
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from owned_elf import initial, indexed, header_projection, scalar_record, mapping_cases, PATTERNS

span, words, sha = shared.span, shared.words, shared.sha
ACTIONS = dict(stop=0, skip=1, requires_external_mapping=2)


def report(name: str, record: object) -> None:
    """Require explicit review intent for this feature's baseline outputs."""
    path = FEATURE / name
    raw = (json.dumps(record, indent=2) + '\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
        return
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed baseline'


def initial_oracle(raw: bytes) -> bytes:
    """Project every initial header/constructor field and the complete zero continuation."""
    import simulationelf
    parsed = simulationelf.read_initial(raw)
    keys = ('record_bytes', 'record_words', 'native_word_parameter_14105e28c', 'initial_record_marker',
            'written_records', 'fixed_word_100', 'native_parameter_14105e284', 'native_parameter_14105e288',
            'next_record_marker', 'nominal_directory_capacity', 'selector_watermark', 'block_count_for_next_update', 'body_bytes', 'end')
    assert parsed['raw'] == raw and parsed['initial_image_complete']
    assert not parsed['populated_records_supported'] and not parsed['field_semantics_complete']
    return words([*parsed['header_words'], *(parsed[key] for key in keys)]) + span(parsed['zero_area'])


def directory_oracle(directory: dict) -> bytes:
    """Keep selected traversal separate from any whole-populated-image acceptance claim."""
    keys = ('index_area_bytes', 'selector', 'next_record_marker', 'nominal_directory_capacity',
            'block_count_for_next_update', 'pair_stream_offset', 'first_record', 'declared_count')
    raw = words([*directory['header_words'], *(directory[key] for key in keys),
                 int(directory['stopped_on_negative']), directory['cursor_stream_offset'] if directory['cursor_stream_offset'] is not None else -1,
                 len(directory['records'])])
    assert not directory['whole_image_complete'] and not directory['external_mapping_applied']
    for record in directory['records']:
        raw += words([record['stream_offset'], record['identifier'], ACTIONS[record['action']]]) + span(record['raw'])
        raw += span(record_oracle(record['raw']))
    return raw


def record_oracle(raw: bytes) -> bytes:
    """Retain all sixty binary32 patterns without quieting NaNs or assigning component roles."""
    import simulationelfrecord
    parsed = simulationelfrecord.read(raw)
    assert parsed['raw'] == raw
    assert all(parsed[key] is None for key in ('component_roles', 'units', 'frame', 'row_associations'))
    return words([parsed['identifier'], ACTIONS[parsed['getter_action']], len(parsed['slot_groups'])]) + b''.join(words(group['patterns']) for group in parsed['slot_groups'])


def mapping_oracle(mapping: dict) -> bytes:
    """Retain unknown destinations and every ordered overwrite explicitly."""
    raw = words([len(mapping['slots'])])
    for slot in mapping['slots']:
        raw += words([int(slot is not None)])
        if slot is not None:
            raw += span(slot)
    raw += words([len(mapping['writes'])])
    for write in mapping['writes']:
        raw += words([write['identifier'], write['destination_index']])
    return raw + words([len(mapping['unassigned_indices']), *mapping['unassigned_indices']])


def freeze_owned() -> None:
    """Check independent constructor images, literal mappings and every selector equation."""
    packets, facts = [], []
    for parameter_a, parameter_b in ((0, 0), (1, 2), (0xffffffff, 0x80000000), (14642, 14642)):
        raw = initial(parameter_a, parameter_b)
        expected = initial_oracle(raw)
        packets.append(span(raw) + span(expected))
        facts.append(dict(parameter_a=parameter_a, parameter_b=parameter_b, bytes=len(raw), sha256=sha(raw), oracle_sha256=sha(expected)))
    packet = b'ELFO' + words([len(packets)]) + b''.join(packets)
    packet += freeze_mapping_cases() + freeze_header_cases() + freeze_store_cases()
    compressed = zlib.compress(packet, 9)
    path = ROOT / 'tests/fixtures/simulation-elf/owned.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(compressed)
    assert path.read_bytes() == compressed
    report('owned-reference.json', dict(fixture_sha256=sha(compressed), decoded_bytes=len(packet), initial_cases=facts, mapping_cases=6, header_selectors=100, store_cases=5))


def freeze_mapping_cases() -> bytes:
    """Derive contexts from actual complete input parses, then compare literal mapping outcomes."""
    import simulationelf
    cases = mapping_cases()
    raw = words([len(cases)])
    for case in cases:
        directory = simulationelf.read_directory(case.raw, selector=case.selector)
        mapped = simulationelf.project_record_mapping(directory, source_to_destination=case.source_map, destination_count=case.destination_count)
        assert tuple(mapped['slots']) == case.expected_slots
        assert tuple((write['identifier'], write['destination_index']) for write in mapped['writes']) == case.expected_writes
        raw += span(case.raw) + words([case.selector, len(case.source_map), *case.source_map, case.destination_count])
        raw += span(directory_oracle(directory)) + span(mapping_oracle(mapped))
    return raw


def freeze_header_cases() -> bytes:
    """Verify all one-based selectors and destruction of prior directory/padding bytes."""
    import simulationelf
    source = bytearray(indexed((7, 0, -1, 9)))
    source[32:248] = b'\xa5' * 216
    source[1048:1224] = b'\x5a' * 176
    struct.pack_into('<2i', source, 4 + 244 + 8, 6, 4)
    raw = span(bytes(source)) + words([100])
    for selector in range(1, 101):
        projected = simulationelf.project_index_update(source, selector=selector)
        expected = header_projection(source, selector)
        assert projected['header_body_bytes'] == expected
        assert not projected['runtime_gates_applied'] and not projected['data_records_written']
        assert projected['write_stream_offset'] == 4
        raw += words([selector, projected['pair_stream_offset'], projected['first_record'], projected['count'], projected['next_record_marker']]) + span(expected)
    return raw


def freeze_store_cases() -> bytes:
    """Use independent full-record bytes for zero-filled and partial store-stage inputs."""
    import simulationelfrecord
    cases = [(0, []), (-1, [[0x80000000]]), (7, [list(PATTERNS[:6])]),
             (0x7fffffff, [list(PATTERNS[n % 2 * 6:n % 2 * 6 + 6]) for n in range(10)]),
             (-0x80000000, [[], [0x7fc12345], [1, 0xff800000]])]
    raw = words([len(cases)])
    for identifier, groups in cases:
        expected = scalar_record(identifier, groups)
        assert simulationelfrecord.project_store(identifier, groups) == expected
        raw += words([identifier, len(groups)]) + b''.join(words([len(group), *group]) for group in groups)
        raw += span(expected) + span(record_oracle(expected))
    return raw


def freeze_native() -> None:
    """Bind the actual installed empty ELF image without claiming populated corpus coverage."""
    from cfb import CFB
    import simulationelf
    catalog = json.loads((FEATURE.parent / 'compound-files/native-reference.json').read_text())['records']
    inputs = [item for item in catalog if any(stream['path'] == 'ELF' for stream in item['streams'])]
    assert len(inputs) == 1
    item = inputs[0]
    carrier = Path(item['path']).read_bytes()
    assert sha(carrier) == item['sha256']
    raw = CFB(Path(item['path'])).read('ELF')
    expected = initial_oracle(raw)
    directory = simulationelf.read_directory(raw, selector=1)
    assert not directory['records'] and directory['declared_count'] == 0
    proposal = simulationelf.project_index_update(raw, selector=1)
    assert proposal['header_body_bytes'] == header_projection(raw, 1)
    assert not proposal['data_records_written']
    edited = bytearray(raw)
    struct.pack_into('<I', edited, 16, 0x12345678)
    initial_oracle(edited)
    report('native-reference.json', dict(path=item['path'], carrier_sha256=sha(carrier), bytes=len(raw), sha256=sha(raw), oracle_sha256=sha(expected), golden_sha256=sha(edited), empty_directory=True, header_only_proposal_sha256=sha(proposal['header_body_bytes'])))
    if '--export' in sys.argv:
        packet = b'ELFN' + words([1]) + b''.join(map(span, (carrier, raw, expected, directory_oracle(directory), proposal['header_body_bytes'], edited)))
        (ROOT / 'target/simulation-elf-native.bin').write_bytes(packet)
        report('native-probe-reference.json', dict(bytes=len(packet), sha256=sha(packet)))


def schema() -> None:
    """Pin the complete initializer, index and scalar-storage clauses."""
    text = (ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md').read_text().replace('\r\n', '\n')
    begin, end = text.index('### 14.107 '), text.index('### 14.110 ')
    raw = (text[begin:end].rstrip() + '\n').encode()
    path = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw
    report('schema-reference.json', dict(issue=115, clauses=['14.107', '14.108', '14.109'], sha256=sha(raw)))


def run() -> bool:
    """Freeze all actual reader modules before import and record verified origins."""
    with tempfile.TemporaryDirectory(prefix='bessemer-simulation-elf-source-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        shared.compound.pin_reader(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(test) for test in (freeze_owned, freeze_native, schema))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('cfb', 'simulationtransfer', 'simulationelf', 'simulationelfrecord'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
