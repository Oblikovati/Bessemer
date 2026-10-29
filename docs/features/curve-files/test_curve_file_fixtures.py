"""Regenerate complete stored-curve fixtures using immutable source readers (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_curve_files import REFERENCE,WORKSPACE,freeze,fixture_family


class CurveFileFixtureTests(unittest.TestCase):
    def test_all_curve_fields_points_and_owned_goldens_regenerate(self) -> None:
        """Require all primitive partitions, complete document bytes and map sizes."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/curve-files'/fixture.name).read_bytes())
