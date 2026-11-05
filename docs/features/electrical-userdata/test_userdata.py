"""Check frozen SQL, lexical, native and identity evidence without installed files."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import unittest
from freeze_userdata import ROOT, case_projection, identity_cases, owned_cases


def packets() -> tuple[list[tuple[str,bytes,bytes]],bytes]:
    raw=gzip.decompress((ROOT / "tests/fixtures/electrical-userdata/queries.bin.gz").read_bytes())
    assert raw[:4]==b"EUQ1"
    count,=struct.unpack_from("<I",raw,4); offset=8
    cases=[]
    for _ in range(count):
        fields=[]
        for _ in range(3):
            length,=struct.unpack_from("<I",raw,offset);offset+=4
            fields.append(raw[offset:offset+length]);offset+=length
        cases.append((fields[0].decode("utf-8"),fields[1],fields[2]))
    return cases,raw[offset:]


def query_keys(encoded: bytes) -> list[tuple[int,int]]:
    count,=struct.unpack_from("<I",encoded,0);offset=4
    keys=[]
    for _ in range(count):
        keys.append(struct.unpack_from("<ii",encoded,offset));offset+=8
        length,=struct.unpack_from("<I",encoded,offset);offset+=4+length
    return keys


class ElectricalUserdataFixtures(unittest.TestCase):
    def test_every_archive_projection_hash_and_identity_transition(self) -> None:
        root=ROOT / "tests/fixtures/electrical-userdata"
        manifest=json.loads((root / "integrity.json").read_text(encoding="utf-8"))
        frozen=(root / "queries.bin.gz").read_bytes()
        self.assertEqual(hashlib.sha256(frozen).hexdigest(),manifest["gzip_sha256"])
        self.assertEqual(hashlib.sha256(gzip.decompress(frozen)).hexdigest(),manifest["raw_sha256"])
        cases,identities=packets();self.assertEqual(len(cases),7)
        self.assertEqual(identities,identity_cases())
        self.assertEqual(sum(len(query_keys(expected)) for _,_,expected in cases),78)
        for (name,raw,expected),record in zip(cases,manifest["cases"],strict=True):
            self.assertEqual(name,record["name"])
            self.assertEqual(hashlib.sha256(raw).hexdigest(),record["archive_sha256"])
            self.assertEqual(hashlib.sha256(expected).hexdigest(),record["projection_sha256"])
            self.assertEqual(case_projection(raw,query_keys(expected)),expected,name)

    def test_owned_images_rebuild_from_committed_construction_inputs(self) -> None:
        cases,_=packets()
        for (name,raw,keys),(frozen_name,frozen,expected) in zip(owned_cases(),cases[:3],strict=True):
            self.assertEqual((name,raw),(frozen_name,frozen))
            self.assertEqual(case_projection(raw,keys),expected)


if __name__ == "__main__":
    unittest.main()
