"""Check native stream spans independently of the owned overflow constructor."""
import hashlib
import json
from pathlib import Path
import struct
import unittest
from writer_pages import OverflowImage
from test_writer_pages import reconstruct_key, reconstruct_value

ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / 'tests/fixtures/database-overflow'
CASES = (
    (0, 3762, 36, 304, (16,), (65554, 65590), ((65590, 65894),)),
    (1, 12, 3, 646, (3,), (12306, 12309), ((12309, 12955),)),
    (2, 38, 36, 4380, (4, 5), (16402, 16438), ((16438, 20480), (20488, 20826))),
)


def native_image(index: int, page_ids: tuple[int, ...]) -> OverflowImage:
    """Load explicit reviewed page identities, without searching; e.g. native_image(2,(4,5))."""
    raw = (FIXTURES / f'native-{index}.db').read_bytes()
    key = (FIXTURES / f'native-{index}.key.bin').read_bytes()
    value = (FIXTURES / f'native-{index}.value.bin').read_bytes()
    pages = tuple((identity, raw[identity * 4096:(identity + 1) * 4096]) for identity in page_ids)
    return OverflowImage(4096, key, value, raw[8192:12288], pages, False)


class NativePageTests(unittest.TestCase):
    def test_reviewed_literal_native_spans_match_complete_expected_values(self) -> None:
        for index, offset, key_length, value_length, page_ids, key_span, value_spans in CASES:
            raw = (FIXTURES / f'native-{index}.db').read_bytes()
            image = native_image(index, page_ids)
            self.assertEqual(raw[slice(*key_span)], image.key)
            self.assertEqual(b''.join(raw[a:b] for a, b in value_spans), image.value)
            self.assertEqual((len(image.key), len(image.value)), (key_length, value_length))
            cell = struct.unpack_from('>IIQHQ', image.bucket, offset)
            self.assertEqual(cell[1:3], (key_length, value_length))
            self.assertEqual(cell[-1], page_ids[0])
            self.assertEqual(reconstruct_key(image), image.key)
            self.assertEqual(reconstruct_value(image), image.value)

    def test_all_native_snapshots_match_recorded_stream_and_payload_hashes(self) -> None:
        path = Path(__file__).with_name('native-sample-reference.json')
        reference = json.loads(path.read_text(encoding='utf-8'))
        self.assertEqual(len(reference['samples']), 3)
        for row in reference['samples']:
            for name, expected in ((row['fixture'], row['database_sha256']),
                                   (row['expected_key_fixture'], row['key_sha256']),
                                   (row['expected_value_fixture'], row['value_sha256'])):
                self.assertEqual(hashlib.sha256((FIXTURES / name).read_bytes()).hexdigest(), expected)


if __name__ == '__main__':
    unittest.main()
