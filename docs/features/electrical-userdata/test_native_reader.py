"""Pin the complete chained-unwind reader rather than its first region (#240)."""
import hashlib
import json
import unittest
from pathlib import Path


class NativeReaderEvidenceTests(unittest.TestCase):
    def test_complete_contiguous_reader(self) -> None:
        report = json.loads(Path(__file__).with_name("reader-native-byte-verification.json").read_text())
        cursor = int(report["address"], 16)
        for region in report["regions"]:
            self.assertEqual(int(region["begin"], 16), cursor)
            cursor = int(region["end"], 16)
            if region["flags"] & 4:
                self.assertEqual(region["parent"], report["address"])
        self.assertEqual(cursor, int(report["end"], 16))
        decoded = b"".join(bytes.fromhex(item["bytes"]) for item in report["instructions"])
        self.assertEqual(decoded, bytes.fromhex(report["native_bytes"]))
        self.assertEqual(len(decoded), 225)
        self.assertEqual(report["original_retained_bytes"], 15)

    def test_installed_artifact_binding(self) -> None:
        import pefile
        report = json.loads(Path(__file__).with_name("reader-native-byte-verification.json").read_text())
        artifact = Path(report["artifact"])
        if not artifact.is_file():
            self.skipTest("Original Electrical DLL is unavailable")
        raw = artifact.read_bytes()
        self.assertEqual(hashlib.sha256(raw).hexdigest(), report["sha256"])
        executable = pefile.PE(data=raw, fast_load=True)
        start = int(report["address"], 16) - executable.OPTIONAL_HEADER.ImageBase
        self.assertEqual(executable.get_data(start, 225), bytes.fromhex(report["native_bytes"]))


if __name__ == "__main__":
    unittest.main()
