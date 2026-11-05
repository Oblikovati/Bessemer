"""Complete original-reader legacy payload and map-identity oracle (#206)."""
from pathlib import Path
import hashlib
import json
import os
import struct
import unittest
from legacy_reference_fixtures import TIMES, class_header, component_record, legacy_reference_cases

ROOT = Path(__file__).resolve().parents[3]


class LegacyReferenceTests(unittest.TestCase):
    def test_complete_records_and_owned_streams_preserve_raw_presence_time_and_aliases(self) -> None:
        """Check270 bare cases and27 complete owned streams without a wrapper map slot."""
        rows = legacy_reference_cases()
        self.assertEqual(len(rows), 297)
        packet = bytearray(b'BLR1' + struct.pack('<I', len(rows)))
        for context, version, presence, kind, edited, raw in rows:
            value = self.parse_owned(raw, presence, kind) if context else self.parse_record(raw, version, presence, kind)
            self.check_values(value, presence, kind, edited)
            packet += struct.pack('<IIiIII', context, version, presence, kind, int(edited), len(raw)) + raw
        destination = Path(os.environ['BESSEMER_LEGACY_REFERENCE_FIXTURES'])
        destination.mkdir(parents=True, exist_ok=True)
        (destination / 'cases.bin').write_bytes(packet)
        manifest = dict(cases=len(rows), bare=270, owned=27, bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest())
        (destination / 'manifest.json').write_bytes((json.dumps(manifest, indent=2) + '\n').encode())

    def parse_record(self, raw: bytes, version: int, presence: int, kind: int, context: str = 'ordinary_external_xml') -> dict[str, object]:
        """Use unchanged original readers and shared identities; e.g. parse_record(raw,19000,-1,2)."""
        import modelcomponents, modelnode
        from mfcobjects import ObjectArchive
        archive = ObjectArchive(raw, version)
        archive.feature_context = context
        value = archive.object()
        if presence and kind:
            self.assertIs(archive.object(), value['legacy_reference']['object'])
        slots = 5 if presence and kind == 2 else 3
        self.assertEqual((archive.p, len(archive.map)), (len(raw), slots))
        return value

    def parse_owned(self, raw: bytes, presence: int, kind: int) -> dict[str, object]:
        """Require complete original configuration and feature EOF; e.g. parse_owned(raw,-1,2)."""
        import swcontainer, modelprefix, resolvedfeatures
        import freeze_trim_edges
        from sheet_owned_fixtures import SavedSheetMetalDocument
        document = swcontainer.Document((ROOT / 'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
        manager = (ROOT / 'tests/fixtures/sheet-metal/manager-0.bin').read_bytes()
        supplied = SavedSheetMetalDocument(document, manager, raw)
        _, context = modelprefix.read(supplied, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
        self.assertEqual((context.p, len(context.map)), (len(context.b), 97))
        parsed = resolvedfeatures.read_supported(supplied, 'Contents/Config-0-ResolvedFeatures')
        slots = 101 if presence and kind == 2 else 99
        self.assertEqual((parsed['end'], parsed['tail'], len(parsed['archive'].map)), (len(raw), b'', slots))
        if presence and kind:
            self.assertIs(parsed['items'][1], parsed['items'][0]['legacy_reference']['object'])
        return parsed['items'][0]

    def check_values(self, value: dict[str, object], presence: int, kind: int, edited: bool) -> None:
        """Keep raw presence and time bits without native cleanup; e.g. check_values(value,-1,2,False)."""
        self.assertEqual((value['unknown_84'], value['has_legacy_reference']), (presence, presence))
        self.assertEqual(value['unknown_a0'], -19)
        if not presence:
            self.assertNotIn('legacy_reference', value)
            return
        legacy = value['legacy_reference']
        self.assertEqual(legacy['time_word'], TIMES[kind] ^ int(edited))
        if kind == 0:
            self.assertIsNone(legacy['object'])
        elif kind == 1:
            self.assertIs(legacy['object'], value)
        else:
            self.assertEqual(legacy['object'], dict(_class='moNodeName_c', name='legacy Ω'))

    def test_all_record_prefixes_invalid_pointer_and_wrong_context_fail(self) -> None:
        """Check variable fresh-object extent and the ordinary context boundary."""
        from swarchive import ArchiveError
        for context, version, presence, kind, _, raw in legacy_reference_cases():
            if context:
                continue
            self.assert_record_prefixes(raw, version, presence, kind)
        raw = bytearray(component_record(19000, 1, 1, False, False))
        struct.pack_into('<H', raw, len(class_header('moCompRef_c', 7)) + 30, 0x7ffe)
        with self.assertRaises(ArchiveError):
            self.parse_record(bytes(raw), 19000, 1, 1)
        with self.assertRaises(ArchiveError):
            self.parse_record(component_record(19000, 1, 0, False, False), 19000, 1, 0, 'simulation_embedded')

    def assert_record_prefixes(self, raw: bytes, version: int, presence: int, kind: int) -> None:
        """Require every byte of the declared record/alias sequence; e.g. assert_record_prefixes(raw,19000,-1,2)."""
        from swarchive import ArchiveError
        for end in range(len(raw)):
            with self.assertRaises((ArchiveError, ValueError, AssertionError)):
                self.parse_record(raw[:end], version, presence, kind)
