"""Independent backend framing boundaries and strict unsupported-version gates."""
import gzip
import struct
import tempfile
import unittest
from pathlib import Path
import freeze_backend as frozen
from swarchive import ArchiveError


class BackendBaselineTests(unittest.TestCase):
    def test_packet_regenerates_exactly(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            frozen.freeze(root)
            relative = Path("tests/fixtures/flow-metadata/backend.bin.gz")
            self.assertEqual(gzip.decompress((root / relative).read_bytes()),
                             gzip.decompress((frozen.ROOT / relative).read_bytes()))

    def test_independent_full_known_prefix_reconstruction(self) -> None:
        for name, raw in frozen.cases():
            parsed = frozen.flowmediumbackend.read_backend_prefix(raw)
            rebuilt = bytes((parsed["version"],))
            for table in parsed["tables"]:
                self.assertEqual(len(rebuilt), table["start"], name)
                rebuilt += struct.pack("<Bi", table["version"], table["count"])
                if table["count"]:
                    rebuilt += struct.pack("<i", table["first_kind_i32"])
                rebuilt += table.get("first_key_raw", b"")
            self.assertEqual(len(rebuilt), parsed["prefix_end"], name)
            self.assertEqual(rebuilt + parsed["opaque_tail"], raw, name)
            self.assertFalse(parsed["full_record_schema"])
            self.assertFalse(parsed["native_runtime_state_reproduced"])

    def test_all_proper_known_prefixes_are_rejected(self) -> None:
        for _, raw in frozen.cases():
            parsed = frozen.flowmediumbackend.read_backend_prefix(raw)
            for length in range(parsed["prefix_end"]):
                with self.assertRaises(ArchiveError):
                    frozen.flowmediumbackend.read_backend_prefix(raw[:length])

    def test_nonzero_versions_and_negative_counts_are_rejected(self) -> None:
        raw = bytes(11)
        for position in (0, 1, 6):
            for version in range(1, 256):
                changed = bytearray(raw)
                changed[position] = version
                with self.assertRaises(ArchiveError):
                    frozen.flowmediumbackend.read_backend_prefix(changed)
        for position in (2, 7):
            changed = bytearray(raw)
            changed[position:position + 4] = struct.pack("<i", -1)
            with self.assertRaises(ArchiveError):
                frozen.flowmediumbackend.read_backend_prefix(changed)

    def test_empty_backend_framing_does_not_consume_or_validate_outer_tail(self) -> None:
        for tail in (b"", b"\x78\x9c\xff", bytes(range(256))):
            parsed = frozen.flowmediumbackend.read_backend_prefix(bytes(11) + tail)
            self.assertTrue(parsed["backend_framing_complete"])
            self.assertEqual(parsed["prefix_end"], 11)
            self.assertEqual(parsed["opaque_tail"], tail)
            self.assertFalse(parsed["full_record_schema"])
