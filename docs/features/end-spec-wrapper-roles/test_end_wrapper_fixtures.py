"""Regenerate complete native-bound end-specification wrapper graphs without Bessemer-generated expectations (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_end_wrappers import REFERENCE, WORKSPACE, freeze, fixture_family, additional_witnesses


class EndWrapperFixtureTests(unittest.TestCase):
    def test_all_graphs_and_owned_edits_regenerate_exactly(self) -> None:
        """Compare primitive partitions, source aliases and all frozen bytes; e.g. python -m unittest test_end_wrapper_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/end-spec-wrapper-roles'/fixture.name).read_bytes())

    def test_nineteen_wrappers_at_three_callers_have_complete_versions_styles_and_signed_edits(self) -> None:
        """Keep all native shapes and distinct equal roots; e.g. styles 2 and 5."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),1140)
        self.assertEqual(facts['reads'],169376)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(114)))
        self.assertEqual(len(facts['owned']),133)

    def test_null_and_complete_unrelated_witnesses_regenerate_exactly(self) -> None:
        """Keep null preservation and expected-class rejection independent; e.g. three null and six unrelated records."""
        expected=json.loads(Path(__file__).with_name('additional-reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=additional_witnesses(REFERENCE,destination)
            self.assertEqual(actual,expected)
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/end-spec-wrapper-roles'/fixture.name).read_bytes())


if __name__=='__main__':
    unittest.main()
