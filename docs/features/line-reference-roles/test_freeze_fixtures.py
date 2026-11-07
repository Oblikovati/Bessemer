"""Verify independent line-family payload generation and alias locations (#46)."""
from pathlib import Path
from tempfile import TemporaryDirectory
import unittest
from freeze_fixtures import ROLES, WORKSPACE, freeze, role_wire
from owned_revolution_fixtures import owned_stream, verify_owned_read


class LineRoleFixtureTests(unittest.TestCase):
    def test_all_payloads_and_oracles_regenerate_identically(self) -> None:
        with TemporaryDirectory() as temporary:
            output = Path(temporary)
            result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', output)
            self.assertEqual(len(result['cases']), 10)
            for generated in output.iterdir():
                self.assertEqual(generated.read_bytes(),
                                 (WORKSPACE/'tests/fixtures/line-reference-roles'/generated.name).read_bytes())

    def test_declared_pointer_positions_keep_the_same_inherited_identity(self) -> None:
        for kind, (width, offsets) in ROLES.items():
            raw = role_wire(kind)
            self.assertEqual(len(raw), width)
            self.assertTrue(all(raw[offset:offset+2] == b'\x01\x00' for offset in offsets))

    def test_unknown_axis_kind_and_incomplete_reference_read_reject(self) -> None:
        with self.assertRaisesRegex(ValueError, 'expected null, line or temporary'):
            owned_stream('unrelated')
        with self.assertRaisesRegex(ValueError, 'expected'):
            verify_owned_read({'end': 0, 'tail': b'extra'}, 'null', 369)


if __name__ == '__main__':
    unittest.main()
