"""Observe independently assembled complete parameter graphs with original Source (#202)."""
from pathlib import Path
import hashlib
import json
import os
import struct
import unittest
from parameter_bytes import (parameter_graph, PRIMARY_BITS, ALTERNATE_BITS,
                             PRIMARY_INTEGERS, ALTERNATE_INTEGERS, PRESENCE_BYTES)
from freeze_trim_edges import (source_module, saved_manager, SavedSheetMetalDocument,
                               observe_primitives, verify_primitive_coverage)

ROOT = Path(__file__).resolve().parents[3]


class OwnedParameterReferenceTests(unittest.TestCase):
    def test_complete_parameter_graphs_match_literal_roles_and_name_identity(self) -> None:
        """Check all six classes, four pointer/value styles and six primary edits."""
        pin = Path(os.environ['BESSEMER_PARAMETER_PIN'])
        container = source_module('swcontainer', pin)
        original = container.Document((ROOT / 'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
        manager, _ = saved_manager(pin, original, 0)
        destination = Path(os.environ['BESSEMER_PARAMETER_FIXTURES'])
        destination.mkdir(parents=True, exist_ok=True)
        rows = [self.check_graph(original, manager, kind, style, False, destination)
                for kind in range(6) for style in range(4)]
        rows += [self.check_graph(original, manager, kind, 0, True, destination) for kind in range(6)]
        self.assertEqual(len(rows), 30)
        (destination / 'manifest.json').write_bytes((json.dumps(rows, indent=2) + '\n').encode())

    def check_graph(self, original: object, manager: bytes, kind: int, style: int,
                    edited: bool, destination: Path) -> dict[str, object]:
        """Require complete EOF, exact map accounting and primitive coverage; e.g. check_graph(doc,mgr,0,0,False,out)."""
        pin = Path(os.environ['BESSEMER_PARAMETER_PIN'])
        prefix, features, archive = (source_module(name, pin) for name in ('modelprefix', 'resolvedfeatures', 'swarchive'))
        _, context = prefix.read(SavedSheetMetalDocument(original, manager, b''), 'Contents/Config-0',
                                 through_part_resolved=True, configuration_context=True)
        raw, slots = parameter_graph(kind, style, edited, len(context.map) - 1)
        with observe_primitives(archive.Archive, raw) as reads:
            parsed = features.read_supported(SavedSheetMetalDocument(original, manager, raw), 'Contents/Config-0-ResolvedFeatures')
        self.assertEqual((parsed['end'], parsed['tail'], len(parsed['archive'].map)), (len(raw), b'', slots))
        verify_primitive_coverage(raw, reads)
        actual = parsed['items'][0]
        self.check_names(actual, kind, style)
        self.check_values(actual, kind, style, edited)
        name = f'owned-{kind}-{style}' + ('-edited' if edited else '') + '.bin'
        (destination / name).write_bytes(raw)
        return dict(file=name, inherited_slots=len(context.map), final_slots=slots, wire_bytes=len(raw),
                    primitive_calls=len(reads), sha256=hashlib.sha256(raw).hexdigest())

    def check_names(self, actual: dict[str, object], kind: int, style: int) -> None:
        """Distinguish both physical names including null and shared pointers; e.g. check_names(value,0,1)."""
        node = actual['node']
        self.assertEqual([node[key] for key in ('unknown_0c', 'flags_28', 'id', 'unknown_2c', 'tool_tip_name')],
                         [-2, 0xffffffff, -3, 0x80000000, 'help'])
        first = node['name']
        second = actual['parameter_name' if kind < 4 else 'name_40']
        self.assertEqual(first, None if style == 0 else dict(_class='moNodeName_c', name='node Ω'))
        self.assertEqual(second, None if style == 3 else dict(_class='moNodeName_c', name='node Ω' if style == 1 else 'parameter λ'))
        if style == 1:
            self.assertIs(first, second)
        else:
            self.assertIsNot(first, second)

    def check_values(self, actual: dict[str, object], kind: int, style: int, edited: bool) -> None:
        """Compare float images without arithmetic or NaN equality; e.g. check_values(value,0,2,False)."""
        key = 'value' if kind < 4 else 'raw_value_50'
        if kind == 5:
            self.assertEqual(actual[key], 'updated' if edited else ('', 'A\0Ω', 'ABC', '😀\0z')[style])
            return
        presence = 'has_alternate_value' if kind < 4 else 'override_presence_60'
        alternate = 'alternate_value' if kind < 4 else 'raw_override_58'
        self.assertEqual(actual[presence], PRESENCE_BYTES[style])
        self.assertEqual(alternate in actual, bool(PRESENCE_BYTES[style]))
        if kind == 3:
            self.assertEqual(actual[key], 12345 if edited else PRIMARY_INTEGERS[style])
            if PRESENCE_BYTES[style]:
                self.assertEqual(actual[alternate], ALTERNATE_INTEGERS[style])
            return
        self.assertEqual(struct.pack('<d', actual[key]), struct.pack('<Q', 0x3ff4000000000000 if edited else PRIMARY_BITS[style]))
        if PRESENCE_BYTES[style]:
            self.assertEqual(struct.pack('<d', actual[alternate]), struct.pack('<Q', ALTERNATE_BITS[style]))
