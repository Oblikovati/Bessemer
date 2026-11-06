"""Pin supplementary metadata against the unchanged full-wire baseline."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import field_oracle
import freeze_fields


class ExtraFieldOracleTests(unittest.TestCase):
    def test_complete_regeneration_preserves_both_packets(self) -> None:
        baseline = field_oracle.baseline
        before = (baseline.ROOT / "tests/fixtures/flow-block-extras/extras.bin.gz").read_bytes()
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            freeze_fields.freeze(root)
            relative = Path("tests/fixtures/flow-block-extras/fields.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((baseline.ROOT / relative).read_bytes()))
        self.assertEqual((baseline.ROOT / "tests/fixtures/flow-block-extras/extras.bin.gz").read_bytes(), before)

    def test_empty_version_eight_mixed_fields_share_the_count_end_offset(self) -> None:
        raw = struct.pack("<iiB", 8, -1, 0) + b"\1\0\1" + bytes(20) + bytes(4) + bytes(16) + b"\0\0\0\0\2\0\0"
        parsed = field_oracle.baseline.flowblockextras.read_decoded(raw, leaf_count=1, enabled=True)
        oracle = field_oracle.FieldOracle(8)
        oracle.record(parsed)
        self.assertEqual(oracle.wire, raw)
        oracle.verify_coverage()
        empty_strided = [layout for layout in oracle.layouts if layout[1] == 0 and layout[3] == 12]
        self.assertEqual(empty_strided, [(36, 0, 4, 12), (36, 0, 8, 12)])

    def test_coverage_rejects_overlap_even_when_values_rebuild(self) -> None:
        oracle = field_oracle.FieldOracle(0)
        scope = field_oracle.entity("Record")
        oracle.scalar(scope, "Version", "i", 0)
        oracle.register(scope, "Version", "i", (0,), 0, 4)
        with self.assertRaises(AssertionError):
            oracle.verify_coverage()


if __name__ == "__main__":
    unittest.main()
