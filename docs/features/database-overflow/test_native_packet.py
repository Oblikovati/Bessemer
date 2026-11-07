"""Authenticate native regression packets and reject incomplete source spans."""
import json
from pathlib import Path
import struct
import tempfile
from typing import cast
import unittest
from native_packet import NativeCensusRow, checked_payloads, native_packet, native_record, native_value

FOLDER = Path(__file__).resolve().parent
FIXTURES = FOLDER.parents[2] / 'tests/fixtures/database-overflow'


class FakeNativeImages:
    """Own an injected temporary image root; e.g. FakeNativeImages(root).insert(row,raw)."""
    def __init__(self, root: Path) -> None:
        self.root = root

    def insert(self, row: NativeCensusRow, raw: bytes) -> None:
        (self.root / f"{row['ordinal']}.bin").write_bytes(raw)


def native_samples() -> list[NativeCensusRow]:
    """Load reviewed fixture identities; e.g. native_samples()[0]."""
    raw = json.loads((FOLDER / 'native-sample-reference.json').read_text(encoding='utf-8'))
    return cast(list[NativeCensusRow], raw['samples'])


class NativePacketTests(unittest.TestCase):
    def test_all_reviewed_native_sources_match_complete_record_headers_and_payloads(self) -> None:
        for index, row in enumerate(native_samples()):
            raw = (FIXTURES / f'native-{index}.db').read_bytes()
            key, value = checked_payloads(row, raw)
            self.assertEqual(key, (FIXTURES / f'native-{index}.key.bin').read_bytes())
            self.assertEqual(value, (FIXTURES / f'native-{index}.value.bin').read_bytes())
            record = native_record(row, raw)
            count = len(set(row['key_page_ids'] + row['value_page_ids']))
            self.assertEqual(struct.unpack_from('<IIIQQI', record),
                (row['ordinal'], row['extent'], len(key), len(value), row['cell'][-1], count))
            self.assertTrue(record.endswith(key + value))

    def test_changed_source_and_early_end_cannot_create_a_native_expected_payload(self) -> None:
        row = native_samples()[0]
        raw = bytearray((FIXTURES / 'native-0.db').read_bytes())
        raw[-1] ^= 1
        with self.assertRaises(AssertionError):
            checked_payloads(row, bytes(raw))
        original = (FIXTURES / 'native-0.db').read_bytes()
        with self.assertRaises(AssertionError):
            native_value(original, 4096, row['cell'][-1], 9000)

    def test_cycles_and_out_of_extent_checkpoint_reject_incomplete_value_bytes(self) -> None:
        row = native_samples()[2]
        raw = bytearray((FIXTURES / 'native-2.db').read_bytes())
        struct.pack_into('>Q', raw, 5 * 4096, 4)
        with self.assertRaises(AssertionError):
            native_value(bytes(raw), 4096, 4, 12000)
        struct.pack_into('>H', raw, 4 * 4096 + 16, 4097)
        with self.assertRaises(AssertionError):
            native_value(bytes(raw), 4096, 4, row['cell'][3])

    def test_injected_temporary_image_root_builds_exactly_complete_record_frames(self) -> None:
        row = native_samples()[0]
        raw = (FIXTURES / 'native-0.db').read_bytes()
        with tempfile.TemporaryDirectory() as directory:
            fake = FakeNativeImages(Path(directory))
            fake.insert(row, raw)
            packet = native_packet([row] * 5140, fake.root)
        record = native_record(row, raw)
        self.assertEqual(packet, b'BOV1\x14\x14\0\0' +
                         (struct.pack('<I', len(record)) + record) * 5140)
        with self.assertRaises(AssertionError):
            native_packet([], Path('unused-root'))


if __name__ == '__main__':
    unittest.main()
