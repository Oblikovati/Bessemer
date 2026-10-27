"""Regenerate complete face-blend item/pair fixtures and independent point edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_face_blend import freeze, REFERENCE, WORKSPACE


class FaceBlendFixtureTests(unittest.TestCase):
    def test_complete_reference_records_and_point_edits_regenerate(self) -> None:
        """Compare every byte, observed primitive and map slot; e.g. python -m unittest test_face_blend_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'reads', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/face-blend-items'/fixture.name).read_bytes())

    def test_both_classes_presence_bytes_versions_and_identity_modes_are_present(self) -> None:
        """Keep null/fresh/shared pointers and zero versus all nonzero branches; e.g. presence 128."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 120)
        self.assertEqual(facts['reads'], 4947)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(6)))
        self.assertEqual({row['label'].split('/')[-1] for row in facts['cases']}, {'0', '1', '128', '255'})
        self.assertEqual([row['slots'] for row in facts['owned']], [99, 103, 107, 99, 105, 111, 107, 111])


if __name__ == '__main__':
    unittest.main()
