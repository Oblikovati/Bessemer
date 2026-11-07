"""Owned framing and in-memory comparison probes remain deterministic and bounded."""
from dataclasses import dataclass
from pathlib import Path
from types import ModuleType
import hashlib
import json
import struct
import sys
import tempfile
import unittest

sys.path.insert(0,str(Path(__file__).resolve().parents[1]/"docs/features/display-lists"))
from owned_root_inventory import RootFrameCursor,inventory_group,inventory_roots,fixed_context_roots
from predicate_probe import PredicateCollector,ProbedSourceFinder,ProbedSourceLoader,instrument_source,predicate_report


@dataclass
class FakeOwnedManifest:
    name: str = "owned-test"
    payload: bytes = struct.pack("<ii",1,0)

    def write(self, root: Path, layout: str = "standard") -> Path:
        row = {"version":19000,"kind":"part","scenario":0,"bytes":len(self.payload),
            "sha256":hashlib.sha256(self.payload).hexdigest()}
        header = fixture_context(row,layout)
        framed = struct.pack("<I",1)+header+blob(self.payload)+blob(b"trace")
        if layout=="standard":framed+=blob(b"scalar")
        (root/(self.name+".bin")).write_bytes(framed)
        manifest = root/(self.name+".json")
        manifest.write_text(json.dumps({"fixtures":[row],"sha256":hashlib.sha256(framed).hexdigest()}))
        return manifest


def blob(payload: bytes) -> bytes:
    return struct.pack("<I",len(payload))+payload


def fixture_context(row: dict[str,object], layout: str) -> bytes:
    if layout=="file":
        row.update(file_version=547,aggregate_entries=3,objects=2)
        return struct.pack("<IIBBII",19000,547,0,0,3,2)
    if layout=="version":
        row.pop("kind");row.pop("scenario");row["edited"]=False
        return struct.pack("<IB",19000,0)
    if layout=="legacy":
        row.pop("kind");row["two_dimensional"]=True
        return struct.pack("<IBB",19000,1,0)
    return struct.pack("<IBB",19000,0,0)


class OwnedFramingTests(unittest.TestCase):
    def test_cursor_rejects_negative_overrun_truncation_and_trailing_bytes(self) -> None:
        for size in [-1,9]:
            with self.assertRaisesRegex(ValueError,"expected within 2 bytes"):
                RootFrameCursor(b"xx").take(size)
        with self.assertRaises(ValueError):RootFrameCursor(b"x").u32()
        with self.assertRaisesRegex(ValueError,"expected 2"):RootFrameCursor(b"xx").finish()
        cursor=RootFrameCursor(blob(b"abc"))
        self.assertEqual(cursor.blob(),b"abc");cursor.finish()

    def test_all_persisted_root_frame_shapes_preserve_their_explicit_context(self) -> None:
        variants=[("owned-test","standard"),("graphics-references","graphics"),
            ("file-context-test","file"),("version-range","version"),("legacy-buckets","legacy")]
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary)
            for name,layout in variants:
                manifest=FakeOwnedManifest(name).write(root,layout)
                cases=inventory_group(manifest,root)
                self.assertEqual(len(cases),1)
                self.assertEqual(cases[0].kind,"part")
                self.assertEqual(cases[0].file_version,547 if layout=="file" else 19000)
                self.assertEqual(cases[0].context_row()["external_ids"],[])

    def test_inventory_detects_hash_corruption_and_missing_roots(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary)
            manifest=FakeOwnedManifest().write(root)
            self.assertEqual(len(inventory_roots(root,root)),1)
            (root/"owned-test.bin").write_bytes(b"bad")
            with self.assertRaisesRegex(ValueError,"hash .*expected"):
                inventory_group(manifest,root)
            self.assertEqual(inventory_group(root/"missing.json",root),[])


class FixedContextInventoryTests(unittest.TestCase):
    def test_assembly_modes_and_external_ids_keep_their_declared_context(self) -> None:
        payload=b"owned full root"
        row={"bytes":len(payload),"sha256":hashlib.sha256(payload).hexdigest()}
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary)
            (root/"component-mode-7.bin").write_bytes(payload)
            (root/"external-shaded-assembly.bin").write_bytes(payload)
            (root/"component-vectors.json").write_text(json.dumps([dict(row,mode=7)]))
            (root/"shaded-vectors.json").write_text(json.dumps({"fixtures":[dict(row,
                fixture="external-shaded-assembly",external_components=["owned-component"])]}))
            records=fixed_context_roots(root,root)
            self.assertEqual([record.kind for record in records],["assembly","assembly"])
            self.assertEqual(records[0].external_ids,("owned-component",))
            self.assertEqual(records[1].external_ids,())
            self.assertTrue(all(record.version==19000 for record in records))


class ComparisonProbeTests(unittest.TestCase):
    def test_values_and_short_circuit_order_match_the_uninstrumented_function(self) -> None:
        source="def choose(v):\n    return v > 1 and v < 4\n"
        collector=PredicateCollector()
        namespace={"_bessemer_record_predicate":collector.record}
        exec(instrument_source(source,"owned.py","owned",collector),namespace)
        plain={};exec(source,plain)
        for version in [0,2,5]:
            collector.active=(f"root-{version}",version,"part",547)
            self.assertEqual(namespace["choose"](version),plain["choose"](version))
        observations=predicate_report(collector)
        self.assertEqual(len(observations),2)
        self.assertTrue(all(row["both_outcomes"] for row in observations))
        self.assertEqual([sum(state["evaluations"] for state in row["observed"].values())
            for row in observations],[3,2])

    def test_loader_compiles_the_current_source_and_ignores_other_imports(self) -> None:
        collector=PredicateCollector(active=("owned",1,"part",19000))
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary);path=root/"dl_doc.py"
            path.write_text("def check(v):\n    return v < 2\n",encoding="utf-8-sig")
            finder=ProbedSourceFinder(root,collector)
            self.assertIsNone(finder.find_spec("unrelated"))
            self.assertIsNotNone(finder.find_spec("dl_doc"))
            module=ModuleType("dl_doc")
            ProbedSourceLoader("dl_doc",str(path),collector).exec_module(module)
            self.assertTrue(module.check(1))
            self.assertEqual(len(predicate_report(collector)),1)


if __name__=="__main__":
    unittest.main()
