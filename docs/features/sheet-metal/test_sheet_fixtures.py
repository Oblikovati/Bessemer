"""Check full sheet-metal fixtures, raw-span projections and persisted state (#54)."""
from pathlib import Path
from typing import cast
import json
import tempfile
import unittest
from freeze_sheet_metal import freeze, WORKSPACE, REFERENCE, VERSIONS
from sheet_fixture_bytes import SheetFixtureOptions, sheet_object
from sheet_primitive_projection import dynamic_payload_ranges, project_payload_words, payload_word_spans
from sheet_reference_observer import install_sheet_context
from object_fixture_observer import object_cursor
from primitive_inventory import PrimitiveRead
from freeze_blank_part import source_module


class SheetFixtureTests(unittest.TestCase):
    def test_regeneration_matches_all_fields_raw_payloads_and_owned_manager_states(self) -> None:
        """Exercise every builder and source identity check; e.g. python -m unittest test_sheet_fixtures."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            observed = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(observed[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/sheet-metal'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_retains_both_states_stride_counts_zero_descriptors_and_compatibility_gates(self) -> None:
        """Pin 316 complete cases and both source/projected read totals; e.g. 0x2bc5 in VERSIONS."""
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 316)
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 90406)
        self.assertEqual(sum(row['source_reads'] for row in facts['cases']), 89802)
        self.assertEqual([sum(row['kind'] == kind for row in facts['cases']) for kind in range(8)], [61, 61, 24, 24, 61, 61, 24, 0])
        self.assertTrue({0x2bae, 0x2baf, 0x2bb9, 0x2bba, 0x2bc4, 0x2bc5} <= set(VERSIONS))
        self.assertEqual([row['slots'] for row in facts['owned']], [101, 113, 115, 113, 115, 115, 115])

    def test_raw_projection_is_byte_exact_scoped_and_keeps_source_provenance(self) -> None:
        """A source raw payload remains raw unless its exact dynamic span was observed; e.g. project_payload_words(raw, reads, [])."""
        raw = bytes(range(16));read = PrimitiveRead(0, 16, 'raw', raw.hex(), 'sheetfeatures.py', 'dynamic_array', 30)
        self.assertEqual(project_payload_words(raw, [read], []), [read])
        projected = project_payload_words(raw, [read], [(0, 16)])
        self.assertEqual([(item.start, item.end, item.kind) for item in projected], [(0, 8, 'Q'), (8, 16, 'Q')])
        self.assertEqual(b''.join(bytes.fromhex(item.raw_hex) for item in projected), raw)
        self.assertTrue(all((item.source_file, item.source_function, item.source_line) == ('sheetfeatures.py', 'dynamic_array', 30) for item in projected))
        with self.assertRaisesRegex(ValueError, 'expected exact independently observed raw-read spans'):
            project_payload_words(raw, [read], [(0, 8)])
        with self.assertRaisesRegex(ValueError, 'expected a nonempty raw span divisible by eight'):
            payload_word_spans(PrimitiveRead(0, 7, 'raw', raw[:7].hex(), 'source.py', 'read', 1))

    def test_dynamic_observation_restores_source_functions_after_failure(self) -> None:
        """External observation hooks never escape their operation; e.g. with dynamic_payload_ranges(source, raw)."""
        module = source_module('sheetfeatures', REFERENCE);original = module.dynamic_array
        raw = bytes(4);cursor = object_cursor(REFERENCE, raw, 19000, None)
        with self.assertRaisesRegex(RuntimeError, 'named observer failure'):
            with dynamic_payload_ranges(REFERENCE, raw) as ranges:
                self.assertIsNone(module.dynamic_array(cursor))
                self.assertEqual(ranges, [])
                raise RuntimeError('named observer failure')
        self.assertIs(module.dynamic_array, original)

    def test_template_rejects_unverified_context_before_body_and_invalid_fixture_shapes_fail(self) -> None:
        """Keep state provenance independent of class names or plausible bytes; e.g. sheet_object(1, 19000, options)."""
        raw, _ = sheet_object(1, 19000, SheetFixtureOptions(False, False, 0))
        cursor = object_cursor(REFERENCE, raw, 19000, None)
        install_sheet_context(cursor, REFERENCE, 0, False)
        error = source_module('swarchive', REFERENCE).ArchiveError
        with self.assertRaisesRegex(error, 'persisted configuration-manager context'):
            cursor.object()
        self.assertEqual(cursor.p, 6+len('moTemplateSheetMetal_c'))
        for kind, allowance, slot in [(4, 0, 0), (-1, 0, 0), (0, 5, 0), (0, 0, -1)]:
            with self.assertRaisesRegex(ValueError, 'expected kind 0..3, allowance 0..4 and nonnegative slot'):
                sheet_object(kind, 19000, SheetFixtureOptions(False, False, allowance), slot)


if __name__ == '__main__':
    unittest.main()
