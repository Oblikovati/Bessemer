"""Keep all native-layout fixtures bound to immutable independent readers (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_move_copy import REFERENCE,WORKSPACE,freeze,fixture_family


class MoveCopyFixtureTests(unittest.TestCase):
    def test_all_graphs_observations_and_owned_edit_goldens_regenerate(self) -> None:
        """Compare complete independent results, not implementation-derived expectations."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/move-copy-bodies'/fixture.name).read_bytes())
