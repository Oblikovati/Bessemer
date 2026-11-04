"""Complete owned parameter roles in independently loaded older native contexts (#202)."""
from pathlib import Path
import hashlib
import json
import os
import struct
import unittest
from parameter_bytes import (parameter_graph, parameter_string, PRIMARY_BITS,
                             PRIMARY_INTEGERS, PRESENCE_BYTES)
from test_native_primary_reference import EditedNativeContext
from verify_actual_reference import RootPacket, read_native_case
from verify_parameter_exports import observe_parameters

ROOT = Path(__file__).resolve().parents[3]


class VersionContextReferenceTests(unittest.TestCase):
    def test_every_class_and_style_completes_in_both_older_native_contexts(self) -> None:
        """Retain60 complete original/edited graphs across versions11000 and18000."""
        packet = RootPacket((ROOT / 'target/resolved-native-probe.bin').read_bytes())
        self.assertEqual((packet.take(4), packet.word()), (b'BRF1', 906))
        remaining, manifest = {11000, 18000}, []
        for case in range(906):
            version, label, document = read_native_case(packet)
            if version not in remaining:
                continue
            manifest.extend(self.check_context(document, case, version, label))
            remaining.remove(version)
            if not remaining:
                break
        self.assertFalse(remaining)
        self.assertEqual(len(manifest), 60)
        destination = Path(os.environ['BESSEMER_PARAMETER_FIXTURES'])
        (destination / 'manifest.json').write_bytes((json.dumps(manifest, indent=2) + '\n').encode())

    def check_context(self, document: object, case: int, version: int, label: str) -> list[dict[str, object]]:
        """Derive map identities only from the actual complete inherited context; e.g. check_context(doc,0,11000,label)."""
        import modelprefix
        _, context = modelprefix.read(document, document.feature_stream.removesuffix('-ResolvedFeatures'),
                                      through_part_resolved=True, configuration_context=True)
        self.assertEqual(context.p, len(context.b))
        destination = Path(os.environ['BESSEMER_PARAMETER_FIXTURES'])
        destination.mkdir(parents=True, exist_ok=True)
        return [self.check_graph(document, destination, case, version, label, len(context.map), kind, style, edited)
                for kind in range(6) for style, edited in [(style, False) for style in range(4)] + [(0, True)]]

    def check_graph(self, document: object, destination: Path, case: int, version: int, label: str,
                    slots: int, kind: int, style: int, edited: bool) -> dict[str, object]:
        """Require a complete original feature parse and exact literal value image; e.g. check_graph(doc,out,0,11000,label,97,0,0,False)."""
        raw, _ = parameter_graph(kind, style, edited, slots - 1)
        supplied = EditedNativeContext(document, document.feature_stream, raw)
        _, rows = observe_parameters(supplied, document.feature_stream)
        self.assertEqual(len(rows), 1)
        row = rows[0]
        self.assertEqual(row['kind'], kind)
        self.assertEqual(row['primary'], self.expected_primary(kind, style, edited))
        self.assertEqual(row['presence'], None if kind == 5 else PRESENCE_BYTES[style])
        name = f'{version}-{kind}-{style}' + ('-edited' if edited else '') + '.bin'
        (destination / name).write_bytes(raw)
        return dict(file=name, case=case, version=version, label=label, inherited_slots=slots,
                    bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())

    def expected_primary(self, kind: int, style: int, edited: bool) -> tuple[object, ...]:
        """Preserve literal string prefixes and complete scalar bits; e.g. expected_primary(5,2,False)."""
        if kind == 5:
            return 2, 'updated' if edited else ('', 'A\0Ω', 'ABC', '😀\0z')[style], parameter_string(style, edited)
        if kind == 3:
            return 1, struct.pack('<i', 12345 if edited else PRIMARY_INTEGERS[style])
        return 0, struct.pack('<Q', 0x3ff4000000000000 if edited else PRIMARY_BITS[style])
