"""Source-accepted literal controls for complete intersection sketch callers (#184)."""
from pathlib import Path
import importlib
import struct
import unittest
from owned_intersection_match import ReferenceMatchWords, reference_match_literal, intersection_match_body, intersection_match_sketch, intersection_match_stream

CONTROL_FIRST: tuple[ReferenceMatchWords, ...] = ((0, -2, -2**31, 2**31 - 1), (1, -2**31, -2, -1), (3, 2**31 - 1, 0, 1), (65535, 0, 2**31 - 1, -2**31))
CONTROL_SECOND: tuple[ReferenceMatchWords, ...] = tuple((state ^ 65535, -2, back, span) for state, ordinal, span, back in CONTROL_FIRST)


class OwnedIntersectionMatchTests(unittest.TestCase):
    def test_literal_signed_extremes(self) -> None:
        for words in CONTROL_FIRST + CONTROL_SECOND:
            literal = reference_match_literal(words)
            self.assertEqual(len(literal), 14)
            self.assertEqual(literal, struct.pack('<H3i', *words))
            self.assertEqual(struct.unpack('<H3i', literal), words)

    def test_literal_range_errors(self) -> None:
        for words in ((-1, 0, 0, 0), (65536, 0, 0, 0), (0, -2**31 - 1, 0, 0), (0, 0, 2**31, 0), (0, 0, 0, -2**31 - 1)):
            with self.assertRaises(ValueError) as failure:
                reference_match_literal(words)
            self.assertIn(repr(words), str(failure.exception))
            self.assertIn('expected u16 state and three signed32 words', str(failure.exception))

    def test_exact_four_group_shape(self) -> None:
        for groups in ((), CONTROL_FIRST[:3], CONTROL_FIRST + CONTROL_FIRST[:1]):
            with self.assertRaises(ValueError) as failure:
                intersection_match_body(groups)
            self.assertIn(repr(groups), str(failure.exception))
            self.assertIn('exactly four physical groups', str(failure.exception))

    def test_complete_body_and_sketch_spans(self) -> None:
        fragment = intersection_match_body(CONTROL_FIRST)
        owner, starts = intersection_match_sketch(CONTROL_FIRST)
        self.assertEqual(len(starts), 4)
        self.assertIn(fragment, owner)
        for start, words in zip(starts, CONTROL_FIRST, strict=True):
            self.assertEqual(owner[start:start + 20], struct.pack('<H3i3H', *words, 0, 0, 0))

    def test_complete_stream_matches_source_accepted_literal(self) -> None:
        raw, starts = intersection_match_stream(CONTROL_FIRST, CONTROL_SECOND)
        fixture = Path(__file__).resolve().parents[3] / 'tests/fixtures/intersection-reference-match/controls.bin'
        self.assertEqual(raw, fixture.read_bytes())
        self.assertEqual(starts, [440, 460, 480, 500, 1187, 1207, 1227, 1247])

    def test_unchanged_source_complete_caller_and_aliases(self) -> None:
        from dimension_state_owned import owned_dimension_context
        from freeze_owned_features import ReplacedFeatureDocument, STREAM
        document, _ = owned_dimension_context()
        raw, _ = intersection_match_stream(CONTROL_FIRST, CONTROL_SECOND)
        parsed = importlib.import_module('resolvedfeatures').read_supported(ReplacedFeatureDocument(document, raw), STREAM)
        self.assertEqual(parsed['end'], len(raw))
        self.assertTrue(parsed['list_complete'])
        items = parsed['items']
        self.assertEqual(len(items), 3)
        self.assertIs(items[0], items[2])
        self.assertIsNot(items[0], items[1])
        self.assertEqual(len(parsed['archive'].map), 100)
        for item, expected in zip(items[:2], (CONTROL_FIRST, CONTROL_SECOND), strict=True):
            full = item['styled_intersections']['intersections'][1]
            for group, words in zip(('38', '80', 'c8', '110'), expected, strict=True):
                actual = full['edges_48'][0]['group_' + group]
                self.assertEqual(tuple(actual[name] for name in ('raw_00', 'raw_20', 'raw_24', 'raw_28')), words)
