"""Regenerate exact detail bodies and both proven current-part branches (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_detail_references import REFERENCE,WORKSPACE,freeze,fixture_family
from owned_detail_references import freeze_current_units


class ComponentDetailFixtureTests(unittest.TestCase):
    def test_complete_bare_and_null_owned_graphs_regenerate(self) -> None:
        """Compare every byte and primitive partition against actual reference readers; e.g. shared details."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/component-detail-references'/fixture.name).read_bytes())

    def test_complete_fresh_and_cached_current_unit_contexts_regenerate(self) -> None:
        """Resolve actual source root/unit indices before each owned graph; e.g. current-2-5.bin."""
        expected=json.loads(Path(__file__).with_name('current-unit-reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze_current_units(REFERENCE,destination)
            self.assertEqual(actual['owned'],expected['owned'])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/component-detail-references'/fixture.name).read_bytes())

    def test_owned_current_unit_inventory_keeps_both_exact_nonnull_branches(self) -> None:
        """Keep six styles and one independent edit for each branch; e.g. fresh versus inherited unit identity."""
        facts=json.loads(Path(__file__).with_name('current-unit-reference.json').read_text())
        self.assertEqual(len(facts['owned']),14)
        self.assertEqual({row['mode'] for row in facts['owned']},{1,2})
        self.assertTrue(all(row['root']>0 and row['unit']>row['root'] for row in facts['owned']))


if __name__=='__main__':unittest.main()
