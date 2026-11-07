"""Positive empty-frame oracles without pinning the reader bug as intended behavior."""
from __future__ import annotations
import tempfile
import unittest
from pathlib import Path
import freeze_empty as frozen


class EmptyContainerBaselineTests(unittest.TestCase):
    def test_every_registered_key_has_a_source_accepted_thirty_byte_empty_wire(self) -> None:
        rows = frozen.accepted_rows()
        self.assertEqual(len(rows), 1000)
        self.assertEqual(len({row[:4] for row in rows}), 1000)
        for row in rows:
            self.assertEqual(len(row), 46)
            self.assertEqual(row[:4], row[16:20])
            self.assertEqual(row[12:16], row[24:28])
            self.assertEqual(row[28:], bytes(18))

    def test_native_signature_rows_reuse_the_existing_owned_fixtures(self) -> None:
        rows = frozen.native_rows()
        self.assertEqual(len(rows), 3)
        for index, row in enumerate(rows):
            self.assertEqual(row[0], index)
            self.assertEqual(len(row), 41)
            raw = (frozen.ROOT / 'tests/fixtures' / frozen.FIXTURES[index]).read_bytes()
            self.assertEqual(row[1:5], raw[:4])

    def test_reference_builder_failure_is_reproduced_without_changing_the_source(self) -> None:
        self.assertIn('off', frozen.reference_builder_failure())
        self.assertEqual(len(frozen.table()), 1000)

    def test_injected_output_root_regenerates_exact_frozen_packets(self) -> None:
        with tempfile.TemporaryDirectory(prefix='bessemer-empty-') as directory:
            root = Path(directory)
            proof = frozen.freeze(root)
            for name in ('empty', 'native'):
                relative = Path('tests/fixtures/container-empty') / (name + '.bin')
                expected = (frozen.ROOT / relative).read_bytes()
                self.assertEqual((root / relative).read_bytes(), expected)
                self.assertEqual(frozen.digest(expected), proof['packets'][name]['sha256'])

    def test_signature_and_filler_boundaries_reject_wrong_shapes(self) -> None:
        for value in (None, True, 0, '', 'x' * 8, '00 00 00'):
            with self.assertRaisesRegex(ValueError, 'expected'):
                frozen.hex_word(value)
        for value in (None, True, -1, 256, '12'):
            with self.assertRaisesRegex(ValueError, 'expected'):
                frozen.filler_byte(value)
        with self.assertRaisesRegex(ValueError, 'expected'):
            frozen.signature_values(None)
        self.assertEqual(frozen.hex_word('ABCDEF12'), 'abcdef12')
        self.assertEqual(frozen.filler_byte(255), 255)
