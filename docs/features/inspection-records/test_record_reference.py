"""Independent ordered-record oracles for #193; run through the frozen Source runner."""
from pathlib import Path
import unittest
import inspectionrecords as records

ROOT = Path(__file__).resolve().parents[3]


class InspectionRecordReferenceTests(unittest.TestCase):
    def test_repeated_exact_containers_and_native_attribute_spelling(self) -> None:
        """Retain arbitrary direct element tags while rejecting container/name decoys."""
        raw = (ROOT / 'tests/fixtures/inspection-records/load-projections.xml').read_bytes()
        records_view = records.read(raw)['sheets'][0]['groups'][0]['characteristics']
        loaded = records_view[0]['loaded_measurements']
        self.assertEqual([(m['Name'], m['Value']) for m in loaded], [('N', 'Pass'), ('', ''), ('W', ''), ('last', '-0.00')])
        self.assertEqual(loaded[0], dict(Name='N', Value='Pass', PartName='P', Type='custom', CreationTime='arbitrary'))
        properties = records_view[0]['loaded_custom_properties']
        self.assertEqual([(p['Name'], p['Value']) for p in properties], [('P', '1'), ('P', '2'), ('', ''), ('W', '')])
        self.assertEqual(records_view[1]['loaded_measurements'], [])
        self.assertEqual(records_view[1]['loaded_custom_properties'], [])

    def test_order_scope_and_missing_values(self) -> None:
        """Keep literal names, direct-child scope, duplicates and absence distinct."""
        raw = (ROOT / 'tests/fixtures/inspection-records/ordered.xml').read_bytes()
        view = records.read(raw)
        self.assertIs(view['xml_bytes'], raw)
        self.assertEqual([s['attributes']['Name'] for s in view['sheets']], ['first', 'second'])
        sheet = view['sheets'][0]
        self.assertEqual([d['attributes']['Uid'] for d in sheet['documents']], ['D', 'D'])
        self.assertEqual([g['attributes']['Name'] for g in sheet['groups']], ['G', 'G'])
        first, second = sheet['groups'][0]['characteristics']
        self.assertEqual(first['attributes']['Nominal'], '3.503 / 3.496')
        self.assertEqual(first['attributes']['Extension'], '\u03bb')
        self.assertEqual(second['attributes']['Nominal'], '-0.00')
        self.assertEqual(first['measurements'], [{'Name': 'same', 'Value': ''}, {'Name': 'same'}])
        self.assertEqual(first['custom_properties'], [{'Name': 'P', 'Value': '1'}, {'Name': 'P', 'Value': ''}, {}])
        self.assertEqual(second['measurements'], [])

    def test_native_load_projection_is_separate(self) -> None:
        """Native element-child projections include unknown tags with empty defaults."""
        raw = (ROOT / 'tests/fixtures/inspection-records/ordered.xml').read_bytes()
        first = records.read(raw)['sheets'][0]['groups'][0]['characteristics'][0]
        expected = [('same', ''), ('same', 'Pass'), ('same', ''), ('qualified', 'extension'), ('', '')]
        self.assertEqual([(m['Name'], m['Value']) for m in first['loaded_measurements']], expected)
        self.assertTrue(all(m['Type'] == m['PartName'] == m['CreationTime'] == '' for m in first['loaded_measurements']))
        self.assertEqual(first['loaded_custom_properties'], [
            {'Name': 'P', 'Value': '1'}, {'Name': 'P', 'Value': '2'},
            {'Name': 'P', 'Value': ''}, {'Name': '', 'Value': ''},
            {'Name': 'Q', 'Value': 'extension'},
        ])

    def test_caller_supplied_source_identity(self) -> None:
        """Use the published abc MD5 vector without following the stored source path."""
        raw = (ROOT / 'tests/fixtures/inspection-records/ordered.xml').read_bytes()
        document = records.read(raw)['sheets'][0]['documents'][0]
        identity = records.compare_document_source(document, b'abc')
        self.assertEqual(identity['actual_md5'], '900150983cd24fb0d6963f7d28e17f72')
        self.assertTrue(identity['size_matches'])
        self.assertTrue(identity['md5_matches'])
        changed = records.compare_document_source(document, b'abd')
        self.assertTrue(changed['size_matches'])
        self.assertFalse(changed['md5_matches'])
        self.assertFalse(records.compare_document_source(document, b'ab')['size_matches'])

    def test_populated_tutorial_records(self) -> None:
        """Reuse the accepted decoded tutorial without introducing a cipher dependency."""
        raw = (ROOT / 'tests/fixtures/standalone-inspection/historical.xml').read_bytes()
        sheet = records.read(raw)['sheets'][0]
        self.assertEqual(len(sheet['documents']), 1)
        self.assertEqual(len(sheet['groups']), 12)
        characteristics = [c for g in sheet['groups'] for c in g['characteristics']]
        self.assertEqual(len(characteristics), 12)
        self.assertEqual(sum(len(c['measurements']) for c in characteristics), 12)
        self.assertEqual(sum(len(c['custom_properties']) for c in characteristics), 120)
        self.assertTrue(all(c['measurements'][0]['Value'] == '' for c in characteristics))
