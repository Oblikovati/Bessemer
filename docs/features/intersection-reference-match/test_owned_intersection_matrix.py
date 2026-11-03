"""Independent denominator and signed-domain checks for bounded owned cases (#184)."""
import struct
import unittest
from owned_intersection_match import reference_match_literal
from owned_intersection_matrix import intersection_match_matrix_inputs, intersection_match_matrix_groups


class OwnedIntersectionMatrixTests(unittest.TestCase):
    def test_cartesian_domain_and_unique_order(self) -> None:
        inputs = list(intersection_match_matrix_inputs())
        self.assertEqual(len(inputs), 1344)
        self.assertEqual(len(set(inputs)), 1344)
        self.assertEqual(inputs, sorted(inputs))
        self.assertEqual({row[2] for row in inputs}, {0, 1, 2, 3, 4, 5, 65535})
        self.assertEqual({row[3] for row in inputs}, {-2, -2**31, 2**31 - 1})

    def test_all_physical_groups_preserve_independent_words(self) -> None:
        for span, backward, state, ordinal in intersection_match_matrix_inputs():
            first, second = intersection_match_matrix_groups(span, backward, state, ordinal)
            self.assertEqual(len(first), 4)
            self.assertEqual(len(second), 4)
            self.assertNotEqual(first, second)
            self.assertEqual(first[0], (state, ordinal, span, backward))
            self.assertEqual(second[0], (state ^ 65535, ~ordinal, backward, span))
            for words in first + second:
                self.assertEqual(reference_match_literal(words), struct.pack('<H3i', *words))

    def test_all_u16_states_remain_literal_without_enum_filtering(self) -> None:
        for state in range(65536):
            literal = reference_match_literal((state, -2, -2**31, 2**31 - 1))
            self.assertEqual(literal[:2], state.to_bytes(2, 'little'))
            self.assertEqual(literal[2:], bytes.fromhex('feffffff00000080ffffff7f'))
