"""Reproduce every independent deform, geometry, matching and caller observation (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_deformations import REFERENCE,WORKSPACE,freeze_dataset


class DeformationFixtureTests(unittest.TestCase):
    def test_all_complete_graphs_and_owned_goldens_regenerate(self) -> None:
        """Compare every primitive partition, identity count and owned byte."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze_dataset(REFERENCE,destination)
            for key in ('cases','owned','reads','packet_sha256','geometry','matching'):self.assertEqual(actual[key],expected[key])
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual['callers'][key],expected['callers'][key])
            for fixture in destination.rglob('*'):
                if fixture.is_file():self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/deformations'/fixture.relative_to(destination)).read_bytes())
