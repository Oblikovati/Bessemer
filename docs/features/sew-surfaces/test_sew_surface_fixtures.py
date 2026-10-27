"""Regenerate complete native-bound sew surfaces and their real keeper callers (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_sew_surfaces import FAMILY, REFERENCE, WORKSPACE, freeze


class SewSurfaceFixtureTests(unittest.TestCase):
    def test_all_graphs_target_aliases_and_owned_flag_edit_regenerate(self) -> None:
        """Keep primitive partitions and complete byte streams independent; e.g. python -m unittest test_sew_surface_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,FAMILY)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for p in destination.iterdir():
                self.assertEqual(p.read_bytes(),(WORKSPACE/'tests/fixtures/sew-surfaces'/p.name).read_bytes())

    def test_two_exact_classes_and_real_keeper_owners_are_present(self) -> None:
        """Keep null/present arrays, raw IEEE bits and nonnull body-header callers; e.g. keeper style 5."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),108)
        self.assertEqual(facts['reads'],23619)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(12)))
        self.assertEqual(len(facts['owned']),13)


if __name__=='__main__':
    unittest.main()
