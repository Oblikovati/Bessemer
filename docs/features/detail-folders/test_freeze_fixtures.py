"""Regression checks for the independent detail-folder fixture builder."""
from pathlib import Path
import tempfile
import unittest
import freeze_fixtures as fixture
from primitive_inventory import PrimitiveRead


class DetailFixtureTests(unittest.TestCase):
    def test_numeric_arrays_preserve_every_bit_and_leave_other_raw_fields_alone(self) -> None:
        read = PrimitiveRead(8, 24, 'raw', '0000000000000080000000000000f07f', 'dl_ui.py', 'read_gcBucketPoly_c', 373)
        parts = fixture.numeric_fragments([read])
        self.assertEqual([(p.start,p.end,p.kind) for p in parts], [(8,16,'d'),(16,24,'d')])
        self.assertEqual(''.join(p.raw_hex for p in parts), read.raw_hex)
        text = PrimitiveRead(0,4,'raw','41424344','dl_ui.py','read_moLineViz_c',260)
        self.assertEqual(fixture.numeric_fragments([text]),[text])
        with self.assertRaisesRegex(ValueError,'expected complete f64'):
            fixture.numeric_fragments([PrimitiveRead(0,7,'raw','00'*7,'dl_ui.py','read_gcBucketPoly_c',373)])

    def test_complete_generation_reproduces_owned_wire_fixtures(self) -> None:
        source = fixture.WORKSPACE.parent/'inventor-interface/solidworks'
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            report = fixture.freeze(source, output)
            self.assertEqual(len(report['cases']),7)
            for record in report['cases']:
                name = str(record['name'])
                expected = fixture.WORKSPACE/'tests/fixtures/detail-folders'/(name+'.bin')
                self.assertEqual((output/(name+'.bin')).read_bytes(),expected.read_bytes())

    def test_short_name_framing_rejects_a_long_fixture_name(self) -> None:
        with self.assertRaisesRegex(ValueError,'name length 255'):
            fixture.feature_body(fixture.ObjectBuilder(1),'A'*255)


if __name__ == '__main__':
    unittest.main()
