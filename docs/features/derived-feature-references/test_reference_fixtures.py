"""Regenerate complete reference and generic-caller observations (#54)."""
from pathlib import Path
import json
import tempfile
import unittest
from freeze_references import REFERENCE,WORKSPACE,freeze,fixture_family,wrapper_family
from freeze_trim_edges import ObjectFixtureFamily


class ReferenceFixtureTests(unittest.TestCase):
    def check_family(self, report: str, fixture_folder: str, family: ObjectFixtureFamily) -> None:
        """Compare each complete primitive partition and owned byte; e.g. check_family('reference.json',folder,family)."""
        expected=json.loads(Path(__file__).with_name(report).read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination,family)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures'/fixture_folder/fixture.name).read_bytes())

    def test_complete_reference_and_surface_body_graphs_regenerate(self) -> None:
        """Keep signed presence, full inherited bodies and shared identity; e.g. a present body."""
        self.check_family('reference.json','derived-feature-references',fixture_family())

    def test_each_generic_caller_regenerates_for_both_new_types(self) -> None:
        """Keep all prior callers unchanged while extending their exact family."""
        self.check_family('wrapper-reference.json','derived-reference-wrappers',wrapper_family(REFERENCE))
