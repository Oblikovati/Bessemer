"""Check actual Bessemer edit outputs with an immutable original Source (#199)."""
from pathlib import Path
import gzip
import hashlib
import json
import struct
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[3]
FEATURE = Path(__file__).resolve().parent
RESULTS = ROOT / 'target/spline-equation-actual-results'
sys.path.insert(0, str(ROOT / 'docs/features/model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot


class EquationPacket:
    def __init__(self, raw: bytes) -> None:
        """Own a bounded packet cursor; e.g. EquationPacket(packet_bytes)."""
        self.raw = raw
        self.position = 0

    def word(self, signed: bool = False) -> int:
        """Read a literal little-endian word; e.g. packet.word(True)."""
        value, = struct.unpack('<i' if signed else '<I', self.take(4))
        return value

    def take(self, length: int) -> bytes:
        """Require a complete span; e.g. packet.take(4)."""
        end = self.position + length
        assert self.position <= end <= len(self.raw), f'{length} at {self.position}; expected complete packet span'
        value = self.raw[self.position:end]
        self.position = end
        return value

    def frame(self) -> bytes:
        """Read a bounded document/text frame; e.g. packet.frame()."""
        length = self.word()
        assert length <= 16_000_000, f'{length}; expected frame no larger than 16MB'
        return self.take(length)


class ActualEquationReferenceTests(unittest.TestCase):
    def test_fresh_process_packets_are_identical(self) -> None:
        """Require both complete fresh-process exports to agree byte for byte."""
        self.assertEqual((RESULTS / 'results-1.bin').read_bytes(), (RESULTS / 'results-2.bin').read_bytes())

    def test_actual_documents_match_original_saved_roles_through_eof(self) -> None:
        """Observe all 906 actual edited complete documents, not regenerated expected bytes."""
        packet = EquationPacket((RESULTS / 'results-1.bin').read_bytes())
        count = 0
        while packet.position < len(packet.raw):
            with self.subTest(case=count):
                self.check_actual_document(packet)
            count += 1
        self.assertEqual(count, 906)
        self.assertEqual(packet.position, len(packet.raw))

    def check_actual_document(self, packet: EquationPacket) -> None:
        """Compare Bessemer's exposed roles with the unmodified reader; e.g. self.check_actual_document(packet)."""
        import swcontainer
        import resolvedfeatures
        document = swcontainer.Document(packet.frame(), table={})
        raw = packet.frame()
        self.assertEqual(document.read('Contents/Config-0-ResolvedFeatures'), raw)
        parsed = resolvedfeatures.read_supported(document, 'Contents/Config-0-ResolvedFeatures')
        self.assertEqual((parsed['end'], parsed['tail'], parsed['list_complete']), (len(raw), b'', True))
        self.assertEqual(len(parsed['items']), 1)
        splines = parsed['items'][0]['splines']
        self.assertEqual(len(splines), 1)
        actual = splines[0]
        self.assertEqual(actual['flags_32c'], packet.word())
        self.assertEqual(actual['equation_word'], packet.word(True))
        self.assertEqual(actual['unknown_1e0'], packet.word(True))
        self.assertEqual(actual['equation_strings'], [packet.frame().decode('utf-8') for _ in range(6)])


def retain_actual_snapshot(destination: Path) -> None:
    """Retain the complete import/hash proof losslessly; e.g. retain_actual_snapshot(evidence_root)."""
    raw = (destination / 'fixture-reference-snapshot.json').read_bytes()
    packed = gzip.compress(raw, mtime=0)
    (FEATURE / 'actual-source-snapshot.json.gz').write_bytes(packed)
    report = json.loads(raw)
    index = {key: report[key] for key in ('source_python_files', 'tests_run', 'successful', 'snapshot_identity')}
    index.update(raw_sha256=hashlib.sha256(raw).hexdigest(), gzip_sha256=hashlib.sha256(packed).hexdigest(),
                 original_snapshot_gzip='actual-source-snapshot.json.gz')
    (FEATURE / 'actual-source-snapshot.json').write_bytes((json.dumps(index, indent=2) + '\n').encode())


def main() -> int:
    """Freeze before any Source imports and run actual-result checks; e.g. main()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-equation-actual-', dir=ROOT / 'target') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        suite = unittest.defaultTestLoader.loadTestsFromTestCase(ActualEquationReferenceTests)
        result = unittest.TextTestRunner(verbosity=2, failfast=True).run(suite)
        record_snapshot(pin, hashes, result, RESULTS, ('sketchprefix',))
        retain_actual_snapshot(RESULTS)
        return 0 if result.wasSuccessful() else 1


if __name__ == '__main__':
    raise SystemExit(main())
