"""Reproduce complete Toolbox records and independent Unicode edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_toolbox import freeze, WORKSPACE, VERSIONS
from toolbox_fixture_bytes import toolbox_object, FLOAT_BITS, ansi_string


class ToolboxFixtureTests(unittest.TestCase):
    def test_regeneration_matches_owned_bytes_oracles_and_all_case_facts(self) -> None:
        """Exercise both reference readers and all constructors; e.g. python -m unittest test_freeze_toolbox."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(WORKSPACE.parent/'inventor-interface/solidworks', destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(observed[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),
                    (WORKSPACE/'tests/fixtures/configuration-toolbox'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_keeps_both_classes_versions_and_every_primitive(self) -> None:
        """Keep 72 class/collection/float cases; e.g. 19000 in VERSIONS."""
        self.assertTrue({0x27cc, 19000} <= set(VERSIONS))
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 72)
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 1836)
        self.assertEqual([sum(row['kind'] == kind for row in facts['cases']) for kind in (0, 1)], [36, 36])

    def test_invalid_fixture_options_and_oversized_ansi_strings_fail(self) -> None:
        """Reject inputs outside construction contracts; e.g. toolbox_object(0, 0, FLOAT_BITS[0], -1)."""
        for kind, mode, bits, slot in [(2, 0, FLOAT_BITS[0], 0), (0, 3, FLOAT_BITS[0], 0),
                (0, 0, 1, 0), (0, 0, FLOAT_BITS[0], -1)]:
            with self.assertRaisesRegex(ValueError, 'expected known kind/mode/float and nonnegative slot'):
                toolbox_object(kind, mode, bits, slot)
        with self.assertRaisesRegex(ValueError, 'expected <=65534 bytes'):
            ansi_string('A'*65535)


if __name__ == '__main__':
    unittest.main()
