"""Pin a positive inherited-name alias through original complete Source readers (#202)."""
from pathlib import Path
import hashlib
import json
import os
import struct
import unittest
from parameter_bytes import inherited_parameter_graph
from freeze_trim_edges import source_module, saved_manager, SavedSheetMetalDocument
from verify_parameter_exports import observe_parameters

ROOT = Path(__file__).resolve().parents[3]


class InheritedParameterReferenceTests(unittest.TestCase):
    def test_inherited_name_identity_and_independent_value_edit_reach_complete_eof(self) -> None:
        """Require both physical pointers to select one configuration-owned Part1 name."""
        pin = Path(os.environ['BESSEMER_PARAMETER_PIN'])
        container, prefix = (source_module(name, pin) for name in ('swcontainer', 'modelprefix'))
        document = container.Document((ROOT / 'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
        manager, _ = saved_manager(pin, document, 0)
        _, context = prefix.read(SavedSheetMetalDocument(document, manager, b''), 'Contents/Config-0',
                                 through_part_resolved=True, configuration_context=True)
        identity = next(index for index, value in enumerate(context.map)
                        if isinstance(value, dict) and value.get('_class') == 'moNodeName_c')
        self.assertEqual(context.map[identity]['name'], 'Part1')
        destination = Path(os.environ['BESSEMER_PARAMETER_FIXTURES'])
        destination.mkdir(parents=True, exist_ok=True)
        rows = [self.check_graph(document, manager, identity, len(context.map), edited, destination) for edited in (False, True)]
        (destination / 'manifest.json').write_bytes((json.dumps(rows, indent=2) + '\n').encode())

    def check_graph(self, document: object, manager: bytes, identity: int, slots: int,
                    edited: bool, destination: Path) -> dict[str, object]:
        """Bind independent wire bytes to actual inherited identity; e.g. check_graph(doc,mgr,4,97,False,out)."""
        raw, final_slots = inherited_parameter_graph(identity, slots - 1, edited)
        _, features = observe_parameters(SavedSheetMetalDocument(document, manager, raw), 'Contents/Config-0-ResolvedFeatures')
        self.assertEqual(len(features), 1)
        value = features[0]
        self.assertEqual(value['definition'], (65535, None, 1, slots + 1, 'moDensityParameter_c', 7))
        self.assertEqual(value['names'][0], value['names'][1])
        for name in value['names']:
            self.assertEqual(name['origin'], 1)
            self.assertEqual(name['tag'], (identity, None, 2, identity, 'moNodeName_c', 1))
            self.assertEqual(name['text'], ('Part1', b'\xff\xfe\xff\x05' + 'Part1'.encode('utf-16le')))
        self.assertEqual(value['primary'], (0, struct.pack('<Q', 0xc023000000000000 if edited else 0x3ff4000000000000)))
        self.assertEqual((value['presence'], value['alternate']), (0, None))
        name = 'edited.bin' if edited else 'original.bin'
        (destination / name).write_bytes(raw)
        return dict(file=name, inherited_name_identity=identity, inherited_slots=slots, final_slots=final_slots,
                    wire_bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())
