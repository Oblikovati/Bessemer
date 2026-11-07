"""Regenerate complete fold records and verify scoped raw-byte projections (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_fold_transforms import freeze, WORKSPACE, REFERENCE, VERSIONS, FLAGS
from fold_fixture_bytes import FoldFixtureOptions, fold_object
from fold_reference_observer import project_fold_bytes
from primitive_inventory import PrimitiveRead


class FoldFixtureTests(unittest.TestCase):
    def test_regeneration_matches_all_fields_slots_bytes_and_two_independent_edits(self) -> None:
        """Regenerate complete objects and documents from approved readers; e.g. python -m unittest test_fold_fixtures."""
        expected = json.loads((Path(__file__).parent/'reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary)
            actual = freeze(REFERENCE, destination)
            for key in ('cases', 'owned', 'packet_sha256'):
                self.assertEqual(actual[key], expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(), (WORKSPACE/'tests/fixtures/fold-transforms'/fixture.name).read_bytes(), fixture.name)

    def test_matrix_retains_every_class_extension_unknown_flags_and_native_widths(self) -> None:
        """Pin 105 complete cases with both matrix-presence shapes; e.g. FLAGS includes 0xff."""
        facts = json.loads((Path(__file__).parent/'reference.json').read_text())
        self.assertEqual(len(facts['cases']), 105)
        self.assertEqual([sum(row['kind'] == kind for row in facts['cases']) for kind in range(4)], [33, 33, 33, 6])
        self.assertEqual(set(VERSIONS), {0x27cc, 19000, 0xffffffff})
        self.assertEqual(set(FLAGS), {0, 0x1f, 0x2f, 0x4f, 0xff})
        self.assertEqual(sum(row['reads'] for row in facts['cases']), 11460)
        self.assertEqual(sum(row['source_reads'] for row in facts['cases']), 11280)
        self.assertEqual([row['slots'] for row in facts['owned']], [105, 109, 109, 109])

    def test_byte_projection_requires_exact_source_payloads_and_preserves_provenance(self) -> None:
        """Raw class names and empty reads never become byte-array fields; e.g. project_fold_bytes(raw, reads, expected)."""
        raw = bytes([0, 255, 128, 127])
        read = PrimitiveRead(0, 4, 'raw', raw.hex(), 'foldtransforms.py', '<lambda>', 47)
        empty = PrimitiveRead(4, 4, 'raw', '', 'foldtransforms.py', '<lambda>', 47)
        projected = project_fold_bytes(raw, [read, empty], [raw])
        self.assertEqual([(item.start, item.end, item.kind) for item in projected], [(0, 1, 'B'), (1, 2, 'B'), (2, 3, 'B'), (3, 4, 'B')])
        self.assertEqual(b''.join(bytes.fromhex(item.raw_hex) for item in projected), raw)
        self.assertTrue(all((item.source_file, item.source_function, item.source_line) == ('foldtransforms.py', '<lambda>', 47) for item in projected))
        foreign = PrimitiveRead(0, 4, 'raw', raw.hex(), 'mfcobjects.py', 'object', 11)
        self.assertEqual(project_fold_bytes(raw, [foreign], []), [foreign])
        with self.assertRaisesRegex(ValueError, 'expected independently decoded byte arrays'):
            project_fold_bytes(raw, [read], [raw[:3]])

    def test_invalid_constructor_shapes_reject_before_archive_emission(self) -> None:
        """Validate class, map and byte-width inputs; e.g. fold_object(4, 19000, options)."""
        for kind, slot, flag in [(4, 0, 0), (-1, 0, 0), (0, -1, 0), (0, 0, -1), (0, 0, 256)]:
            with self.assertRaisesRegex(ValueError, 'expected kind 0..3, nonnegative slot and u8 flags'):
                fold_object(kind, 19000, FoldFixtureOptions(flags=flag), slot)


if __name__ == '__main__':
    unittest.main()
