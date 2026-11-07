"""Regenerate complete stock boundaries and independent scalar edits (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_stock import REFERENCE,WORKSPACE,freeze,fixture_family
from freeze_body_wrappers import fixture_family as wrapper_family


class StockFixtureTests(unittest.TestCase):
    def test_new_wrapper_caller_graphs_preserve_the_exact_expanded_inventory(self) -> None:
        """Regenerate complete callers for the twenty-first wrapper class."""
        expected=json.loads(Path(__file__).with_name('wrapper-reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,wrapper_family(REFERENCE))
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/body-reference-wrappers'/fixture.name).read_bytes())

    def test_complete_graphs_and_owned_edits_regenerate(self) -> None:
        """Compare every independent primitive and fixture byte; e.g. a suppressed body with attributes."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,fixture_family())
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/stock-features'/fixture.name).read_bytes())

    def test_distinct_kernel_controls_and_historical_collections_remain_covered(self) -> None:
        """Retain four types, nine versions and six styles including huge zero-count widths."""
        facts=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(facts['cases']),216)
        self.assertEqual(facts['reads'],51378)
        self.assertEqual(len(facts['owned']),28)
