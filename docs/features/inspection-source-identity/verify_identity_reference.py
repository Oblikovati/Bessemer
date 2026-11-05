"""Run unchanged Source comparison against owned metadata boundaries (#195)."""
import unittest
from pathlib import Path
import hashlib
import os
import json
import inspectionrecords
from freeze_identity import RFC_VECTORS, metadata_inputs, metadata_xml, owned_inputs


class SourceIdentityReferenceTests(unittest.TestCase):
    def test_every_owned_binary_source_compares_with_exact_metadata(self) -> None:
        """Exercise the original operation with all 172 independently cross-checked inputs."""
        for label, source in owned_inputs():
            checksum = hashlib.md5(source).hexdigest()
            document = inspectionrecords.read(metadata_xml(str(len(source)), checksum))['sheets'][0]['documents'][0]
            actual = inspectionrecords.compare_document_source(document, source)
            expected = dict(stored_size=str(len(source)), actual_size=len(source), size_matches=True,
                            stored_md5=checksum, actual_md5=checksum, md5_matches=True)
            self.assertEqual(actual, expected, label)

    def test_native_pdf_hashes_match_independent_dotnet_variants(self) -> None:
        """Cross-check original, changed and truncated source checksums with another provider."""
        source = Path(os.environ['BESSEMER_INSPECTION_PDF_SOURCE']).read_bytes()
        proof = Path(__file__).parent / 'dotnet-reference-verification.json'
        expected = json.loads(proof.read_text())['native_source_pdf']
        changed = source[:-1] + bytes((source[-1] ^ 1,))
        self.assertEqual(hashlib.md5(source).hexdigest(), expected['md5'])
        self.assertEqual(hashlib.md5(changed).hexdigest(), expected['modified_md5'])
        self.assertEqual(hashlib.md5(source[:-1]).hexdigest(), expected['truncated_md5'])

    def test_native_tutorial_and_modified_or_truncated_caller_bytes(self) -> None:
        """Bind the original tutorial PDF independently of its stored SourcePath."""
        source = Path(os.environ['BESSEMER_INSPECTION_PDF_SOURCE']).read_bytes()
        self.assertEqual(hashlib.sha256(source).hexdigest(), 'e7769d7679d8e845849e733af21e36013bd88922afae77a7193840dc38263591')
        fixture = Path(__file__).resolve().parents[3] / 'tests/fixtures/standalone-inspection/historical.xml'
        document = inspectionrecords.read(fixture.read_bytes())['sheets'][0]['documents'][0]
        identity = inspectionrecords.compare_document_source(document, source)
        self.assertEqual(identity['actual_size'], 164616)
        self.assertEqual(identity['actual_md5'], '32c2e42f4813cfedb8edc1d7a5b39b8e')
        self.assertTrue(identity['size_matches'] and identity['md5_matches'])
        changed = source[:-1] + bytes((source[-1] ^ 1,))
        modified = inspectionrecords.compare_document_source(document, changed)
        self.assertTrue(modified['size_matches'])
        self.assertFalse(modified['md5_matches'])
        truncated = inspectionrecords.compare_document_source(document, source[:-1])
        self.assertFalse(truncated['size_matches'] or truncated['md5_matches'])

    def test_exact_stored_text_comparisons_preserve_missing_and_empty_fields(self) -> None:
        """Require original Source results for all 22 spelling/default cases."""
        for label, size, checksum in metadata_inputs():
            xml = metadata_xml(size, checksum)
            view = inspectionrecords.read(xml)
            document = view['sheets'][0]['documents'][0]
            self.assertEqual(document['attributes']['SourcePath'], 'Z:/never-open/from-stored-path.pdf')
            actual = inspectionrecords.compare_document_source(document, b'abc')
            expected = dict(stored_size=size, actual_size=3, size_matches=size == '3',
                            stored_md5=checksum, actual_md5=RFC_VECTORS[2][1],
                            md5_matches=checksum == RFC_VECTORS[2][1])
            self.assertEqual(actual, expected, label)


if __name__ == '__main__':
    unittest.main()
