"""Verify independent axis fixture repeatability and raw coordinate construction."""
from pathlib import Path
from tempfile import TemporaryDirectory
import struct
import unittest
from freeze_fixtures import BASE, CASES, WORKSPACE, freeze


class AxisFixtureTests(unittest.TestCase):
    def test_all_owned_objects_and_oracles_regenerate_identically(self) -> None:
        with TemporaryDirectory() as temporary:
            output = Path(temporary)
            report = freeze(WORKSPACE.parent/'inventor-interface/solidworks', output)
            self.assertEqual(len(report['cases']), len(CASES))
            for generated in output.iterdir():
                self.assertEqual(generated.read_bytes(),
                                 (WORKSPACE/'tests/fixtures/reference-axes'/generated.name).read_bytes())

    def test_axis_base_has_exact_size_signed_zero_and_nan_payload(self) -> None:
        self.assertEqual(len(BASE), 112)
        self.assertEqual(BASE[48:64], struct.pack('<QQ', 0x8000000000000000, 0x7ff8000000000042))


if __name__ == '__main__':
    unittest.main()
