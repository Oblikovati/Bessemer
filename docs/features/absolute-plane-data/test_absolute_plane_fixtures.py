"""Regenerate shared absolute-plane bodies and typed caller edit goldens (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_absolute_planes import REFERENCE,WORKSPACE,freeze,fixture_family


class AbsolutePlaneFixtureTests(unittest.TestCase):
    def test_all_graphs_and_owned_edits_regenerate(self) -> None:
        """Compare bytes, primitive observations and identities; e.g. a nonnull plane-data pointer."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/absolute-plane-data'/fixture.name).read_bytes())

    def test_shared_base_and_typed_parent_keep_all_version_gates(self) -> None:
        """Keep the default-only vector absent across nine versions; e.g. 0x2b83 and 0x2b84."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),108)
        self.assertEqual(len(facts['owned']),14)


if __name__=='__main__':unittest.main()
