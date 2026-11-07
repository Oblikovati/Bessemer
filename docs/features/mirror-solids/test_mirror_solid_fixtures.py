"""Reproduce all complete mirror-solid graphs from independent bytes (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_mirror_solids import freeze, REFERENCE, WORKSPACE


class MirrorSolidFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_independent_edit_regenerate(self) -> None:
        """Compare bytes, primitive spans and exact map sizes; e.g. python -m unittest test_mirror_solid_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/mirror-solids'/fixture.name).read_bytes())

    def test_all_strict_version_boundaries_and_complete_nested_types_are_present(self) -> None:
        """Require every gated edge, empty/nonempty list and distinct root graph; e.g. 0x3760 and 0x3761."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 198)
        self.assertEqual(facts['reads'], 23487)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(18)))
        self.assertEqual(len({row['version'] for row in facts['cases']}), 11)
        self.assertEqual(len(facts['owned']), 19)


if __name__ == '__main__':
    unittest.main()
