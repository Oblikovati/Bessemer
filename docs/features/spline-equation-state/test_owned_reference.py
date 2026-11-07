"""Independent Source observation of complete equation state fixtures (#199)."""
from pathlib import Path
import json
import os
import unittest
from itertools import product
from mfcobjects import ObjectArchive
import sketchprefix
from freeze_trim_edges import ObjectFixtureFamily, owned
from spline_equation_state_bytes import equation_collection, equation_sketch, expected_equation_state, write_literal_positions, equation_alias_graph

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
        write_literal_positions(DESTINATION)
        self.assertEqual(len(observations), 18)

    def test_alias_graph_has_four_physical_splines_and_exact_shared_roots(self) -> None:
        """Observe distinct definitions, aliases and null without inventing alias bodies."""
        destination = DESTINATION / 'alias-graphs'
        destination.mkdir(parents=True, exist_ok=True)
        observations = [self.check_alias_graph(edited, destination) for edited in range(-1, 4)]
        (destination / 'manifest.json').write_bytes((json.dumps(observations, indent=2) + '\n').encode())

    def check_alias_graph(self, edited: int, destination: Path) -> dict[str, int]:
        """Observe a complete independently built alias graph; e.g. self.check_alias_graph(2, temporary)."""
        from freeze_trim_edges import source_module, saved_manager, SavedSheetMetalDocument, observe_primitives, verify_primitive_coverage
        reference = Path(os.environ['BESSEMER_EQUATION_PIN'])
        container, prefix, features, archive = (source_module(name, reference) for name in ('swcontainer', 'modelprefix', 'resolvedfeatures', 'swarchive'))
        original = container.Document((ROOT / 'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
        manager, _ = saved_manager(reference, original, 0)
        _, context = prefix.read(SavedSheetMetalDocument(original, manager, b''), 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
        raw, slots = equation_alias_graph(len(context.map) - 1, edited)
        with observe_primitives(archive.Archive, raw) as reads:
            parsed = features.read_supported(SavedSheetMetalDocument(original, manager, raw), 'Contents/Config-0-ResolvedFeatures')
        self.assertEqual((parsed['end'], parsed['tail'], len(parsed['archive'].map)), (len(raw), b'', slots))
        verify_primitive_coverage(raw, reads)
        self.check_alias_roles(parsed['items'], edited)
        (destination / f'graph-{edited + 1}.bin').write_bytes(raw)
        return dict(edited=edited, wire_bytes=len(raw), slots=slots, primitive_reads=len(reads))

    def check_alias_roles(self, items: list[object], edited: int) -> None:
        """Check identity and literal roles independently; e.g. self.check_alias_roles(items,-1)."""
        self.assertEqual(len(items), 5)
        self.assertIs(items[0], items[1])
        self.assertIsNone(items[2])
        self.assertIs(items[3], items[4])
        splines = items[0]['splines'] + items[3]['splines']
        self.assertEqual(len(splines), 4)
        for ordinal, (actual, style) in enumerate(zip(splines, [0, 4, 1, 2])):
            expected = expected_equation_state(ordinal // 2, style, edited == ordinal)
            self.assertEqual(actual['flags_32c'], expected['flags_32c'])
            self.assertEqual(actual['equation_strings'], expected['strings'])
            self.assertEqual(actual['equation_word'], expected['compatibility_word'])
            self.assertEqual(actual['unknown_1e0'], expected['saved_option'])
