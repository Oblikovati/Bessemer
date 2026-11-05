"""Independent complete carrier, lexical and corruption boundaries for Feature231."""
import gzip
import hashlib
import json
from pathlib import Path
import shutil
import tempfile
import unittest
import warnings
from archive_fixtures import (ElectricalCase, duplicate_members, expected_entries,
                              native_cases, owned_archive, owned_cases)
from freeze_fixtures import counted, fixture_packet, freeze, optional
from ini_reference import IniKind, ini_records
from zip_reference import check_replacement, inspect_zip, region

FEATURE = Path(__file__).resolve().parent
FIXTURES = FEATURE.parents[2] / 'tests/fixtures/electrical-projects'


def cases() -> tuple[ElectricalCase, ...]:
    """Capture only the intentional duplicate-name warning; e.g. cases()."""
    with warnings.catch_warnings(record=True) as captured:
        warnings.simplefilter('always')
        owned = owned_cases()
    assert all('Duplicate name' in str(item.message) for item in captured)
    return native_cases(FIXTURES, FEATURE / 'native-sources.json') + owned


class ElectricalFixtureTests(unittest.TestCase):
    def test_all_native_and_owned_complete_partitions_match_every_independent_entry(self) -> None:
        all_cases = cases()
        counts = tuple(len(expected_entries(case)[0]) for case in all_cases)
        self.assertEqual(counts, (12, 12, 12, 13, 0, 4, 1, 2))
        for case in all_cases:
            expected, comment = expected_entries(case)
            parsed = inspect_zip(case.raw, comment, len(expected))
            for actual, wanted in zip(parsed, expected):
                self.assertEqual((actual.name, actual.flags, actual.method, actual.crc, actual.payload,
                                  actual.extra, actual.comment),
                                 (wanted.name, wanted.flags, wanted.method, wanted.crc, wanted.payload,
                                  wanted.extra, wanted.comment))

    def test_archive_comment_signature_fixture_keeps_the_constructed_member_and_full_comment(self) -> None:
        case = cases()[6]
        expected, comment = expected_entries(case)
        self.assertIn(b'PK\x05\x06', comment)
        parsed = inspect_zip(case.raw, comment, 1)
        self.assertEqual(parsed[0].payload, expected[0].payload)
        self.assertEqual(parsed[0].name, 'Archive.ini')

    def test_owned_edits_have_exact_opaque_headers_and_encoded_unselected_members(self) -> None:
        entries = duplicate_members()
        with warnings.catch_warnings(record=True):
            before = owned_archive(entries, 0, b'comment')
            changed = ((entries[0][0], entries[0][1] + b'\xa5'),) + entries[1:]
            after = owned_archive(changed, 0, b'comment')
        original = inspect_zip(before, b'comment', 4)
        result = inspect_zip(after, b'comment', 4)
        check_replacement(original, result, 0)
        self.assertEqual(result[3].name, 'C:\\folder\\opaque.bin')

    def test_lexical_duplicates_empty_keys_comments_sections_and_extra_carriage_returns(self) -> None:
        raw = b'[S]\r\r\n=first\rK= second=part \nK=other\n#k=v\n #k=v\n[]\n'
        records = ini_records(raw)
        self.assertIsNotNone(records)
        assert records is not None
        self.assertEqual(b''.join(record.raw for record in records), raw)
        pairs = tuple((record.key, record.value) for record in records if record.kind == IniKind.FIELD)
        self.assertEqual(pairs, ((b'', b'first'), (b'K', b' second=part '),
                                 (b'K', b'other'), (b' #k', b'v')))
        self.assertEqual(records[-1].section, b'')

    def test_all_ascii_splitline_controls_follow_the_actual_lexical_reference(self) -> None:
        raw = b'[S]\r\r\nK=A\vK=B\fK=C\x1cK=D\x1dK=E\x1eK=F\rK=G\n'
        records = ini_records(raw)
        assert records is not None
        self.assertEqual(b''.join(record.raw for record in records), raw)
        values = tuple(record.value for record in records if record.kind == IniKind.FIELD)
        self.assertEqual(values, (b'A\v', b'B\f', b'C\x1c', b'D\x1d', b'E\x1e', b'F', b'G'))
        self.assertIsNone(ini_records(b'\xff'))
        self.assertEqual(ini_records(b''), ())

    def test_counted_optional_fields_preserve_null_empty_and_nul_bytes(self) -> None:
        self.assertEqual(counted(b'\0x'), b'\x02\0\0\0\0x')
        self.assertEqual(optional(None), b'\xff\xff\xff\xff')
        self.assertEqual(optional(b''), bytes(4))
        for start, length in ((-1, 1), (0, 4), (4, 0)):
            with self.assertRaises(AssertionError):
                region(b'abc', start, length)

    def test_corrupt_payload_crc_and_nonzero_disk_fields_are_rejected(self) -> None:
        case = cases()[5]
        parsed = inspect_zip(case.raw, b'duplicates', 4)
        changed = bytearray(case.raw)
        changed[parsed[0].local_offset + len(parsed[0].local_header)] ^= 1
        with self.assertRaises(AssertionError):
            inspect_zip(bytes(changed), b'duplicates', 4)
        changed = bytearray(case.raw)
        changed[len(changed) - 22 - len(b'duplicates') + 4] = 1
        with self.assertRaises(AssertionError):
            inspect_zip(bytes(changed), b'duplicates', 4)

    def test_frozen_packet_integrity_and_injected_root_regeneration(self) -> None:
        proof = json.loads((FIXTURES / 'archive-cases-integrity.json').read_text())
        compressed = (FIXTURES / 'archive-cases.bin.gz').read_bytes()
        raw = gzip.decompress(compressed)
        self.assertEqual(hashlib.sha256(raw).hexdigest(), proof['raw_sha256'])
        self.assertEqual(hashlib.sha256(compressed).hexdigest(), proof['compressed_sha256'])
        with tempfile.TemporaryDirectory() as temporary, warnings.catch_warnings(record=True):
            output = Path(temporary)
            for case in native_cases(FIXTURES, FEATURE / 'native-sources.json'):
                shutil.copyfile(FIXTURES / case.name, output / case.name)
            self.assertEqual(fixture_packet(output, FEATURE / 'native-sources.json'), raw)
            freeze(output, FEATURE / 'native-sources.json')
            self.assertEqual((output / 'archive-cases.bin.gz').read_bytes(), compressed)


if __name__ == '__main__':
    unittest.main()
