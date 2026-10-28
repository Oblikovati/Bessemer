"""Regenerate complete four-field wrapper regressions and independent edit goldens (#46)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_intersection_wrappers import REFERENCE, WORKSPACE, freeze_family


class IntersectionWrapperFixtureTests(unittest.TestCase):
    def test_all_complete_graphs_and_owned_edits_regenerate(self) -> None:
        """Compare exact bytes, primitive partitions and actual identities; e.g. python -m unittest test_intersection_wrapper_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze_family(REFERENCE,destination)
            for key in ('cases','owned','reads','packet_sha256','unrelated'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/intersection-wrapper-roles'/fixture.name).read_bytes())

    def test_all_four_roles_cover_fresh_cached_null_and_wrong_class_shapes(self) -> None:
        """Keep the finite family and inherited boundary matrix explicit; e.g. nineteen classes and sixteen versions."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),1824)
        self.assertEqual(facts['reads'],978482)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(114)))
        self.assertEqual(len(facts['owned']),133)
        self.assertEqual(len(facts['unrelated']),4)


if __name__=='__main__':
    unittest.main()
