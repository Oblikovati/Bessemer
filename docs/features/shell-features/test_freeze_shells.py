"""Reproduce independent shell packets and reject invalid builder options (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_shells import freeze, shell_variants, WORKSPACE
from shell_fixture_bytes import shell_object


class ShellFixtureTests(unittest.TestCase):
    def test_complete_regeneration_matches_owned_bytes_oracles_and_case_facts(self) -> None:
        """Exercise every builder and observer; e.g. python -m unittest test_freeze_shells."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(WORKSPACE.parent/'inventor-interface/solidworks', destination)
            self.assertEqual(observed['cases'], expected['cases'])
            self.assertEqual(observed['owned'], expected['owned'])
            self.assertEqual(observed['packet_sha256'], expected['packet_sha256'])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),
                    (WORKSPACE/'tests/fixtures/shell-features'/fixture.name).read_bytes(), fixture.name)

    def test_variants_retain_both_versions_modes_and_silhouette_derivatives(self) -> None:
        """Keep each versioned body distinct; e.g. len(shell_variants()) == 18."""
        variants = shell_variants()
        self.assertEqual(len(variants), 18)
        self.assertEqual(len(set(variants)), 18)
        self.assertEqual({version for version, _, _, _ in variants}, {0x27cc, 19000})
        self.assertEqual(sum(silhouette for _, _, _, silhouette in variants), 2)

    def test_invalid_modes_flags_slots_and_silhouette_contexts_fail_explicitly(self) -> None:
        """Reject malformed fixture options; e.g. shell_object(19000, False, 'null', flag=256)."""
        for arguments in [('invalid', None, False, 0), ('null', -1, False, 0),
                          ('null', 256, False, 0), ('null', None, False, -1),
                          ('shared', None, True, 0)]:
            with self.assertRaises(ValueError):
                shell_object(19000, False, *arguments)


if __name__ == '__main__':
    unittest.main()
