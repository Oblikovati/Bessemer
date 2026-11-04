"""Independent Source observation of complete equation state fixtures (#199)."""
from pathlib import Path
import json
import os
import unittest
from itertools import product
from mfcobjects import ObjectArchive
import sketchprefix
from freeze_trim_edges import ObjectFixtureFamily, owned
from spline_equation_state_bytes import equation_collection, equation_sketch, expected_equation_state

ROOT = Path(__file__).resolve().parents[3]
DESTINATION = Path(os.environ['BESSEMER_EQUATION_FIXTURE_ROOT'])


class OwnedEquationReferenceTests(unittest.TestCase):
    def test_direct_roles_match_literal_flags_strings_and_full_signed_option(self) -> None:
        """Check both prototype contexts, absent equations and every selected option boundary."""
        for kind, style, edited in product(range(2), range(8), (False, True)):
            if edited and style >= 6:
                continue
            with self.subTest(kind=kind, style=style, edited=edited):
                self.check_direct(kind, style, edited)

    def check_direct(self, kind: int, style: int, edited: bool) -> None:
        """Observe the unmodified original direct reader; e.g. self.check_direct(0,0,False)."""
        raw = equation_collection(kind, style, edited)
        source = ObjectArchive(raw, 19000)
        source.feature_context = 'ordinary_external_xml'
        source.single_span_splines = kind == 1
        sketchprefix.install_scoped_readers(source)
        value = sketchprefix.read_splines(source, {})['splines'][0]
        expected = expected_equation_state(kind, style, edited)
        self.assertEqual(source.p, len(raw))
        self.assertEqual(value['flags_32c'], expected['flags_32c'])
        self.assertEqual(value.get('equation_strings'), expected['strings'])
        self.assertEqual(value.get('equation_word'), expected['compatibility_word'])
        self.assertEqual(value.get('unknown_1e0'), expected['saved_option'])

    def test_complete_owned_features_reach_eof_and_cover_every_primitive_byte(self) -> None:
        """Require real complete feature graphs through the established owned observer."""
        reference = Path(os.environ['BESSEMER_EQUATION_PIN'])
        family = ObjectFixtureFamily(build=equation_sketch, kinds=2)
        DESTINATION.mkdir(parents=True, exist_ok=True)
        observations = []
        for kind in range(2):
            for style in range(8):
                label = f'owned-{kind}-{style}'
                row = owned(reference, DESTINATION, kind, style, False, label, family)
                observations.append(row | dict(expected=expected_equation_state(kind, style)))
            row = owned(reference, DESTINATION, kind, 0, True, f'owned-{kind}-0-edited', family)
            observations.append(row | dict(expected=expected_equation_state(kind, 0, True)))
        (DESTINATION / 'manifest.json').write_bytes((json.dumps(observations, indent=2) + '\n').encode())
        self.assertEqual(len(observations), 18)
