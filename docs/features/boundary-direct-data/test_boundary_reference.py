"""Original-reader complete boundary/direct-body oracle (#205)."""
from pathlib import Path
import hashlib
import json
import os
import struct
import unittest
from boundary_data_fixtures import SCENARIOS, boundary_cases, boundary_record

ROOT = Path(__file__).resolve().parents[3]


class BoundaryReferenceTests(unittest.TestCase):
    def test_complete_objects_and_owned_graphs_match_literal_direct_values(self) -> None:
        """Check78 bare gate cases and17 complete owned streams before retaining goldens."""
        rows = boundary_cases()
        self.assertEqual(len(rows), 95)
        packet = bytearray(b'BBD1' + struct.pack('<I', len(rows)))
        for context, version, style, edited, raw in rows:
            value = self.parse_owned(raw) if context else self.parse_record(raw, version)
            self.check_values(value, version, style, edited)
            packet += struct.pack('<5I', context, version, style, int(edited), len(raw)) + raw
        destination = Path(os.environ['BESSEMER_BOUNDARY_FIXTURES'])
        destination.mkdir(parents=True, exist_ok=True)
        (destination / 'cases.bin').write_bytes(packet)
        manifest = dict(cases=len(rows), bare=78, owned=17, bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest())
        (destination / 'manifest.json').write_bytes((json.dumps(manifest, indent=2) + '\n').encode())

    def parse_record(self, raw: bytes, version: int) -> dict[str, object]:
        """Require original full EOF and exactly one class/object pair; e.g. parse_record(raw,19000)."""
        import modelboundary
        from mfcobjects import ObjectArchive
        archive = ObjectArchive(raw, version)
        archive.feature_context = 'ordinary_external_xml'
        value = archive.object()
        self.assertEqual((archive.p, len(archive.map)), (len(raw), 3))
        return value

    def parse_owned(self, raw: bytes) -> dict[str, object]:
        """Use complete original inherited configuration and feature readers; e.g. parse_owned(raw)."""
        import swcontainer, modelprefix, resolvedfeatures
        import freeze_trim_edges
        from sheet_owned_fixtures import SavedSheetMetalDocument
        document = swcontainer.Document((ROOT / 'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
        manager = (ROOT / 'tests/fixtures/sheet-metal/manager-0.bin').read_bytes()
        supplied = SavedSheetMetalDocument(document, manager, raw)
        _, context = modelprefix.read(supplied, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
        self.assertEqual((context.p, len(context.map)), (len(context.b), 97))
        parsed = resolvedfeatures.read_supported(supplied, 'Contents/Config-0-ResolvedFeatures')
        self.assertEqual((parsed['end'], parsed['tail'], len(parsed['archive'].map)), (len(raw), b'', 99))
        return parsed['items'][0]

    def check_values(self, value: dict[str, object], version: int, style: int, edited: bool) -> None:
        """Preserve wire values instead of native duplicate/zero/sentinel rewrites; e.g. check_values(value,19000,4,False)."""
        temporary, presence, _, _ = SCENARIOS[style]
        self.assertEqual(value['temporary_count'], temporary)
        self.assertEqual(len(value['temporary_records']), max(temporary, 0))
        if temporary > 0:
            first = 0x11223344 if edited and not presence else 0xffffffff
            self.assertEqual(value['temporary_records'][0], dict(time_0=first, time_8=0x80000000))
        if version <= 0x332c:
            self.assertNotIn('has_direct_model_data', value)
            return
        self.assertEqual(value['has_direct_model_data'], presence)
        self.assertEqual(value['unknown_ec'], -1)
        if presence:
            self.check_downstream(value['direct_model_data'], version, style, edited)

    def check_downstream(self, value: dict[str, object], version: int, style: int, edited: bool) -> None:
        """Check signed counts, duplicate pairs and exact tail gate; e.g. check_downstream(value,19000,4,False)."""
        _, _, pairs, values = SCENARIOS[style]
        self.assertEqual((value['pair_count'], value['value_count']), (pairs, values))
        expected = [dict(key=-7, value=-2147483648), dict(key=-7, value=2147483647)] if pairs > 0 else []
        self.assertEqual(value['pairs'], expected)
        self.assertEqual(value['values'], [0, -2147483648, 0] if values > 0 else [])
        self.assertEqual(value['unknown_38'], 42 if edited else -17)
        self.assertEqual(value.get('values_40'), [-2147483648, 2147483647] if version >= 0x3359 else None)

    def test_every_bare_prefix_and_impossible_temporary_count_fails(self) -> None:
        """Reject partial direct values and impossible positive framing."""
        from swarchive import ArchiveError
        for context, version, _, _, raw in boundary_cases():
            if context:
                continue
            for end in range(len(raw)):
                with self.assertRaises((ArchiveError, ValueError)):
                    self.parse_record(raw[:end], version)
        raw = bytearray(boundary_record(19000, 4))
        offset = 6 + len(b'moBEntRef_c') + 42
        struct.pack_into('<i', raw, offset, 2147483647)
        with self.assertRaises(ArchiveError):
            self.parse_record(bytes(raw), 19000)
