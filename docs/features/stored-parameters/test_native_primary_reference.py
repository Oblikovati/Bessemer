"""Independent native first-operation edit spans and complete edited Source replays (#202)."""
from pathlib import Path
import copy
import hashlib
import json
import os
import struct
import unittest
from verify_actual_reference import RootPacket, read_native_case
from native_primary_observer import observe_native_primary
from verify_parameter_exports import observe_parameters

ROOT = Path(__file__).resolve().parents[3]


class EditedNativeContext:
    def __init__(self, original: object, stream: str, replacement: bytes) -> None:
        """Override one supplied stream without touching original input; e.g. EditedNativeContext(doc,stream,raw)."""
        self.original, self.stream, self.replacement = original, stream, replacement

    def names(self) -> list[str]:
        """Retain the original explicit model version; e.g. context.names()."""
        return self.original.names()

    def read(self, name: str) -> bytes:
        """Return only the selected replacement; e.g. context.read(stream)."""
        return self.replacement if name == self.stream else self.original.read(name)


class NativePrimaryReferenceTests(unittest.TestCase):
    def test_first_native_configuration_and_feature_edits_change_only_one_original_double(self) -> None:
        """Prove a complete operation in both contexts before expanding the native edit workload."""
        packet = RootPacket((ROOT / 'target/resolved-native-probe.bin').read_bytes())
        self.assertEqual((packet.take(4), packet.word()), (b'BRF1', 906))
        version, label, document = read_native_case(packet)
        observed = observe_native_primary(document, document.feature_stream)
        rows = [self.check_edit(document, observed, context) for context in range(2)]
        destination = Path(os.environ['BESSEMER_PARAMETER_FIXTURES'])
        destination.mkdir(parents=True, exist_ok=True)
        raw = b'BPN1' + struct.pack('<I', 2) + b''.join(struct.pack('<IIIIQQ', *row) for row in rows)
        (destination / 'spans.bin').write_bytes(raw)
        report = dict(case=0, version=version, label=label, rows=rows, packet_sha256=hashlib.sha256(raw).hexdigest())
        (destination / 'manifest.json').write_bytes((json.dumps(report, indent=2) + '\n').encode())

    def check_edit(self, document: object, observed: tuple[object, ...], context: int) -> list[int]:
        """Use original reader positions, then independently reparse the changed bytes; e.g. check_edit(doc,rows,0)."""
        rows, spans = observed[context]
        self.assertGreater(len(rows), 0)
        offset, original = spans[0]
        self.assertEqual(rows[0]['primary'], (0, original))
        bits, = struct.unpack('<Q', original)
        stream = document.feature_stream if context else document.feature_stream.removesuffix('-ResolvedFeatures')
        raw = document.read(stream)
        self.assertEqual(raw[offset:offset+8], original)
        changed = raw[:offset] + struct.pack('<Q', bits ^ 1) + raw[offset+8:]
        expected = copy.deepcopy([group[0] for group in observed])
        expected[context][0]['primary'] = (0, struct.pack('<Q', bits ^ 1))
        actual = observe_parameters(EditedNativeContext(document, stream, changed), document.feature_stream)
        self.assertEqual(list(actual), expected)
        self.assertEqual(document.read(stream), raw)
        return [context, 0, rows[0]['definition'][3], offset, bits, bits ^ 1]
