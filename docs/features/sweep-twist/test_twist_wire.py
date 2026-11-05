"""Keep the new wire oracle distinct from unextended production Source (#211)."""
from pathlib import Path
from typing import cast
import os
import struct
import unittest
import twist_wire_fixtures as fixture
from swarchive import ArchiveError


class SweepTwistWireTests(unittest.TestCase):
    def test_every_byte_and_ieee_pair_has_exact_payload_and_original_object_slots(self) -> None:
        rows = fixture.twist_cases()
        self.assertEqual(len(rows), 6144)
        for version, value, first, second, raw in rows:
            result, slots = fixture.read_twist(raw, version)
            self.assertEqual((slots, result['raw_08']), (3, value))
            self.assertEqual(struct.pack('<d', cast(float, result['raw_10'])), struct.pack('<Q', first))
            self.assertEqual(struct.pack('<d', cast(float, result['raw_18'])), struct.pack('<Q', second))
            self.assertEqual(raw[-17:], struct.pack('<BQQ', value, first, second))

    def test_unextended_original_source_still_rejects_the_fresh_class_after_extensions(self) -> None:
        raw = fixture.twist_record(255, 1, 0x7FF8000000001234)
        for version in fixture.VERSIONS:
            self.assertNotIn(fixture.CLASS, fixture.twist_archive(raw, version, False).readers)
            with self.assertRaises(ArchiveError) as rejected:
                fixture.read_twist(raw, version, False)
            self.assertIn(fixture.CLASS, str(rejected.exception))
            fixture.read_twist(raw, version, True)
            with self.assertRaises(ArchiveError):
                fixture.read_twist(raw, version, False)

    def test_schema_old_context_prefixes_and_trailing_bytes_remain_failures(self) -> None:
        for version in fixture.VERSIONS:
            raw = fixture.twist_record(128, 0x8000000000000000, 1)
            for end in range(len(raw)):
                self.assert_invalid_twist(raw[:end], version)
            with self.assertRaises(ArchiveError):
                fixture.read_twist(raw + bytes(1), version)
        for schema in (0, 2, 65535):
            with self.assertRaises(ArchiveError):
                fixture.read_twist(fixture.twist_record(2, 0, 1, schema), 19000)
        with self.assertRaises(ArchiveError):
            fixture.read_twist(fixture.twist_record(0, 0, 0), 0x27CB)

    def test_complete_packet_is_written_to_the_injected_fixture_root(self) -> None:
        destination = os.environ.get('BESSEMER_SWEEP_TWIST_FIXTURES')
        self.assertIsNotNone(destination)
        fixture.write_twist_fixtures(Path(cast(str, destination)))

    def assert_invalid_twist(self, raw: bytes, version: int) -> None:
        with self.assertRaises((ArchiveError, ValueError, AssertionError)):
            fixture.read_twist(raw, version)
