"""Independent selector fixture coverage and isolated regeneration."""
import gzip
import hashlib
import json
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from freeze_selection import ROOT, freeze, guid_cases, selection_row


class SelectionFixtureTests(unittest.TestCase):
    def test_all_guid_positions_and_literal_nil_transitions(self) -> None:
        keys = guid_cases()
        self.assertEqual(len(set(keys)), 17)
        for index, key in enumerate(keys[1:]):
            self.assertEqual(key[index], index + 1)
            self.assertEqual(sum(value != 0 for value in key), 1)
        nil = bytes(16)
        self.assertEqual(selection_row(nil, nil, 0), nil + nil + bytes([0, 1, 0]) + nil + b"\0")
        self.assertEqual(selection_row(nil, keys[1], 0)[32:35], bytes([0, 2, 1]))
        self.assertEqual(selection_row(nil, keys[1], 255)[32:35], bytes([255, 0, 0]))

    def test_regeneration_uses_a_temporary_output_root(self) -> None:
        target = ROOT / "tests/fixtures/flow-settings/selection.bin.gz"
        original = target.read_bytes()
        with TemporaryDirectory(prefix="bessemer-selection-") as temporary:
            staging = Path(temporary)
            (staging / "tests/fixtures/flow-settings").mkdir(parents=True)
            (staging / "docs/features/flow-settings").mkdir(parents=True)
            freeze(staging)
            self.assertEqual((staging / "tests/fixtures/flow-settings/selection.bin.gz").read_bytes(), original)
            report = json.loads((staging / "docs/features/flow-settings/fixture-verification.json").read_text())
        self.assertEqual(hashlib.sha256(gzip.decompress(original)).hexdigest(), report["sha256"])
        self.assertEqual(report["cases"], 73984)


if __name__ == "__main__":
    unittest.main()
