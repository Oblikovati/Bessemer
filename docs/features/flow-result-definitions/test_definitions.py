"""Independent exact framing equations and strict malformed-definition cases."""
import json
import struct
import tempfile
import unittest
from pathlib import Path
import definition_requests as requests
import freeze_definitions as freeze
from swarchive import ArchiveError


def check_definition(value: dict) -> None:
    body = value['definition']
    version = body['version']
    arrays = [body['first_array'], body['second_array']] + body['extra_arrays']
    present = [array for array in arrays if array is not None]
    assert len(present) == (0, 1, 2, 4)[version]
    assert (body['allocation_byte'] is not None) == (version >= 2)
    expected = 4 + int(version >= 2)
    for array in present:
        assert array['marker'] == 1
        assert array['count'] == len(array['patterns'])
        assert array['end'] - array['start'] == 12 + 4 * array['count']
        expected += 12 + 4 * array['count']
    assert body['end'] - body['start'] == expected
    assert value['guid_raw'] == requests.reference.IMS_GUID_RAW


def check_group(group: dict, ordinal: int) -> None:
    assert group['count'] == len(group['objects'])
    assert ordinal == 0 or group['count'] == 0
    extent = 4 + 4 * group['count']
    for obj in group['objects']:
        check_definition(obj)
        extent += 16 + obj['definition']['end'] - obj['definition']['start']
    assert group['end'] - group['start'] == extent


def accumulate_fields(obj: dict, identifiers: set[int], patterns: list[set[int]], flags: dict[int, set[int]]) -> None:
    identifiers.add(obj['identifier_i32'] & 0xffffffff)
    body = obj['definition']
    if body['version'] >= 2:
        flags[body['version']].add(body['allocation_byte'])
    for ordinal, array in enumerate([body['first_array'], body['second_array']] + body['extra_arrays']):
        if array is not None:
            patterns[ordinal].update(array['patterns'])


def small_body() -> bytes:
    return requests.decoded([(0xffffffff, requests.definition(3, [[0, 0xffffffff], [7], [], [8]], 255))])


class DefinitionFixtureTests(unittest.TestCase):
    def test_all_owned_framing_equations_and_complete_flags(self):
        for name, _, stored in requests.cases():
            value = requests.parse(stored)
            self.assertEqual(value['version'], 1, name)
            self.assertEqual(len(value['groups']), 5, name)
            self.assertEqual(value['end'], len(requests.reference.inflate(stored)), name)
            self.assertTrue(value['complete_decoded_framing'])
            self.assertFalse(value['native_runtime_state_reproduced'])
            for ordinal, group in enumerate(value['groups']):
                check_group(group, ordinal)

    def test_scalar_bits_arrays_flags_versions_and_duplicate_order(self):
        identifiers = set()
        patterns = [set() for _ in range(4)]
        flags = {2: set(), 3: set()}
        for _, _, stored in requests.cases():
            for obj in requests.parse(stored)['groups'][0]['objects']:
                accumulate_fields(obj, identifiers, patterns, flags)
        self.assertTrue(set(requests.PATTERNS) <= identifiers)
        self.assertTrue(all(set(requests.PATTERNS) <= values for values in patterns))
        self.assertEqual(flags, {2: set(range(256)), 3: set(range(256))})

    def test_native_projection_and_payload_provenance(self):
        rows, proof = requests.native_cases()
        self.assertEqual(len(rows), 1)
        value = requests.parse(rows[0][2])
        body = value['groups'][0]['objects'][0]['definition']
        self.assertEqual((body['version'], body['allocation_byte']), (2, 0))
        self.assertEqual([body[key]['count'] for key in ('first_array', 'second_array')], [3876, 4056])
        self.assertEqual((proof['record_start'], proof['record_end'], proof['stored_bytes']), (65378, 72157, 6767))

    def test_all_small_proper_prefixes_and_suffix_are_rejected(self):
        raw = small_body()
        for size in range(len(raw)):
            with self.assertRaises(ArchiveError):
                requests.reference.read_decoded(raw[:size])
        with self.assertRaises(ArchiveError):
            requests.reference.read_decoded(raw + b'x')

    def test_versions_markers_guids_and_counts_are_strict(self):
        raw = small_body()
        corruptions = [(0, b'\2'), (9, bytes(16)), (25, struct.pack('<i', 4)),
                       (29, struct.pack('<i', 0)), (33, struct.pack('<Q', 1 << 31)),
                       (1, struct.pack('<i', -1)), (1, struct.pack('<i', 0x7fffffff))]
        for offset, replacement in corruptions:
            changed = raw[:offset] + replacement + raw[offset + len(replacement):]
            with self.assertRaises(ArchiveError):
                requests.reference.read_decoded(changed)
        for ordinal in range(1, 5):
            empty = bytearray(requests.decoded([]))
            struct.pack_into('<i', empty, 1 + ordinal * 4, 1)
            with self.assertRaises(ArchiveError):
                requests.reference.read_decoded(empty)

    def test_every_guid_bit_change_and_duplicate_identifier_position(self):
        raw = small_body()
        for bit in range(128):
            changed = bytearray(raw)
            changed[9 + bit // 8] ^= 1 << (bit % 8)
            with self.assertRaises(ArchiveError):
                requests.reference.read_decoded(changed)
        rows = requests.shape_cases()
        stored = next(stored for name, _, stored in rows if name == 'objects-257')
        objects = requests.parse(stored)['groups'][0]['objects']
        self.assertEqual([obj['identifier_i32'] for obj in objects], [7 if i % 2 == 0 else -1 for i in range(257)])
        self.assertEqual([obj['definition']['version'] for obj in objects], [i % 4 for i in range(257)])

    def test_exact_compression_and_record_identity(self):
        stored = requests.zlib.compress(small_body())
        corrupt = bytearray(stored)
        corrupt[-1] ^= 1
        for changed in (stored[:-1], stored + b'x', stored + stored, bytes(corrupt)):
            with self.assertRaises(ArchiveError):
                requests.parse(changed)
        for record in (dict(identifier=501, size=len(stored), payload=stored),
                       dict(identifier=500, size=len(stored) + 1, payload=stored)):
            with self.assertRaises(ArchiveError):
                requests.reference.read_record(record)

    def test_regeneration_matches_frozen_packet_and_manifest(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            freeze.freeze(root)
            for relative in ('tests/fixtures/flow-result-definitions/record500.bin.gz',
                             'docs/features/flow-result-definitions/baseline-verification.json'):
                self.assertEqual((root / relative).read_bytes(), (requests.ROOT / relative).read_bytes())
        proof = json.loads((requests.ROOT / 'docs/features/flow-result-definitions/baseline-verification.json').read_text(encoding='utf-8'))
        self.assertLessEqual(proof['bytes'], 16_000_000)


if __name__ == '__main__':
    unittest.main()
