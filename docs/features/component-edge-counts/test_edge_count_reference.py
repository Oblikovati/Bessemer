"""Complete signed-count and optional-pair checks through original readers (#204)."""
from pathlib import Path
import hashlib
import json
import os
import struct
import unittest
from edge_count_fixtures import FLAGS, COUNTS, edge_record, owned_edge_graph

ROOT = Path(__file__).resolve().parents[3]


class EdgeCountReferenceTests(unittest.TestCase):
    def test_complete_edge_records_and_owned_graphs_preserve_signed_counts_and_pair_order(self) -> None:
        """Validate40 records and40 complete owned configurations/features before retaining goldens."""
        packet = bytearray(b'BCE1' + struct.pack('<I', 40))
        for kind in range(2):
            for flags in FLAGS:
                for count in COUNTS:
                    raw = edge_record(kind, flags, count, 2)
                    archive, value = self.parse_record(raw)
                    self.assertEqual(archive.p, len(raw))
                    self.check_values(value, flags, count)
                    owned = owned_edge_graph(kind, flags, count)
                    self.check_owned(owned, flags, count)
                    packet += struct.pack('<IiiI', kind, flags, count, len(owned)) + owned
        self.retain_packet(bytes(packet))

    def parse_record(self, raw: bytes) -> tuple[object, dict[str, object]]:
        """Call unchanged original object/body readers; e.g. parse_record(raw)."""
        import modelcomponents
        from mfcobjects import ObjectArchive
        archive = ObjectArchive(raw, 19000)
        archive.feature_context = 'ordinary_external_xml'
        return archive, archive.object()

    def check_values(self, value: dict[str, object], flags: int, count: int) -> None:
        """Distinguish stored counts from executed pointer loops; e.g. check_values(value,8,-1)."""
        self.assertEqual(value['reference_count'], count)
        self.assertEqual(value['unknown_188'], flags)
        self.assertEqual(value['unknown_18c'], 0xfedcba98)
        self.assertEqual(len(value['references']), max(count, 0))
        if count == 2:
            self.assertIs(value['references'][1], value)
        has_pair = flags & 6 != 6 and bool(flags & 8)
        self.assertEqual('composite_references' in value, has_pair)
        if has_pair:
            self.assertIs(value['composite_references'][0], value)
            self.assertIsNone(value['composite_references'][1])

    def check_owned(self, raw: bytes, flags: int, count: int) -> None:
        """Use full unchanged model/configuration readers, including inherited maps; e.g. check_owned(raw,8,-1)."""
        import swcontainer, modelprefix, resolvedfeatures
        import freeze_trim_edges
        from sheet_owned_fixtures import SavedSheetMetalDocument
        document = swcontainer.Document((ROOT / 'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
        manager = (ROOT / 'tests/fixtures/sheet-metal/manager-0.bin').read_bytes()
        supplied = SavedSheetMetalDocument(document, manager, raw)
        _, context = modelprefix.read(supplied, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
        self.assertEqual((context.p, len(context.map)), (len(context.b), 97))
        parsed = resolvedfeatures.read_supported(supplied, 'Contents/Config-0-ResolvedFeatures')
        self.assertEqual((parsed['end'], parsed['tail'], parsed['list_complete']), (len(raw), b'', True))
        self.check_values(parsed['items'][0], flags, count)

    def test_every_record_prefix_and_impossible_positive_count_fails(self) -> None:
        """Require full bodies, ordered optional pairs and positive bounds."""
        from swarchive import ArchiveError
        for kind in range(2):
            for flags in FLAGS:
                for count in COUNTS:
                    raw = edge_record(kind, flags, count, 2)
                    for end in range(len(raw)):
                        with self.assertRaises((ArchiveError, ValueError)):
                            self.parse_record(raw[:end])
            with self.assertRaises(ArchiveError):
                self.parse_record(edge_record(kind, 0, 2147483647, 2))

    def retain_packet(self, raw: bytes) -> None:
        """Write only to the explicit generated oracle directory; e.g. retain_packet(raw)."""
        destination = Path(os.environ['BESSEMER_EDGE_COUNT_FIXTURES'])
        destination.mkdir(parents=True, exist_ok=True)
        (destination / 'cases.bin').write_bytes(raw)
        manifest = dict(cases=40, bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())
        (destination / 'manifest.json').write_bytes((json.dumps(manifest, indent=2) + '\n').encode())
