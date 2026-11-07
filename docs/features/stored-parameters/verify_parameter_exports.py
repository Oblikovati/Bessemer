"""Compare actual parameter outputs with immutable original Source observations (#202)."""
from __future__ import annotations
import argparse
from pathlib import Path
import gzip
import hashlib
import json
import sys
import tempfile
import unittest
from typing import TYPE_CHECKING
if TYPE_CHECKING:
    from parameter_packet import ParameterPacket
    from verify_actual_reference import RootPacket

ROOT = Path(__file__).resolve().parents[3]
FEATURE = Path(__file__).resolve().parent
RESULTS = ROOT / 'target/stored-parameter-actual-results'
sys.path.insert(0, str(ROOT / 'docs/features/model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot


def observe_parameters(document: object, stream: str) -> tuple[list[dict[str, object]], list[dict[str, object]]]:
    """Execute original complete readers under an observation-only profiler; e.g. observe_parameters(doc,stream)."""
    import mfcobjects, swarchive, modelprefix, resolvedfeatures
    from source_parameter_observer import ParameterObserver
    original, cstring = mfcobjects.ObjectArchive.object, swarchive.Archive.cstring
    observer = ParameterObserver(original.__code__, cstring.__code__)
    previous = sys.getprofile()
    try:
        sys.setprofile(observer)
        _, context = modelprefix.read(document, stream.removesuffix('-ResolvedFeatures'),
                                      through_part_resolved=True, configuration_context=True)
        parsed = resolvedfeatures.read_supported(document, stream)
    finally:
        sys.setprofile(previous)
    assert mfcobjects.ObjectArchive.object is original and swarchive.Archive.cstring is cstring
    assert context.p == len(context.b), 'expected complete original configuration EOF'
    assert parsed['tail'] == b'' and parsed['list_complete'], 'expected complete original feature EOF'
    return observer.rows(context), observer.rows(parsed['archive'])


class ActualParameterExportTests(unittest.TestCase):
    def test_a_owned_actual_exports_cover_all_classes_and_literal_boundaries(self) -> None:
        """Compare all30 actual exports to original readers, including null names and signaling NaNs."""
        import swcontainer
        from parameter_packet import ParameterPacket
        from sheet_owned_fixtures import SavedSheetMetalDocument
        first = (RESULTS / 'owned-1.bin').read_bytes()
        self.assertEqual(first, (RESULTS / 'owned-2.bin').read_bytes())
        actual = ParameterPacket(first)
        document = swcontainer.Document((ROOT / 'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
        manager = (ROOT / 'tests/fixtures/sheet-metal/manager-0.bin').read_bytes()
        for kind in range(6):
            for style, edited in [(style, 0) for style in range(4)] + [(0, 1)]:
                self.assertEqual([actual.word() for _ in range(3)], [kind, style, edited])
                filename = f'owned-{kind}-{style}' + ('-edited' if edited else '') + '.bin'
                raw = (ROOT / 'tests/fixtures/stored-parameters' / filename).read_bytes()
                _, expected = observe_parameters(SavedSheetMetalDocument(document, manager, raw), 'Contents/Config-0-ResolvedFeatures')
                self.assertEqual(actual.parameters(), expected, filename)
        self.assertEqual(actual.position, len(actual.raw))

    def test_b_native_actual_exports_are_repeatable_and_equal_original_source(self) -> None:
        """Compare every parameter value and identity in both contexts for all906 native cases."""
        from parameter_packet import ParameterPacket
        from verify_actual_reference import RootPacket, read_native_case
        first = (RESULTS / 'native-1.bin').read_bytes()
        self.assertEqual(first, (RESULTS / 'native-2.bin').read_bytes())
        actual = ParameterPacket(first)
        native = RootPacket((ROOT / 'target/resolved-native-probe.bin').read_bytes())
        self.assertEqual(native.take(4), b'BRF1')
        self.assertEqual(native.word(), 906)
        totals = [0, 0]
        for case in range(906):
            with self.subTest(case=case):
                counts = self.check_native_case(native, actual)
                totals = [before + count for before, count in zip(totals, counts)]
        self.assertEqual(totals, [4430, 39487])
        self.assertEqual(actual.position, len(actual.raw))
        self.assertEqual(native.position, len(native.raw))

    def check_native_case(self, native: RootPacket, actual: ParameterPacket) -> list[int]:
        """Compare one complete input context to actual Rust output; e.g. self.check_native_case(native,actual)."""
        from verify_actual_reference import read_native_case
        version, label, document = read_native_case(native)
        self.assertEqual(actual.word(), version)
        self.assertEqual(actual.frame().decode('utf-8'), label)
        expected = observe_parameters(document, document.feature_stream)
        for rows in expected:
            self.assertEqual(actual.parameters(), rows, label)
        return [len(rows) for rows in expected]


def retain_parameter_export_snapshot(destination: Path) -> None:
    """Retain all original source/import evidence losslessly; e.g. retain_parameter_export_snapshot(results)."""
    raw = (destination / 'fixture-reference-snapshot.json').read_bytes()
    packed = gzip.compress(raw, mtime=0)
    assert gzip.decompress(packed) == raw, 'expected lossless complete source snapshot'
    (FEATURE / 'actual-source-snapshot.json.gz').write_bytes(packed)
    report = json.loads(raw)
    index = {key: report[key] for key in ('source_python_files', 'source_origins', 'tests_run', 'successful', 'snapshot_identity')}
    index.update(original_snapshot_gzip='actual-source-snapshot.json.gz',
                 original_snapshot_sha256=hashlib.sha256(raw).hexdigest(), gzip_sha256=hashlib.sha256(packed).hexdigest())
    (FEATURE / 'actual-source-snapshot.json').write_bytes((json.dumps(index, indent=2) + '\n').encode())


def main() -> int:
    """Freeze before importing upstream modules, then compare complete outputs; e.g. main()."""
    owned_only = export_owned_only()
    with tempfile.TemporaryDirectory(prefix='bessemer-parameter-exports-', dir=ROOT / 'target') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        sys.path.insert(0, str(ROOT / 'docs/features/resolved-root-graph'))
        sys.path.insert(0, str(ROOT / 'docs/features/trim-edge-identifiers'))
        import freeze_trim_edges
        suite = unittest.TestSuite([ActualParameterExportTests('test_a_owned_actual_exports_cover_all_classes_and_literal_boundaries')]) if owned_only else unittest.defaultTestLoader.loadTestsFromTestCase(ActualParameterExportTests)
        result = unittest.TextTestRunner(verbosity=2, failfast=True).run(suite)
        record_snapshot(pin, hashes, result, RESULTS, ('modelnode', 'dimensions', 'attributes'),
                        ('swcontainer', 'modelprefix', 'resolvedfeatures', 'mfcobjects', 'swarchive'))
        retain_parameter_export_snapshot(RESULTS)
        return 0 if result.wasSuccessful() else 1


def export_owned_only() -> bool:
    """Permit an early owned-output sanity check before native exports finish; e.g. export_owned_only()."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--owned-only', action='store_true')
    return bool(parser.parse_args().owned_only)


if __name__ == '__main__':
    raise SystemExit(main())
