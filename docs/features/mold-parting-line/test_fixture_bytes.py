"""Verify independent parting-line byte graphs and aliases (#216)."""
import unittest
from fixture_bytes import bare_line_cases, owned_line_cases, verify_line_cases


class PartingLineFixtureTests(unittest.TestCase):
    def test_every_bare_and_owned_graph_reproduces_exact_bytes_and_hashes(self) -> None:
        """Keep all seventeen byte oracles reproducible; e.g. python test_fixture_bytes.py."""
        self.assertEqual(len(bare_line_cases()), 5)
        self.assertEqual(len(owned_line_cases()), 12)
        verify_line_cases()

    def test_imported_map_and_parent_element_identities_are_explicit(self) -> None:
        """Pin owner and shared element identities independently; e.g. owned parent98/element112."""
        raw, slots, ids = owned_line_cases()['owned-serialized-0.bin']
        self.assertGreater(len(raw), 0)
        self.assertEqual((slots, ids['feature'], ids['element']), (117, 98, 112))


if __name__ == '__main__':
    unittest.main()
