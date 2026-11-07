"""Reproduce complete imported/base-body fixtures, including kernel carriers and name edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_imported import freeze, CLASSES, MODES, VERSIONS, WORKSPACE
from imported_fixture_bytes import imported_object, base_suffix


class ImportedFixtureTests(unittest.TestCase):
    def test_regeneration_matches_all_owned_bytes_oracles_and_case_facts(self) -> None:
        """Exercise all constructors and verifiers; e.g. python -m unittest test_freeze_imported."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(WORKSPACE.parent/'inventor-interface/solidworks', destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(observed[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),
                    (WORKSPACE/'tests/fixtures/imported-bodies'/fixture.name).read_bytes(), fixture.name)

    def test_class_mode_and_vector_gate_coverage_retains_48_distinct_contexts(self) -> None:
        """Keep class inheritance distinct from the vector gate; e.g. base_suffix('moImported_c', 19000, True)."""
        variants = {(name, version, rich, mode) for name in CLASSES for version in VERSIONS
                    for rich in (False, True) for mode in MODES}
        self.assertEqual(len(variants), 48)
        self.assertEqual(base_suffix('moImported_c', 19000, True), b'')
        self.assertEqual(base_suffix('moBaseBody_c', 0x2bcc, True), b'')
        self.assertEqual(len(base_suffix('moBaseBody_c', 0x2bcd, True)), 16)

    def test_invalid_class_mode_and_inherited_slot_are_rejected(self) -> None:
        """Reject malformed fixture context; e.g. imported_object('moBaseBody_c', 19000, False, 'external', -1)."""
        for name, mode, slot in [('invalid', 'external', 0), ('moImported_c', 'invalid', 0),
                                  ('moBaseBody_c', 'compressed', -1)]:
            with self.assertRaisesRegex(ValueError, 'known class/mode'):
                imported_object(name, 19000, False, mode, slot)


if __name__ == '__main__':
    unittest.main()
