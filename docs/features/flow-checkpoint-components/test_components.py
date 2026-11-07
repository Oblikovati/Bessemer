"""Reference authentication, independent layouts and malformed component boundaries."""
from __future__ import annotations
import gzip
import json
import struct
import tempfile
import unittest
from pathlib import Path
import checkpoint_requests as requests
import checkpoint_wire as wire
import freeze_components as freeze
from swarchive import ArchiveError


class CheckpointBaselineTests(unittest.TestCase):
    def test_original_five_synthetic_examples_authenticate_and_exclude_parent_and_codec_claims(self) -> None:
        rows, evidence = requests.reference_cases()
        self.assertEqual(len(rows), 5)
        self.assertIn('no version-6 native fixture', ' '.join(evidence['limitations']))
        for _, _, kind, raw in rows:
            parsed = wire.READERS[kind](raw)
            self.assertEqual(parsed['end'], len(raw))
            self.assertTrue(parsed['complete_component_framing'])
            self.assertFalse(parsed['complete_parent_record'])
            self.assertFalse(parsed['engineering_fields_assigned'])

    def test_cyclic_dimensions_and_absence_have_closed_form_lengths(self) -> None:
        for kind in ('graph2', 'auxiliary'):
            self.check_dimensions(kind)

    def check_dimensions(self, kind: str) -> None:
        for name, _, _, raw in requests.dimension_cases(kind):
            parsed = wire.READERS[kind](raw)
            dimensions = parsed['dimensions']
            self.assertEqual(len(raw), 24 + sum(dimensions), name)
            self.assertEqual([group['length'] for group in parsed['groups']], [dimensions[1], dimensions[2], dimensions[0]])
            self.assertEqual(parsed['member_triplet_i32'], (-0x80000000, -1, 0x7fffffff))
            self.assertTrue(all(row['row'] is None for group in parsed['groups'] for row in group['rows']))

    def test_row_counts_and_container_lengths_have_independent_formulas(self) -> None:
        for kind in ('graph2', 'auxiliary'):
            for name, _, _, raw in requests.row_cases(kind):
                count = int(name.rsplit('-', 1)[1])
                self.assertEqual(len(raw), 29 + count * (5 if kind == 'graph2' else 4))
                self.assertEqual(wire.READERS[kind](raw)['end'], len(raw))
        for name, _, kind, raw in requests.container_count_cases():
            count = int(name.rsplit('-', 1)[1])
            self.assertEqual(len(raw), (38 if kind == 'graph2' else 12) + 28 * count)
            self.assertEqual(wire.READERS[kind](raw)['end'], len(raw))

    def test_every_presence_and_tag_byte_preserves_the_reader_and_writer_distinction(self) -> None:
        for tag in range(256):
            body = wire.integers([0]) if tag > 1 else b''
            raw = wire.one_row(2, wire.row([7], bytes([tag]) + body))
            slot = wire.READERS['graph2'](raw)['groups'][2]['rows'][0]['row']['slots'][0]
            self.assertEqual(slot['tag_byte'], tag)
            self.assertEqual(slot['canonical_writer_tag'], tag <= 2)
            self.assertEqual(slot['container'] is not None, tag > 1)
        for kind in ('graph2', 'auxiliary'):
            for _, _, _, raw in requests.presence_cases(kind):
                self.check_presence(kind, raw)

    def check_presence(self, kind: str, raw: bytes) -> None:
        for group in wire.READERS[kind](raw)['groups']:
            flag = group['presence_raw'][0]
            self.assertEqual(group['rows'][0]['present_byte'], flag)
            self.assertEqual(group['rows'][0]['row'] is not None, flag != 0)

    def test_every_small_prefix_and_all_extensions_are_rejected(self) -> None:
        for _, _, kind, raw in requests.cases():
            self.check_prefixes(kind, raw)

    def check_prefixes(self, kind: str, raw: bytes) -> None:
        reader = wire.READERS[kind]
        if len(raw) <= 128:
            for length in range(len(raw)):
                self.assertRaises(ArchiveError, reader, raw[:length])
        self.assertRaises(ArchiveError, reader, raw + b'\0')

    def test_negative_and_maximal_counts_fail_before_unbounded_iteration(self) -> None:
        for count in (-0x80000000, -1, 0x7fffffff):
            self.check_bad_counts(count)
        for pattern in wire.WORD_PATTERNS:
            if pattern:
                raw = struct.pack('<IiQ', pattern, 1, 0) + wire.integers([0])
                self.assertRaises(ArchiveError, wire.READERS['chunks'], raw)
        self.assertRaises(ArchiveError, wire.READERS['chunks'], wire.integers([0, 0]) + bytes(8))

    def check_bad_counts(self, count: int) -> None:
        for kind in ('graph2', 'auxiliary'):
            self.assertRaises(ArchiveError, wire.READERS[kind], wire.integers([count, 0, 0, 0, 0, 0]))
            self.assertRaises(ArchiveError, wire.READERS[kind], wire.one_row(0, wire.integers([count])))
        for position in range(3):
            raw = wire.integers([0] * position + [count] + [0] * (2 - position))
            self.assertRaises(ArchiveError, wire.READERS['trailing'], raw)
        self.assertRaises(ArchiveError, wire.READERS['chunks'], wire.integers([0, count]) + bytes(8))
        self.assertRaises(ArchiveError, wire.READERS['chunks'], wire.integers([0, 1]) + bytes(8) + wire.integers([count]))

    def test_chunk_declarations_do_not_decode_or_validate_opaque_contents(self) -> None:
        raw = wire.chunks([b'\xffbad-zlib\0', b''], 0xffffffffffffffff)
        parsed = wire.READERS['chunks'](raw)
        self.assertEqual(parsed['declared_raw_bytes'], 0xffffffffffffffff)
        self.assertEqual(parsed['chunks'][0]['encoded_bytes'], b'\xffbad-zlib\0')
        self.assertFalse(parsed['codec_decoded'])
        self.assertFalse(parsed['decoded_size_verified'])
        self.assertEqual(wire.words([0xffffffffffffffff], 8), b'\xff' * 8)
        self.assertEqual(wire.signed_pattern(0x80000000), -0x80000000)

    def test_all_packets_and_manifests_regenerate_byte_for_byte_with_existing_bounds(self) -> None:
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            freeze.freeze(root)
            proof_path = Path('docs/features/flow-checkpoint-components/baseline-verification.json')
            self.assertEqual((root / proof_path).read_bytes(), (wire.ROOT / proof_path).read_bytes())
            proof = json.loads((root / proof_path).read_text())
            self.assertEqual(proof['cases'], 2899)
            for family, metadata in proof['packets'].items():
                path = Path('tests/fixtures/flow-checkpoint-components') / f'{family}.bin.gz'
                self.assertEqual((root / path).read_bytes(), (wire.ROOT / path).read_bytes())
                packet = gzip.decompress((root / path).read_bytes())
                self.assertEqual(len(packet), metadata['bytes'])
                self.assertLessEqual(len(packet), 16_000_000)
                self.assertEqual(requests.digest(packet), metadata['sha256'])
