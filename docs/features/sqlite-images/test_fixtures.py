"""Validate owned frozen SQLite images without installed application files."""
from __future__ import annotations

import gzip
import hashlib
import io
import json
import struct
import unittest
from freeze_fixtures import FIXTURES, SqlText, owned_cases, projection, value


def read_blob(stream: io.BytesIO) -> bytes:
    length = struct.unpack("<I", stream.read(4))[0]
    payload = stream.read(length)
    assert len(payload) == length
    return payload


def frozen_cases() -> list[tuple[str, bytes, bytes]]:
    raw = gzip.decompress((FIXTURES / "images.bin.gz").read_bytes())
    stream = io.BytesIO(raw)
    assert stream.read(4) == b"SQI1"
    count = struct.unpack("<I", stream.read(4))[0]
    cases = [(read_blob(stream).decode("utf-8"), read_blob(stream), read_blob(stream)) for _ in range(count)]
    assert stream.read() == b""
    return cases


class FrozenSqliteImages(unittest.TestCase):
    def test_packet_integrity_and_every_independent_schema_row_projection(self) -> None:
        frozen = (FIXTURES / "images.bin.gz").read_bytes()
        manifest = json.loads((FIXTURES / "integrity.json").read_text(encoding="utf-8"))
        raw = gzip.decompress(frozen)
        self.assertEqual(hashlib.sha256(frozen).hexdigest(), manifest["gzip_sha256"])
        self.assertEqual(hashlib.sha256(raw).hexdigest(), manifest["raw_sha256"])
        self.assertEqual((len(frozen), len(raw)), (manifest["gzip_bytes"], manifest["raw_bytes"]))
        cases = frozen_cases()
        self.assertEqual(len(cases), 18)
        for (name, image, expected), record in zip(cases, manifest["cases"], strict=True):
            self.assertEqual(name, record["name"])
            self.assertEqual(hashlib.sha256(image).hexdigest(), record["image_sha256"])
            self.assertEqual(hashlib.sha256(expected).hexdigest(), record["projection_sha256"])
            self.assertEqual(projection(image), expected, name)

    def test_owned_reconstruction_and_value_tags_remain_distinct(self) -> None:
        frozen = frozen_cases()
        for index, (name, image) in enumerate(owned_cases()):
            self.assertEqual(name, frozen[index][0])
            self.assertEqual(projection(image), frozen[index][2])
        self.assertEqual(value(None), b"\0")
        self.assertEqual(value(0), b"\1" + bytes(8))
        self.assertEqual(value(0.0), b"\2" + bytes(8))
        self.assertEqual(value(""), b"\3" + bytes(4))
        self.assertEqual(value(SqlText(b"\x80\xff")), b"\3\x02\0\0\0\x80\xff")
        self.assertEqual(value(b""), b"\4" + bytes(4))


if __name__ == "__main__":
    unittest.main()
