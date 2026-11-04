"""Validate actual Rust comparison outputs against unchanged Source (#195)."""
from __future__ import annotations
from pathlib import Path
from typing import BinaryIO, Iterator, NamedTuple, TypedDict
import hashlib
import os
import struct
import unittest
import inspectionrecords
from freeze_identity import metadata_inputs, metadata_xml, owned_inputs

WORKSPACE = Path(__file__).resolve().parents[3]


class IdentityValues(TypedDict):
    stored_size: str | None
    actual_size: int
    size_matches: bool
    stored_md5: str | None
    actual_md5: str
    md5_matches: bool


class ResultRow(NamedTuple):
    identity: tuple[int, int]
    xml: bytes
    values: IdentityValues
    contract: tuple[int, ...]


class PacketReader:
    def __init__(self, stream: BinaryIO) -> None:
        self.stream = stream

    def exact(self, length: int) -> bytes:
        raw = self.stream.read(length)
        assert len(raw) == length, f'{len(raw)} bytes; expected complete {length}-byte result field'
        return raw

    def word(self) -> int:
        return struct.unpack('<I', self.exact(4))[0]

    def count64(self) -> int:
        return struct.unpack('<Q', self.exact(8))[0]

    def blob(self) -> bytes:
        length = self.word()
        assert length <= 1_000_000, f'blob length {length}; expected <=1000000'
        return self.exact(length)

    def optional_text(self) -> str | None:
        length = self.word()
        if length == 0xffffffff:
            return None
        assert length <= 1_000_000, f'text length {length}; expected <=1000000'
        return self.exact(length).decode('utf-8')

    def values(self) -> IdentityValues:
        size = self.optional_text()
        actual_size = self.count64()
        md5 = self.optional_text()
        actual_md5 = self.blob().decode('utf-8')
        flags = self.exact(2)
        assert all(flag in (0, 1) for flag in flags)
        return dict(stored_size=size, actual_size=actual_size, stored_md5=md5,
                    actual_md5=actual_md5, size_matches=bool(flags[0]), md5_matches=bool(flags[1]))

    def contract(self) -> tuple[int, ...]:
        completion, mode, diagnostics = self.exact(3)
        length, unread = self.count64(), self.count64()
        histories, parents = self.word(), self.word()
        copied, identity = self.exact(2)
        return completion, mode, diagnostics, length, unread, histories, parents, copied, identity


def result_rows(path: Path) -> Iterator[ResultRow]:
    with path.open('rb') as stream:
        reader = PacketReader(stream)
        assert reader.exact(4) == b'ISCR'
        count = reader.word()
        assert count == 197, f'{count} results; expected 197'
        for _ in range(count):
            identity = reader.word(), reader.word()
            yield ResultRow(identity, reader.blob(), reader.values(), reader.contract())
        assert stream.read(1) == b'', 'result packet suffix; expected exact EOF'


def canonical_xml(source: bytes) -> bytes:
    checksum = hashlib.md5(source).hexdigest()
    return (f"<i:SOLIDWORKSINSPECTION xmlns:i='http://www.extensiblecad.com'><INSPECTION_SHEET><DOCUMENT><DOCUMENT Uid='D' SourcePath='Z:/never-open/from-stored-path.pdf' ModelSize='{len(source)}' ModelMD5HashData='{checksum}'/></DOCUMENT></INSPECTION_SHEET></i:SOLIDWORKSINSPECTION>").encode()


def expected_cases() -> dict[tuple[int, int], tuple[bytes, bytes]]:
    cases = {(0, index): (canonical_xml(raw), raw) for index, (_, raw) in enumerate(owned_inputs())}
    cases.update({(1, index): (metadata_xml(size, md5), b'abc')
                  for index, (_, size, md5) in enumerate(metadata_inputs())})
    xml = (WORKSPACE / 'tests/fixtures/standalone-inspection/historical.xml').read_bytes()
    source = Path(os.environ['BESSEMER_INSPECTION_PDF_SOURCE']).read_bytes()
    assert hashlib.sha256(source).hexdigest() == 'e7769d7679d8e845849e733af21e36013bd88922afae77a7193840dc38263591'
    variants = source, source[:-1] + bytes((source[-1] ^ 1,)), source[:-1]
    cases.update({(2, index): (xml, raw) for index, raw in enumerate(variants)})
    assert len(cases) == 197
    return cases


def packet_pair() -> tuple[Path, Path]:
    return (Path(os.environ['BESSEMER_INSPECTION_IDENTITY_FIRST']),
            Path(os.environ['BESSEMER_INSPECTION_IDENTITY_SECOND']))


class ActualIdentityResultTests(unittest.TestCase):
    def test_all_actual_rust_results_equal_original_source_and_complete_contract(self) -> None:
        cases = expected_cases()
        seen: set[tuple[int, int]] = set()
        for row in result_rows(packet_pair()[0]):
            self.assertNotIn(row.identity, seen)
            xml, source = cases[row.identity]
            self.assertEqual(row.xml, xml)
            document = inspectionrecords.read(xml)['sheets'][0]['documents'][0]
            expected = inspectionrecords.compare_document_source(document, source)
            self.assertEqual(row.values, expected, row.identity)
            self.assertEqual(row.contract, (0, 0, 7, len(source), 0, 1, 1, 1, 1))
            seen.add(row.identity)
        self.assertEqual(seen, set(cases))

    def test_fresh_process_packets_are_exactly_identical(self) -> None:
        first, second = packet_pair()
        with first.open('rb') as left, second.open('rb') as right:
            while True:
                block = left.read(1024 * 1024)
                self.assertEqual(block, right.read(len(block)))
                if not block:
                    self.assertEqual(right.read(1), b'')
                    break
