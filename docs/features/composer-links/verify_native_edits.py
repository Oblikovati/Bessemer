"""Check complete Rust reindexes and isolated lexical edits using frozen actual readers (#104)."""
from pathlib import Path
import hashlib
import json
import re
import struct
import sys
import tempfile
import unittest
from xml.parsers import expat
import freeze_links as baseline


def first_ident_geom(xml: bytes) -> tuple[int, int]:
    """Use independent Expat element offsets, then bound the actual unqualified attribute value."""
    parser = expat.ParserCreate()
    spans = []
    def start(name: str, attributes: dict[str, str]) -> None:
        if not spans and 'IdentGeom' in attributes:
            spans.append(attribute_span(xml, parser.CurrentByteIndex))
    parser.StartElementHandler = start
    parser.Parse(xml, True)
    assert spans, 'expected a native IdentGeom occurrence'
    return spans[0]


def attribute_span(raw: bytes, start: int) -> tuple[int, int]:
    """Do not mistake quoted greater-than signs or comments for tag boundaries."""
    quote = None
    end = start + 1
    while end < len(raw):
        value = raw[end]
        if quote == value:
            quote = None
        elif quote is None and value in (34, 39):
            quote = value
        elif quote is None and value == 62:
            break
        end += 1
    for match in re.finditer(rb'([^\s=/>]+)\s*=\s*([\x22\x27])(.*?)\2', raw[start:end], re.DOTALL):
        if match.group(1) == b'IdentGeom':
            return start + match.start(3), start + match.end(3)
    raise AssertionError('expected complete unqualified IdentGeom value')


def verify_pair(scene: bytes, original: bytes, changed: bytes, expected_oracle: bytes) -> dict[str, object]:
    """Require a single exact byte replacement and every field of the actual complete reindex."""
    import composerscene
    import composerxmllinks
    begin, end = first_ident_geom(original)
    replacement = 'Bessemer unmatched reference Ω'.encode()
    assert changed == original[:begin] + replacement + original[end:]
    decoded = composerscene.read_decoded(scene, texture_import_enabled=True)
    assert decoded['end'] == len(scene)
    projection = composerxmllinks.read(changed, scene=decoded)
    assert baseline.oracle_bytes(projection) == expected_oracle
    return dict(original_xml_sha256=hashlib.sha256(original).hexdigest(),
                edited_xml_sha256=hashlib.sha256(changed).hexdigest(),
                edited_oracle_sha256=hashlib.sha256(expected_oracle).hexdigest())


def verify_bundle() -> None:
    """Bind every original scene/XML pair to its retained input identities."""
    raw = (baseline.ROOT / 'target/composer-links-native-edits.bin').read_bytes()
    records = json.loads((baseline.FEATURE / 'native-reference.json').read_bytes())
    assert raw[:4] == b'LXED' and struct.unpack_from('<I', raw, 4)[0] == len(records) == 38
    cursor, results = 8, []
    for record in records:
        sizes = struct.unpack_from('<4I', raw, cursor)
        cursor += 16
        parts = []
        for size in sizes:
            parts.append(raw[cursor:cursor + size])
            cursor += size
        assert hashlib.sha256(parts[0]).hexdigest() == record['scene_sha256']
        assert hashlib.sha256(parts[1]).hexdigest() == record['xml_sha256']
        results.append(verify_pair(*parts))
    assert cursor == len(raw)
    baseline.report('native-edit-reference.json', dict(files=38, bundle_sha256=hashlib.sha256(raw).hexdigest(), records=results))


def main() -> None:
    """Pin every reference module before reading any actual scene/index helper."""
    with tempfile.TemporaryDirectory(prefix='bessemer-links-edits-') as temporary:
        pin = Path(temporary)
        hashes = baseline.copy_sources(pin)
        sys.meta_path.insert(0, baseline.FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(verify_bundle))
        assert result.wasSuccessful() and result.testsRun == 1 and not result.skipped
        baseline.record_snapshot(pin, hashes, result, baseline.FEATURE, ('composerscene', 'composerxmllinks'), ())
        (baseline.FEATURE / 'fixture-reference-snapshot.json').replace(baseline.FEATURE / 'native-edit-source-snapshot.json')


if __name__ == '__main__':
    main()
