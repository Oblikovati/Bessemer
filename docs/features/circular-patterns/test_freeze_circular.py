"""Reproduce all complete circular-pattern fixtures and validate local reader registration (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_circular import freeze, circular_variants, configure_cursor, WORKSPACE
from circular_fixture_bytes import circular_object
from object_fixture_observer import object_cursor, observe_object_body
from freeze_blank_part import source_module


class CircularFixtureTests(unittest.TestCase):
    def test_regeneration_matches_complete_owned_bytes_oracles_and_case_facts(self) -> None:
        """Exercise every builder and observer; e.g. python -m unittest test_freeze_circular."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(WORKSPACE.parent/'inventor-interface/solidworks', destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(observed[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),
                    (WORKSPACE/'tests/fixtures/circular-patterns'/fixture.name).read_bytes(), fixture.name)

    def test_variants_cover_both_geometry_gate_sides_and_invalid_options(self) -> None:
        """Retain 22 distinct contexts; e.g. len(circular_variants()) == 22."""
        variants = circular_variants()
        self.assertEqual(len(variants), 22)
        self.assertEqual(len(set(variants)), 22)
        self.assertTrue({0x3efa, 0x3efb} <= {version for version, *_ in variants})
        for line, flag, first_slot in [('invalid', 255, 0), ('null', -1, 0),
                                      ('line', 256, 0), ('temporary', 255, -1)]:
            with self.assertRaises(ValueError):
                circular_object(19000, False, False, line=line, flag=flag, first_slot=first_slot)

    def test_reader_registration_is_local_and_observation_requires_exact_map_and_eof(self) -> None:
        """Keep the independent reader's global registry untouched; e.g. object_cursor(source, raw, 19000, None)."""
        source = WORKSPACE.parent/'inventor-interface/solidworks'
        raw, slots = circular_object(19000, False, True)
        object_cursor(source, raw, 19000, lambda cursor: configure_cursor(cursor, source))
        objects = source_module('mfcobjects', source)
        before = dict(objects.ObjectArchive.readers)
        cursor = object_cursor(source, raw, 19000, lambda item: configure_cursor(item, source))
        self.assertIn('moTempAxisRef_w', cursor.readers)
        self.assertEqual(objects.ObjectArchive.readers, before)
        for changed, expected_slots in [(raw, slots+1), (raw+b'NEXT', slots)]:
            with self.assertRaisesRegex(ValueError, 'end/map'):
                observe_object_body(source, changed, expected_slots, 0, 19000, 'invalid')


if __name__ == '__main__':
    unittest.main()
