"""Owned literal and full original-reader acceptance (#188)."""
from pathlib import Path
from typing import cast
import struct
import sys
import unittest
from dimension_state_owned import owned_dimension_context
from lookup_key_owned import LOOKUP_KEY_VALUES, checked_lookup_key, lookup_key_transform_bytes, owned_lookup_key_root
from lookup_key_corpus import build_lookup_key_corpus, persist_lookup_key_corpus

class OwnedLookupKeyTests(unittest.TestCase):
    def test_key_boundaries_preserve_signed_values_and_reject_invalid_shapes(self) -> None:
        for value in LOOKUP_KEY_VALUES:
            self.assertEqual(checked_lookup_key(value), value)
        for value in (-2147483649, 2147483648, True, 1.5, '7'):
            with self.assertRaisesRegex(ValueError, 'expected a full signed i32'):
                checked_lookup_key(cast(int, value))
        for profile in (-1, 4, True):
            with self.assertRaisesRegex(ValueError, 'expected integer0..3'):
                lookup_key_transform_bytes(profile)

    def test_first_complete_body_has_independent_literal_prefix_and_key_offsets(self) -> None:
        owned = owned_lookup_key_root(-2147483648, 2147483647, 0)
        tag = struct.pack('<iHHHH', -71, 3, 65535, 1, 16) + b'moSketchExtRef_w'
        body = struct.pack('<HHHiiHH5i6ihH', 0, 65535, 0, -2147483646, 1879048192, 0, 0, -2147483647, 2147483647, -2147483648, 2147483647, -13, -31, 27, -39, -43, -47, -53, -2, 0)
        self.assertEqual(len(tag), 28)
        self.assertEqual(len(body), 66)
        self.assertEqual(owned.raw[:94], tag + body)
        self.assertEqual(owned.key_offsets[0], (54, 58))
        self.assertEqual(owned.raw[-6:], struct.pack('<HI', 98, 0))
        self.assertEqual(owned.keys[1], (2147483647, -2147483648))

    def test_complete_cartesian_roots_and_primitive_goldens_remain_unchanged(self) -> None:
        document, _ = owned_dimension_context()
        corpus = build_lookup_key_corpus(document)
        feature = Path(__file__).resolve().parent
        fixture = feature.parents[2] / 'tests/fixtures/sketch-reference-configuration-keys'
        reference = persist_lookup_key_corpus(feature, fixture, corpus, '--write-owned-key-fixtures' in sys.argv)
        self.assertEqual(reference['complete_roots'], 1200)
        self.assertEqual(reference['literal_keys'], 1600)
        self.assertEqual(len(corpus.cases), 400)
