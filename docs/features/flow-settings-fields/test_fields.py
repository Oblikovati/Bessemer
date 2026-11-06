"""Independent literal expectations and isolated reference regeneration."""
import gzip
import hashlib
import json
import struct
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from freeze_fields import ROOT, freeze, owned_wire, expected_bytes, native_wires


class DiagnosticFieldFixtures(unittest.TestCase):
    def test_literal_fractional_defaults_and_guard_precedence(self) -> None:
        bits = struct.unpack("<Q", struct.pack("<d", 0.25))[0]
        expected = expected_bytes(owned_wire(bits, 0))
        values = [struct.unpack_from("<i", expected, 6 + index * 17 + 13)[0] for index in range(7)]
        self.assertEqual(values, [1, 0, 1, 0, 6, 2, 1])
        overridden = expected_bytes(owned_wire(bits, -1))
        self.assertEqual([struct.unpack_from("<i", overridden, 6 + index * 17 + 13)[0]
                          for index in (4, 5, 6)], [0, 0, 0])
        missing = expected_bytes(owned_wire(bits, None))
        self.assertEqual([missing[6 + index * 17 + 12] for index in (4, 5, 6)], [1, 1, 1])

    def test_invalid_conversion_precedes_nonzero_guard(self) -> None:
        expected = expected_bytes(owned_wire(0x7ff0000000000000, 1))
        self.assertEqual([expected[6 + index * 17 + 12] for index in range(7)], [2, 2, 0, 2, 2, 2, 2])
        self.assertEqual(expected_bytes(native_wires()[0]), b"\0")

    def test_temporary_regeneration_matches_frozen_packet(self) -> None:
        original = (ROOT / "tests/fixtures/flow-settings-fields/fields.bin.gz").read_bytes()
        with TemporaryDirectory(prefix="bessemer-fields-") as temporary:
            staging = Path(temporary)
            freeze(staging)
            self.assertEqual((staging / "tests/fixtures/flow-settings-fields/fields.bin.gz").read_bytes(), original)
            report = json.loads((staging / "docs/features/flow-settings-fields/fixture-verification.json").read_text())
        self.assertEqual(report["cases"], 141)
        self.assertEqual(hashlib.sha256(gzip.decompress(original)).hexdigest(), report["sha256"])


if __name__ == "__main__":
    unittest.main()
