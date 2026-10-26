"""Verify independent mirror fixture regeneration and tagged/direct graph guards."""
from pathlib import Path
from tempfile import TemporaryDirectory
import unittest
from freeze_fixtures import WORKSPACE, freeze, mirror_variants
from mirror_fixture_bytes import mirror_object
from owned_mirror_fixtures import owned_mirror_stream


class MirrorFixtureTests(unittest.TestCase):
    def test_complete_fixture_packet_owned_streams_and_oracles_regenerate(self) -> None:
        with TemporaryDirectory() as temporary:
            output = Path(temporary)
            result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', output)
            self.assertEqual((len(result['cases']), len(result['owned'])), (28, 4))
            self.assertEqual(sum(row['reads'] for row in result['cases']), 7724)
            for generated in output.iterdir():
                self.assertEqual(generated.read_bytes(),
                                 (WORKSPACE/'tests/fixtures/mirror-patterns'/generated.name).read_bytes())

    def test_shared_face_requires_a_present_tagged_plane(self) -> None:
        for version, tagged in [(0x3745, True), (19000, False)]:
            with self.assertRaisesRegex(ValueError, 'expected version'):
                mirror_object(version, False, tagged, 'shared')
        with self.assertRaisesRegex(ValueError, 'nonnegative slot'):
            mirror_object(19000, False, True, first_slot=-1)
        with self.assertRaisesRegex(ValueError, 'expected shared'):
            owned_mirror_stream(96, 'null', True)

    def test_gate_variants_and_owned_edit_keep_payloads_distinct(self) -> None:
        variants = [(case[0], mirror_object(*case)[0]) for case in mirror_variants()]
        self.assertEqual(len(set(variants)), len(variants))
        original, slots = owned_mirror_stream(96, 'shared')
        edited, edited_slots = owned_mirror_stream(96, 'shared', True)
        self.assertEqual((len(original), slots), (len(edited), edited_slots))
        self.assertNotEqual(original, edited)


if __name__ == '__main__':
    unittest.main()
