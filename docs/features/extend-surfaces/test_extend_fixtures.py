"""Regenerate complete surface-extension fixtures from immutable readers (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_extend_surfaces import REFERENCE,WORKSPACE,freeze,fixture_family


class ExtendSurfaceFixtureTests(unittest.TestCase):
    def test_all_surface_data_keeper_and_owned_goldens_regenerate(self) -> None:
        """Require complete primitive partitions, map identities and owned bytes."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/extend-surfaces'/fixture.name).read_bytes())
