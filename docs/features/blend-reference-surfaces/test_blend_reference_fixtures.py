"""Regenerate complete C-382 classes, distinct native spellings and both edit goldens (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_blend_references import freeze,REFERENCE,WORKSPACE


class BlendReferenceFixtureTests(unittest.TestCase):
    def test_complete_graphs_both_edits_and_native_identities_regenerate(self) -> None:
        """Compare bytes, spans, slot counts and parsed aliases; e.g. python -m unittest test_blend_reference_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/blend-reference-surfaces'/fixture.name).read_bytes())

    def test_exact_component_spelling_surface_tail_and_inherited_boundaries_are_present(self) -> None:
        """Keep fresh/cached wrappers, full data and empty keeper identities; e.g. component styles 4 and 5."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),270);self.assertEqual(facts['reads'],35388)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(18)))
        self.assertEqual(len({row['version'] for row in facts['cases']}),15)
        self.assertEqual(len(facts['owned']),20)


if __name__=='__main__':
    unittest.main()
