"""Regenerate complete geometry-backup graphs and their independent raw-field edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_geometry_backups import freeze, REFERENCE, WORKSPACE


class GeometryBackupFixtureTests(unittest.TestCase):
    def test_complete_observations_aliases_and_independent_edits_regenerate(self) -> None:
        """Compare all emitted bytes, primitive spans and map slots; e.g. python -m unittest test_backup_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'reads', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/sketch-geometry-backups'/fixture.name).read_bytes())

    def test_four_classes_owner_modes_and_complete_handle_encodings_are_all_present(self) -> None:
        """Keep all direct layouts and distinct owner modes; e.g. mode -2, -1 and 0."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 174)
        self.assertEqual(facts['reads'], 3653)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(12)))
        self.assertEqual({row['label'].split('/')[2] for row in facts['cases'] if row['kind'] in (2, 5)}, {'-2', '-1', '0'})
        self.assertEqual([row['slots'] for row in facts['owned']], [99, 105, 99, 101, 99, 101, 99, 101, 105, 101, 101, 101])


if __name__ == '__main__':
    unittest.main()
