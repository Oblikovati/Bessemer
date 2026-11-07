"""Literal surrogate, factory and native boundary oracles with isolated regeneration."""
import gzip
import hashlib
import json
import struct
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from freeze_identities import ROOT, freeze, native_cases, owned_cases, identities
from swarchive import ArchiveError
from process_reference import edit_cases, object_requests, packet


class IdentityFixtures(unittest.TestCase):
    def test_complete_process_oracle_pins_changed_and_noop_requests(self) -> None:
        requests = list(edit_cases())
        self.assertEqual(len(requests), 1080)
        self.assertEqual(sum(changed for _, changed in requests), 612)
        self.assertEqual(packet()[:8], b"FIA1\x16\0\0\0")
        original = identities.read143(native_cases()[0][2])["objects"][0]
        self.assertEqual(object_requests(original)[9], (4, b"\0\0\0\0"))

    def test_native_boundaries_and_known_keys(self) -> None:
        native = native_cases()
        self.assertEqual([len(case[2]) for case in native], [211, 8])
        parsed = identities.read143(native[0][2])
        self.assertEqual([(obj["start"], obj["end"]) for obj in parsed["objects"]], [(8, 25), (25, 116), (116, 207)])
        self.assertEqual([obj["text"] for obj in parsed["objects"]], ["", "79455a38-c499-4b69-9961-88b8517e5800", "924079d1-4a95-48ad-8f9d-7d6af514fb21"])
        self.assertEqual(parsed["tail_values"], ())

    def test_owned_factory_and_raw_utf16_edges(self) -> None:
        cases = owned_cases()
        factory = identities.read143(cases[1][2])
        self.assertEqual([obj["native_class"] for obj in factory["objects"]], ["SBoundaryCondition", "SVSource", "SSSource", "SFan", "SUnknownFeature", "SUnknownFeature"])
        edges = identities.read143(cases[2][2])["objects"]
        self.assertEqual(edges[2]["text_bytes"], b"\0\xd8\0\0")
        self.assertEqual(edges[2]["text"], "\ud800")
        self.assertEqual(edges[-1]["text"], "x")
        self.assertEqual(edges[-1]["text_count"], 7)
        self.assertEqual(edges[-1]["text_bytes"], "x\0tail\0".encode("utf-16le"))

    def test_null_profile_rejects_populated_entries_and_legacy_count_stays_virtual(self) -> None:
        with self.assertRaises(ArchiveError):
            identities.read144_nulls(struct.pack("<IiB", 4, 1, 255))
        payload = owned_cases()[0][2]
        self.assertEqual(identities.read143(payload, -1)["derived_tail"]["count"], 0)
        self.assertEqual(identities.read143(payload, -1)["derived_tail"]["fill_value"], 2)
        self.assertEqual(identities.read143(payload, 2147483647)["derived_tail"]["count"], 2147483647)
        self.assertIsNone(identities.read143(payload)["derived_tail"]["count"])

    def test_temporary_regeneration_preserves_frozen_packet(self) -> None:
        original = (ROOT / "tests/fixtures/flow-feature-identities/identities.bin.gz").read_bytes()
        with TemporaryDirectory(prefix="bessemer-identities-") as temporary:
            staging = Path(temporary)
            freeze(staging)
            self.assertEqual((staging / "tests/fixtures/flow-feature-identities/identities.bin.gz").read_bytes(), original)
            report = json.loads((staging / "docs/features/flow-feature-identities/fixture-verification.json").read_text())
        self.assertEqual(report["cases"], 37)
        self.assertEqual(hashlib.sha256(gzip.decompress(original)).hexdigest(), report["sha256"])


if __name__ == "__main__":
    unittest.main()
