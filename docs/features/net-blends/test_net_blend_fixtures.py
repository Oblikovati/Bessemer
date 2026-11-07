"""Regenerate complete native-bound net-blend graphs and independent word edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_net_blends import FAMILY, REFERENCE, WORKSPACE, freeze


class NetBlendFixtureTests(unittest.TestCase):
    def test_complete_graphs_shared_targets_and_owned_edits_regenerate(self) -> None:
        """Require stable source, complete partitions and unchanged golden bytes; e.g. python -m unittest test_net_blend_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,FAMILY)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/net-blends'/fixture.name).read_bytes())

    def test_both_features_three_dependencies_and_independent_counts_are_present(self) -> None:
        """Keep nineteen versions, six branch styles and separate point counts; e.g. style 1 has N=0 and M=2."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),570)
        self.assertEqual(facts['reads'],205111)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(30)))
        self.assertEqual(len(facts['owned']),34)


if __name__=='__main__':
    unittest.main()
