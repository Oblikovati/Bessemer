"""Freeze every native primary span and independent replacement through original Source (#202)."""
from pathlib import Path
import hashlib
import json
import os
import struct
import unittest
from all_primary_observer import observe_all_primary
from primary_replacements import primary_replacement, string_replacement
from verify_actual_reference import RootPacket, read_native_case

ROOT = Path(__file__).resolve().parents[3]


def frame(raw: bytes) -> bytes:
    """Frame literal oracle bytes without a Bessemer codec; e.g. frame(raw)."""
    return struct.pack('<I', len(raw)) + raw


class AllPrimaryReferenceTests(unittest.TestCase):
    def test_a_owned_all_kind_spans_select_values_not_node_words_or_tooltips(self) -> None:
        """Check all original owned classes/styles against complete unchanged readers."""
        import swcontainer
        from sheet_owned_fixtures import SavedSheetMetalDocument
        document = swcontainer.Document((ROOT / 'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
        manager = (ROOT / 'tests/fixtures/sheet-metal/manager-0.bin').read_bytes()
        for kind in range(6):
            for style in range(4):
                raw = (ROOT / f'tests/fixtures/stored-parameters/owned-{kind}-{style}.bin').read_bytes()
                groups = observe_all_primary(SavedSheetMetalDocument(document, manager, raw), 'Contents/Config-0-ResolvedFeatures')
                rows, spans = groups[1]
                self.assertEqual(len(rows), 1)
                self.check_span(raw, rows[0], spans[0], 19000)

    def test_b_string_replacement_prefix_boundaries_match_original_cstring_reader(self) -> None:
        """Exercise byte/word/dword and reserved narrow0xFFFE with original standalone decoding."""
        import swarchive
        for wide in (False, True):
            for count in (0, 1, 254, 255, 65533, 65534, 65535):
                text, raw = string_replacement('A' * count, b'\xff\xfe\xff' if wide else b'')
                archive = swarchive.Archive(raw, 19000)
                self.assertEqual(archive.cstring(), text)
                self.assertEqual(archive.p, len(raw))

    def test_c_all_native_primary_values_have_exact_original_spans_and_valid_replacements(self) -> None:
        """Retain all43917 original spans; no sparse edit census is substituted."""
        native = RootPacket((ROOT / 'target/resolved-native-probe.bin').read_bytes())
        self.assertEqual((native.take(4), native.word()), (b'BRF1', 906))
        packet, totals = bytearray(b'BPP1' + struct.pack('<I', 906)), [0, 0]
        for case in range(906):
            version, label, document = read_native_case(native)
            packet += struct.pack('<I', version) + frame(label.encode())
            groups = observe_all_primary(document, document.feature_stream)
            for context, (rows, spans) in enumerate(groups):
                stream = document.feature_stream if context else document.feature_stream.removesuffix('-ResolvedFeatures')
                packet += self.context_packet(document.read(stream), rows, spans, version)
                totals[context] += len(rows)
            print(json.dumps(dict(case=case, configuration=totals[0], features=totals[1])), flush=True)
        self.assertEqual((native.position, totals), (len(native.raw), [4430, 39487]))
        self.retain_packet(bytes(packet), totals)

    def context_packet(self, raw: bytes, rows: list[dict[str, object]], spans: list[tuple[int, bytes]], version: int) -> bytes:
        """Bind every original loaded identity to one primary extent and replacement; e.g. self.context_packet(raw,rows,spans,version)."""
        self.assertEqual(len(rows), len(spans))
        packet = bytearray(struct.pack('<I', len(rows)))
        for ordinal, (row, span) in enumerate(zip(rows, spans)):
            replacement = self.check_span(raw, row, span, version)
            kind = row['primary'][0]
            packet += struct.pack('<IIII', ordinal, kind, row['definition'][3], span[0])
            packet += frame(span[1]) + frame(replacement[1])
            packet += frame(replacement[0][1].encode() if kind == 2 else b'')
        return bytes(packet)

    def check_span(self, raw: bytes, row: dict[str, object], span: tuple[int, bytes], version: int) -> tuple[object, bytes]:
        """Require original bytes and independently decoded replacement EOF; e.g. self.check_span(raw,row,span,version)."""
        import swarchive
        offset, original = span
        self.assertEqual(raw[offset:offset+len(original)], original)
        replacement, changed = primary_replacement(row['primary'], original)
        self.assertNotEqual(changed, original)
        archive = swarchive.Archive(changed, version)
        if replacement[0] == 2:
            self.assertEqual(archive.cstring(), replacement[1])
        else:
            value = archive.f64() if replacement[0] == 0 else archive.i32()
            self.assertEqual(struct.pack('<d' if replacement[0] == 0 else '<i', value), changed)
        self.assertEqual(archive.p, len(changed))
        return replacement, changed

    def retain_packet(self, raw: bytes, totals: list[int]) -> None:
        """Write the complete reviewed census into the explicit generated root; e.g. self.retain_packet(raw,totals)."""
        destination = Path(os.environ['BESSEMER_PARAMETER_FIXTURES'])
        destination.mkdir(parents=True, exist_ok=True)
        (destination / 'spans.bin').write_bytes(raw)
        record = dict(cases=906, configuration=totals[0], features=totals[1], bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())
        (destination / 'manifest.json').write_bytes((json.dumps(record, indent=2) + '\n').encode())
