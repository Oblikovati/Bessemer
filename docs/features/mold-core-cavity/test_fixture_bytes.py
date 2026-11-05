"""Regress independently assembled core/cavity graphs and map identities (#215)."""
import unittest
from fixture_bytes import literal_mold_cases, verify_literal_mold_cases


class MoldCoreCavityFixtureTests(unittest.TestCase):
    def test_complete_corpus_reproduces_every_byte_hash_and_identity(self) -> None:
        """Verify all twelve retained graphs; e.g. python -m unittest test_fixture_bytes."""
        cases = literal_mold_cases()
        self.assertEqual(len(cases), 12)
        verify_literal_mold_cases()

    def test_inherited_map_and_owner_slots_are_explicit(self) -> None:
        """Distinguish imported context from fresh maps; e.g. owned owner98 versus bare owner2."""
        cases = literal_mold_cases()
        self.assertEqual(cases['feature-1.bin'][2]['moCompSurfaceBody_c'], 10)
        self.assertEqual(cases['owned-1.bin'][2]['feature'], 98)
        self.assertEqual(cases['owned-1.bin'][1], 115)


if __name__ == '__main__':
    unittest.main()
