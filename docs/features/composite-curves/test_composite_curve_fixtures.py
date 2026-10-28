"""Regenerate independent complete composite curves and owned edit goldens (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_composite_curves import REFERENCE, WORKSPACE, freeze, fixture_family


class CompositeCurveFixtureTests(unittest.TestCase):
    def test_all_complete_graphs_and_owned_edits_regenerate(self) -> None:
        """Compare exact bytes, primitive partitions and identities; e.g. python -m unittest test_composite_curve_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/composite-curves'/fixture.name).read_bytes())

    def test_present_and_suppressed_bodies_cross_all_inherited_gates(self) -> None:
        """Keep native independent counts, raw high bits and all shared-root styles; e.g. styles 2 and 5."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),132)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(12)))
        self.assertEqual(len(facts['owned']),14)


if __name__=='__main__':
    unittest.main()
