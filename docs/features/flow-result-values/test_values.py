"""Independent paired-value extents, raw patterns and strict context contracts."""
import copy
import struct
import tempfile
import unittest
from pathlib import Path
import value_requests as requests
import freeze_values as freeze
from swarchive import ArchiveError


def sample() -> tuple[bytes, bytes]:
    definitions = requests.context([0])
    body = requests.value_body((255, 128, 254), [[77], [0x7ff8000000000042], [88]], [list(range(18))])
    return definitions, b'\2' + body


def read(raw: bytes, definitions: bytes) -> dict:
    return requests.reference.read_decoded(raw, definitions=requests.definitions.parse(definitions), fresh_objects=True)


def check_object(value: dict) -> None:
    assert value['version'] == 3
    extent = 7
    for flag, key in (('first_flag_byte', 'optional_538'), ('last_flag_byte', 'optional_690')):
        assert (value[flag] != 0) == (value[key] is not None)
    arrays = [value[key] for key in ('optional_538', 'main_418', 'optional_690') if value[key] is not None]
    for array in arrays:
        assert array['marker'] == 1 and array['count'] == len(array['patterns'])
        assert array['end'] - array['start'] == 12 + 8 * array['count']
        extent += 12 + 8 * array['count']
    assert (value['bl_flag_byte'] != 0) == (value['bl'] is not None)
    if value['bl'] is not None:
        extent += check_bl(value['bl'])
    assert value['end'] - value['start'] == extent


def check_bl(bl: dict) -> int:
    assert bl['version'] == 0 and bl['native_state_byte_after_load'] == 0
    assert bl['count'] == len(bl['rows']) and all(len(row) == 18 for row in bl['rows'])
    extent = 12 + 144 * bl['count']
    assert bl['end'] - bl['start'] == extent
    return extent


def accumulate_patterns(value: dict, flags: list[set[int]], patterns: list[set[int]]) -> None:
    for ordinal, key in enumerate(('first_flag_byte', 'bl_flag_byte', 'last_flag_byte')):
        flags[ordinal].add(value[key])
    for ordinal, key in enumerate(('optional_538', 'main_418', 'optional_690')):
        if value[key] is not None:
            patterns[ordinal].update(value[key]['patterns'])
    if value['bl'] is not None:
        accumulate_bl(value['bl']['rows'], patterns)


def accumulate_bl(rows: list[tuple[int, ...]], patterns: list[set[int]]) -> None:
    for row in rows:
        for column, pattern in enumerate(row):
            patterns[column + 3].add(pattern)


def assert_rejected(test: unittest.TestCase, raw: bytes, definitions: bytes) -> None:
    with test.assertRaises(ArchiveError):
        read(raw, definitions)


