"""Literal wire boundaries, runtime context and isolated fixture regeneration."""
import gzip
import hashlib
import json
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from freeze_body import ROOT, corpus, freeze, flowsettingsbody
from process_reference import carrier, changed_bytes, packet


class BodyFixtureTests(unittest.TestCase):
    def test_process_oracle_carries_all_edit_requests(self) -> None:
        frozen = gzip.decompress((ROOT / "tests/fixtures/flow-settings-body/body.bin.gz").read_bytes())
        expected = packet()
        self.assertEqual(expected[:len(frozen)], frozen)
        self.assertEqual(expected[len(frozen):len(frozen) + 8], b"BSE1\x91\0\0\0")
        self.assertEqual(changed_bytes(dict(kind="u8"), b"\xfe"), b"\1")
        self.assertEqual(changed_bytes(dict(kind="u32"), b"\0\0\0\0"), b"\1\0\0\x80")
        self.assertEqual(carrier(b"")[:2], b"\x08\0")

    def test_literal_lengths_order_and_full_nonboolean_byte(self) -> None:
        rows = corpus()["synthetic_fixtures"]
        self.assertEqual([row["end"] for row in rows], [24, 28, 29, 30, 31, 36, 37, 38, 39, 40, 41, 45])
        parsed = flowsettingsbody.read_payload(bytes.fromhex(rows[-1]["payload_hex"]), 0)
        fields = parsed["fields"]
        self.assertEqual([fields[name]["offset"] for name in ("member_10", "member_11", "member_88")], [29, 30, 31])
        self.assertEqual(fields["member_50"]["value"], bytes(range(1, 9)))
        self.assertEqual(fields["member_3c"]["value"], 0x55667788)
        self.assertEqual(parsed["member_0f"], dict(value=250, status="stored_u8"))

    def test_legacy_context_is_never_inferred_from_stored_arrays(self) -> None:
        for row in corpus()["synthetic_fixtures"][:2]:
            raw = bytes.fromhex(row["payload_hex"])
            self.assertEqual(flowsettingsbody.read_payload(raw)["member_0f"]["status"], "requires_runtime_integer_slot_88")
            self.assertEqual(flowsettingsbody.read_payload(raw, 0)["member_0f"]["value"], 1)
            self.assertEqual(flowsettingsbody.read_payload(raw, -2147483648)["member_0f"]["value"], 0)

    def test_temporary_regeneration_keeps_original_goldens(self) -> None:
        original = (ROOT / "tests/fixtures/flow-settings-body/body.bin.gz").read_bytes()
        with TemporaryDirectory(prefix="bessemer-body-") as temporary:
            staging = Path(temporary)
            freeze(staging)
            self.assertEqual((staging / "tests/fixtures/flow-settings-body/body.bin.gz").read_bytes(), original)
            report = json.loads((staging / "docs/features/flow-settings-body/fixture-verification.json").read_text())
        self.assertEqual(report["synthetic_versions"], 12)
        self.assertEqual(report["context_projections"], 72)
        self.assertEqual(len(report["native_absences"]), 3)
        self.assertEqual(hashlib.sha256(gzip.decompress(original)).hexdigest(), report["sha256"])


if __name__ == "__main__":
    unittest.main()
