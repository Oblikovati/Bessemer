"""Independent complete equation graphs in persisted legacy contexts (#199)."""
from pathlib import Path
import hashlib
import json
import os
import struct
import shutil
import unittest
from spline_equation_state_bytes import equation_sketch, expected_equation_state
from freeze_trim_edges import source_module, observe_primitives, verify_primitive_coverage
from freeze_owned_features import ReplacedFeatureDocument

ROOT = Path(__file__).resolve().parents[3]
STREAM = 'Contents/Config-0-ResolvedFeatures'


class VersionEquationReferenceTests(unittest.TestCase):
    def test_legacy_context_graphs_preserve_roles_through_complete_eof(self) -> None:
        """Read both prototypes and every stored role style in versions11000 and18000."""
        for version in (11000, 18000):
            with self.subTest(version=version):
                self.check_version_context(version)
        shutil.copyfile(Path(os.environ['BESSEMER_EQUATION_CONTEXT_ROOT']) / 'contexts.json',
                        Path(os.environ['BESSEMER_EQUATION_VERSION_ROOT']) / 'contexts.json')

    def check_version_context(self, version: int) -> None:
        """Use exact frozen native context bytes; e.g. self.check_version_context(11000)."""
        pin = Path(os.environ['BESSEMER_EQUATION_PIN'])
        folder = Path(os.environ['BESSEMER_EQUATION_CONTEXT_ROOT']) / str(version)
        container = source_module('swcontainer', pin)
        document = container.Document((folder / 'document.SLDPRT').read_bytes(), table={})
        for stream, filename in [('Contents/Config-0-ModelHeader', 'header.bin'), ('Contents/Config-0', 'configuration.bin'), ('Contents/CMgr', 'manager.bin')]:
            self.assertEqual(document.read(stream), (folder / filename).read_bytes())
        destination = Path(os.environ['BESSEMER_EQUATION_VERSION_ROOT']) / str(version)
        destination.mkdir(parents=True, exist_ok=True)
        for name in ('document.SLDPRT', 'header.bin', 'configuration.bin', 'manager.bin'):
            shutil.copyfile(folder / name, destination / name)
        observations = [self.check_version_graph(document, version, kind, style, False, destination) for kind in range(2) for style in range(8)]
        observations += [self.check_version_graph(document, version, kind, 0, True, destination) for kind in range(2)]
        (destination / 'manifest.json').write_bytes((json.dumps(observations, indent=2) + '\n').encode())

    def check_version_graph(self, document: object, version: int, kind: int, style: int, edited: bool, destination: Path) -> dict[str, object]:
        """Build with actual inherited slots and the saved manager; e.g. self.check_version_graph(doc,11000,0,0,False,output)."""
        pin = Path(os.environ['BESSEMER_EQUATION_PIN'])
        prefix, features, archive = (source_module(name, pin) for name in ('modelprefix', 'resolvedfeatures', 'swarchive'))
        _, context = prefix.read(document, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
        feature, slots = equation_sketch(kind, version, style, edited, len(context.map) - 1)
        raw = struct.pack('<iH', -123, 1) + feature + (bytes(4) if context.sheet_metal_helper_850 else b'')
        with observe_primitives(archive.Archive, raw) as reads:
            parsed = features.read_supported(ReplacedFeatureDocument(document, raw), STREAM)
        self.assertEqual((parsed['end'], parsed['tail'], len(parsed['archive'].map)), (len(raw), b'', slots))
        verify_primitive_coverage(raw, reads)
        self.check_version_roles(parsed['items'][0]['splines'][0], kind, style, edited)
        name = f'owned-{kind}-{style}' + ('-edited' if edited else '') + '.bin'
        (destination / name).write_bytes(raw)
        return dict(file=name, version=version, inherited_slots=len(context.map), final_slots=slots, helper=context.sheet_metal_helper_850,
                    wire_bytes=len(raw), primitive_calls=len(reads), sha256=hashlib.sha256(raw).hexdigest())

    def check_version_roles(self, actual: dict[str, object], kind: int, style: int, edited: bool) -> None:
        """Compare original reader results to literal role expectations; e.g. self.check_version_roles(spline,0,0,False)."""
        expected = expected_equation_state(kind, style, edited)
        self.assertEqual(actual['flags_32c'], expected['flags_32c'])
        self.assertEqual(actual.get('equation_strings'), expected['strings'])
        self.assertEqual(actual.get('equation_word'), expected['compatibility_word'])
        self.assertEqual(actual.get('unknown_1e0'), expected['saved_option'])
