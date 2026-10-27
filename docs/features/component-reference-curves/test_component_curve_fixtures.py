"""Regenerate complete component-reference curve and wrapper identity fixtures (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_component_curves import freeze, REFERENCE, WORKSPACE


class ComponentCurveFixtureTests(unittest.TestCase):
    def test_complete_records_and_independent_id_edits_regenerate(self) -> None:
        """Compare every emitted byte, primitive span and map slot; e.g. python -m unittest test_component_curve_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'reads', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/component-reference-curves'/fixture.name).read_bytes())

    def test_version_transitions_unit_targets_and_distinct_equal_records_are_present(self) -> None:
        """Keep eleven versions and all signed ID/unsigned time branches; e.g. time 0xffffffff."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 198)
        self.assertEqual(facts['reads'], 11961)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(6)))
        self.assertEqual(len({row['version'] for row in facts['cases']}), 11)
        self.assertEqual([row['slots'] for row in facts['owned']], [99,101,105,101,103,107,105,107])


if __name__ == '__main__':
    unittest.main()
