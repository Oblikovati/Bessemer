"""Independent fixture wire construction and native source binding checks."""
import gzip
import hashlib
import json
import unittest
from freeze_carriers import ROOT, blob, freeze, native_cases, owned, packet_case


class FlowFixtureTests(unittest.TestCase):
    def test_literal_owned_wire_and_packet(self) -> None:
        self.assertEqual(blob(b"a"), b"\x01" + b"\0" * 7 + b"a")
        raw = owned(7, [(19, b"x")])
        self.assertEqual(raw, bytes.fromhex("070013000000010000000000000078"))
        packet = packet_case("a", raw)
        self.assertTrue(packet.startswith(blob(b"a") + blob(raw)))
        self.assertTrue(packet.endswith(bytes.fromhex("1300000002000000000000000100000000000000")))

    def test_regeneration_retains_native_sources_and_complete_packet(self) -> None:
        target = ROOT / "tests/fixtures/flow-carriers/carriers.bin.gz"
        before = target.read_bytes()
        freeze()
        self.assertEqual(target.read_bytes(), before)
        report = json.loads((ROOT / "docs/features/flow-carriers/fixture-verification.json").read_text())
        packet = gzip.decompress(before)
        self.assertEqual(hashlib.sha256(packet).hexdigest(), report["packet_sha256"])
        self.assertEqual(len(packet), report["packet_bytes"])
        self.assertEqual(sum(len(raw) for _, raw in native_cases()), 2_936_303)


if __name__ == "__main__":
    unittest.main()
