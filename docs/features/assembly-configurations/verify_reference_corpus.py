"""Bind complete ordinary/routing assembly configuration baselines to actual Source (#99)."""
from pathlib import Path
import hashlib
import json
import sys
import tempfile
import time
import unittest

FEATURE = Path(__file__).resolve().parent
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import SOURCE, FrozenSourceImports, copy_sources, record_snapshot

INPUTS = ('assembly-resolved-assembly-tail-prefix-corpus.json',
          'route-assembly-resolved-assembly-tail-prefix-corpus.json')


def manifest_snapshot(path: Path) -> bytes:
    """Keep reviewed JSON stable across text checkout conventions; e.g. manifest_snapshot(path)."""
    return path.read_bytes().replace(b'\r\n', b'\n')


class ManifestSnapshotTests(unittest.TestCase):
    def test_checkout_newlines_do_not_change_json_or_escaped_string_values(self) -> None:
        """Exercise real temporary files without depending on a writable user home."""
        expected = b'{\n  "value": "literal\\r\\n"\n}\n'
        with tempfile.TemporaryDirectory(prefix='bessemer-manifest-') as temporary:
            path = Path(temporary) / 'fixture.json'
            for raw in (expected, expected.replace(b'\n', b'\r\n')):
                path.write_bytes(raw)
                self.assertEqual(manifest_snapshot(path), expected)
                self.assertEqual(json.loads(manifest_snapshot(path)), {'value': 'literal\r\n'})


def reference_cases() -> list[tuple[str, dict[str, object]]]:
    """Select root kinds from the verified manifest rather than filenames; e.g. reference_cases()."""
    result = []
    for name in INPUTS:
        report = json.loads((FEATURE / name).read_bytes())
        assert not report['errors'] and not report['stops']
        result.extend((report['requested_root'], row) for row in report['prefixes'])
    assert len(result) == 180
    return result


def verify_assemblies() -> None:
    """Require physical EOF and inherited section boundaries for every complete configuration."""
    from swcontainer import Document
    from modelprefix import read
    results = []
    started = time.perf_counter()
    for kind, row in reference_cases():
        document = Document(Path(row['path']), table={})
        raw = document.read(row['stream'])
        assert hashlib.sha256(raw).hexdigest() == row['stream_sha256']
        root, cursor = read(document, row['stream'], root_class=kind, assembly_resolved_stage='assembly_tail')
        assert cursor.p == row['end'] == len(raw) and root['_class'] == kind
        assert root['unit_component']['owner'] is root
        assert root['lightweight_end'] == root['resolved']['prefix_start']
        assert root['resolved']['prefix_end'] == root['resolved']['end']
        header = document.read(row['stream'] + '-ModelHeader')
        results.append(dict(path=row['path'], stream=row['stream'], version=cursor.version, root_class=kind,
                            bytes=len(raw), map_slots=len(cursor.map), stream_sha256=row['stream_sha256'],
                            header_sha256=hashlib.sha256(header).hexdigest()))
    report = dict(cases=len(results), configuration_bytes=sum(row['bytes'] for row in results), records=results,
                  seconds=time.perf_counter() - started, reader_functions_replaced=False)
    (FEATURE / 'reference-corpus.json').write_bytes((json.dumps(report, indent=2) + '\n').encode())


def main() -> None:
    """Pin the reference and record exact origins after the complete baseline; e.g. main()."""
    for name in INPUTS:
        source = SOURCE / 'research' / name
        destination = FEATURE / name
        raw = manifest_snapshot(source)
        if destination.exists():
            assert manifest_snapshot(destination) == raw, f'{name}; expected the reviewed native manifest'
        else:
            destination.write_bytes(raw)
    with tempfile.TemporaryDirectory(prefix='bessemer-assembly-reference-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        suite = unittest.TestSuite([unittest.defaultTestLoader.loadTestsFromTestCase(ManifestSnapshotTests),
                                   unittest.FunctionTestCase(verify_assemblies)])
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        assert result.wasSuccessful() and result.testsRun == 2 and not result.skipped
        record_snapshot(pin, hashes, result, FEATURE, ('assemblylightweight', 'assemblyresolved', 'assemblyresolvedtail', 'routeassembly'),
                        ('swcontainer', 'modelprefix', 'mfcobjects'))
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'reference-source-snapshot.json')


if __name__ == '__main__':
    main()
