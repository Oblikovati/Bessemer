"""Reproduce complete owned external-unit graphs with actual source readers (#54)."""
from pathlib import Path
import json,tempfile,unittest
from freeze_units import ROOT,REFERENCE,owned,ObjectFixtureFamily,unit_object
class UnitFixtureTests(unittest.TestCase):
    """Compare actual saved-context EOF, every primitive and every identity slot."""
    def test_immutable_owned_regeneration(self) -> None:
        """Rebuild all full-context records without reader substitutions."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())['owned']
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);family=ObjectFixtureFamily(build=unit_object)
            rows=[owned(REFERENCE,destination,0,style,False,f'owned-{style}',family) for style in range(6)]
            rows.append(owned(REFERENCE,destination,0,2,True,'owned-2-edited',family))
            self.assertEqual(rows,expected)
            for p in destination.iterdir():
                self.assertEqual(p.read_bytes(),(ROOT/'tests/fixtures/external-unit-components'/p.name).read_bytes())
