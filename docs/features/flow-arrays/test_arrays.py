"""Independent signed-array fixture construction and frozen native source checks."""
import gzip
import hashlib
import json
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from freeze_arrays import ROOT, freeze, native_case, owned_cases, packet_case, signed_array


class ArrayFixtureTests(unittest.TestCase):
    def test_literal_signed_wire_and_all_owned_selector_profiles(self) -> None:
        self.assertEqual(signed_array([-1, -0x80000000, 0x7FFFFFFF]),
                         bytes.fromhex("03000000ffffffff00000080ffffff7f"))
        cases = owned_cases()
        self.assertEqual(len(cases), 12)
        for case in cases:
            self.assertTrue(packet_case(*case))
        wrapped = (ROOT / "tests/fixtures/flow-arrays/wrong-wrapper.gz").read_bytes()
        self.assertEqual(gzip.decompress(wrapped), bytes.fromhex("0000000001000000ffffffff05000000"))

    def test_regeneration_preserves_packet_and_native_inputs(self) -> None:
        target = ROOT / "tests/fixtures/flow-arrays/arrays.bin.gz"
        before = target.read_bytes()
        with TemporaryDirectory(prefix="bessemer-array-fixture-") as temporary:
            staging = Path(temporary)
            (staging / "tests/fixtures/flow-arrays").mkdir(parents=True)
            (staging / "docs/features/flow-arrays").mkdir(parents=True)
            freeze(staging)
            self.assertEqual((staging / "tests/fixtures/flow-arrays/arrays.bin.gz").read_bytes(), before)
            proof = json.loads((staging / "docs/features/flow-arrays/fixture-verification.json").read_text())
        decoded = gzip.decompress(before)
        self.assertEqual(hashlib.sha256(decoded).hexdigest(), proof["sha256"])
        self.assertEqual(len(decoded), proof["bytes"])
        self.assertEqual(len(native_case()[1]), 102498)


if __name__ == "__main__":
    unittest.main()
