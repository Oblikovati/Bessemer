"""Compare actual root snapshots to frozen original Source observations (#201)."""
from pathlib import Path
import gzip
import hashlib
import json
import struct
import sys
import tempfile
import unittest
from source_root_observer import OriginalObjectObserver, RootObservation

ROOT = Path(__file__).resolve().parents[3]
FEATURE = Path(__file__).resolve().parent
RESULTS = ROOT / 'target/resolved-root-actual-results'
sys.path.insert(0, str(ROOT / 'docs/features/model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot


class RootPacket:
    def __init__(self, raw: bytes) -> None:
        """Own a bounded packet cursor; e.g. RootPacket(packet_bytes)."""
        self.raw = raw
        self.position = 0

    def take(self, size: int) -> bytes:
        """Require a complete span; e.g. packet.take(4)."""
        end = self.position + size
        assert self.position <= end <= len(self.raw), f'{size} at {self.position}; expected complete packet span'
        value = self.raw[self.position:end]
        self.position = end
        return value

    def word(self) -> int:
        """Read an unsigned little-endian word; e.g. packet.word()."""
        value, = struct.unpack('<I', self.take(4))
        return value

    def frame(self) -> bytes:
        """Read a bounded UTF-8 frame; e.g. packet.frame()."""
        size = self.word()
        assert size <= 1_000_000, f'{size}; expected metadata frame at most1MB'
        return self.take(size)

    def observation(self, ordinal: int) -> RootObservation:
        """Decode actual exported values, without a reference implementation; e.g. packet.observation(0)."""
        assert self.word() == ordinal, f'{ordinal}; expected physical root ordinal'
        tag = self.word()
        present = self.word()
        assert present in (0, 1), f'{present}; expected extension presence0/1'
        extension = self.word() if present else None
        kind, index = self.word(), self.word()
        name, schema = self.frame().decode('utf-8'), self.word()
        return RootObservation(tag, extension, kind, index, name, schema, self.word(), self.word())


class FrozenRootContext:
    def __init__(self, version: int, label: str, parts: list[bytes]) -> None:
        """Expose only exact frozen context streams; e.g. FrozenRootContext(version,label,parts)."""
        configuration = label.rsplit('::', 1)[1]
        self.version = version
        self.feature_stream = configuration + '-ResolvedFeatures'
        names = [configuration + '-ModelHeader', configuration, 'Contents/CMgr', self.feature_stream]
        self.streams = dict(zip(names, parts))

    def names(self) -> list[str]:
        """Retain the explicit native model version; e.g. context.names()."""
        return [f'_MO_VERSION_{self.version}/', *self.streams]

    def read(self, name: str) -> bytes:
        """Require a supplied complete stream; e.g. context.read('Contents/CMgr')."""
        assert name in self.streams, f'{name!r}; expected a frozen selected context stream'
        return self.streams[name]


def read_native_case(packet: RootPacket) -> tuple[int, str, FrozenRootContext]:
    """Read the existing BRF1 packet without rebuilding inputs; e.g. read_native_case(packet)."""
    version = packet.word()
    sizes = [packet.word() for _ in range(5)]
    parts = [packet.take(size) for size in sizes]
    label = parts[0].decode('utf-8')
    return version, label, FrozenRootContext(version, label, parts[1:])


def observe_original_roots(document: object, stream: str) -> list[RootObservation]:
    """Call original readers under an observation-only profiler; e.g. observe_original_roots(doc,stream)."""
    import mfcobjects
    import resolvedfeatures
    original = mfcobjects.ObjectArchive.object
    observer = OriginalObjectObserver(original.__code__)
    previous = sys.getprofile()
    try:
        sys.setprofile(observer)
        parsed = resolvedfeatures.read_supported(document, stream)
    finally:
        sys.setprofile(previous)
    assert mfcobjects.ObjectArchive.object is original, 'expected original method identity unchanged'
    assert parsed['tail'] == b'' and parsed['list_complete'], 'expected complete Source feature EOF'
    archive = parsed['archive']
    return [observer.records[id(archive), start] for start, _ in parsed['spans']]


class ActualRootReferenceTests(unittest.TestCase):
    def test_a_original_observer_matches_owned_null_alias_and_definition_literals(self) -> None:
        """Exercise null and alias observation separately from the native corpus's zero-null population."""
        import swcontainer
        from sheet_owned_fixtures import SavedSheetMetalDocument
        original = swcontainer.Document((ROOT / 'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
        raw = (ROOT / 'tests/fixtures/spline-equation-state/alias-graphs/graph-0.bin').read_bytes()
        manager = (ROOT / 'tests/fixtures/sheet-metal/manager-0.bin').read_bytes()
        actual = observe_original_roots(SavedSheetMetalDocument(original, manager, raw), 'Contents/Config-0-ResolvedFeatures')
        self.assertEqual([row.tag for row in actual], [65535, 98, 0, 65535, 100])
        self.assertEqual([row.index for row in actual], [98, 98, 0, 100, 100])
        self.assertEqual([row.kind for row in actual], [1, 2, 0, 1, 2])
        self.assertEqual([row.target_kind for row in actual], [2, 2, 0, 2, 2])
        self.assertEqual([row.schema for row in actual], [7, 7, 0, 7, 7])

    def test_b_fresh_process_exports_match_and_all_native_roots_equal_original_source(self) -> None:
        """Compare every exported native wire value and loaded identity, without replacing the accounting oracle."""
        first = (RESULTS / 'results-1.bin').read_bytes()
        self.assertEqual(first, (RESULTS / 'results-2.bin').read_bytes())
        actual = RootPacket(first)
        native = RootPacket((ROOT / 'target/resolved-native-probe.bin').read_bytes())
        self.assertEqual(native.take(4), b'BRF1')
        self.assertEqual(native.word(), 906)
        count = 0
        for case in range(906):
            with self.subTest(case=case):
                count += self.check_native_roots(native, actual)
        self.assertEqual(count, 26789)
        self.assertEqual(native.position, len(native.raw))
        self.assertEqual(actual.position, len(actual.raw))

    def check_native_roots(self, native: RootPacket, actual: RootPacket) -> int:
        """Compare one complete selected context; e.g. self.check_native_roots(native,actual)."""
        version, label, document = read_native_case(native)
        self.assertEqual(actual.word(), version)
        self.assertEqual(actual.frame().decode('utf-8'), label)
        expected = observe_original_roots(document, document.feature_stream)
        self.assertEqual(actual.word(), len(expected))
        for ordinal, row in enumerate(expected):
            self.assertEqual(actual.observation(ordinal), row)
        return len(expected)


def retain_snapshot(destination: Path) -> None:
    """Preserve every import/hash fact losslessly; e.g. retain_snapshot(evidence)."""
    raw = (destination / 'fixture-reference-snapshot.json').read_bytes()
    packed = gzip.compress(raw, mtime=0)
    (FEATURE / 'actual-source-snapshot.json.gz').write_bytes(packed)
    report = json.loads(raw)
    index = {key: report[key] for key in ('source_python_files', 'tests_run', 'successful', 'snapshot_identity')}
    index.update(raw_sha256=hashlib.sha256(raw).hexdigest(), gzip_sha256=hashlib.sha256(packed).hexdigest(),
                 original_snapshot_gzip='actual-source-snapshot.json.gz')
    (FEATURE / 'actual-source-snapshot.json').write_bytes((json.dumps(index, indent=2) + '\n').encode())


def main() -> int:
    """Freeze before imports and verify actual exports; e.g. main()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-root-actual-', dir=ROOT / 'target') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        sys.path.insert(0, str(ROOT / 'docs/features/trim-edge-identifiers'))
        import freeze_trim_edges
        suite = unittest.defaultTestLoader.loadTestsFromTestCase(ActualRootReferenceTests)
        result = unittest.TextTestRunner(verbosity=2, failfast=True).run(suite)
        record_snapshot(pin, hashes, result, RESULTS, ('sketchprefix',))
        retain_snapshot(RESULTS)
        return 0 if result.wasSuccessful() else 1


if __name__ == '__main__':
    raise SystemExit(main())
