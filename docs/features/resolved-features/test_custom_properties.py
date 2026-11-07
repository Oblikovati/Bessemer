"""Custom-property expected bytes are read independently of the Rust writer."""
from pathlib import Path
import tempfile
import unittest
from freeze_custom_properties import WORKSPACE, freeze_properties, short_unicode


class CustomPropertyOracleTests(unittest.TestCase):
    def test_independent_original_and_edit_match_frozen_owned_fixtures(self) -> None:
        expected = WORKSPACE / 'tests/fixtures/resolved-features'
        with tempfile.TemporaryDirectory(prefix='bessemer-custom-property-oracle-') as temporary:
            root = Path(temporary)
            report = freeze_properties(WORKSPACE.parent / 'inventor-interface/solidworks', root)
            self.assertEqual((report['bytes'], report['primitive_reads']), (130, 41))
            for name in ['custom-properties.bin', 'custom-properties-edited.bin', 'custom-properties-primitives.tsv']:
                self.assertEqual((root / name).read_bytes(), (expected / name).read_bytes())

    def test_short_fixture_encoder_rejects_an_extended_length(self) -> None:
        with self.assertRaises(ValueError):
            short_unicode('x'*255)


if __name__ == '__main__':
    unittest.main()
