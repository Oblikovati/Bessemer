"""Frozen process-verification frames must retain explicit application context (#39)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
import tempfile
import unittest

sys.path.insert(0,str(Path(__file__).resolve().parents[1]/"docs/features/display-lists"))
from verify_decal_processes import frozen_cases


class FakeContextFixtureGenerator:
    def __init__(self, identities: list[str] | None) -> None:
        self.row = {"version":19000,"kind":"assembly","scenario":0}
        if identities is not None:self.row["external_ids"] = identities
        self.payload = b"owned framing fixture"
        self.frame = struct.pack("<IBBI",19000,1,0,len(self.payload))+self.payload+bytes(8)

    def __call__(self, version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
        assert (version,kind,scenario)==(19000,"assembly",0)
        return self.frame,dict(self.row)

    def write(self, directory: Path) -> tuple[Path,Path]:
        framed = struct.pack("<I",1)+self.frame
        manifest = {"fixtures":[self.row],"sha256":hashlib.sha256(framed).hexdigest()}
        manifest_path,fixture_path = directory/"manifest.json",directory/"fixture.bin"
        manifest_path.write_text(json.dumps(manifest),encoding="utf-8")
        fixture_path.write_bytes(framed)
        return manifest_path,fixture_path


class FrozenFixtureContextTests(unittest.TestCase):
    def verify_context(self, identities: list[str] | None, expected: list[str]) -> None:
        generator = FakeContextFixtureGenerator(identities)
        with tempfile.TemporaryDirectory(prefix="bessemer-fixture-context-") as directory:
            manifest,fixture = generator.write(Path(directory))
            records = frozen_cases(manifest,fixture,generator)
        self.assertEqual(len(records),1)
        self.assertEqual(records[0][0]["external_ids"],expected)
        self.assertEqual(records[0][1],generator.payload)

    def test_preserves_explicit_component_ids_including_case_and_unicode(self) -> None:
        self.verify_context(["Assem1","assem1","component 鋼"],["Assem1","assem1","component 鋼"])

    def test_only_missing_declarations_receive_the_empty_fallback(self) -> None:
        self.verify_context(None,[])
        self.verify_context([],[])


if __name__=="__main__":
    unittest.main()
