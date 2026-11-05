"""Literal role, nil and order expectations plus temporary reference regeneration."""
import gzip
import hashlib
import json
import struct
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from freeze_links import ROOT, A, B, C, NIL, wire, candidate, lookup_row, related_row, freeze


class LinkFixtureTests(unittest.TestCase):
    def test_roles_order_and_nil(self) -> None:
        raw = wire(A, B, C)
        cpt, fld, cfld = [candidate(kind, raw, str(kind)) for kind in (1, 2, 4)]
        self.assertEqual(struct.unpack_from("<i", lookup_row([fld, cfld, cpt], 4, C), 20)[0], 1)
        self.assertEqual(struct.unpack_from("<i", lookup_row([fld, cpt, cfld], 4, C), 20)[0], 1)
        self.assertEqual(struct.unpack_from("<i", lookup_row([fld], 4, C), 20)[0], -1)
        nil = candidate(1, wire(NIL, NIL, NIL), "nil")
        self.assertEqual(struct.unpack_from("<i", lookup_row([nil], 1, NIL), 20)[0], -1)
        self.assertEqual(related_row(raw, [fld]), struct.pack("<iiB", -1, -1, 0))

    def test_regeneration_uses_temporary_directories(self) -> None:
        original = (ROOT / "tests/fixtures/flow-links/links.bin.gz").read_bytes()
        with TemporaryDirectory(prefix="bessemer-links-") as temporary:
            staging = Path(temporary)
            freeze(staging)
            self.assertEqual((staging / "tests/fixtures/flow-links/links.bin.gz").read_bytes(), original)
            report = json.loads((staging / "docs/features/flow-links/fixture-verification.json").read_text())
        self.assertEqual(report["cases"], 21)
        self.assertEqual(report["requests"], 1260)
        self.assertEqual(hashlib.sha256(gzip.decompress(original)).hexdigest(), report["sha256"])


if __name__ == "__main__":
    unittest.main()
