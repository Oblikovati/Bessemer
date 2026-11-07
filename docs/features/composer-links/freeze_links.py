"""Pin complete local XML/geode equality and lexical edit oracles (#104)."""
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
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot, SOURCE


def retain(path: Path, raw: bytes) -> None:
    """Replace only with explicit review intent, e.g. retain(path, oracle_bytes)."""
    if '--write' in sys.argv:
        path.write_bytes(raw)
        return
    assert path.read_bytes() == raw, f'{path}; expected unchanged reference baseline'


def report(name: str, value: object) -> None:
    """Retain named facts and source hashes without serializing native payloads."""
    retain(FEATURE / name, (json.dumps(value, indent=2) + '\n').encode())


def word(value: int) -> bytes:
    """Encode oracle counts in a fixed machine-independent form."""
    return struct.pack('<I', value)


def text(value: str) -> bytes:
    """Keep surrogate code units authoritative in the complete binary scene input."""
    return value.encode('utf-16-le', errors='surrogatepass') + b'\0\0'


def scene_bytes(keys: tuple[str | None, ...], leading_face: bool = False) -> bytes:
    """Construct real complete binary scenes, never preseed a fake decoded scene dictionary."""
    import test_composerscene as builders
    raw = struct.pack('<d', 7.9) + text('source') + word(0) + word(0)
    if leading_face:
        raw += builders.face('f')
    for index, key in enumerate(keys):
        body = text(key) if key is not None else b''
        raw += b'u\0' + text(f'geode-{index}') + word(len(body)) + body + word(0)
    return raw


def owned_cases() -> list[tuple[str, bytes, bytes]]:
    """Retain exact equality, duplicate ordering, namespace/Unicode cases and an edit golden."""
    def xml(body: str) -> bytes:
        return ('<DocumentSMG Version="7.900000">' + body + '</DocumentSMG>').encode()
    lexical = b'\xef\xbb\xbf<?xml version="1.0" encoding="UTF-8"?>\n<DocumentSMG Version="7.9" Keep="yes"><!--keep--><?keep x?><M IdentGeom =  \'abc\' Other="same"/></DocumentSMG>\n'
    return [
        ('exact', scene_bytes(('abc', 'ABC', 'other')), xml('<List><Mesh Ident="actor" IdentGeom="abc"/><Mesh IdentGeom="ABC"/><Mesh IdentGeom=" abc "/></List>')),
        ('duplicates', scene_bytes(('x', 'x', '', None)), xml('<A IdentGeom="x"/><B IdentGeom="x"/><C IdentGeom=""/><D/><E IdentGeom="missing"/>')),
        ('group-order', scene_bytes(('x', 'y', 'x')), xml('<A IdentGeom="missing"/>')),
        ('namespaces', scene_bytes(('a&b',)), xml('<U xmlns:q="urn:q" A="&amp;">text<!--keep--><q:M IdentGeom="a&amp;b" q:Note="x"/><M q:IdentGeom="a&amp;b"/></U>')),
        ('lexical', scene_bytes(('abc', 'ABC')), lexical),
        ('lexical-edited', scene_bytes(('abc', 'ABC')), lexical.replace(b"'abc'", b"'ABC'")),
        ('root-reference', scene_bytes(('abc',)), b'<DocumentSMG Version="7.9" IdentGeom="abc"/>'),
        ('unicode', scene_bytes(('Ω𐐀', 'e\u0301')), xml('<M IdentGeom="Ω𐐀"/><M IdentGeom="é"/><M IdentGeom="Ref:Ext:file#Ω𐐀"/>')),
        ('surrogate-key', scene_bytes(('\ud800', None)), xml('<M IdentGeom="ordinary"/>')),
        ('entry-indices', scene_bytes(('abc', 'abc'), True), xml('<M IdentGeom="abc"/>')),
        ('empty-index', scene_bytes(()), xml('')),
    ]


def utf8(value: str) -> bytes:
    """Serialize parsed XML text without altering its namespace/whitespace distinctions."""
    raw = value.encode('utf-8')
    return word(len(raw)) + raw


def number_list(values: tuple[int, ...] | list[int]) -> bytes:
    """Serialize ordered candidate/path identities, including empty lists."""
    return word(len(values)) + b''.join(word(value) for value in values)


def attributes(values: dict[str, str]) -> bytes:
    """Compare maps in a total order independent of implementation hash iteration."""
    return word(len(values)) + b''.join(utf8(key) + utf8(value) for key, value in sorted(values.items()))


