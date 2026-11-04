"""Complete independent alternate/presence edit matrix where native population is zero (#202)."""
from pathlib import Path
import copy
import hashlib
import json
import os
import struct
import unittest
from parameter_bytes import parameter_graph
from verify_parameter_exports import observe_parameters

ROOT = Path(__file__).resolve().parents[3]


class AlternateMatrixReferenceTests(unittest.TestCase):
    def test_all_numeric_classes_and_positive_presence_bytes_have_complete_expected_graphs(self) -> None:
        """Cover15 positive alternate graphs,15 value flips and all3825 nonzero presence requests."""
        import swcontainer
        document = swcontainer.Document((ROOT / 'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
        manager = (ROOT / 'tests/fixtures/sheet-metal/manager-0.bin').read_bytes()
        packet, count = bytearray(b'BPA1' + struct.pack('<I', 3840)), 0
        for kind in range(5):
            for style in range(1, 4):
                frames = self.graph_matrix(document, manager, kind, style)
                packet.extend(b''.join(frames))
                count += len(frames)
                print(json.dumps(dict(kind=kind, style=style, checked=count)), flush=True)
        self.assertEqual(count, 3840)
        self.retain_packet(bytes(packet), count)

    def graph_matrix(self, document: object, manager: bytes, kind: int, style: int) -> list[bytes]:
        """Start from an independent graph already accepted by original Source; e.g. graph_matrix(doc,mgr,0,1)."""
        from sheet_owned_fixtures import SavedSheetMetalDocument
        raw, _ = parameter_graph(kind, style)
        self.assertEqual(raw, (ROOT / f'tests/fixtures/stored-parameters/owned-{kind}-{style}.bin').read_bytes())
        _, rows = observe_parameters(SavedSheetMetalDocument(document, manager, raw), 'Contents/Config-0-ResolvedFeatures')
        self.assertEqual(len(rows), 1)
        width = 4 if kind == 3 else 8
        presence_offset = len(raw) - width - 1
        self.assertEqual(raw[presence_offset], rows[0]['presence'])
        frames = [self.check_alternate(document, manager, raw, rows, kind, style, width)]
        frames.extend(self.check_presence(document, manager, raw, rows, kind, style, presence_offset, byte) for byte in range(1, 256))
        return frames

    def check_alternate(self, document: object, manager: bytes, raw: bytes, rows: list[dict[str, object]],
                        kind: int, style: int, width: int) -> bytes:
        """Flip one raw alternate bit independently; e.g. check_alternate(doc,mgr,raw,rows,0,1,8)."""
        original = raw[-width:]
        self.assertEqual(rows[0]['alternate'], (1 if kind == 3 else 0, original))
        value = bytes([original[0] ^ 1]) + original[1:]
        changed = raw[:-width] + value
        expected = copy.deepcopy(rows)
        expected[0]['alternate'] = (1 if kind == 3 else 0, value)
        self.check_complete(document, manager, changed, expected)
        return self.case_frame(kind, style, 0, raw, changed, value)

    def check_presence(self, document: object, manager: bytes, raw: bytes, rows: list[dict[str, object]],
                       kind: int, style: int, offset: int, byte: int) -> bytes:
        """Keep every alternate byte while replacing the full presence octet; e.g. check_presence(doc,mgr,raw,rows,0,1,offset,255)."""
        changed = raw[:offset] + bytes([byte]) + raw[offset+1:]
        expected = copy.deepcopy(rows)
        expected[0]['presence'] = byte
        self.check_complete(document, manager, changed, expected)
        return self.case_frame(kind, style, 1, raw, changed, bytes([byte]))

    def check_complete(self, document: object, manager: bytes, raw: bytes, expected: list[dict[str, object]]) -> None:
        """Require original complete parses and exact identity/value results; e.g. check_complete(doc,mgr,raw,expected)."""
        from sheet_owned_fixtures import SavedSheetMetalDocument
        _, actual = observe_parameters(SavedSheetMetalDocument(document, manager, raw), 'Contents/Config-0-ResolvedFeatures')
        self.assertEqual(actual, expected)

    def case_frame(self, kind: int, style: int, edit: int, original: bytes, changed: bytes, request: bytes) -> bytes:
        """Retain original and complete independent expected images; e.g. case_frame(0,1,0,raw,changed,request)."""
        return struct.pack('<III', kind, style, edit) + b''.join(struct.pack('<I', len(value)) + value for value in (original, changed, request))

    def retain_packet(self, raw: bytes, count: int) -> None:
        """Write only under the explicit generated fixture root; e.g. retain_packet(raw,3840)."""
        destination = Path(os.environ['BESSEMER_PARAMETER_FIXTURES'])
        destination.mkdir(parents=True, exist_ok=True)
        (destination / 'cases.bin').write_bytes(raw)
        report = dict(cases=count, alternate_value_edits=15, presence_requests=3825,
                      bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())
        (destination / 'manifest.json').write_bytes((json.dumps(report, indent=2) + '\n').encode())
