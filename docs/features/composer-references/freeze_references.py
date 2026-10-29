"""Freeze actual supplied-state Composer readers for clauses 14.92–14.93 (#105)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot


def retain(name: str, value: object) -> None:
    """Require reviewed baseline updates, e.g. retain('owned.json', cases)."""
    raw = (json.dumps(value, indent=2, ensure_ascii=True) + '\n').encode()
    path = FEATURE / name
    if '--write' in sys.argv:
        path.write_bytes(raw)
        return
    assert path.read_bytes() == raw, f'{path}; expected unchanged frozen oracle'


def units(value: str) -> list[int]:
    """Retain unpaired UTF-16 units without replacement."""
    raw = value.encode('utf-16-le', 'surrogatepass')
    return list(struct.unpack('<' + 'H' * (len(raw) // 2), raw))


def split_cases() -> list[dict[str, object]]:
    """Use actual Source splits on markers, first separators and raw surrogates."""
    import composerreferences as reference
    inputs = ['', 'plain#guid', 'ref:ext:p#g', 'Ref:Ext:file#guid#tail',
              'Ref:Ext:file', 'xxRef:Ext:file#g', '#junkRef:Ext:f#g',
              '😀Ref:Ext:p#g', 'Ref:Ext:😀#\ud800', '\udc00', 'Ref:Ext:#',
              'Ref:Ext:', ' Ω ', 'Ref:Ext:a\nb#g', '###Ref:Ext:x#last']
    inputs += [prefix + 'Ref:Ext:' + tail for prefix in ('x', '😀', '\ud800', '#', 'ab#c', '1234567😀')
               for tail in ('p#g', 'p', 'p#g#tail', '#', '')]
    cases = []
    for value in inputs:
        result = reference.split_reference(value)
        cases.append(dict(input=units(value), external=result['external'],
                          marker=result['marker_code_unit_index'], separator=result['separator_code_unit_index'],
                          file=None if result['file_reference'] is None else units(result['file_reference']),
                          guid=units(result['guid'])))
    return cases


def path_cases() -> list[dict[str, object]]:
    """Pin mixed separators and dots in directories against actual lexical helpers."""
    import composerreferences as reference
    inputs = ['', 'scenario.xml', 'one/scenario.xml', 'C:\\one\\SCENARIO.XML',
              'C:\\one/two.geom', 'one/two.geom', 'C:\\dir.dot\\file',
              'C:\\one\\two.xml', 'one/two.xml', 'file.', '.xml', 'a\\', 'a/b\\c/d']
    return [dict(input=value, file=reference.get_file(value), folder=reference.get_file_folder(value),
                 base_path=reference.get_file_base_path(value), base=reference.get_file_base(value),
                 initial=reference.initial_file_key(value),
                 fallback=reference.fallback_file_keys(value, 'C:\\project\\part.smgXml')) for value in inputs]


def server_cases() -> list[dict[str, object]]:
    """Retain every lookup/insertion branch, including override-only supplied state."""
    from composergeodeserver import GeodeServer
    states = [([], []), ([('g', 0)], []), ([('g', 7)], []),
              ([('g', 7)], [('g', 0)]), ([('g', 0)], [('g', 9)]),
              ([], [('g', 0)]), ([], [('g', 9)]), ([('g', 0)], [('g', 0)])]
    cases = []
    for primary, override in states:
        for incoming in [0, 7, 13]:
            server = GeodeServer()
            server.seed('FiLe', primary=primary, override=override)
            before = server.get('FILE', 'g', initial_pointer=42)
            registration = server.add('file', 'g', incoming)
            cases.append(dict(primary=primary, override=override, incoming=incoming,
                              before=before, registration=registration,
                              after=server.get('fIlE', 'g', initial_pointer=42),
                              missing=server.get('file', 'G', initial_pointer=42)))
    return cases


def lookup_cases() -> list[dict[str, object]]:
    """Keep duplicate attempts and every stop position with zero-pointer successes."""
    from composergeodeserver import GeodeServer
    import composerreferences as reference
    cases = []
    for file, project in [('C:/one/model.smggeom', 'C:\\project\\part.smgXml'),
                          ('geometry.geom', 'geometry.xml')]:
        keys = reference.fallback_file_keys(file, project)
        for stop in range(5):
            server = GeodeServer()
            if stop < 4:
                server.seed(keys[stop], primary=[('g', 123)])
            result = reference.lookup_nonempty(server, file_key=file, project_path=project, guid='g')
            cases.append(dict(file=file, project=project, seeded=None if stop == 4 else keys[stop],
                              result=result))
    return cases


def owned() -> None:
    """Freeze complete actual-reader projections, never substitute parser functions."""
    cases = dict(splits=split_cases(), paths=path_cases(), servers=server_cases(), lookups=lookup_cases())
    retain('owned-reference.json', cases)
    packet = binary_value(cases)
    fixture = ROOT / 'tests/fixtures/composer-references/owned.bin'
    if '--write' in sys.argv:
        fixture.parent.mkdir(parents=True, exist_ok=True)
        fixture.write_bytes(packet)
    assert fixture.read_bytes() == packet


def binary_value(value: object) -> bytes:
    """Encode a small test-only tagged oracle without introducing a JSON dependency."""
    if value is None:
        return b'\0'
    if type(value) is bool:
        return bytes([2 if value else 1])
    if type(value) is int:
        return b'\3' + struct.pack('<q', value)
    if type(value) is str:
        raw = value.encode()
        return b'\4' + struct.pack('<I', len(raw)) + raw
    if type(value) in (list, tuple):
        return b'\5' + struct.pack('<I', len(value)) + b''.join(map(binary_value, value))
    assert type(value) is dict, f'{type(value)}; expected oracle mapping'
    return b'\6' + struct.pack('<I', len(value)) + b''.join(binary_value(key) + binary_value(item) for key, item in value.items())


def schema() -> None:
    """Bind both full clauses while allowing unrelated upstream documentation edits."""
    source = ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md'
    raw = source.read_bytes()
    text = raw.decode('utf-8').replace('\r\n', '\n')
    begin = text.index('### 14.92 ')
    end = text.index('### 14.94 ', begin)
    snapshot = (text[begin:end].rstrip() + '\n').encode()
    destination = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        destination.write_bytes(snapshot)
    assert destination.read_bytes() == snapshot
    retain('schema-reference.json', dict(clauses=['14.92', '14.93'], requirements=['C-468', 'C-469'],
                                        issue=105, snapshot_sha256=hashlib.sha256(snapshot).hexdigest()))


def native_record(path: str) -> dict[str, object]:
    """Read complete actual binary and XML members before explicitly supplying runtime state."""
    import composer
    import composerscene
    import composergeometry
    import composerxmllinks
    from composergeodeserver import GeodeServer
    document = composer.Document(path)
    geometry = next(entry for entry in document.entries if entry.name.endswith('.smgGeom'))
    xml = next(entry for entry in document.entries if entry.name.endswith('.smgXml'))
    scene_raw, xml_raw = document.read(geometry), document.read(xml)
    scene_raw = composergeometry.decode(scene_raw)['decoded']
    scene = composerscene.read_decoded(scene_raw, texture_import_enabled=True)
    assert scene['end'] == len(scene_raw)
    links = composerxmllinks.read(xml_raw, scene=scene)
    server = GeodeServer()
    for guid, indices in links['guid_index'].items():
        for index in indices:
            server.add(geometry.name, guid, index + 1)
    return native_projection(server, geometry.name, xml.name, links, path, scene_raw, xml_raw)


def native_projection(server: object, geometry: str, project: str, links: dict, path: str, scene: bytes, xml: bytes) -> dict:
    """Capture every supplied-state attempt; raw external text is never a resolved file key."""
    import composerreferences as reference
    references = []
    file = reference.initial_file_key(project)
    for occurrence in links['references']:
        identifier = occurrence['ident_geom']
        split = reference.split_reference(identifier)
        assert not split['external'] and split['guid'] == identifier
        lookup = reference.lookup_nonempty(server, file_key=file, project_path=project, guid=identifier)
        references.append(dict(input=identifier, lookup=lookup))
    return dict(geometry=geometry, project=project, references=references,
                archive_sha256=hashlib.sha256(Path(path).read_bytes()).hexdigest(),
                scene_sha256=hashlib.sha256(scene).hexdigest(), xml_sha256=hashlib.sha256(xml).hexdigest())


def native() -> None:
    """Bind all installed projections and optionally export the complete Rust probe."""
    inputs = json.loads((FEATURE.parent / 'composer-links/native-reference.json').read_text())
    records = [native_record(record['path']) for record in inputs]
    references = [ref for record in records for ref in record['references']]
    packet = binary_value(records)
    report = dict(files=len(records), references=len(references),
                  found=sum(ref['lookup']['pointer'] != 0 for ref in references),
                  attempts=sum(len(ref['lookup']['attempts']) for ref in references),
                  packet_sha256=hashlib.sha256(packet).hexdigest(),
                  identities=[{key: record[key] for key in ('archive_sha256', 'scene_sha256', 'xml_sha256')} for record in records])
    assert (report['files'], report['references'], report['found'], report['attempts']) == (38, 4445, 4416, 4532)
    retain('native-reference.json', report)
    output = b''.join(lookup_bytes(ref['lookup']) for ref in references)
    retain('native-output-reference.json', dict(bytes=len(output), sha256=hashlib.sha256(output).hexdigest()))
    if '--export' in sys.argv:
        (ROOT / 'target/composer-references-native.bin').write_bytes(packet)


def lookup_bytes(lookup: dict) -> bytes:
    """Encode every observable lookup value for independent process comparisons."""
    output = struct.pack('<QI', lookup['pointer'], len(lookup['attempts']))
    for attempt in lookup['attempts']:
        key = attempt['file_key'].encode('utf-16-le', 'surrogatepass')
        result = attempt['result']
        tier = {None: 0, 'override': 1, 'primary': 2}[result['tier']]
        output += struct.pack('<I', len(key) // 2) + key
        output += struct.pack('<BQB', result['found'], result['pointer'], tier)
    return output


def run() -> bool:
    """Freeze module origins before any actual Source import."""
    with tempfile.TemporaryDirectory(prefix='bessemer-reference-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(test) for test in (owned, schema, native))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('composerreferences', 'composergeodeserver', 'composerscene', 'composerxmllinks'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'owned-source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
