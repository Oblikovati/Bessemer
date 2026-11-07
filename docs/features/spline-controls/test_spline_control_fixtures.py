"""Regenerate complete spline-control observations and pin earlier sketch-builder defaults (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_spline_controls import freeze, REFERENCE, WORKSPACE
from sketch_tail_fixture_bytes import WireMap, ordinary


class SplineControlFixtureTests(unittest.TestCase):
    def test_complete_graphs_and_direct_handle_edits_regenerate(self) -> None:
        """Compare every emitted byte, primitive and map slot; e.g. python -m unittest test_spline_control_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'reads', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/spline-controls'/fixture.name).read_bytes())

    def test_prechange_sketch_builder_defaults_remain_byte_identical(self) -> None:
        """Preserve complete legacy sketches after adding a typed user-object callback; e.g. ordinary(mapping, 19000, True)."""
        baseline = json.loads(Path(__file__).with_name('legacy-builder-baselines.json').read_text())
        for case in baseline['cases']:
            mapping = WireMap()
            raw = mapping.new('sgSketch', lambda: ordinary(mapping, case['version'], case['rich'], indicator=case['indicator']))
            self.assertEqual(raw.hex(), case['bytes'])
            self.assertEqual(mapping.slot+1, case['slots'])
        self.assertEqual(len(baseline['cases']), 16)

    def test_all_control_classes_sketch_pointer_modes_and_handle_escapes_are_present(self) -> None:
        """Cover each complete direct layout and finite owner role; e.g. three classes and four styles."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 72)
        self.assertEqual(facts['reads'], 12297)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(12)))
        self.assertEqual([row['slots'] for row in facts['owned']], [99, 101, 99, 101, 99, 101, 101, 101, 101])


if __name__ == '__main__':
    unittest.main()
