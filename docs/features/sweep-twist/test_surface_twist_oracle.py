"""Original enclosing surface readers plus the declared fixture-only twist reader (#211)."""
from pathlib import Path
from typing import cast
import struct
import unittest
import twist_wire_fixtures as fixture
from owned_twist_oracle import OwnedTwistArchive, owned_twist_context, read_owned_twist
from owned_twist_cases import write_owned_twists
from surface_twist_cases import surface_twist_cases
from swarchive import ArchiveError


class SurfaceTwistOracleTests(unittest.TestCase):
    def test_complete_surface_twists_match_literal_fields_slots_and_source_aliases(self) -> None:
        reference = cast(Path, fixture.REFERENCE)
        document, context = owned_twist_context(reference)
        records = surface_twist_cases(len(context.map)-1)
        self.assertEqual(len(records), 40)
        for row in records:
            result = read_owned_twist(reference, document, context, row.raw, True)
            archive = cast(OwnedTwistArchive, result['archive'])
            self.assertEqual(len(archive.map), row.slots)
            for slot, payload in zip(row.object_slots, row.payloads):
                twist = cast(dict[str, object], archive.map[slot])
                self.assertEqual(struct.pack('<Bdd', twist['raw_08'], twist['raw_10'], twist['raw_18']), payload)
            self.assert_surface_aliases(result, row.label)
            with self.assertRaisesRegex(ArchiveError, fixture.CLASS):
                read_owned_twist(reference, document, context, row.raw, False)
        write_owned_twists(Path(__file__).resolve().parents[3] / 'target/sweep-twist-surface-fixtures', records)

    def assert_surface_aliases(self, result: dict[str, object], label: str) -> None:
        if not label.startswith(('surface-2-', 'surface-5-')):
            return
        root = cast(list[dict[str, object]], result['items'])[0]
        surfaces = cast(list[dict[str, object]], root['items'])
        self.assertIs(surfaces[0], surfaces[2])
        self.assertIsNot(surfaces[0], surfaces[1])
        twists = [cast(dict[str, object], cast(dict[str, object], surface['payload'])['data'])['twist_298']
                  for surface in surfaces[:2]]
        if '-shared-' in label:
            self.assertIs(twists[0], twists[1])
        else:
            self.assertIsNot(twists[0], twists[1])
