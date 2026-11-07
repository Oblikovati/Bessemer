"""Check fixture reproducibility and the independent branch product (#217)."""
import unittest
from fixture_bytes import geometry_cases, verify_geometry_cases


class GeometryFixtureTests(unittest.TestCase):
    def test_frozen_bytes_and_hashes_reproduce(self) -> None:
        """Rebuild all 30 complete graphs; e.g. python -m unittest test_fixture_bytes."""
        verify_geometry_cases()
        self.assertEqual(len(geometry_cases()), 30)

    def test_method_and_selector_changes_do_not_invent_an_owned_child(self) -> None:
        """Pin each one-word distinction before the populated child branch; e.g. python -m unittest."""
        cases = geometry_cases()
        rich = cases['rich.bin'][0]
        for name in ['allocated-skipped.bin', 'unallocated-selected.bin']:
            other = cases[name][0]
            self.assertEqual(len(other), len(rich))
            self.assertEqual(sum(a != b for a, b in zip(rich, other)), 1)
        self.assertGreater(len(cases['child.bin'][0]), len(rich))


if __name__ == '__main__':
    unittest.main()
