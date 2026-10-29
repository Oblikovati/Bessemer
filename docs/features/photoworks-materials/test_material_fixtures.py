"""Reproduce all complete material, direct-manager and owned-manager bytes (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_materials import REFERENCE,WORKSPACE,freeze


class PhotoWorksFixtureTests(unittest.TestCase):
    def test_complete_materials_and_managers_regenerate(self) -> None:
        """Keep every primitive partition, slot count and manager edit golden stable."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination)
            for key in ('cases','managers','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/photoworks-materials'/fixture.name).read_bytes())
