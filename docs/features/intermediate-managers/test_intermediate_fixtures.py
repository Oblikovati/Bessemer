"""Reproduce exact-version object gates, caller-bound suffixes and complete managers (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_intermediate_managers import REFERENCE,WORKSPACE,freeze


class IntermediateFixtureTests(unittest.TestCase):
    def test_direct_records_suffixes_and_complete_managers_regenerate(self) -> None:
        """Keep all primitive partitions, slots and independent edit bytes unchanged."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination)
            for key in ('cases','native','tails','dependency_stops','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/intermediate-managers'/fixture.name).read_bytes())
