"""Regenerate complete reference-curve branches and independent signed-word edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_reference_curves import freeze,REFERENCE,WORKSPACE


class ReferenceCurveFixtureTests(unittest.TestCase):
    def test_complete_graphs_cached_array_facts_and_edit_regenerate(self) -> None:
        """Compare all bytes, primitive spans and parsed shared identities; e.g. python -m unittest test_reference_curve_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/reference-curves'/fixture.name).read_bytes())

    def test_all_three_branches_and_three_strict_version_gates_are_present(self) -> None:
        """Keep cached empty/nonempty arrays and positive extra bodies; e.g. 0x2b87 and 0x2b88."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),60);self.assertEqual(facts['reads'],20774)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(6)))
        self.assertEqual(len({row['version'] for row in facts['cases']}),10)
        self.assertEqual(len(facts['owned']),7)


if __name__=='__main__':
    unittest.main()
