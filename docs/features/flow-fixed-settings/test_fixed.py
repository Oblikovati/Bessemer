"""Independent fixture boundaries and isolated regeneration."""
import gzip
import hashlib
import json
import math
import struct
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from freeze_fixed import ROOT, freeze, native_cases, owned_cases, flowfixedsettings, flowresults
from swarchive import ArchiveError


class FixedSlotFixtures(unittest.TestCase):
    def test_native_payloads_and_exact_widths(self) -> None:
        cases = native_cases()
        self.assertEqual(len(cases), 4)
        self.assertEqual(sum(len(case["payload"]) for case in cases), 5920)
        for case in cases:
            count, width = {7: (100, 4), 102: (320, 8)}[case["identifier"]]
            self.assertEqual(len(case["bits"]), count)
            self.assertEqual(len(case["payload"]), count * width)

    def test_special_bits_and_strict_extent(self) -> None:
        integer, numeric = owned_cases()
        self.assertEqual(flowfixedsettings.read_payload(7, integer["payload"])["values"][:5],
                         (0, 1, 2147483647, -2147483648, -1))
        values = flowfixedsettings.read_payload(102, numeric["payload"])["values"]
        self.assertEqual(math.copysign(1, values[1]), -1)
        self.assertTrue(math.isnan(values[5]))
        self.assertEqual(numeric["bits"][5], 0x7ff8000000000042)
        for case in (integer, numeric):
            for size in range(len(case["payload"])):
                with self.assertRaises(ArchiveError):
                    flowfixedsettings.read_payload(case["identifier"], case["payload"][:size])
            with self.assertRaises(ArchiveError):
                flowfixedsettings.read_payload(case["identifier"], case["payload"] + b"\0")

    def test_missing_and_last_occurrence(self) -> None:
        payload = owned_cases()[0]["payload"]
        frame = struct.pack("<IQ", 7, len(payload)) + payload
        self.assertIsNone(flowfixedsettings.read_record(flowresults.read(b"\x07\0"), 7))
        document = flowresults.read(b"\x07\0" + frame + struct.pack("<IQ", 7, 0))
        with self.assertRaises(ArchiveError):
            flowfixedsettings.read_record(document, 7)

    def test_regeneration_isolated_from_goldens(self) -> None:
        original = (ROOT / "tests/fixtures/flow-fixed-settings/slots.bin.gz").read_bytes()
        with TemporaryDirectory(prefix="bessemer-fixed-") as temporary:
            staging = Path(temporary)
            freeze(staging)
            self.assertEqual((staging / "tests/fixtures/flow-fixed-settings/slots.bin.gz").read_bytes(), original)
            report = json.loads((staging / "docs/features/flow-fixed-settings/fixture-verification.json").read_text())
        self.assertEqual(hashlib.sha256(gzip.decompress(original)).hexdigest(), report["sha256"])
        self.assertEqual(report["cases"], 6)


if __name__ == "__main__":
    unittest.main()
