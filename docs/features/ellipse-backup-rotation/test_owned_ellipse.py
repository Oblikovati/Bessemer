"""Original Source readers validate complete independently assembled ellipse contexts (#192)."""
from pathlib import Path
from typing import Protocol, cast
import hashlib
import json
import struct
import sys
import unittest

from freeze_owned_ellipse import ROOT
sys.path.insert(0, str(ROOT/'docs/features/model-archives'))
from primitive_inventory import PrimitiveRead, observe_primitives, verify_primitive_coverage
import resolvedfeatures
import swarchive
import swcontainer


class ReferenceDocument(Protocol):
    def read(self, name: str) -> bytes:
        """Supply one exact logical stream; e.g. document.read('Contents/CMgr')."""
        ...

    def names(self) -> list[str]:
        """Expose the authentic version-marker names; e.g. document.names()."""
        ...


class ReferenceArchive(Protocol):
    p: int
    b: bytes
    map: list[object]


class SelectedEllipseDocument:
    def __init__(self, original: ReferenceDocument, stream: str, replacement: bytes) -> None:
        """Select only the owned feature bytes; e.g. SelectedEllipseDocument(doc, stream, raw)."""
        self.original = original
        self.stream = stream
        self.replacement = replacement

    def read(self, name: str) -> bytes:
        """Keep every unselected context stream authentic; e.g. selected.read('Contents/CMgr')."""
        return self.replacement if name == self.stream else self.original.read(name)

    def names(self) -> list[str]:
        """Retain original version discovery; e.g. selected.names()."""
        return list(self.original.names())


def owned_ellipse_contexts() -> dict[int, tuple[ReferenceDocument, str]]:
    """Retain actual supplied configuration contexts; e.g. owned_ellipse_contexts()[19000]."""
    rows = (ROOT/'tests/fixtures/bitmap-appearance/native-contexts.tsv').read_text().splitlines()
    contexts: dict[int, tuple[ReferenceDocument, str]] = {}
    for row in rows:
        columns = row.split('\t')
        path = Path(columns[1])
        assert hashlib.sha256(path.read_bytes()).hexdigest() == columns[4], path
        original = cast(ReferenceDocument, swcontainer.Document(path, table={}))
        contexts[int(columns[0])] = (original, columns[2] + '-ResolvedFeatures')
    blank = cast(ReferenceDocument, swcontainer.Document(ROOT/'tests/fixtures/blank-part.SLDPRT', table={}))
    contexts[19000] = (blank, 'Contents/Config-0-ResolvedFeatures')
    return contexts


def owned_ellipse_records() -> list[tuple[int, bytes, list[int]]]:
    """Require the entire EBR1 denominator and exact packet EOF; e.g. owned_ellipse_records()."""
    folder = ROOT/'tests/fixtures/ellipse-backup-rotation'
    packet = (folder/'roots.bin').read_bytes()
    assert packet[:8] == b'EBR1' + struct.pack('<I', 108)
    rows = [[int(value) for value in row.split('\t')] for row in (folder/'cases.tsv').read_text().splitlines()]
    cursor = 8
    records: list[tuple[int, bytes, list[int]]] = []
    for index, row in enumerate(rows):
        version, size = struct.unpack_from('<II', packet, cursor)
        assert row[:2] == [index, version]
        cursor += 8
        records.append((version, packet[cursor:cursor + size], row))
        cursor += size
    assert cursor == len(packet) and len(records) == 108
    return records


def read_owned_ellipse(original: ReferenceDocument, stream: str,
                       raw: bytes) -> tuple[dict[str, object], list[PrimitiveRead]]:
    """Observe actual primitive reads without replacing a class reader; e.g. read_owned_ellipse(doc, stream, raw)."""
    selected = SelectedEllipseDocument(original, stream, raw)
    with observe_primitives(swarchive.Archive, raw) as reads:
        parsed = cast(dict[str, object], resolvedfeatures.read_supported(selected, stream))
    verify_primitive_coverage(raw, reads)
    archive = cast(ReferenceArchive, parsed['archive'])
    assert archive.p == len(raw) and archive.b == raw
    return parsed, reads


def verify_owned_ellipse_graph(parsed: dict[str, object], row: list[int]) -> None:
    """Keep physical definitions, same-word arc and first-owner alias distinct; e.g. verify_owned_ellipse_graph(parsed, row)."""
    items = cast(list[dict[str, object]], parsed['items'])
    archive = cast(ReferenceArchive, parsed['archive'])
    assert len(items) == 4 and len(archive.map) == row[9]
    assert items[0] is items[2] and items[0] is not items[1]
    assert [item['_class'] for item in items] == ['moEllipseBackedUpData_c'] * 3 + ['moArcBackedUpData_c']
    assert [item['word_20'] for item in items] == [row[6], row[7], row[6], row[8]]
    definitions = [entry for entry in archive.map if isinstance(entry, dict) and entry.get('_class') == 'moEllipseBackedUpData_c']
    assert len(definitions) == 2 and definitions[0] is items[0] and definitions[1] is items[1]


def verify_owned_ellipse_scalar_reads(raw: bytes, row: list[int],
                                    reads: list[PrimitiveRead]) -> None:
    """Join all three scalar observations to independent literal byte spans; e.g. verify_owned_ellipse_scalar_reads(raw, row, reads)."""
    for index, (offset, expected) in enumerate(zip(row[3:6], row[6:9])):
        literal = struct.pack('<i', expected)
        assert raw[offset:offset + 4] == literal
        matches = [read for read in reads if read.start == offset and read.end == offset + 4]
        assert len(matches) == 1 and matches[0].raw_hex == literal.hex()
        caller = ('modelcomponents.py', 'unknown') if index < 2 else ('sketchprefix.py', 'arc_backup')
        assert (matches[0].source_file, matches[0].source_function) == caller


class OwnedEllipseSourceTests(unittest.TestCase):
    def test_complete_graphs_maps_values_and_primitive_partitions_match_literals(self) -> None:
        """Validate all 108 complete contexts with original readers; e.g. frozen_source.run(...)."""
        contexts = owned_ellipse_contexts()
        primitive_reads = 0
        for version, raw, row in owned_ellipse_records():
            with self.subTest(index=row[0], version=version):
                original, stream = contexts[version]
                parsed, reads = read_owned_ellipse(original, stream, raw)
                verify_owned_ellipse_graph(parsed, row)
                verify_owned_ellipse_scalar_reads(raw, row, reads)
                primitive_reads += len(reads)
        self.assertEqual(primitive_reads, 7569)
        print(json.dumps({'cases': 108, 'physical_ellipses': 216, 'aliases': 108,
                          'unrelated_arcs': 108, 'scalar_checks': 324, 'primitive_reads': primitive_reads}))


if __name__ == '__main__':
    unittest.main()
