"""Regenerate complete field-specific wrapper graphs and explicit unrelated-body rejection input (#46)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_sketch_wrappers import REFERENCE, WORKSPACE, freeze


class SketchWrapperFixtureTests(unittest.TestCase):
    def test_all_complete_bodies_aliases_and_edits_regenerate(self) -> None:
        """Keep independent primitive partitions and golden bytes stable; e.g. python -m unittest test_sketch_wrapper_fixtures."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination)
            for key in ('cases','owned','reads','packet_sha256','unrelated_fresh','source_role_oracle'):
                self.assertEqual(actual[key],expected[key])
            for p in destination.iterdir():
                self.assertEqual(p.read_bytes(),(WORKSPACE/'tests/fixtures/sketch-wrapper-roles'/p.name).read_bytes())

    def test_all_existing_wrapper_classes_and_role_oracle_limits_are_recorded(self) -> None:
        """Separate source body parsing from Rust role enforcement; e.g. a valid NodeName body still has the wrong field type."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),1188)
        self.assertEqual(facts['reads'],71568)
        self.assertEqual({row['kind'] for row in facts['cases']},set(range(108)))
        self.assertEqual(len(facts['owned']),126)
        self.assertFalse(facts['source_role_oracle'])
        self.assertEqual(facts['unrelated_fresh']['slots'],5)


if __name__=='__main__':
    unittest.main()
