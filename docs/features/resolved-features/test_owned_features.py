"""The feature edit oracle is independent of the Rust writer."""
from pathlib import Path
import tempfile
import unittest
from freeze_owned_features import WORKSPACE, freeze_features, independent_plane_edit


class OwnedFeatureOracleTests(unittest.TestCase):
    def test_fresh_reference_reads_match_all_owned_artifacts(self) -> None:
        expected = WORKSPACE / 'tests/fixtures/resolved-features'
        with tempfile.TemporaryDirectory(prefix='bessemer-feature-oracle-') as temporary:
            root = Path(temporary)
            report = freeze_features(WORKSPACE.parent / 'inventor-interface/solidworks',
                WORKSPACE / 'tests/fixtures/blank-part.SLDPRT', root)
            self.assertEqual(report['stream_bytes'], 5558)
            self.assertEqual(report['edited_bytes'], 5568)
            self.assertEqual(report['primitive_reads'], 1518)
            for name in ['owned-features.bin', 'owned-features-plane-edited.bin', 'owned-features-primitives.tsv']:
                self.assertEqual((root/name).read_bytes(), (expected/name).read_bytes())

    def test_independent_edit_rejects_missing_and_ambiguous_original_names(self) -> None:
        for raw in [b'', b'\x0bFront Plane'*2]:
            with self.assertRaisesRegex(ValueError, 'expected one unique'):
                independent_plane_edit(raw)


if __name__ == '__main__':
    unittest.main()
