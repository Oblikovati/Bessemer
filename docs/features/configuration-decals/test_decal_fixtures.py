"""Reproduce complete decal properties, direct maps and owned manager goldens (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_decals import REFERENCE,WORKSPACE,freeze


class DecalFixtureTests(unittest.TestCase):
    def test_complete_decal_graphs_maps_and_managers_regenerate(self) -> None:
        """Require identical primitive partitions, object slots and owned edit bytes."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination)
            for key in ('cases','maps','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/configuration-decals'/fixture.name).read_bytes())
