"""Schema snapshot provenance must survive Windows output without baseline changes."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]/'docs/features/model-archives'))
from schema_snapshot_bytes import PAIRS, repair, section_manifest, verified_snapshot, write_verified_snapshot


class FakeEvidenceRepository:
    def __init__(self, root: Path) -> None:
        self.root = root
        self.environment = {name: value for name, value in os.environ.items() if not name.startswith('GIT_')}
        self.environment.update(GIT_CONFIG_GLOBAL=os.devnull, GIT_CONFIG_NOSYSTEM='1', GIT_ATTR_NOSYSTEM='1')
        subprocess.run(['git', 'init', '--quiet', '--template=', '--object-format=sha1', str(root)],
                       env=self.environment, check=True, capture_output=True)
        attributes = Path(__file__).resolve().parents[1]/'.gitattributes'
        (root/'.gitattributes').write_bytes(attributes.read_bytes())

    def filtered_blob_id(self, relative: str, raw: bytes) -> str:
        result = subprocess.run(['git', '-C', str(self.root), 'hash-object', '--path='+relative, '--stdin'],
                                input=raw, env=self.environment, check=True, capture_output=True)
        return result.stdout.decode('ascii').strip()


class SchemaSnapshotBytesTests(unittest.TestCase):
    def test_git_filters_keep_hash_bound_native_evidence_and_oracle_sources_exact(self) -> None:
        raw = b'header\r\nschema 1\r\nfields u8 f64 f64\r\n'
        expected = hashlib.sha1(f'blob {len(raw)}\0'.encode()+raw).hexdigest()
        paths = ('dib-previews', 'dib-zero-image-size', 'dib-bgr24', 'dib-indexed-rgb', 'dib-rgb555', 'sweep-twist')
        with tempfile.TemporaryDirectory() as temporary:
            repository = FakeEvidenceRepository(Path(temporary))
            for feature in paths:
                for name in ('native-schema-evidence.json', 'schema-snapshot.md', 'fixture_reader.py'):
                    relative = f'docs/features/{feature}/{name}'
                    self.assertEqual(repository.filtered_blob_id(relative, raw), expected, relative)

    def test_crlf_source_bytes_are_not_translated_when_written(self) -> None:
        source = b'### 11.18 Example\r\n\r\nStored words.\r\n'
        expected = {'11.18': hashlib.sha256(source).hexdigest()}
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary)/'snapshot.md'
            self.assertEqual(write_verified_snapshot(path, source, expected), source)
            self.assertEqual(path.read_bytes(), source)

    def test_one_windows_translation_layer_is_repaired(self) -> None:
        source = b'### 11.18 Example\r\n\r\nStored words.\r\n'
        corrupted = source.replace(b'\n', b'\r\n')
        expected = {'11.18': hashlib.sha256(source).hexdigest()}
        self.assertEqual(verified_snapshot(corrupted, expected), source)
        self.assertEqual(verified_snapshot(source, expected), source)

    def test_lf_source_and_multiple_section_boundaries_stay_exact(self) -> None:
        first = b'### 11.17 First\n\nFirst words.\n\n'
        second = b'### 11.18 Second\n\nSecond words.\n'
        expected = {'11.17': hashlib.sha256(first).hexdigest(), '11.18': hashlib.sha256(second).hexdigest()}
        self.assertEqual(verified_snapshot(b'# Snapshot\n\n'+first+second, expected), b'# Snapshot\n\n'+first+second)

    def test_changed_or_missing_sections_are_not_repaired(self) -> None:
        source = b'### 11.18 Example\r\n\r\nStored words.\r\n'
        expected = {'11.18': hashlib.sha256(source).hexdigest()}
        for changed in [source.replace(b'Stored', b'Changed'), b'# Missing section\n']:
            with self.assertRaisesRegex(ValueError, 'expected'):
                verified_snapshot(changed, expected)

    def test_failed_validation_does_not_overwrite_an_existing_file(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary)/'snapshot.md'
            path.write_bytes(b'original artifact')
            with self.assertRaises(ValueError):
                write_verified_snapshot(path, b'not a section', {'11.18': '0'*64})
            self.assertEqual(path.read_bytes(), b'original artifact')

    def test_empty_manifest_and_invalid_utf8_are_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, 'expected'):
            verified_snapshot(b'anything', {})
        with self.assertRaisesRegex(ValueError, 'UTF-8'):
            verified_snapshot(b'\xff', {'11.18': '0'*64})

    def test_manifest_shape_requires_text_sha256_values(self) -> None:
        self.assertEqual(section_manifest({'sections': {'11.18': 'a'*64}}), {'11.18': 'a'*64})
        for value in [None, {}, {'sections': []}, {'sections': {'11.18': 7}}, {'sections': {'11.18': 'A'*64}}]:
            with self.assertRaisesRegex(ValueError, 'expected'):
                section_manifest(value)

    def test_rerun_keeps_the_original_repair_evidence(self) -> None:
        source = b'### 11.18 Example\r\n\r\nStored words.\r\n'
        manifest = {'sections': {'11.18': hashlib.sha256(source).hexdigest()}}
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            base = root/'docs/features/model-archives'
            base.mkdir(parents=True)
            for snapshot, metadata in PAIRS:
                (base/snapshot).write_bytes(source.replace(b'\n', b'\r\n'))
                (base/metadata).write_text(json.dumps(manifest))
            repair(root)
            report = (base/'schema-snapshot-newlines.json').read_bytes()
            repair(root)
            self.assertEqual((base/'schema-snapshot-newlines.json').read_bytes(), report)
            self.assertEqual((base/PAIRS[0][0]).read_bytes(), source)


if __name__ == '__main__':
    unittest.main()
