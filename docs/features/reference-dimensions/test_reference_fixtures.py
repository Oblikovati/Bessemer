"""Repeat independent reference-dimension assembly and observations (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_reference_dimensions import freeze, REFERENCE, WORKSPACE, VERSIONS


class ReferenceFixtureTests(unittest.TestCase):
    def test_every_complete_observation_and_owned_edit_regenerates(self) -> None:
        """Require byte-for-byte regeneration of every fixture; e.g. python -m unittest test_reference_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'reads', 'sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/reference-dimensions'/fixture.name).read_bytes())

    def test_all_version_boundaries_and_complete_classes_are_covered(self) -> None:
        """Keep both sides of every documented wire gate; e.g. 0x46c6 and 0x46c7."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 250)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(5)))
        self.assertEqual({row['version'] for row in facts['cases']}, set(VERSIONS))
        self.assertEqual(len(facts['owned']), 5)


if __name__ == '__main__':
    unittest.main()
