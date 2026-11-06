"""Check owned write expectations against committed independent SQLite operations."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import unittest
from freeze_owned_writes import ROOT,cases,expected_image,logical_snapshot


def packets()->list[tuple[str,bytes,str,bool,bytes]]:
    raw=gzip.decompress((ROOT / "tests/fixtures/electrical-userdata-write/owned.bin.gz").read_bytes())
    assert raw[:4]==b"EUW1"
    count,=struct.unpack_from("<I",raw,4);offset=8;entries=[]
    for _ in range(count):
        fields=[]
        for _ in range(3):
            length,=struct.unpack_from("<I",raw,offset);offset+=4
            fields.append(raw[offset:offset+length]);offset+=length
        success=raw[offset]==1;offset+=1
        length,=struct.unpack_from("<I",raw,offset);offset+=4
        expected=raw[offset:offset+length];offset+=length
        entries.append((fields[0].decode("utf-8"),fields[1],fields[2].decode("utf-8"),success,expected))
    assert offset==len(raw)
    return entries


class OwnedWriteFixtures(unittest.TestCase):
    def test_packet_hashes_and_fifteen_explicit_acceptance_cases(self)->None:
        root=ROOT / "tests/fixtures/electrical-userdata-write";frozen=(root / "owned.bin.gz").read_bytes()
        report=json.loads((root / "integrity.json").read_text(encoding="utf-8"))
        self.assertEqual(hashlib.sha256(frozen).hexdigest(),report["gzip_sha256"])
        self.assertEqual(hashlib.sha256(gzip.decompress(frozen)).hexdigest(),report["raw_sha256"])
        entries=packets();self.assertEqual(len(entries),15)
        for (name,raw,_,success,expected),record in zip(entries,report["cases"],strict=True):
            self.assertEqual((name,success),(record["name"],record["success"]))
            self.assertEqual(hashlib.sha256(raw).hexdigest(),record["archive_sha256"])
            self.assertEqual(hashlib.sha256(expected).hexdigest(),record["expected_sha256"])

    def test_successful_expectations_rebuild_logically_across_engine_versions(self)->None:
        for entry,record in zip(cases(),packets(),strict=True):
            self.assertEqual((entry["name"],entry["success"]),(record[0],record[3]))
            if entry["success"]:
                image=expected_image(entry["image"],entry["replacement"])
                self.assertEqual(logical_snapshot(image),record[4])


if __name__=="__main__":unittest.main()
