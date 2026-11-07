"""Regenerate complete radiated-edge graphs and verify declared-role case coverage (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_radiated_edges import freeze, REFERENCE, WORKSPACE


class RadiatedFixtureTests(unittest.TestCase):
    def test_complete_primitive_observations_and_independent_edits_regenerate(self) -> None:
        """Compare every emitted byte and map count; e.g. python -m unittest test_radiated_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'reads', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/radiated-edge-identifiers'/fixture.name).read_bytes())

    def test_null_self_external_and_declared_alias_cases_are_all_present(self) -> None:
        """Keep complete inheritance and both fresh/cached declared roles; e.g. styles 0, 1 and 2."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 18)
        self.assertEqual({row['kind'] for row in facts['cases']}, {0, 1, 2})
        self.assertEqual([row['slots'] for row in facts['owned']], [99, 103, 107, 107])
        self.assertEqual({row['version'] for row in facts['cases']}, {0x27cc, 19000, 0xffffffff})


if __name__ == '__main__':
    unittest.main()
