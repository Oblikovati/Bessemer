"""Literal role controls and complete Source ownership acceptance (#185)."""
from pathlib import Path
import struct
import sys
import unittest
from dimension_state_owned import owned_dimension_context
from owned_intersection_roles import ROLE_SIGNED_VALUES, ROLE_SELECTORS, ROLE_ORDINALS, ROLE_INDEX_STARTS, ROLE_GROUP_STARTS, role_original_control, role_matrix_inputs, role_reference_words, role_matrix_root, role_domain_edge, role_domain_body, role_full_selector_root
from owned_role_corpus import build_role_corpus, persist_role_corpus
from role_source_oracles import role_expected_scalars, role_scalar_rows
from sketch_tail_fixture_bytes import WireMap, intersection


class OwnedIntersectionRolesTests(unittest.TestCase):
    def test_cartesian_inputs_have_independent_signed_and_selector_domains(self) -> None:
        values = list(role_matrix_inputs())
        self.assertEqual(len(values), 264)
        self.assertEqual(len(set(values)), 264)
        self.assertEqual(values, sorted(values))
        self.assertEqual({row[0] for row in values}, set(ROLE_SIGNED_VALUES))
        self.assertEqual({row[1] for row in values}, set(ROLE_SELECTORS))
        self.assertEqual({row[2] for row in values}, set(ROLE_ORDINALS))

    def test_every_small_root_is_only_literal_word_replacements(self) -> None:
        original = role_original_control()
        for index, selector, ordinal in role_matrix_inputs():
            owned = role_matrix_root(index, selector, ordinal)
            expected = bytearray(original)
            for start, value in zip(ROLE_INDEX_STARTS, (index, ~index), strict=True):
                struct.pack_into('<i', expected, start, value)
            for group, start in enumerate(ROLE_GROUP_STARTS):
                words = role_reference_words(selector, ordinal, group)
                struct.pack_into('<H3i', expected, start, *words)
            self.assertEqual(owned.raw, expected)
            self.assertEqual(len(role_expected_scalars(owned)), 34)
            self.assertEqual(len(role_scalar_rows(0, owned).splitlines()), 10)

    def test_invalid_values_report_the_offending_value_and_expected_shape(self) -> None:
        for values in ((2**31, 0, 0), (-2**31 - 1, 0, 0), (0, 65536, 0), (0, -1, 0), (0, 0, 2**31)):
            with self.assertRaises(ValueError) as failure:
                role_matrix_root(*values)
            self.assertIn('expected u16 state and three signed32 words', str(failure.exception))
        with self.assertRaisesRegex(ValueError, 'group 8.*expected physical group index'):
            role_reference_words(0, 0, 8)

    def test_large_body_has_exact_counts_and_consecutive_complete_edge_groups(self) -> None:
        template = intersection(WireMap(), 19000, False)
        first, words = role_domain_edge(0, template)
        self.assertEqual(len(first), 100)
        self.assertEqual([group[0] for group in words], [0, 1, 2, 3])
        self.assertEqual(struct.unpack_from('<i', first, 16)[0], -2**31)
        body, indices, references = role_domain_body(template)
        self.assertEqual(struct.unpack_from('<i', body, 6)[0], 16384)
        self.assertEqual(len(indices), 16384)
        self.assertEqual(len(references), 65536)
        self.assertEqual([group[0] for _, group in references], list(range(65536)))
        self.assertEqual(len(body), 1671229)
        self.assertEqual(len(role_full_selector_root().raw), 1672584)

    def test_complete_matrix_and_full_u16_root_match_original_source(self) -> None:
        feature = Path(__file__).resolve().parent
        fixture = feature.parents[2] / 'tests/fixtures/intersection-index-classification'
        document, _ = owned_dimension_context()
        payloads = build_role_corpus(document)
        result = persist_role_corpus(feature, fixture, payloads, '--write-role-fixtures' in sys.argv)
        self.assertEqual(len(result['cases']), 265)
        self.assertEqual(result['indices'], 16913)
        self.assertEqual(result['groups'], 67652)
        self.assertEqual(result['primitive_reads'], 676537)
