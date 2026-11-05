"""Check explicit-root native document bindings without using installed files."""
from pathlib import Path
import struct
import tempfile
import unittest
from freeze_native_documents import document_record, framed_text
from test_native_packet import native_samples
from native_packet import NativeCensusRow


class FakeNativeDocuments:
    """Own test sources under an injected root; e.g. FakeNativeDocuments(root).source()."""
    def __init__(self, root: Path) -> None:
        self.root = root

    def source(self) -> NativeCensusRow:
        path = self.root / 'part-μ.sldprt'
        path.write_bytes(b'abc')
        row = native_samples()[0].copy()
        row['path'] = str(path)
        row['stream'] = 'Contents/PMISemanticDataDB'
        return row


class NativeDocumentBindingTests(unittest.TestCase):
    def test_full_original_digest_and_relative_utf8_paths_have_literal_framing(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            fake = FakeNativeDocuments(Path(directory))
            row = fake.source()
            record = document_record(row, fake.root)
        expected = (struct.pack('<I', row['ordinal']) + framed_text('part-μ.sldprt') +
                    framed_text('Contents/PMISemanticDataDB') + struct.pack('<Q', 3))
        self.assertEqual(record[:-32], expected)
        self.assertEqual(record[-32:].hex(),
                         'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad')
        self.assertEqual(framed_text('μ'), b'\x02\0\0\0\xce\xbc')
        self.assertEqual(framed_text(''), b'\0\0\0\0')

    def test_sources_outside_the_explicit_root_are_rejected_before_reading(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            fake = FakeNativeDocuments(Path(directory))
            row = fake.source()
            with self.assertRaises(ValueError):
                document_record(row, fake.root / 'other-root')


if __name__ == '__main__':
    unittest.main()
