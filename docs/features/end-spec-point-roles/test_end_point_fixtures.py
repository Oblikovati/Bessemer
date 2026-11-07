"""Regenerate independent point-role graphs and preserve all pre-injection builder defaults (#46)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_end_point_roles import freeze, REFERENCE, WORKSPACE
from body_feature_fixture_bytes import WireMap, end_spec, revolution_end, tagged


class EndPointFixtureTests(unittest.TestCase):
    def test_complete_observations_and_independent_edits_regenerate(self) -> None:
        """Compare every byte, primitive and map count; e.g. python -m unittest test_end_point_fixtures."""
        expected = json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary);actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'reads', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/end-spec-point-roles'/fixture.name).read_bytes())

    def test_original_complete_builder_defaults_match_owned_prechange_baselines(self) -> None:
        """Keep all existing caller bytes after adding typed point callbacks; e.g. end_spec(mapping, 19000, True)."""
        baseline = json.loads(Path(__file__).with_name('legacy-builder-baselines.json').read_text())
        for case in baseline['cases']:
            if case['name'] in ('moEndSpec_c', 'moRevEndSpec_c'):
                mapping = WireMap();serializer = end_spec if case['name'] == 'moEndSpec_c' else revolution_end
                raw = mapping.new(case['name'], lambda: serializer(mapping, case['version'], case['rich']))
                self.assertEqual(mapping.slot+1, case['slots'])
            else:
                raw = tagged(case['version'], case['name'], case['rich'])
            self.assertEqual(raw.hex(), case['bytes'])
        self.assertEqual(len(baseline['cases']), 32)

    def test_positive_point_family_and_unrelated_role_negatives_are_separate(self) -> None:
        """Require all owners, classes, pointer modes and version gates; e.g. 72 positive and 24 negative graphs."""
        facts = json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']), 96)
        self.assertEqual(sum(row['kind'] % 8 // 2 < 3 for row in facts['cases']), 72)
        self.assertEqual({row['kind'] for row in facts['cases']}, set(range(16)))
        self.assertEqual(sum(row['valid'] for row in facts['owned']), 8)
        self.assertEqual({row['slots'] for row in facts['owned']}, {107})


if __name__ == '__main__':
    unittest.main()
