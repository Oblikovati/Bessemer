"""Compare positive inherited alias exports with unchanged original readers (#202)."""
from pathlib import Path
import unittest
from parameter_packet import ParameterPacket
from verify_parameter_exports import observe_parameters

ROOT = Path(__file__).resolve().parents[3]
RESULTS = ROOT / 'target/stored-parameter-actual-results'


class ActualInheritedParameterTests(unittest.TestCase):
    def test_both_inherited_alias_exports_match_original_source_and_fresh_processes(self) -> None:
        """Exercise the origin1 branch absent from all906 native cases."""
        import swcontainer
        from sheet_owned_fixtures import SavedSheetMetalDocument
        first = (RESULTS / 'inherited-1.bin').read_bytes()
        self.assertEqual(first, (RESULTS / 'inherited-2.bin').read_bytes())
        packet = ParameterPacket(first)
        document = swcontainer.Document((ROOT / 'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
        manager = (ROOT / 'tests/fixtures/sheet-metal/manager-0.bin').read_bytes()
        for edited, name in enumerate(('original.bin', 'edited.bin')):
            self.assertEqual([packet.word() for _ in range(3)], [6, 0, edited])
            raw = (ROOT / 'tests/fixtures/stored-parameters/inherited' / name).read_bytes()
            _, expected = observe_parameters(SavedSheetMetalDocument(document, manager, raw), 'Contents/Config-0-ResolvedFeatures')
            self.assertEqual(packet.parameters(), expected)
            self.assertEqual([name['origin'] for name in expected[0]['names']], [1, 1])
        self.assertEqual(packet.position, len(packet.raw))
