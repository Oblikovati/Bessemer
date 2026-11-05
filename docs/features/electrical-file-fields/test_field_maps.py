"""Verify complete native-order field maps, owned missing columns and fixture hashes."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import unittest
from freeze_field_maps import ROOT, FIELDS, owned_archive, projection


def packets() -> list[tuple[str,bytes,str,bytes]]:
    raw=gzip.decompress((ROOT / "tests/fixtures/electrical-file-fields/maps.bin.gz").read_bytes())
    assert raw[:4]==b"EFF1"
    count,=struct.unpack_from("<I",raw,4);offset=8
    cases=[]
    for _ in range(count):
        fields=[]
        for _ in range(4):
            length,=struct.unpack_from("<I",raw,offset);offset+=4
            fields.append(raw[offset:offset+length]);offset+=length
        cases.append((fields[0].decode("utf-8"),fields[1],fields[2].decode("utf-8"),fields[3]))
    assert offset==len(raw)
    return cases


class ElectricalFieldMaps(unittest.TestCase):
    def test_every_complete_projection_and_all_missing_columns(self) -> None:
        root=ROOT / "tests/fixtures/electrical-file-fields"
        frozen=(root / "maps.bin.gz").read_bytes()
        report=json.loads((root / "integrity.json").read_text(encoding="utf-8"))
        self.assertEqual(hashlib.sha256(frozen).hexdigest(),report["gzip_sha256"])
        self.assertEqual(hashlib.sha256(gzip.decompress(frozen)).hexdigest(),report["raw_sha256"])
        cases=packets();self.assertEqual(len(cases),44)
        for (name,raw,missing,expected),record in zip(cases,report["cases"],strict=True):
            self.assertEqual(name,record["name"])
            self.assertEqual(hashlib.sha256(raw).hexdigest(),record["archive_sha256"])
            self.assertEqual(hashlib.sha256(expected).hexdigest(),record["projection_sha256"])
            self.assertEqual(projection(raw),(missing or None,expected),name)
        self.assertEqual({missing for _,_,missing,_ in cases if missing},{field["name"] for field in FIELDS})

    def test_owned_reconstruction_preserves_logical_values_across_engine_versions(self) -> None:
        cases=packets()
        self.assertEqual(projection(owned_archive(None,False)),projection(cases[0][1]))
        self.assertEqual(projection(owned_archive(None,True)),projection(cases[1][1]))
        for field,(_,raw,missing,_) in zip(FIELDS,cases[2:40],strict=True):
            self.assertEqual(missing,field["name"])
            self.assertEqual(projection(owned_archive(field["name"],False)),projection(raw))

    def test_all_native_byte_bindings_retain_explicit_static_scope(self) -> None:
        proof=json.loads((ROOT / "docs/features/electrical-file-fields/native-byte-verification.json").read_text(encoding="utf-8"))
        self.assertFalse(proof["native_execution"])
        self.assertEqual(proof["sha256"],"a7162b856edf13280dab542fd5b7feb1c377b659c5cad2ef910bc3cebea33299")
        self.assertEqual(len(proof["checked_ranges"]),82)
        self.assertTrue(all(record["bytes"]>0 and len(record["sha256"])==64 for record in proof["checked_ranges"]))


if __name__=="__main__":
    unittest.main()
