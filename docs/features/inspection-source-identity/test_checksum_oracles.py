"""Pin independent expected checksums before implementing the compatibility operation (#195)."""
import hashlib
import struct
import unittest
from freeze_identity import FIXTURES, LARGE_LENGTHS, RFC_VECTORS, build_packet, manifest_rows, owned_inputs
from freeze_identity import metadata_inputs, metadata_packet


class ChecksumOracleTests(unittest.TestCase):
    def test_metadata_packet_preserves_absent_empty_and_noncanonical_text_cases(self) -> None:
        """Keep every declared metadata case and independently fixed exact-match count."""
        cases = metadata_inputs()
        self.assertEqual(len(cases), 22)
        self.assertEqual(sum(size == '3' for _, size, _ in cases), 9)
        self.assertEqual(sum(checksum == RFC_VECTORS[2][1] for _, _, checksum in cases), 13)
        packet = (FIXTURES / 'metadata.bin').read_bytes()
        self.assertEqual(packet, metadata_packet())
        self.assertEqual(packet[:4], b'ISMC')
        self.assertEqual(struct.unpack_from('<I', packet, 4), (22,))

    def test_published_rfc_vectors_match_python_provider(self) -> None:
        """Require all seven literal RFC1321 A.5 expectations without deriving them from Rust."""
        for raw, expected in RFC_VECTORS:
            self.assertEqual(hashlib.md5(raw).hexdigest(), expected)

    def test_owned_packet_and_manifest_are_exactly_reproducible(self) -> None:
        """Pin the binary artifact and every payload offset independently of the future implementation."""
        packet = (FIXTURES / 'vectors.bin').read_bytes()
        self.assertEqual(packet, build_packet())
        self.assertEqual((FIXTURES / 'vectors.tsv').read_text(), manifest_rows())
        self.assertEqual(packet[:4], b'ISMD')
        self.assertEqual(struct.unpack_from('<I', packet, 4), (172,))
        rows = manifest_rows().splitlines()[1:]
        self.assertEqual(len(rows), 172)
        for row in rows:
            _, _, offset, length, md5, sha256 = row.split('\t')
            raw = packet[int(offset):int(offset) + int(length)]
            self.assertEqual(hashlib.md5(raw).hexdigest(), md5)
            self.assertEqual(hashlib.sha256(raw).hexdigest(), sha256)
        last = rows[-1].split('\t')
        self.assertEqual(int(last[2]) + int(last[3]), len(packet))

    def test_all_padding_positions_blocks_and_degenerate_patterns_are_present(self) -> None:
        """Require the declared boundary coverage and unique case identities."""
        cases = owned_inputs()
        labels = {label for label, _ in cases}
        self.assertEqual(len(labels), 172)
        self.assertTrue({f'pattern-{length}' for length in range(130)} <= labels)
        self.assertTrue({f'pattern-{length}' for length in LARGE_LENGTHS} <= labels)
        self.assertTrue({'zeros-56', 'ones-56', 'zeros-64', 'ones-64', 'million-a'} <= labels)


if __name__ == '__main__':
    unittest.main()