class ValueFixtureTests(unittest.TestCase):
    def test_complete_frames_and_independent_extent_equations(self):
        for _, _, definitions, stored in requests.cases():
            value = requests.parse(definitions, stored)
            self.assertEqual(value['version'], 2)
            self.assertTrue(value['complete_decoded_framing'] and value['fresh_objects_assumed'])
            self.assertFalse(value['native_runtime_state_reproduced'] or value['engineering_columns_assigned'])
            self.assertEqual(value['end'], len(requests.reference.inflate(stored)))
            extent = 1
            for obj in value['objects']:
                check_object(obj)
                self.assertEqual(obj['start'], extent)
                extent = obj['end']
            self.assertEqual(extent, value['end'])

    def test_every_flag_byte_and_all_raw_patterns_reach_every_column(self):
        flags = [set() for _ in range(3)]
        patterns = [set() for _ in range(21)]
        for _, _, definitions, stored in requests.cases():
            for obj in requests.parse(definitions, stored)['objects']:
                accumulate_patterns(obj, flags, patterns)
        self.assertEqual(flags, [set(range(256))] * 3)
        self.assertTrue(all(set(requests.wides.RAW_PATTERNS) <= values for values in patterns))

    def test_native_pair_and_both_carrier_provenances(self):
        rows, proof = requests.native_cases()
        value = requests.parse(rows[0][2], rows[0][3])
        obj = value['objects'][0]
        self.assertEqual((obj['identifier_i32'], obj['first_flag_byte'], obj['bl_flag_byte'], obj['last_flag_byte']), (0, 0, 1, 1))
        self.assertEqual((obj['main_418']['count'], obj['optional_690']['count'], obj['bl']['count']), (4056, 180, 3876))
        self.assertEqual(value['end'], 592076)
        self.assertEqual((proof['record_start'], proof['record_end'], proof['stored_bytes']), (1256070, 1593852, 337770))
        self.assertNotEqual(proof['source_sha256'], proof['definitions']['source_sha256'])

    def test_fresh_objects_and_exact_definition_profile_are_required(self):
        definitions, raw = sample()
        context = requests.definitions.parse(definitions)
        for fresh in (False, 1, None, 'fresh'):
            with self.assertRaises(ArchiveError):
                requests.reference.read_decoded(raw, definitions=context, fresh_objects=fresh)
        for version in (0, 1, 3, 4):
            changed = copy.deepcopy(context)
            changed['groups'][0]['objects'][0]['definition']['version'] = version
            with self.assertRaises(ArchiveError):
                requests.reference.read_decoded(raw, definitions=changed, fresh_objects=True)
        for flag in range(1, 256):
            changed = copy.deepcopy(context)
            changed['groups'][0]['objects'][0]['definition']['allocation_byte'] = flag
            with self.assertRaises(ArchiveError):
                requests.reference.read_decoded(raw, definitions=changed, fresh_objects=True)

    def test_incomplete_wrong_guid_and_collection_contexts_are_rejected(self):
        definitions, raw = sample()
        context = requests.definitions.parse(definitions)
        wrong_guid = copy.deepcopy(context)
        wrong_guid['groups'][0]['objects'][0]['guid_raw'] = bytes(16)
        wrong_count = copy.deepcopy(context)
        wrong_count['groups'][0]['count'] = 2
        secondary = copy.deepcopy(context)
        secondary['groups'][1] = copy.deepcopy(context['groups'][0])
        candidates = [dict(context, version=0), dict(context, complete_decoded_framing=False),
                      dict(context, groups=context['groups'][:4]), wrong_guid, wrong_count, secondary]
        for changed in candidates:
            with self.assertRaises(ArchiveError):
                requests.reference.read_decoded(raw, definitions=changed, fresh_objects=True)

    def test_object_counts_duplicates_and_definition_arrays_are_independent(self):
        rows = requests.object_cases()
        _, _, definitions, stored = next(row for row in rows if row[0] == 'objects-257')
        values = requests.parse(definitions, stored)['objects']
        self.assertEqual([obj['identifier_i32'] for obj in values], [7 if i % 2 == 0 else -1 for i in range(257)])
        self.assertEqual([obj['main_418']['patterns'] for obj in values], [(i,) for i in range(257)])
        _, _, definitions, stored = next(row for row in rows if row[0] == 'identifier-00000000')
        body = requests.definitions.parse(definitions)['groups'][0]['objects'][0]['definition']
        self.assertEqual((body['first_array']['count'], body['second_array']['count']), (3, 7))
        self.assertEqual(requests.parse(definitions, stored)['objects'][0]['main_418']['count'], 1)

    def test_truncations_suffixes_and_body_versions_are_strict(self):
        definitions, raw = sample()
        for length in range(len(raw)):
            with self.assertRaises(ArchiveError):
                read(raw[:length], definitions)
        obj = read(raw, definitions)['objects'][0]
        for offset, replacement in ((0, b'\1'), (1, struct.pack('<i', 4)),
                                    (obj['bl']['start'], struct.pack('<i', 1)), (len(raw), b'x')):
            changed = raw[:offset] + replacement + raw[offset + len(replacement):]
            with self.assertRaises(ArchiveError):
                read(changed, definitions)

    def test_shared_markers_counts_and_bl_native_transfer_limits(self):
        definitions, raw = sample()
        obj = read(raw, definitions)['objects'][0]
        for key in ('optional_538', 'main_418', 'optional_690'):
            offset = obj[key]['start']
            for replacement in (struct.pack('<i', 0), struct.pack('<i', -1)):
                assert_rejected(self, raw[:offset] + replacement + raw[offset + 4:], definitions)
            for count in (1 << 31, 0xffffffffffffffff):
                changed = raw[:offset + 4] + struct.pack('<Q', count) + raw[offset + 12:]
                assert_rejected(self, changed, definitions)
        offset = obj['bl']['start'] + 4
        for count in (2, 1 << 31, 0xffffffffffffffff):
            with self.assertRaises(ArchiveError):
                read(raw[:offset] + struct.pack('<Q', count) + raw[offset + 8:], definitions)

    def test_exact_zlib_framing_and_record_wrapper_identity(self):
        definitions, raw = sample()
        stored = requests.zlib.compress(raw)
        corrupt = bytearray(stored)
        corrupt[-1] ^= 1
        for payload in (stored[:-1], stored + b'x', stored + stored, bytes(corrupt)):
            with self.assertRaises(ArchiveError):
                requests.parse(definitions, payload)
        for identifier, size in ((500, len(stored)), (501, len(stored) + 1)):
            with self.assertRaises(ArchiveError):
                requests.reference.read_record(dict(identifier=identifier, size=size, payload=stored),
                                               definitions=requests.definitions.parse(definitions), fresh_objects=True)

    def test_regeneration_matches_owned_packet_and_verification(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            freeze.freeze(root)
            for relative in ('tests/fixtures/flow-result-values/record501.bin.gz',
                             'docs/features/flow-result-values/baseline-verification.json'):
                self.assertEqual((root / relative).read_bytes(), (requests.ROOT / relative).read_bytes())


if __name__ == '__main__':
    unittest.main()
