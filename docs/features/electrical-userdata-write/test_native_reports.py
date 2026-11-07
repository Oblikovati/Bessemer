"""Regression for #239: report hashes bind portable committed LF bytes."""
import hashlib
import json
from pathlib import Path
import unittest

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[2]


class NativeWriteReports(unittest.TestCase):
    def test_all_four_report_hashes_match_canonical_repository_bytes(self)->None:
        summary=json.loads((HERE / "native-write-investigation.json").read_text(encoding="utf-8"))
        total=0
        for archive in summary["archives"]:
            raw=(HERE / archive["edits_file"]).read_bytes()
            self.assertNotIn(b"\r\n",raw)
            self.assertEqual(hashlib.sha256(raw).hexdigest(),archive["edits_sha256"])
            edits=json.loads(raw)
            self.assertEqual(len(edits),archive["edit_count"])
            self.assertEqual(len({(e["object_id"],e["object_no"],e["slot"]) for e in edits}),len(edits))
            self.assertTrue(all(0<=e["slot"]<20 for e in edits))
            original=ROOT / "tests/fixtures/electrical-projects" / archive["name"]
            self.assertEqual(hashlib.sha256(original.read_bytes()).hexdigest(),archive["archive_sha256"])
            total+=len(edits)
        self.assertEqual(total,680)


if __name__=="__main__":unittest.main()
