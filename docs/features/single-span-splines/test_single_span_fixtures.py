"""Check independent single-span fixture reproducibility (#54)."""
from pathlib import Path
import unittest,json,tempfile
from freeze_single_span import ROOT,REFERENCE,fixture_rows,owned,owned_sketch,ObjectFixtureFamily
class SingleSpanFixtureTests(unittest.TestCase):
    """Compare full bytes and oracles without replacing source readers."""
    def test_immutable_regeneration(self) -> None:
        """Reproduce each native-field expectation through the full independent reader."""
        rows=fixture_rows();expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        self.assertEqual(len(rows),expected['cases'])
        self.assertEqual(sum(n for _,n in rows),expected['fields'])
        self.assertEqual(b''.join(b for b,_ in rows),(ROOT/'tests/fixtures/single-span-splines/cases.bin').read_bytes())

    def test_complete_owned_sketches_regenerate_all_bytes_and_slots(self) -> None:
        """Use real resolved-feature context for all seven owned sketches."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())['owned']
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);family=ObjectFixtureFamily(build=owned_sketch)
            actual=[owned(REFERENCE,destination,0,style,False,f'owned-{style}',family) for style in range(6)]
            actual.append(owned(REFERENCE,destination,0,2,True,'owned-2-edited',family))
            self.assertEqual(actual,expected)
            for p in destination.iterdir():
                self.assertEqual(p.read_bytes(),(ROOT/'tests/fixtures/single-span-splines'/p.name).read_bytes())