def oracle_bytes(value: dict[str, object]) -> bytes:
    """Retain every local-index result from the actual reference reader."""
    raw = attributes(value['root_attributes']) + word(len(value['guid_index']))
    for key, candidates in value['guid_index'].items():
        units = key.encode('utf-16-le', errors='surrogatepass')
        raw += word(len(units) // 2) + units + number_list(candidates)
    raw += number_list(value['unkeyed_geodes']) + number_list(value['unreferenced_geodes'])
    raw += word(len(value['references']))
    for reference in value['references']:
        raw += number_list(reference['element_path']) + utf8(reference['tag']) + attributes(reference['attributes'])
        raw += utf8(reference['ident_geom']) + utf8(reference['status']) + number_list(reference['geode_entry_indices'])
    return raw


def verify_pair(scene_wire: bytes, xml: bytes) -> tuple[dict[str, object], bytes]:
    """Require actual complete binary decoding before the XML equality projection."""
    import composerscene
    import composerxmllinks
    scene = composerscene.read_decoded(scene_wire, texture_import_enabled=True)
    assert scene['end'] == len(scene_wire)
    projection = composerxmllinks.read(xml, scene=scene)
    assert projection['xml_bytes'] == xml and not projection['runtime_resolution_reproduced']
    oracle = oracle_bytes(projection)
    packet = word(len(scene_wire)) + word(len(xml)) + word(len(oracle)) + scene_wire + xml + oracle
    facts = dict(scene_sha256=hashlib.sha256(scene_wire).hexdigest(), xml_sha256=hashlib.sha256(xml).hexdigest(),
                 xml_bytes=len(xml), geodes=sum(len(group) for group in projection['guid_index'].values()) + len(projection['unkeyed_geodes']),
                 references=len(projection['references']), statuses={status: sum(ref['status'] == status for ref in projection['references']) for status in ('unique', 'unmatched', 'empty', 'ambiguous')},
                 oracle_sha256=hashlib.sha256(oracle).hexdigest())
    return facts, packet


def freeze_owned() -> None:
    """Retain full inputs and every field of the independently decoded expected index."""
    facts, packets = [], []
    for name, scene, xml in owned_cases():
        record, packet = verify_pair(scene, xml)
        facts.append(dict(name=name, **record))
        packets.append(packet)
    retain(ROOT / 'tests/fixtures/composer-links/links.bin', b'XLNK' + word(len(packets)) + b''.join(packets))
    report('owned-reference.json', facts)


def native_inputs(row: dict[str, object]) -> tuple[bytes, bytes]:
    """Bind both project members to the original reviewed SMG carrier."""
    import composer
    import composergeometry
    raw = Path(row['path']).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == row['sha256']
    document = composer.Document(raw)
    geometry = next(entry for entry in document.entries if entry.name.endswith('.smgGeom'))
    xml = next(entry for entry in document.entries if entry.name.endswith('.smgXml'))
    return composergeometry.decode(document.read(geometry))['decoded'], document.read(xml)


def freeze_native() -> None:
    """Retain all 38 complete local indexes, with exact native totals and every expected result."""
    catalog = json.loads((SOURCE / 'research/composer-project-corpus.json').read_bytes())
    facts, packets = [], []
    for row in catalog['files']:
        scene, xml = native_inputs(row)
        record, packet = verify_pair(scene, xml)
        facts.append(dict(path=row['path'], archive_sha256=row['sha256'], **record))
        if '--export' in sys.argv:
            packets.append(packet)
    assert (len(facts), sum(row['xml_bytes'] for row in facts), sum(row['geodes'] for row in facts),
            sum(row['references'] for row in facts)) == (38, 21529648, 1787, 4445)
    assert sum(row['statuses']['unique'] for row in facts) == 4416
    assert sum(row['statuses']['unmatched'] for row in facts) == 29
    report('native-reference.json', facts)
    if packets:
        export_native(packets)


def export_native(packets: list[bytes]) -> None:
    """Keep large native byte expectations local while retaining their exact identity."""
    raw = b'XLNK' + word(len(packets)) + b''.join(packets)
    (ROOT / 'target/composer-links-native.bin').write_bytes(raw)
    report('native-probe-reference.json', dict(bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest()))


def pin_schema() -> None:
    """Bind the exact local-equality contract without requiring unrelated document prose to freeze."""
    path = SOURCE.parent / 'docs/solidworks-file-format/specification.md'
    text = path.read_text(encoding='utf-8')
    start = text.index('### 14.91 ')
    end = text.index('\n### ', start)
    raw = (text[start:end].rstrip() + '\n').encode()
    retain(FEATURE / 'schema-snapshot.md', raw)
    original = hashlib.sha256(path.read_bytes()).hexdigest()
    if '--write' not in sys.argv:
        original = json.loads((FEATURE / 'schema-reference.json').read_bytes())['source_sha256']
    report('schema-reference.json', dict(clause='14.91', requirement='C-467', issue=104,
           source_sha256=original, snapshot_sha256=hashlib.sha256(raw).hexdigest()))


def main() -> None:
    """Pin complete actual readers before loading binary-scene or XML helpers."""
    with tempfile.TemporaryDirectory(prefix='bessemer-composer-links-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in (freeze_owned, freeze_native, pin_schema))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        assert result.wasSuccessful() and result.testsRun == 3 and not result.skipped
        record_snapshot(pin, hashes, result, FEATURE, ('composer', 'composerscene', 'composerxmllinks'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'owned-source-snapshot.json')


if __name__ == '__main__':
    main()
