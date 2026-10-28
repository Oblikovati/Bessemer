"""Regenerate complete native-derived dimensions and actual collection identities (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_additional_dimensions import REFERENCE, WORKSPACE, freeze, fixture_family
from additional_dimension_bytes import edit_common_word


class AdditionalDimensionFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_owned_edits_regenerate(self) -> None:
        """Compare every observed byte, primitive and map slot; e.g. python -m unittest test_additional_dimension_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/additional-valued-dimensions'/fixture.name).read_bytes())

    def test_exact_classes_and_full_boundary_matrix_remain_present(self) -> None:
        """Keep three reviewed classes, seventeen versions and six identity/caller styles; e.g. fixed-transform presence and G2 legacy bounds."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),306)
        self.assertEqual(facts['reads'],56073)
        self.assertEqual(len(facts['owned']),21)

    def test_common_word_edit_requires_the_observed_original_prefix(self) -> None:
        """Reject a wrong splice boundary instead of editing arbitrary bytes; e.g. edit_common_word(bytes(100),19000)."""
        with self.assertRaisesRegex(ValueError,'expected common i32 -1'):
            edit_common_word(bytes(100),19000)


if __name__=='__main__':
    unittest.main()
